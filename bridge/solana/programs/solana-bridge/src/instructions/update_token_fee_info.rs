use anchor_lang::prelude::*;
use crate::states::{
    bridge_config::*,
    token_config::*
};
use crate::errors::BridgeTokenError;
use crate::errors::MessageError;
use crate::errors::BridgeLimiterError;
use crate::errors::BridgeConfigError;
use crate::errors::BridgeError;

use crate::states::message_verifier::{MessageVerifier, MESSAGE_VERIFIER_SEED};
use crate::states::message_config::{MessageConfig,MESSAGE_CONFIG_SEED};
use crate::states::message;
use crate::states::chain_limit::{ChainLimit, CHAIN_LIMIT_SEED};

use crate::states::committee::{Committee, COMMITTEE_SEED};
use crate::instructions::verify_message::verify_bridge_signature;
use std::ops::DerefMut;
use crate::events::TokenFeeInfoUpdated;



#[derive(Accounts)]
pub struct UpdateTokenFeeInfo<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[message::UPDATE_TOKEN_FEE_INFO],
            (verifier.key().as_ref())
        ],
        bump
    )]
    pub message_config: Box<Account<'info, MessageConfig>>,

    #[account(
        mut,
        constraint = token_config.load()?.config == bridge_config.key() @ BridgeConfigError::InvalidConfigPubkey,
        constraint = token_config.load()?.chain == chain_limit.key() @ BridgeLimiterError::InvalidLimiterPubkey,
        constraint = token_config.key() == crate::util::token_config_pda(token_config.load()?.token_id).0
            @ BridgeTokenError::InvalidTokenIdNotSupported
    )]
    pub token_config: AccountLoader<'info, TokenConfigAccount>,


    #[account(
        mut,
        seeds = [CHAIN_LIMIT_SEED.as_bytes(), &[chain_limit.load()?.get_chain_id()], (bridge_config.key().as_ref())],
        bump = chain_limit.load()?.bump[0],
        constraint = chain_limit.load()?.config == bridge_config.key() @ BridgeLimiterError::InvalidLimiterPubkey
    )]
    pub chain_limit: AccountLoader<'info, ChainLimit>,

    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = bridge_config.load()?.bump[0],
        constraint = bridge_config.load()?.supported_chains.contains(&chain_limit.load()?.get_chain_id())
            @ BridgeError::UnsupportedCrossToChainId
    )]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    #[account(
        mut,
        seeds = [MESSAGE_VERIFIER_SEED.as_bytes(), (committee.key().as_ref())],
        bump = verifier.load()?.bump[0],
    )]
    pub verifier: AccountLoader<'info, MessageVerifier>,

    #[account(
        mut,
        seeds = [COMMITTEE_SEED.as_bytes(), (bridge_config.key().as_ref())],
        bump = committee.load()?.bump[0],
    )]
    pub committee:  AccountLoader<'info, Committee>,

    pub system_program: Program<'info, System>,
}

pub fn update_token_fee_info_with_signature(
   ctx: Context<UpdateTokenFeeInfo>, 
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
    let chain_limit = ctx.accounts.chain_limit.load_mut()?;

    let mut committee = ctx.accounts.committee.load_mut()?;
    let message=message::create_message(message_type, version, nonce, chain_id, payload.clone());
    require!(message_type==message::UPDATE_TOKEN_FEE_INFO,MessageError::InvalidMessageType);

    verify_bridge_signature(
        &mut message_config,
        &mut bridge_config,
        &mut verifier,
        &mut committee,
        &message,
        signatures
    )?; 
    //(sending_chain_id, token_id, mode, fee_value, min_fee_value)
    let (sending_chain_id, token_id, mode, fee_value, min_fee_value) = message::decode_update_fee_info_payload(&payload)?;

    require!(chain_limit.get_chain_id()==sending_chain_id, BridgeLimiterError::InvalidChainId);


    let mut token_config_loader = ctx.accounts.token_config.load_mut()?;

    require!(token_config_loader.token_id == token_id,BridgeTokenError::InvalidTokenIdNotSupported);

    token_config_loader.update_fee_info(mode, fee_value, min_fee_value)?;

    msg!("emit TokenFeeInfoUpdated");
    //emit event
    emit!(TokenFeeInfoUpdated {
        nonce,
        token_id,
        mode,
        fee_value,
        min_fee_value,
    });

    Ok(())
}
