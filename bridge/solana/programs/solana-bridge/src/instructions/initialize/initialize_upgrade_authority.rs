use anchor_lang::prelude::*;
use crate::states::upgrade_authority::{UpgradeAuthority, UPGRADE_AUTHORITY_SEED};
use crate::states::committee::Committee;
use crate::errors::AdminError;
use crate::errors::BridgeCommitteeError;


#[derive(Accounts)]
pub struct InitializeUpgradeAuthority<'info> {

    #[account(
        mut,
        // address = crate::admin::id() @ AdminError::NotApproved
    )]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        space = UpgradeAuthority::SPACE,
        seeds = [UPGRADE_AUTHORITY_SEED.as_bytes(), committee.key().as_ref()],
        bump,
    )]
    pub upgrade_authority: Account<'info, UpgradeAuthority>,

    #[account(
        constraint = committee.load()?.member_count > 0 @ BridgeCommitteeError::InvalidCommitteeSize
    )]
    pub committee: AccountLoader<'info, Committee>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_upgrade_authority(
    ctx: Context<InitializeUpgradeAuthority>,
    enabled: bool,
) -> Result<()> {
    let upgrade_authority = &mut ctx.accounts.upgrade_authority;
    let bump = ctx.bumps.upgrade_authority;


    
    upgrade_authority.initialize(
        enabled,
        [bump],
        ctx.accounts.committee.key(),
    );
    Ok(())
}

