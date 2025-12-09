use anchor_lang::prelude::*;

use crate::states::benfen_bridge::BenfenBridge;
use crate::events::EmergencyOperation;
use crate::states::message_verifier::MessageVerifier;
use crate::states::message_config::{MessageConfig,MESSAGE_CONFIG_SEED};
use crate::states::message;
use crate::states::bridge_config::BridgeConfig;
use crate::states::committee::Committee;
use crate::instructions::verify_message::verify_bridge_signature;
use std::ops::DerefMut;
// use crate::errors::BridgeError;
use crate::errors::MessageError;


#[derive(Accounts)]
pub struct UpdateEmergencyOp<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub bridge: Account<'info, BenfenBridge>,

    #[account(
        init_if_needed,
        payer = signer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[message::EMERGENCY_OP],
            verifier.key().as_ref()
        ],
        bump
    )]
    pub message_config: Box<Account<'info, MessageConfig>>,

    #[account(mut)]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    #[account(mut)]
    pub verifier: AccountLoader<'info, MessageVerifier>,

    #[account(mut)]
    pub committee:  AccountLoader<'info, Committee>,

        /// To create a new program account
    pub system_program: Program<'info, System>,

}


pub fn update_emergency_op_with_signatures(
    ctx: Context<UpdateEmergencyOp>,
    message_type: u8, 
    version: u8,
    nonce: u64,
    chain_id: u8,
    payload: Vec<u8>,  
    signatures: Vec<Vec<u8>>,
) -> Result<()> {
    let bridge = &mut ctx.accounts.bridge;
    let mut message_config =  ctx.accounts.message_config.deref_mut();
    let mut bridge_config = ctx.accounts.bridge_config.load_mut()?;
    let mut verifier = ctx.accounts.verifier.load_mut()?;
    let mut committee = ctx.accounts.committee.load_mut()?;
    let message=message::create_message(message_type, version, nonce, chain_id, payload.clone());

    require!(message_type==message::EMERGENCY_OP,MessageError::InvalidMessageType);


     verify_bridge_signature(
        &mut message_config,
        &mut bridge_config,
        &mut verifier,
        &mut committee,
        &message,
        signatures
    )?; 

    let emergency_op = message::decode_emergency_op_payload(&payload)?;

    if emergency_op{
        bridge.pause()?;
    }else{
        bridge.unpause()?;
    }

    msg!("emit EmergencyOperation");

    emit!(
        EmergencyOperation{
            nonce: nonce,
            is_freezing: emergency_op,
        }
    );
    Ok(())
}

