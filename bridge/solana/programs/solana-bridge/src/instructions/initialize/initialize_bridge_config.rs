use anchor_lang::prelude::*;
use crate::states::bridge_config::{CONFIG_SEED,BridgeConfig};
use crate::errors::AdminError;



#[derive(Accounts)]
pub struct InitializeBridgeConfig<'info> {
    #[account(
        mut,
        address = crate::admin::id() @ AdminError::NotApproved
    )]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = BridgeConfig::LEN,
        seeds = [CONFIG_SEED.as_bytes()],
        bump,
    )]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    pub system_program: Program<'info, System>,
}


pub fn initialize_bridge_config(ctx: Context<InitializeBridgeConfig>, chain_id: u8) -> Result<()> {
    let mut bridge_config = ctx.accounts.bridge_config.load_init()?;
    bridge_config.initialize(ctx.bumps.bridge_config,chain_id)?;
    Ok(())
}

