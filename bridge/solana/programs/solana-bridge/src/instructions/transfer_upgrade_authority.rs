use anchor_lang::prelude::*;
use anchor_lang::solana_program::bpf_loader_upgradeable;
use crate::states::upgrade_authority::UpgradeAuthority;
use crate::errors::BridgeUpgradeError;
use crate::errors::AdminError;

#[derive(Accounts)]
pub struct TransferUpgradeAuthority<'info> {
    /// CHECK: Loader  校验
    pub program: UncheckedAccount<'info>,

    #[account(
        mut,
        address = crate::admin::id() @ AdminError::NotApproved
    )]
    pub old_upgrade_authority: Signer<'info>,

    /// PDA 管理升级
    pub new_upgrade_authority: Box<Account<'info, UpgradeAuthority>>,
     /// CHECK:
    #[account(mut)]
    pub program_data: UncheckedAccount<'info>,
    /// CHECK: Loader 校验
    pub bpf_loader_upgradeable: UncheckedAccount<'info>,
}


pub fn transfer_upgrade_authority(
    ctx: Context<TransferUpgradeAuthority>,
) -> Result<()> {

  require!(ctx.accounts.old_upgrade_authority.is_signer, BridgeUpgradeError::MissingSignature);

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