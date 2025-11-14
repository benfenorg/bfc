use anchor_lang::prelude::*;
use crate::states::committee::Committee;
use crate::states::upgrade_authority::UpgradeAuthority;
use crate::states::message::{ create_message,decode_upgrade_payload};
use crate::errors::BridgeUpgradeError;
use crate::errors::BridgeConfigError;
use solana_program::program::invoke_signed;
use std::convert::Into;
use crate::events::ProgramUpgradeEvent;


#[derive(Accounts)]
pub struct UpgradeProgram<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub spill: SystemAccount<'info>,

    #[account(
       mut,
       constraint = upgrade_authority.enabled == true @ BridgeUpgradeError::UnauthorizedUpgrade,
    )]
    pub upgrade_authority: Box<Account<'info, UpgradeAuthority>>,


    #[account(mut)]
    pub committee: AccountLoader<'info, Committee>,

    /// Buffer account containing the new program code
    /// CHECK: Buffer account validation is handled by BPF loader
    #[account(mut)]
    pub buffer: UncheckedAccount<'info>,

    /// Program account to be upgraded
    /// CHECK: Program account validation is handled by BPF loader
    #[account(mut)]
    pub program: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key()),
    )]
    pub program_data: Box<Account<'info, ProgramData>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
  
    /// BPF loader program for upgrade operations
    /// CHECK: BPF loader program validation
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
    ctx.accounts.upgrade_authority.can_upgrade(&ctx.accounts.clock)?;

    // create message and verify signatures
    let message = create_message(message_type, version, nonce, chain_id, payload.clone());
   
    let committee = &mut ctx.accounts.committee.load_mut()?;
    committee.verify_signatures(signatures, &message)?;

    // decode upgrade payload 
    let (proxy, implementation, upgrade_version) = decode_upgrade_payload(&payload)?;

    // Verify version number - new version must be greater than current
    require!(
        upgrade_version > ctx.accounts.upgrade_authority.current_version,
        BridgeUpgradeError::ProgramVersionMismatch
    );
    
    // Verify proxy and implementation addresses match expected accounts
    require!(proxy == ctx.accounts.program.key(), BridgeConfigError::InvalidConfigPubkey);
    require!(implementation == ctx.accounts.buffer.key(), BridgeConfigError::InvalidConfigPubkey);

   // Verify program data account has correct upgrade authority
    require!(
        ctx.accounts.program_data.upgrade_authority_address
            == Some(ctx.accounts.upgrade_authority.key()),
        BridgeUpgradeError::InvalidProgramData
    );

    // Verify buffer is owned by the upgrade authority
    require!(
        *ctx.accounts.buffer.owner == ctx.accounts.upgrade_authority.key(),
        BridgeUpgradeError::InvalidBufferOwner
    );

    // Prepare upgrade instruction
    let upgrade_ix = solana_program::bpf_loader_upgradeable::upgrade(
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
    invoke_signed(&upgrade_ix, &account_infos, &[&ctx.accounts.upgrade_authority.seeds()])?;

    let old_version = ctx.accounts.upgrade_authority.current_version;
    ctx.accounts.upgrade_authority.update_version(upgrade_version, &ctx.accounts.clock);

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
