use anchor_lang::prelude::*;
use crate::states::committee::{Committee, COMMITTEE_SEED};
use crate::states::upgrade_authority::{UpgradeAuthority, UPGRADE_AUTHORITY_SEED};
use crate::util::BPF_LOADER_UPGRADEABLE_ID;
use crate::errors::BridgeUpgradeError;
use crate::errors::AdminError;
use crate::errors::BridgeError;

#[derive(Accounts)]
pub struct TransferUpgradeAuthority<'info> {
    /// CHECK: validated via address + program_data constraint
    #[account(address = crate::id())]
    pub program: UncheckedAccount<'info>,

    #[account(
        mut,
        address = crate::admin::id() @ AdminError::NotApproved
    )]
    pub old_upgrade_authority: Signer<'info>,

    #[account(
        seeds = [COMMITTEE_SEED.as_bytes(), (crate::util::bridge_config_pda().0.as_ref())],
        bump = committee.load()?.bump[0],
    )]
    pub committee: AccountLoader<'info, Committee>,

    /// PDA 管理升级
    #[account(
        seeds = [UPGRADE_AUTHORITY_SEED.as_bytes(), (committee.key().as_ref())],
        bump = new_upgrade_authority.bump[0],
        constraint = new_upgrade_authority.committee
            == committee.key()
            @ BridgeError::InvalidCommittee
    )]
    pub new_upgrade_authority: Box<Account<'info, UpgradeAuthority>>,
    #[account(
        mut,
        constraint = program_data.key()
            == solana_loader_v3_interface::get_program_data_address(&program.key())
            @ BridgeUpgradeError::InvalidProgramData,
        constraint = program_data.upgrade_authority_address == Some(old_upgrade_authority.key())
            @ BridgeUpgradeError::InvalidProgramData,
    )]
    pub program_data: Box<Account<'info, ProgramData>>,

    /// CHECK: kept to explicitly pin the loader program id
    #[account(address = BPF_LOADER_UPGRADEABLE_ID)]
    pub bpf_loader_upgradeable: UncheckedAccount<'info>,
}


pub fn transfer_upgrade_authority(
    ctx: Context<TransferUpgradeAuthority>,
) -> Result<()> {

  let ix = solana_loader_v3_interface::instruction::set_upgrade_authority(
       &ctx.accounts.program.key(),
     &ctx.accounts.old_upgrade_authority.key(),
    Some(&ctx.accounts.new_upgrade_authority.key()), 
    );

   anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.program.to_account_info(),
            ctx.accounts.old_upgrade_authority.to_account_info(), 
            ctx.accounts.new_upgrade_authority.to_account_info(), 
            ctx.accounts.program_data.to_account_info(),           
            ctx.accounts.bpf_loader_upgradeable.to_account_info(),    
        ],
    )?;
    Ok(())
}
