use anchor_lang::prelude::*;

use anchor_spl::token::TokenAccount;
use anchor_spl::token_interface::{Mint, TokenInterface};
use crate::states::chain_limit::ChainLimit;
use crate::states::message::{self,decode_token_transfer_payload};
use crate::states::token_config::TokenConfigAccount;
use crate::states::message_config::{MessageConfig,MESSAGE_CONFIG_SEED};
use crate::states::bridge_config::BridgeConfig;
use std::cell::RefMut;
use std::ops::DerefMut;
use crate::states::message_verifier::MessageVerifier;
use crate::instructions::verify_message::verify_bridge_signature;
use crate::states::committee::Committee;
use crate::states::benfen_bridge::*;
use crate::events::TokensClaimed;
use crate::states::process_transfer::{ProcessTransfer, PROCESSED_TRANSFER_SEED};
use crate::util::token::transfer_from_vault_to_user;
use crate::errors::BridgeTokenError;
use crate::errors::MessageError;
use crate::errors::BridgeError;

#[derive(Accounts)]
#[instruction(chain_id: u8, nonce: u64)]
pub struct CrossOut<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    //store user coin account
    #[account(
        mut,
        constraint = token_account.mint == token_mint.key() @ BridgeTokenError::InvalidTokenMint,
    )]
    pub token_account: Account<'info, TokenAccount>,

    //bridge store coin account
    #[account(
        mut,
        constraint = token_vault.mint == token_mint.key() @ BridgeTokenError::InvalidTokenMint,
        constraint = token_vault.owner == bridge.key() @ BridgeTokenError::InvalidTokenOwner,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    //message config account
    #[account(
        init_if_needed,
        payer = signer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[message::TOKEN_TRANSFER],
            verifier.key().as_ref()
        ],
        bump
    )]
    pub message_config: Account<'info, MessageConfig>,


    #[account(
        init_if_needed,
        payer = signer,
        space = ProcessTransfer::SPACE,
        seeds = [PROCESSED_TRANSFER_SEED.as_bytes(),&[message::TOKEN_TRANSFER],&[chain_id], nonce.to_be_bytes().as_ref()],
        bump ,
    )]
    //处理benfen 上的交易记录
    pub process_transfer: Account<'info, ProcessTransfer>,

    //chain limit account
    #[account(mut)]
    pub chain_limit: AccountLoader<'info, ChainLimit>,

    //token config account
    #[account(mut)]
    pub token_config: AccountLoader<'info, TokenConfigAccount>,

    //message verifier account
    #[account(mut)]
    pub verifier: AccountLoader<'info, MessageVerifier>,

    //committee account
    #[account(mut)]
    pub committee:  AccountLoader<'info, Committee>,
    #[account(mut)]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    #[account(
        mut,
        constraint = bridge.config == bridge_config.load()?.key() @ BridgeError::InvalidBridgeConfig,
        constraint = bridge.committee == committee.load()?.key() @ BridgeError::InvalidCommittee,
    )]

    pub bridge: Account<'info, BenfenBridge>,

    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
    /// To create a new program account
    pub system_program: Program<'info, System>
}


pub fn cross_out_with_signature(
    ctx: Context<CrossOut>, 
    chain_id: u8,
    nonce: u64,
    message_type: u8, 
    version: u8,
    payload: Vec<u8>,  
    signatures: Vec<Vec<u8>>,   
) -> Result<()> {
    ctx.accounts.bridge.require_not_paused()?;
    let bridge = &ctx.accounts.bridge;
    let mut message_config =  ctx.accounts.message_config.deref_mut();
    let mut bridge_config = ctx.accounts.bridge_config.load_mut()?;
    let mut verifier = ctx.accounts.verifier.load_mut()?;
    let mut committee = ctx.accounts.committee.load_mut()?;
    let mut token_config = ctx.accounts.token_config.load_mut()?;
    let  process_transfer = ctx.accounts.process_transfer.deref_mut();
    let mut  chain_limit = ctx.accounts.chain_limit.load_mut()?;
    let token_mint = ctx.accounts.token_mint.clone();
    let mut token_vault=ctx.accounts.token_vault.to_account_info();
    let mut token_account=ctx.accounts.token_account.to_account_info();
    let mut token_program=ctx.accounts.token_program.to_account_info();
    let token_transfer_payload=decode_token_transfer_payload(
        &payload
    )?;
    require!(
        token_transfer_payload.target_chain_id==bridge_config.chain_id,
        BridgeError::InvalidTargetChainId
    );
    require!(bridge_config.is_chain_supported(chain_id), BridgeError::UnsupportedCrossToChainId);
    require!(message_type==message::TOKEN_TRANSFER,MessageError::InvalidMessageType);
    let message=message::create_message(message_type, version, nonce, chain_id, payload.clone());
    verify_bridge_signature(
        &mut message_config,
        &mut bridge_config,
        &mut verifier,
        &mut committee,
        &message,
        signatures
    )?; 

  
    require!(
        !process_transfer.is_processed(),
        BridgeError::TokenTransferAlreadyProcessed
    );
   
    require!(
        ctx.accounts.token_account.owner==token_transfer_payload.recipient_address,
        BridgeError::InvalidRecipientAddress
    );
    
    let mut bdecimal = token_config.benfen_decimal;
    if token_transfer_payload.token_id==3 || token_transfer_payload.token_id==4 {
       bdecimal=token_config.original_decimal;
    };

    let slp_token_adjusted_amount=message::convert_benfen_to_slp_decimal(
       bdecimal, 
        token_mint.decimals,
        token_transfer_payload.amount,
    )?;

    transfer_tokens_from_vault_internal(
        &bridge,
        &mut chain_limit,
        &mut token_config,
        &mut token_vault,
        &mut token_account,
        &mut token_program,
        slp_token_adjusted_amount
    )?;


    process_transfer.mark_processed()?;

    msg!("emit TokensClaimed");
    //触发event
    emit!(
        TokensClaimed{
            nonce: nonce,
            source_chain_id: chain_limit.get_chain_id(),
            target_chain_id: bridge_config.chain_id,
            token_id: token_config.token_id,
            amount: slp_token_adjusted_amount,
            sender_address: token_transfer_payload.sender_address,
            recipient_address: token_transfer_payload.recipient_address,
        }
    );

    Ok(())
}



fn transfer_tokens_from_vault_internal<'info>(
    benfen_bridge: &Account<'info, BenfenBridge>,
    chain_limiter: &mut RefMut<ChainLimit>,
    token_config: &mut RefMut<TokenConfigAccount>,
    token_vault: &mut AccountInfo<'info>,
    user_vault: &mut AccountInfo<'info>,
    token_program: &mut AccountInfo<'info>,
    amount: u64
) -> Result<()> {
    //检查bridge 是否暂停
    benfen_bridge.require_not_paused()?;
    let price= token_config.price();
    let decimal =token_config.decimal();
    //
    //检查是否超过24小时限额
    require!(
        !chain_limiter.will_amount_exceed_limit(
            amount, 
            price,
            decimal
        ),
        BridgeError::CrossOutAmountExceedLimit
    );


    transfer_from_vault_to_user(
        benfen_bridge,
        token_vault,
        user_vault,
        token_program,
        amount
    )?;
    chain_limiter.record_bridge_transfers(amount, price, decimal)?;
    
    Ok(())
}




