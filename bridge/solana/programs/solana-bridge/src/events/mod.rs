use anchor_lang::prelude::*;


#[event]
pub struct EmergencyOperation {
    pub nonce: u64,
    pub is_freezing: bool,
}


#[event]
pub struct TokensDeposited {
    pub nonce: u64,
    pub source_chain_id: u8,
    pub target_chain_id: u8,
    pub token_id: u64,
    pub amount: u64,
    pub sender_address: Pubkey,
    pub recipient_address: Vec<u8>,
}


#[event]
pub struct TokensClaimed {
    pub nonce: u64,
    pub source_chain_id: u8,
    pub target_chain_id: u8,
    pub token_id: u64,
    pub amount: u64,
    pub sender_address: Vec<u8>,
    pub recipient_address: Pubkey,
}


#[event]
pub struct ProgramUpgradeEvent {
   pub nonce: u64,
    pub program: Pubkey,
    pub buffer: Pubkey,
    pub old_version: u8,
    pub new_version: u8,
    pub timestamp: i64,
}

#[event]
pub struct ChainLimitUpdated {
    pub nonce: u64,
    pub source_chain_id: u8,
    pub new_limit: u64,
}

#[event]
pub struct SingleTransferLimitUpdated {
    pub nonce: u64,
    pub source_chain_id: u8,
    pub new_limit: u64,
}


#[event]
pub struct TokenAddedEvent {
    pub nonce: u64,
    pub token_id: u64,
    pub mint_address: Pubkey,
    pub benfen_decimal: u8,
    pub token_price: u64,
    pub total_token_count: u64,
}


#[event]
pub struct TokenPriceUpdated {
    pub nonce: u64,    
    pub token_id: u64,
    pub new_price: u64,
}

#[event]
pub struct BlocklistUpdatedEvent {
    pub nonce: u64,    
    pub blocklist: Vec<Vec<u8>>,
    pub is_blocklist: bool,
}


