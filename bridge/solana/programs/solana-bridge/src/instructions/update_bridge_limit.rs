use anchor_lang::prelude::*;

use crate::states::bridge_config::BridgeConfig;
use crate::states::chain_limit::ChainLimit;
use crate::states::message_config::{MessageConfig, MESSAGE_CONFIG_SEED};
use crate::states::message_verifier::MessageVerifier;
use crate::states::message;
use crate::states::committee::Committee;
use crate::instructions::verify_message::verify_bridge_signature;
use std::ops::DerefMut;

use crate::errors::MessageError;
use crate::errors::BridgeLimiterError;
use crate::errors::BridgeConfigError;
use crate::errors::BridgeError;
use crate::events::ChainLimitUpdated;

// use crate::states::committee::Committee;


#[derive(Accounts)]
pub struct UpdateBridgeLimiter<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[message::UPDATE_BRIDGE_LIMIT],
            verifier.key().as_ref()
        ],
        bump
    )]
    pub message_config: Box<Account<'info, MessageConfig>>,

    #[account(
        mut,
        address = crate::util::bridge_config_pda().0 @ BridgeConfigError::InvalidConfigPubkey
    )]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    #[account(
        mut,
        address = crate::util::chain_limit_pda(&bridge_config.key(), chain_limit.load()?.get_chain_id()).0
            @ BridgeLimiterError::InvalidLimiterPubkey,
        constraint = chain_limit.load()?.config == bridge_config.key() @ BridgeLimiterError::InvalidLimiterPubkey,
        constraint = bridge_config.load()?.supported_chains.contains(&chain_limit.load()?.get_chain_id())
            @ BridgeError::UnsupportedCrossToChainId
    )]
    pub chain_limit: AccountLoader<'info, ChainLimit>,

    #[account(
        mut,
        address = crate::util::message_verifier_pda(&committee.key()).0 @ MessageError::InvalidMessageVerifier
    )]
    pub verifier: AccountLoader<'info, MessageVerifier>,


    #[account(
        mut,
        address = crate::util::committee_pda(&bridge_config.key()).0 @ BridgeError::InvalidCommittee
    )]
    pub committee:  AccountLoader<'info, Committee>,

    pub system_program: Program<'info, System>,
}


pub fn update_limit_with_signatures(
    ctx: Context<UpdateBridgeLimiter>, 
    message_type: u8, 
    version: u8,
    nonce: u64,
    chain_id: u8,
    payload: Vec<u8>,  
    signatures: Vec<Vec<u8>>,
) -> Result<()> {
    let mut message_config =  ctx.accounts.message_config.deref_mut();
    let mut bridge_config = ctx.accounts.bridge_config.load_mut()?;
    let mut verifier = ctx.accounts.verifier.load_mut()?;
    let mut committee = ctx.accounts.committee.load_mut()?;
    let  mut chain_limit = ctx.accounts.chain_limit.load_mut()?;

    let message=message::create_message(message_type, version, nonce, chain_id, payload.clone());
    require!(message_type==message::UPDATE_BRIDGE_LIMIT,MessageError::InvalidMessageType);

    verify_bridge_signature(
        &mut message_config,
        &mut bridge_config,
        &mut verifier,
        &mut committee,
        &message,
        signatures
    )?; 
    
    let (source_chain_id, new_limit)=message::decode_update_limit_payload(&payload)?;
    require!(chain_limit.get_chain_id()==source_chain_id, BridgeLimiterError::InvalidChainId);
    require!(bridge_config.supported_chains.contains(&source_chain_id), BridgeLimiterError::UnsupportedChain);
    chain_limit.set_total_limit(new_limit);
    msg!("emit ChainLimitUpdated");
    emit!(ChainLimitUpdated {
        nonce,
        source_chain_id,
        new_limit,
    });
    Ok(())
}
