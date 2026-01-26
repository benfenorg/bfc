use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use anchor_spl::token_interface::{Mint, TokenInterface};
use crate::states::chain_limit::ChainLimit;
use crate::states::message::*;

use crate::states::token_config::TokenConfigAccount;
use crate::states::message_config::{MessageConfig,MESSAGE_CONFIG_SEED};
use crate::states::bridge_config::BridgeConfig;
use crate::states::message_verifier::MessageVerifier;
use crate::events::TokensDeposited;

use std::ops::DerefMut;
use crate::util::token::*;
use crate::states::benfen_bridge::*;
use crate::errors::BridgeError;
// use crate::errors::MessageError;
use crate::errors::BridgeTokenError;

const BENFEN_ADDRESS_LENGTH: usize = 32;

#[derive(Accounts)]

pub struct CrossIn<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        constraint = token_account.mint == token_vault.mint @ BridgeTokenError::InvalidTokenMint,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = token_vault.mint == token_mint.key() @ BridgeTokenError::InvalidTokenMint,
        constraint = token_vault.owner == bridge.key() @ BridgeTokenError::InvalidTokenOwner,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[TOKEN_TRANSFER],
            verifier.key().as_ref()
        ],
        bump
    )]
    pub message_config: Box<Account<'info, MessageConfig>>,

    #[account(
        mut,
        constraint = token_config.load()?.mint == token_mint.key() @ BridgeTokenError::InvalidTokenMint,
        constraint = token_config.load()?.mint == token_account.mint @ BridgeTokenError::InvalidTokenMint,
    )]
    pub token_config: AccountLoader<'info, TokenConfigAccount>,

    #[account(mut)]
    pub chain_limit: AccountLoader<'info, ChainLimit>,

   #[account(
        constraint = bridge_config.load()?.supported_chains.contains(&chain_limit.load()?.get_chain_id()) @ BridgeError::UnsupportedCrossToChainId,
    )]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

   
    #[account(
        mut,
        constraint = bridge.config == bridge_config.load()?.key() @ BridgeError::InvalidBridgeConfig,
    )]
    pub bridge: Account<'info, BenfenBridge>,

    pub verifier: AccountLoader<'info, MessageVerifier>,

    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,

    /// To create a new program account
    pub system_program: Program<'info, System>,

}

//对于sol代币要先转成 wsol之后再进行操作
//sdk有接口 调用时需要先转成wsol
pub fn cross_in<'info>(
    ctx: Context<'_, '_, '_, 'info, CrossIn<'info>>,
    amount: u64,
    benfen_address: Vec<u8>,
    target_token_id: u64,
) -> Result<()> {
    //先检查bridge 是否暂停，
    ctx.accounts.bridge.require_not_paused()?;
    require!(benfen_address.len() == BENFEN_ADDRESS_LENGTH, BridgeError::InvalidBenfenAddress);
    require!(amount > 0, BridgeError::InvalidAmount);
    
    let bridge_conifg_loader=ctx.accounts.bridge_config.load()?;
    let chain_limit=ctx.accounts.chain_limit.load()?;
    let token_account = ctx.accounts.token_account.deref_mut();
    let message =ctx.accounts.message_config.deref_mut();
    
    if message.verifier == Pubkey::default() {
        message.verifier = ctx.accounts.verifier.key();
        message.message_type = TOKEN_TRANSFER;
        message.nonce = 0;
    }

    let source_chain_id=bridge_conifg_loader.chain_id;
    let dst_chain_id = chain_limit.get_chain_id();

    //检查 dst_chain_id 是否支持
    require!(bridge_conifg_loader.supported_chains.contains(&dst_chain_id), BridgeError::UnsupportedCrossToChainId);

    //检查token 是否支持
   let token_config = ctx.accounts.token_config.load()?;


   let token_mint = ctx.accounts.token_mint.key();
   
   require!(token_account.mint == token_mint, BridgeError::InvalidTokenId);
   //require!(token_config.token_id() == token_id, BridgeError::UnsupportedTokenId);

   
    let single_transfer_limit = chain_limit.get_single_transfer_limit();
    let usd_amount = chain_limit.calculate_amount_in_usd(amount, token_config.price(), token_config.decimal())?;
    require!(usd_amount < single_transfer_limit, BridgeError::SingleTransferAmountExceedsLimit);
 

    //检查用户余额是否足够
    require!(token_account.amount >= amount, BridgeError::InsufficientBalance);

   


    transfer_from_user_to_bridge_vault(
        &ctx.accounts.payer,
         &ctx.accounts.token_account.to_account_info(),
        &ctx.accounts.token_vault.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        amount
    )?;

    let adjusted_amount = convert_slp_to_benfen_decimal(  ctx.accounts.token_mint.decimals, token_config.benfen_decimal, amount)?;
  
    msg!("emit TokensDeposited");
    //触发 event
    emit!(TokensDeposited {
        nonce: message.nonce(),
        source_chain_id,
        target_chain_id: dst_chain_id,
        token_id: token_config.token_id(),
        target_token_id,
        amount: adjusted_amount,
        sender_address: ctx.accounts.payer.key(),
        recipient_address: benfen_address,
    });
    message.increment_nonce();
    Ok(())
}
