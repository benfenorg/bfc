use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use crate::{
    instructions::verify_message::verify_bridge_signature, 
    states::{
        bridge_config::*, chain_limit::{ChainLimit}, committee::Committee, message, message_config::{MessageConfig,MESSAGE_CONFIG_SEED}, message_verifier::MessageVerifier, token_config::*
    }
};
use anchor_spl::token_interface::TokenAccount;
use crate::states::benfen_bridge::{BenfenBridge,VAULT_SEED,BENFEN_BRIDGE_SEED};
use crate::states::message_verifier::MESSAGE_VERIFIER_SEED;
use crate::states::chain_limit::CHAIN_LIMIT_SEED;
use crate::states::committee::COMMITTEE_SEED;
// use crate::states::vault::{UniversalVault,UNIVERSAL_VAULT_SEED};
use crate::errors::MessageError;
use crate::errors::BridgeTokenError;
use crate::errors::BridgeError;
use crate::errors::BridgeLimiterError;
use std::cell::{Ref, RefMut};
use std::ops::DerefMut;


use crate::events::TokenAddedEvent;


#[derive(Accounts)]
#[instruction(token_id: u64)]
pub struct AddToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = MessageConfig::SPACE,
        seeds = [
            MESSAGE_CONFIG_SEED.as_bytes(),
            &[message::ADD_SVM_TOKENS],
            verifier.key().as_ref()
        ],

        bump
    )]
    pub message_config: Box<Account<'info, MessageConfig>>,

    /// Token vault for the pool
    #[account(
        init,
        seeds =[
            VAULT_SEED.as_bytes(),
            token_id.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        token::mint = token_mint,
        token::authority = benfen_bridge,
        token::token_program = token_program,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init, 
        seeds = [TOKEN_CONFIG_SEED.as_bytes(), token_id.to_be_bytes().as_ref()], 
        bump, 
        payer = payer, 
        space = TokenConfigAccount::SPACE
    )]
    pub token_config: AccountLoader<'info, TokenConfigAccount>,

    #[account(
        mut,
        seeds = [MESSAGE_VERIFIER_SEED.as_bytes(), committee.key().as_ref()],
        bump = verifier.load()?.bump[0],
    )]
    pub verifier: AccountLoader<'info, MessageVerifier>,

    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = bridge_config.load()?.bump[0],
    )]
    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    #[account(
        mut,
        seeds = [COMMITTEE_SEED.as_bytes(), bridge_config.key().as_ref()],
        bump = committee.load()?.bump[0],
    )]
    pub committee:  AccountLoader<'info, Committee>,

    #[account(
        seeds = [CHAIN_LIMIT_SEED.as_bytes(), &[chain_limit.load()?.get_chain_id()], bridge_config.key().as_ref()],
        bump = chain_limit.load()?.bump[0],
        constraint = chain_limit.load()?.config == bridge_config.key() @ BridgeLimiterError::InvalidLimiterPubkey,
        constraint = bridge_config.load()?.supported_chains.contains(&chain_limit.load()?.get_chain_id())
            @ BridgeError::UnsupportedCrossToChainId
    )]
    pub chain_limit: AccountLoader<'info, ChainLimit>,

    #[account(
        seeds = [BENFEN_BRIDGE_SEED.as_bytes(), committee.key().as_ref()],
        bump = benfen_bridge.bump[0],
        constraint = benfen_bridge.config == bridge_config.key() @ BridgeError::InvalidBridgeConfig,
        constraint = benfen_bridge.committee == committee.key() @ BridgeError::InvalidCommittee
    )]
    pub benfen_bridge: Account<'info, BenfenBridge>,

    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
    /// To create a new program account
    pub system_program: Program<'info, System>,
}


pub fn add_token_with_signatures(
    ctx: Context<AddToken>,
    token_id: u64,
    message_type: u8, 
    version: u8,
    nonce: u64,
    chain_id: u8,
    payload: Vec<u8>,  
    signatures: Vec<Vec<u8>>,
) -> Result<()> {
    let mut message_config: &mut Account<'_, MessageConfig> =  ctx.accounts.message_config.deref_mut();
    //message_config.initialize(ctx.accounts.verifier.key(), message_type);
    let mut bridge_config = ctx.accounts.bridge_config.load_mut()?;
    let mut verifier = ctx.accounts.verifier.load_mut()?;
    let mut token_config = ctx.accounts.token_config.load_init()?;
    let mut committee = ctx.accounts.committee.load_mut()?;
    let  chain_limit = ctx.accounts.chain_limit.load()?;
    //ctx.accounts.chain_limit.key();
    let message=message::create_message(message_type, version, nonce, chain_id, payload.clone());
    require!(message_type==message::ADD_SVM_TOKENS,MessageError::InvalidMessageType);


    verify_bridge_signature(
        &mut message_config,
        &mut bridge_config,
        &mut verifier,
        &mut committee,
        &message,
        signatures
    )?; 
    let payload=message::decode_add_token_payload(&payload)?;

    require!(token_id == payload.token_id, BridgeTokenError::InvalidTokenId);
    require!(
        ctx.accounts.token_mint.key() == payload.token_address,
        BridgeTokenError::InvalidTokenMint
    );


    add_token_internal(
        &mut bridge_config,
        &mut token_config,
        &chain_limit,
        &ctx.accounts.token_mint,
        payload.token_id,
        payload.benfen_decimal,
        payload.original_decimal,
        payload.token_price,
        payload.native,
        nonce,
    )?;

    Ok(())
}



fn add_token_internal(
    bridge_config:  &mut RefMut< BridgeConfig>,
    token_config: &mut RefMut< TokenConfigAccount>,
    chain_limit: &Ref<ChainLimit>,
    token_mint: &InterfaceAccount<Mint>,
    token_id: u64,
    benfen_decimal: u8,
    original_decimal: u8,
    price: u64,
    native: bool,
    nonce: u64,
)-> Result<()>{
    require!(token_mint.decimals > 0, BridgeTokenError::InvalidFungibleTokenDecimals);
    
    // 验证benfen_decimal不能为0
    require!(benfen_decimal > 0, BridgeTokenError::InvalidTokenBenfenDecimal);

    // 验证original_decimal不能为0
    require!(original_decimal > 0, BridgeTokenError::InvalidTokenOriginalDecimal);
    

    // token_config
    token_config.initialize(
        bridge_config.key(),
        chain_limit.key(),
        token_mint.key(),
        token_id,
        price,
        token_mint.decimals,
        benfen_decimal,
        original_decimal,
        if native { 1 } else { 0 },
    )?;

    
    // 增加代币计数
    bridge_config.increment_token_count()?;
    let total_token_count = bridge_config.token_count;



    msg!("emit TokenAddedEvent");
    
    // 发出代币添加事件
    emit!(TokenAddedEvent {
        nonce,
        token_id,
        mint_address: token_mint.key(),
        benfen_decimal,
        token_price: price,
        total_token_count,
    });
    


    Ok(())
}
