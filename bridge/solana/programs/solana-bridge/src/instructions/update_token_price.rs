use anchor_lang::prelude::*;
use crate::states::{
    bridge_config::*,
    token_config::*
};
use crate::errors::BridgeTokenError;
use crate::errors::MessageError;

use crate::states::message_verifier::MessageVerifier;
use crate::states::message_config::{MessageConfig,MESSAGE_CONFIG_SEED};
use crate::states::message;

use crate::states::committee::Committee;
use crate::instructions::verify_message::verify_bridge_signature;
use std::ops::DerefMut;
use crate::events::TokenPriceUpdated;



#[derive(Accounts)]
pub struct UpdateTokenPrice<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[message::UPDATE_TOKEN_PRICE],
            verifier.key().as_ref()
        ],
        bump
    )]
    pub message_config: Box<Account<'info, MessageConfig>>,

    #[account(
        mut,
    )]
    pub token_config: AccountLoader<'info, TokenConfigAccount>,

    #[account(mut)]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    #[account(mut)]
    pub verifier: AccountLoader<'info, MessageVerifier>,

    #[account(mut)]
    pub committee:  AccountLoader<'info, Committee>,

    pub system_program: Program<'info, System>,
}

pub fn update_token_price_with_signature(
   ctx: Context<UpdateTokenPrice>, 
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
    let message=message::create_message(message_type, version, nonce, chain_id, payload.clone());
    require!(message_type==message::UPDATE_TOKEN_PRICE,MessageError::InvalidMessageType);

    verify_bridge_signature(
        &mut message_config,
        &mut bridge_config,
        &mut verifier,
        &mut committee,
        &message,
        signatures
    )?; 
    let (token_id,price)=message::decode_update_token_price_payload(&payload)?;

    let mut token_config_loader = ctx.accounts.token_config.load_mut()?;

  require!(token_config_loader.token_id == token_id,BridgeTokenError::InvalidTokenIdNotSupported);

  token_config_loader.update_price(price)?;

  msg!("emit TokenPriceUpdated");

  //emit event
  emit!(TokenPriceUpdated {
    nonce,
    token_id,
    new_price:price,
  });

  Ok(())
}
