use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::id::ID;

// Rust types of VaultInfo in crates/sui-framework/packages/bfc-system/sources/treasury/vault.move
#[derive(Debug, Serialize, JsonSchema, Deserialize, Clone)]
pub struct VaultInfo {
    pub vault_id: ID,
    pub position_number: u32,
    pub state: u8,
    pub state_counter: u32,
    pub max_counter_times: u32,
    pub last_sqrt_price: u128,
    pub coin_a_balance: u64,
    pub coin_b_balance: u64,
    pub coin_a_type: String,
    pub coin_b_type: String,
    pub tick_spacing: u32,
    pub spacing_times: u32,
    pub liquidity: u128,
    pub current_sqrt_price: u128,
    pub current_tick_index: u32,
    pub is_pause: bool,
    pub index: u64,
    pub base_point: u64,
    pub bfc_accrued_consume: u64,
    pub last_bfc_rebalance_amount: u64,
}
