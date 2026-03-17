use anchor_lang::prelude::*;
use anchor_lang::solana_program::bpf_loader_upgradeable;
use crate::states::committee::Committee;
use crate::states::upgrade_authority::{UpgradeAuthority, UPGRADE_AUTHORITY_SEED};
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
        address = crate::util::committee_pda(&crate::util::bridge_config_pda().0).0
            @ BridgeError::InvalidCommittee
    )]
    pub committee: AccountLoader<'info, Committee>,

    /// PDA 管理升级
    #[account(
        seeds = [UPGRADE_AUTHORITY_SEED.as_bytes(), committee.key().as_ref()],
        bump = new_upgrade_authority.bump[0],
        address = crate::util::upgrade_authority_pda(&committee.key()).0
            @ BridgeUpgradeError::UnauthorizedUpgrade,
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
    #[account(address = bpf_loader_upgradeable::ID)]
    pub bpf_loader_upgradeable: UncheckedAccount<'info>,
}


pub fn transfer_upgrade_authority(
    ctx: Context<TransferUpgradeAuthority>,
) -> Result<()> {

  let ix = bpf_loader_upgradeable::set_upgrade_authority(
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
