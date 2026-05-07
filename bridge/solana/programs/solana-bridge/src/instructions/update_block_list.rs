use anchor_lang::prelude::*;
use crate::states::bridge_config::{BridgeConfig, CONFIG_SEED};
use crate::states::message;
use crate::states::message_verifier::{MessageVerifier, MESSAGE_VERIFIER_SEED};
use crate::states::message_config::{MessageConfig,MESSAGE_CONFIG_SEED};

use crate::states::committee::{Committee, COMMITTEE_SEED};
use crate::instructions::verify_message::verify_bridge_signature;
use std::ops::DerefMut;
use crate::events::BlocklistUpdatedEvent;
use crate::errors::MessageError;
use crate::errors::BridgeCommitteeError;



#[derive(Accounts)]
pub struct UpdateBlockList<'info> {
    #[account(
        mut,
    )]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[message::BLOCKLIST],
            (verifier.key().as_ref())
        ],
        bump
    )]
    pub message_config: Box<Account<'info, MessageConfig>>,

    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = bridge_config.load()?.bump[0],
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

pub fn update_block_list_with_signatures(
    ctx: Context<UpdateBlockList>, 
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
    //let token_mint = ctx.accounts.token_mint.deref();

    require!(message_type==message::BLOCKLIST,MessageError::InvalidMessageType);


    verify_bridge_signature(
        &mut message_config,
        &mut bridge_config,
        &mut verifier,
        &mut committee,
        &message,
        signatures
    )?; 


    let block_list = message::decode_blocklist_payload(&payload)?;

    // let committee = &mut ctx.accounts.committee.load_mut()?;
    for address in &block_list.addresses {
        require!(committee.is_member(address), BridgeCommitteeError::NotCommitteeMember);
    }
    //和 evm 值保持一致
    let flag=if block_list.is_blocklisted{
         1 //true
    }else{
         0 //false
    };

    committee.update_blocklist(&block_list.addresses, flag )?;

    let mut active_stake: u32 = 0;
    let member_count = committee.member_count as usize;
    for i in 0..member_count {
        let m = committee.members[i];
        if m.is_blocklisted == 0 {
            active_stake = active_stake.saturating_add(m.stake as u32);
        }
    }
    require!(active_stake >= committee.min_stake_required as u32, BridgeCommitteeError::InsufficientStake);

    msg!("emit BlocklistUpdatedEvent");

    emit!(BlocklistUpdatedEvent {
        nonce,
        blocklist: block_list.addresses.iter().map(|x| x.to_vec()).collect(),
        is_blocklist: block_list.is_blocklisted,
    });

    Ok(())
}  
