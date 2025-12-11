use anchor_lang::prelude::*;
use anchor_lang::solana_program::bpf_loader_upgradeable;
use crate::states::upgrade_authority::UpgradeAuthority;
use crate::errors::BridgeUpgradeError;
use crate::errors::AdminError;

#[derive(Accounts)]
pub struct TransferUpgradeAuthority<'info> {
    /// CHECK: Program account address validation is handled by BPF upgrade loader 
    pub program: UncheckedAccount<'info>,

    #[account(
        mut,
        address = crate::admin::id() @ AdminError::NotApproved
    )]
    pub old_upgrade_authority: Signer<'info>,

    /// New upgrade authority holder, managed by PDA
    pub new_upgrade_authority: Box<Account<'info, UpgradeAuthority>>,

    /// Program data account that stores the program's executable data
    /// CHECK: Program data account is validated by BPF upgrade loader
    #[account(mut)]
    pub program_data: UncheckedAccount<'info>,
    /// BPF upgrade loader program for handling upgrade authority transfer
    /// CHECK: Loader validation
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