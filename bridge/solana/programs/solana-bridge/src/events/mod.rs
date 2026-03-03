//! 统一的事件系统
//! 集中管理所有模块的事件定义

use anchor_lang::prelude::*;
// 事件模块定义
// 所有事件类型已整合到统一的事件系统中 ============================================================================
// 桥接核心事件
// ============================================================================

/// 桥接暂停事件
#[event]
pub struct EmergencyOperation {
    pub nonce: u64,
    pub is_freezing: bool,
}

// ============================================================================
// 转账事件
// ============================================================================

/// 跨链转账发起事件
#[event]
pub struct TokensDeposited {
    pub nonce: u64,
    pub source_chain_id: u8,
    pub target_chain_id: u8,
    pub token_id: u64,
    pub target_token_id: u64,
    pub amount: u64,
    pub sender_address: Pubkey,
    pub recipient_address: Vec<u8>,
}


/// 跨链转账完成事件
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

/// 跨链转账限制事件
#[event]
pub struct ChainLimitUpdated {
    pub nonce: u64,
    pub source_chain_id: u8,
    pub new_limit: u64,
}

/// 单笔转账限制事件
#[event]
pub struct SingleTransferLimitUpdated {
    pub nonce: u64,
    pub source_chain_id: u8,
    pub new_limit: u64,
}

#[event]
pub struct MinSingleTransferLimitUpdated {
    pub nonce: u64,
    pub source_chain_id: u8,
    pub new_limit: u64,
}

// ============================================================================
// 代币事件
// ============================================================================
// 添加代币事件
#[event]
pub struct TokenAddedEvent {
    pub nonce: u64,
    pub token_id: u64,
    pub mint_address: Pubkey,
    pub benfen_decimal: u8,
    pub token_price: u64,
    pub total_token_count: u64,
}


/// 代币价格更新事件
#[event]
pub struct TokenPriceUpdated {
    pub nonce: u64,    
    pub token_id: u64,
    pub new_price: u64,
}

#[event]
pub struct TokenFeeInfoUpdated {
    pub nonce: u64,    
    pub token_id: u64,
    pub mode: u8,
    pub fee_value: u64,
    pub min_fee_value: u64,
}

#[event]
pub struct BlocklistUpdatedEvent {
    pub nonce: u64,    
    pub blocklist: Vec<Vec<u8>>,
    pub is_blocklist: bool,
}


