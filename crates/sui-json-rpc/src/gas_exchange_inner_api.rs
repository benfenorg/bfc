use std::sync::Arc;
use sui_core::authority::AuthorityState;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::error::{SuiError, SuiResult};
use sui_types::sui_system_state::sui_system_state_summary::SuiSystemStateSummary;
use sui_types::sui_system_state::SuiSystemStateTrait;

async fn exchange_rates(
    state: &Arc<AuthorityState>,
    gas_coin: ObjectID,
) -> SuiResult<u64> {
    let system_state = state.database.get_sui_system_state_object()?;
    let system_state_summary: SuiSystemStateSummary = system_state.into_sui_system_state_summary();
    let gas_coin_map = system_state_summary.gas_coin_map;
    let coin_addr = SuiAddress::from(gas_coin);
    let result = gas_coin_map.iter().find(|(address, _)| address == &coin_addr);
    match result {
        Some((_address, value)) => Ok(*value),
        None =>  Err(SuiError::Unknown("not found gas coin.".to_string())),
    }
}
