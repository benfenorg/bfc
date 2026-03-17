use crate::errors::{BridgeConfigError, BridgeError, BridgeUpgradeError};
use crate::states::committee::Committee;
use crate::states::message::{EXTEND_PROGRAM, create_message, decode_extend_payload};
use crate::states::upgrade_authority::UpgradeAuthority;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::bpf_loader_upgradeable;
use crate::errors::MessageError;
use crate::instructions::verify_message::verify_bridge_signature;
use crate::states::message_verifier::MessageVerifier;
use crate::states::message_config::{MessageConfig,MESSAGE_CONFIG_SEED};
use crate::states::{
    bridge_config::*,
};

use std::ops::DerefMut;


#[derive(Accounts)]
pub struct ExtendProgram<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        address = crate::util::bridge_config_pda().0 @ BridgeConfigError::InvalidConfigPubkey
    )]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    #[account(
        mut,
        constraint = upgrade_authority.committee == committee.key() @ BridgeError::InvalidCommittee,
        address = crate::util::committee_pda(&bridge_config.key()).0 @ BridgeError::InvalidCommittee
    )]
    pub committee: AccountLoader<'info, Committee>,


    #[account(
        mut,
        address = crate::util::message_verifier_pda(&committee.key()).0 @ MessageError::InvalidMessageVerifier
    )]
    pub verifier: AccountLoader<'info, MessageVerifier>,
    #[account(
        init_if_needed,
        payer = payer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[EXTEND_PROGRAM],
            verifier.key().as_ref()
        ],
        bump
    )]
    pub message_config: Box<Account<'info, MessageConfig>>,

    #[account(
        mut,
        constraint = upgrade_authority.enabled == true @ BridgeUpgradeError::UnauthorizedUpgrade,
        address = crate::util::upgrade_authority_pda(&committee.key()).0 @ BridgeUpgradeError::UnauthorizedUpgrade
    )]
    pub upgrade_authority: Box<Account<'info, UpgradeAuthority>>,
    /// CHECK: validated via program_data constraint and CPI
    #[account(
        mut,
        address = crate::id(),
        constraint = program.owner == &bpf_loader_upgradeable::ID @ BridgeUpgradeError::InvalidProgramData
    )]
    pub program: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key())
            @ BridgeUpgradeError::InvalidProgramData,
        constraint = program_data.key()
            == solana_loader_v3_interface::get_program_data_address(&program.key())
            @ BridgeUpgradeError::InvalidProgramData,
    )]
    pub program_data: Box<Account<'info, ProgramData>>,

    pub system_program: Program<'info, System>,
    /// CHECK: kept to explicitly pin the loader program id
    #[account(address = bpf_loader_upgradeable::ID)]
    pub bpf_loader: UncheckedAccount<'info>,
}


pub fn extend_program_space_with_signatures(
    ctx: Context<ExtendProgram>,
    message_type: u8,
    version: u8,
    nonce: u64,
    chain_id: u8,
    payload: Vec<u8>,
    signatures: Vec<Vec<u8>>,
) -> Result<()> {
    let mut message_config = ctx.accounts.message_config.deref_mut();
    let mut bridge_config = ctx.accounts.bridge_config.load_mut()?;
    let mut verifier = ctx.accounts.verifier.load_mut()?;
    let mut committee = ctx.accounts.committee.load_mut()?;
    let message = create_message(message_type, version, nonce, chain_id, payload.clone());

    require!(message_type==EXTEND_PROGRAM,MessageError::InvalidMessageType);

    verify_bridge_signature(
        &mut message_config,
        &mut bridge_config,
        &mut verifier,
        &mut committee,
        &message,
        signatures
    )?; 

    let (program, additional_bytes) = decode_extend_payload(&payload)?;
    require!(
        program == ctx.accounts.program.key(),
        BridgeConfigError::InvalidConfigPubkey
    );
    let program_address = ctx.accounts.program.key();
    let expected_program_data = solana_loader_v3_interface::get_program_data_address(&program_address);

    require!(
        expected_program_data == ctx.accounts.program_data.key().to_bytes().into(),
        BridgeConfigError::InvalidConfigPubkey
    );

    let ix_data = solana_loader_v3_interface::instruction::extend_program_checked(
        &program_address,
        &ctx.accounts.upgrade_authority.key().to_bytes().into(),
        Some(&ctx.accounts.payer.key().to_bytes().into()),
        additional_bytes,
    );
    let account_infos = vec![
        ctx.accounts.program_data.to_account_info(),
        ctx.accounts.program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.upgrade_authority.to_account_info(),
    ];

    anchor_lang::solana_program::program::invoke_signed(
        &ix_data,
        &account_infos,
        &[&ctx.accounts.upgrade_authority.seeds()],
    )?;

    Ok(())
}
