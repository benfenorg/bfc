use crate::errors::BridgeConfigError;
use crate::errors::BridgeError;
use crate::errors::BridgeUpgradeError;
use crate::errors::MessageError;
use crate::events::ProgramUpgradeEvent;
use crate::states::committee::{Committee, COMMITTEE_SEED};
use crate::states::message::{create_message, decode_upgrade_payload, UPGRADE_PROGRAM};
use crate::states::message_config::{MessageConfig, MESSAGE_CONFIG_SEED};
use crate::states::message_verifier::{MessageVerifier, MESSAGE_VERIFIER_SEED};
use crate::util::BPF_LOADER_UPGRADEABLE_ID;
use bincode::deserialize;

use crate::instructions::verify_message::verify_bridge_signature;
use crate::states::bridge_config::*;
use crate::states::upgrade_authority::{UpgradeAuthority, UPGRADE_AUTHORITY_SEED};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use solana_loader_v3_interface::state::UpgradeableLoaderState;
use std::convert::Into;
use std::ops::DerefMut;

#[derive(Accounts)]
pub struct UpgradeProgram<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = bridge_config.load()?.bump[0],
    )]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    #[account(
        mut,
        seeds = [COMMITTEE_SEED.as_bytes(), bridge_config.key().as_ref()],
        bump = committee.load()?.bump[0],
    )]
    pub committee: AccountLoader<'info, Committee>,

    #[account(
        mut,
        seeds = [MESSAGE_VERIFIER_SEED.as_bytes(), committee.key().as_ref()],
        bump = verifier.load()?.bump[0],
    )]
    pub verifier: AccountLoader<'info, MessageVerifier>,

    #[account(
        init_if_needed,
        payer = payer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[UPGRADE_PROGRAM],
            verifier.key().as_ref()
        ],
        bump
    )]
    pub message_config: Box<Account<'info, MessageConfig>>,

    #[account(
       mut,
       constraint = upgrade_authority.enabled == true @ BridgeUpgradeError::UnauthorizedUpgrade,
       constraint = upgrade_authority.committee == committee.key() @ BridgeError::InvalidCommittee,
       seeds = [UPGRADE_AUTHORITY_SEED.as_bytes(), committee.key().as_ref()],
       bump = upgrade_authority.bump[0],
    )]
    pub upgrade_authority: Box<Account<'info, UpgradeAuthority>>,

    /// CHECK: validated by constraints + runtime checks
    #[account(
        mut,
        address = crate::id(),
        constraint = program_data.key() == solana_loader_v3_interface::get_program_data_address(program.key) @ BridgeConfigError::InvalidConfigPubkey
    )]
    pub program: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key()),
    )]
    pub program_data: Box<Account<'info, ProgramData>>,

    #[account(mut)]
    pub spill: SystemAccount<'info>,

    /// CHECK: validated by owner constraint + runtime state parsing
    #[account(
        mut,
        constraint = buffer.owner == &BPF_LOADER_UPGRADEABLE_ID @ BridgeUpgradeError::InvalidBufferOwner
    )]
    pub buffer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,

    /// CHECK: kept to explicitly pin the loader program id
    #[account(address = BPF_LOADER_UPGRADEABLE_ID)]
    pub bpf_loader: UncheckedAccount<'info>,
}

