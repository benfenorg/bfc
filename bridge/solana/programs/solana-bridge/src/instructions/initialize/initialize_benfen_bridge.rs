use anchor_lang::prelude::*;
use crate::states::benfen_bridge::*;
use crate::states::bridge_config::BridgeConfig;
use crate::states::committee::Committee;
// use crate::states::vault::{UniversalVault,UNIVERSAL_VAULT_SEED};
use crate::errors::AdminError;

/// Initialize the BenfenBridge account
#[derive(Accounts)]
pub struct InitializeBenfenBridge<'info> {
    #[account(
        mut,
        // address = crate::admin::id() @ AdminError::NotApproved
    )]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = BenfenBridge::SPACE,
        seeds = [BENFEN_BRIDGE_SEED.as_bytes(),committee.key().as_ref()],
        bump
    )]
    pub bridge: Account<'info, BenfenBridge>,


    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    pub committee: AccountLoader<'info, Committee>,
     
    pub system_program: Program<'info, System>,
}

pub fn initialize_benfen_bridge(
    ctx: Context<InitializeBenfenBridge>,
) -> Result<()> {
    let bridge = &mut ctx.accounts.bridge;
    
    let committee_key = ctx.accounts.committee.key();
    let config_key = ctx.accounts.bridge_config.key();
    
    bridge.initialize(
        committee_key,
        config_key,
        ctx.bumps.bridge,
    )?;

    Ok(())
}
