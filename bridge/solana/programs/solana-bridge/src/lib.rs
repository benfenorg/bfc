use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;
pub mod errors;
pub mod events;
pub mod util;
use instructions::*;

#[cfg(feature = "devnet")]
declare_id!("AU4BgoC2aooMq8txbEirQgkwQebgeuZT4m6XF8iuArcL");
#[cfg(feature = "mainnet")]
declare_id!("G1hnNcpssQaM3C9sKorjXLbo2zoDJoVHviYX8MoRNTgS");
#[cfg(all(not(feature = "devnet"), not(feature = "mainnet")))]
declare_id!("G1hnNcpssQaM3C9sKorjXLbo2zoDJoVHviYX8MoRNTgS");
pub mod admin {
    use anchor_lang::prelude::*;
    use std::str::FromStr;
    pub fn id() -> Pubkey {
        Pubkey::from_str("C5LJkjmJUm3gVcdyWFGD9GiAJtitgAgpe4fJwfQ6ffgB").unwrap()
    }
}


#[program]
pub mod benfen_bridge {
    use super::*;
    pub fn initialize_bridge_config(
        ctx: Context< InitializeBridgeConfig>,
        chain_id: u8,
    ) -> Result<()> {
        instructions::initialize_bridge_config(ctx, chain_id)
    }

    pub fn initialize_committee(
        ctx: Context< InitializeCommittee>,
        addresses: Vec<[u8; 20]>,
        stakes: Vec<u16>,
        min_stake_required: u16
    ) -> Result<()> {
        instructions::initialize_committee(ctx, addresses, stakes, min_stake_required)
    }

    pub fn initialize_message_verifier(
        ctx: Context< InitializeMessageVerifier>,
    ) -> Result<()> {
        instructions::initialize_message_verifier(ctx)
    }

    pub fn initialize_bridge_limiter(
        ctx: Context<InitializeBridgeLimiter>,
        chain_id: u8,
        limit: u64,
        min_usd_limit: u64,
        max_usd_limit: u64,
    ) -> Result<()> {
        instructions::initialize_bridge_limiter(ctx, chain_id, limit, min_usd_limit, max_usd_limit) 
    }

    pub fn initialize_benfen_bridge(
        ctx: Context< InitializeBenfenBridge>,
    ) -> Result<()> {
        instructions::initialize_benfen_bridge(ctx)
    }

    pub fn initialize_upgrade_authority(
        ctx: Context<InitializeUpgradeAuthority>,
        enabled: bool,
    ) -> Result<()> {
        instructions::initialize_upgrade_authority(ctx, enabled)
    }


    pub fn cross_token_to_bridge<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CrossIn<'info>>,
        amount: u64,
        benfen_address: Vec<u8>,
        target_token_id: u64,
    ) -> Result<()> {
        instructions::cross_in(ctx, amount, benfen_address, target_token_id)
    }

    pub fn extend_program(
        ctx: Context<ExtendProgram>,
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::extend_program_space_with_signatures(ctx, message_type, version, nonce, chain_id, payload, signatures)
    }

    pub fn cross_out<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CrossOut<'info>>,
        chain_id: u8,
        nonce: u64,
        message_type: u8, 
        version: u8,
        payload: Vec<u8>,  
        signatures: Vec<Vec<u8>>,   
    ) -> Result<()> {
        instructions::cross_out_with_signature(ctx, chain_id,nonce, message_type, version, payload, signatures)
    }



    pub fn add_token_to_bridge(
        ctx: Context<AddToken>,
        token_id: u64, 
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,  
        signatures: Vec<Vec<u8>>
    ) -> Result<()> {
        instructions::add_token::add_token_with_signatures(ctx,token_id,message_type,version, nonce, chain_id, payload, signatures)
    }

    pub fn update_token_price(
        ctx: Context<UpdateTokenPrice>, 
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,  
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::update_token_price_with_signature(ctx, message_type, version, nonce, chain_id, payload, signatures)
    }

    pub fn update_token_fee_info(
        ctx: Context<UpdateTokenFeeInfo>, 
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,  
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::update_token_fee_info_with_signature(ctx, message_type, version, nonce, chain_id, payload, signatures)
    }

    pub fn update_block_list(
        ctx: Context<UpdateBlockList>,
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::update_block_list_with_signatures(ctx, message_type, version, nonce, chain_id, payload, signatures)
    }

    pub fn update_emergency_op(
        ctx: Context<UpdateEmergencyOp>,
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::update_emergency_op_with_signatures(ctx, message_type, version, nonce, chain_id, payload, signatures)
    }   


    pub fn update_bridge_limiter(
        ctx: Context<UpdateBridgeLimiter>,
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::update_limit_with_signatures(ctx, message_type, version, nonce, chain_id, payload, signatures)
    }


    pub fn update_single_transfer_limit(
        ctx: Context<UpdateMaxSingleTransferLimit>,
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::update_max_single_transfer_limit_with_signatures(ctx, message_type, version, nonce, chain_id, payload, signatures)
    }

    pub fn update_min_single_transfer_limit(
        ctx: Context<UpdateMinSingleTransferLimit>,
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::update_min_single_transfer_limit_with_signatures(ctx, message_type, version, nonce, chain_id, payload, signatures)
    }

    pub fn upgrade_program(
        ctx: Context<UpgradeProgram>,
        message_type: u8, 
        version: u8,
        nonce: u64,
        chain_id: u8,
        payload: Vec<u8>,
        signatures: Vec<Vec<u8>>,
    ) -> Result<()> {
        instructions::upgrade_program_with_signatures(ctx, message_type, version, nonce, chain_id, payload, signatures)
    }

    pub fn transfer_upgrade_authority(
        ctx: Context<TransferUpgradeAuthority>,
    ) -> Result<()> {
        instructions::transfer_upgrade_authority(ctx)
    }

}

