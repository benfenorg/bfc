use crate::errors::BridgeConfigError;
use crate::errors::MessageError;
use crate::errors::BridgeUpgradeError;
use crate::events::ProgramUpgradeEvent;
use crate::states::committee::Committee;
use crate::states::message::{UPGRADE_PROGRAM, create_message, decode_upgrade_payload};
use crate::states::message_config::{MessageConfig,MESSAGE_CONFIG_SEED};
use crate::states::message_verifier::MessageVerifier;

use crate::states::upgrade_authority::UpgradeAuthority;
use crate::instructions::verify_message::verify_bridge_signature;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use std::convert::Into;
use crate::states::{
    bridge_config::*
};
use std::ops::DerefMut;

#[derive(Accounts)]
pub struct UpgradeProgram<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    #[account(mut)]
    pub committee: AccountLoader<'info, Committee>,

    #[account(mut)]
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
    )]
    pub upgrade_authority: Box<Account<'info, UpgradeAuthority>>,

    /// CHECK:
    #[account(mut)]
    pub program: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key()),
    )]
    pub program_data: Box<Account<'info, ProgramData>>,
    #[account(mut)]
    pub spill: SystemAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub buffer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,

    /// CHECK:
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
    let mut message_config =  ctx.accounts.message_config.deref_mut();
    let mut bridge_config = ctx.accounts.bridge_config.load_mut()?;
    let mut verifier = ctx.accounts.verifier.load_mut()?;
    let mut committee = ctx.accounts.committee.load_mut()?;

    // create message and verify signatures
    let message = create_message(message_type, version, nonce, chain_id, payload.clone());

    require!(message_type == UPGRADE_PROGRAM, MessageError::InvalidMessageType);

    verify_bridge_signature(
        &mut message_config,
        &mut bridge_config,
        &mut verifier,
        &mut committee,
        &message,
        signatures
    )?; 


    // decode upgrade payload
    let (proxy, implementation, upgrade_version) = decode_upgrade_payload(&payload)?;

    // 验证版本号
    require!(
        upgrade_version > ctx.accounts.upgrade_authority.current_version,
        BridgeUpgradeError::ProgramVersionMismatch
    );

    require!(
        proxy == ctx.accounts.program.key(),
        BridgeConfigError::InvalidConfigPubkey
    );
    require!(
        implementation == ctx.accounts.buffer.key(),
        BridgeConfigError::InvalidConfigPubkey
    );

    require!(
        ctx.accounts.program_data.upgrade_authority_address
            == Some(ctx.accounts.upgrade_authority.key()),
        BridgeUpgradeError::InvalidProgramData
    );

    // require!(*ctx.accounts.buffer.owner == bpf_loader_upgradeable::id(), BridgeUpgradeError::InvalidBufferOwner);
    // let buffer_data = ctx.accounts.buffer.try_borrow_data()?;
    // let state: UpgradeableLoaderState = deserialize(&buffer_data).map_err(|_| BridgeUpgradeError::InvalidBufferOwner)?;
    
    // match state {
    //     UpgradeableLoaderState::Buffer { authority_address } => {
    //         require!(authority_address == Some(ctx.accounts.upgrade_authority.key()), BridgeUpgradeError::InvalidBufferOwner);
    //     }
    //     _ => return Err(BridgeUpgradeError::InvalidBufferOwner.into()),
    // }


    // Ensure the program PDA matches the supplied program data account
    let expected_program_data =
        solana_loader_v3_interface::get_program_data_address(&ctx.accounts.program.key());

    require!(
        expected_program_data == ctx.accounts.program_data.key(),
        BridgeConfigError::InvalidConfigPubkey
    );

    // Prepare upgrade instruction
    let upgrade_ix = solana_loader_v3_interface::instruction::upgrade(
        &ctx.accounts.program.key(),
        &ctx.accounts.buffer.key(),
        &ctx.accounts.upgrade_authority.key(),
        &ctx.accounts.spill.key(),
    );

    // Create account infos for CPI
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

    //Update Program Version And record timestamp
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
