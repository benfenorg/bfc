use anchor_lang::prelude::*;

use crate::states::chain_limit::{ChainLimit,CHAIN_LIMIT_SEED};
// use crate::states::bridge_limit::{BridgeLimiter,BRIDGE_LIMIT_SEED};
use crate::states::bridge_config::BridgeConfig;
use crate::states::committee::Committee;
use crate::errors::AdminError;



#[derive(Accounts)]
#[instruction(chain_id:u8)]
pub struct InitializeBridgeLimiter<'info> {
    #[account(
        mut,
        address = crate::admin::id() @ AdminError::NotApproved
    )]
    pub payer: Signer<'info>,


    #[account(
        init,
        payer = payer,
        space = ChainLimit::SPACE,
        seeds = [CHAIN_LIMIT_SEED.as_bytes(),&[chain_id],(bridge_config.key().as_ref())],
        bump,
    )]
    pub chain_limit: AccountLoader<'info, ChainLimit>,

    #[account(mut)]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    pub committee: AccountLoader<'info, Committee>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_bridge_limiter(
    ctx: Context<InitializeBridgeLimiter>,
    chain_id: u8,
    limit: u64,
    min_usd_limit: u64,
    max_usd_limit: u64,
) -> Result<()> {
    let mut chain_limit = ctx.accounts.chain_limit.load_init()?;
   

    chain_limit.initialize(
        ctx.bumps.chain_limit,
        ctx.accounts.bridge_config.key(),
        chain_id,
        min_usd_limit,
        max_usd_limit,
        limit
    );
    ctx.accounts.bridge_config.load_mut()?.add_target_chain(chain_id)?;
    Ok(())
}