pub fn upgrade_program_with_signatures(
    ctx: Context<UpgradeProgram>,
    message_type: u8,
    version: u8,
    nonce: u64,
    chain_id: u8,
    payload: Vec<u8>,
    signatures: Vec<Vec<u8>>,
) -> Result<()> {
    //
    // ctx.accounts
    //     .upgrade_authority
    //     .can_upgrade(&ctx.accounts.clock)?;

    // -----------------------------
    // 1) Build message + verify signatures
    // -----------------------------
    require!(
        message_type == UPGRADE_PROGRAM,
        MessageError::InvalidMessageType
    );

    // create message and verify signatures
    let message = create_message(message_type, version, nonce, chain_id, payload.clone());

    {
        // Load state accounts
        let mut message_config = ctx.accounts.message_config.deref_mut();
        let mut bridge_config = ctx.accounts.bridge_config.load_mut()?;
        let mut verifier = ctx.accounts.verifier.load_mut()?;
        let mut committee = ctx.accounts.committee.load_mut()?;

        verify_bridge_signature(
            &mut message_config,
            &mut bridge_config,
            &mut verifier,
            &mut committee,
            &message,
            signatures,
        )?;
        // mutable borrows of loader accounts drop here
    }

    // -----------------------------
    // 2) Decode payload and validate parameters
    // -----------------------------
    let (proxy, implementation, upgrade_version) = decode_upgrade_payload(&payload)?;

    // Only allow monotonic version upgrades
    require!(
        upgrade_version > ctx.accounts.upgrade_authority.current_version,
        BridgeUpgradeError::ProgramVersionMismatch
    );

    // Payload must match the provided accounts
    require!(
        proxy == ctx.accounts.program.key(),
        BridgeConfigError::InvalidConfigPubkey
    );
    require!(
        implementation == ctx.accounts.buffer.key(),
        BridgeConfigError::InvalidConfigPubkey
    );

    // Program must be an upgradeable loader program
    require!(
        ctx.accounts.program.owner == &BPF_LOADER_UPGRADEABLE_ID,
        BridgeUpgradeError::InvalidProgramData
    );

    // ProgramData upgrade authority must match our UpgradeAuthority account
    require!(
        ctx.accounts.program_data.upgrade_authority_address
            == Some(ctx.accounts.upgrade_authority.key()),
        BridgeUpgradeError::InvalidProgramData
    );

    // -----------------------------
    // 3) Validate buffer account is a real Buffer and controlled by our authority
    // -----------------------------
    // NOTE: Checking owner alone is not sufficient; we must ensure it is *Buffer* state
    // and the buffer's authority matches our upgrade authority.
    {
        let buffer_data = ctx.accounts.buffer.try_borrow_data()?;
        let state: UpgradeableLoaderState =
            deserialize(&buffer_data).map_err(|_| BridgeUpgradeError::InvalidBufferOwner)?;

        match state {
            UpgradeableLoaderState::Buffer { authority_address } => {
                require!(
                    authority_address == Some(ctx.accounts.upgrade_authority.key()),
                    BridgeUpgradeError::InvalidBufferOwner
                );
            }
            _ => return Err(BridgeUpgradeError::InvalidBufferOwner.into()),
        }
        // Explicitly drop buffer data borrow before CPI
        drop(buffer_data);
    }
    // -----------------------------
    // 4) Invoke loader upgrade instruction via CPI
    // -----------------------------
    let upgrade_ix = solana_loader_v3_interface::instruction::upgrade(
        &ctx.accounts.program.key(),
        &ctx.accounts.buffer.key(),
        &ctx.accounts.upgrade_authority.key(),
        &ctx.accounts.spill.key(),
    );

    // Account metas must match the loader's expected order
    let account_infos = vec![
        ctx.accounts.program_data.to_account_info(),
        ctx.accounts.program.to_account_info(),
        ctx.accounts.buffer.to_account_info(),
        ctx.accounts.spill.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        ctx.accounts.clock.to_account_info(),
        ctx.accounts.upgrade_authority.to_account_info(),
    ];

    // Execute upgrade instruction
    invoke_signed(
        &upgrade_ix,
        &account_infos,
        &[&ctx.accounts.upgrade_authority.seeds()],
    )?;

    // -----------------------------
    // 5) Update local version + emit event
    // -----------------------------
    let old_version = ctx.accounts.upgrade_authority.current_version;
    ctx.accounts
        .upgrade_authority
        .update_version(upgrade_version, &ctx.accounts.clock);

    msg!("emit ProgramUpgradeEvent");
    //emit event
    emit!(ProgramUpgradeEvent {
        nonce,
        program: ctx.accounts.program.key(),
        buffer: ctx.accounts.buffer.key(),
        old_version,
        new_version: upgrade_version,
        timestamp: ctx.accounts.clock.unix_timestamp,
    });

    Ok(())
}
