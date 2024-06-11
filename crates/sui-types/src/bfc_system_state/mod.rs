// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::dynamic_field::{
    get_dynamic_field_from_store, get_dynamic_field_object_from_store, Field,
};
use crate::error::SuiError;
use crate::object::{MoveObject, Object};
use crate::storage::ObjectStore;
use anyhow::Result;
use sui_protocol_config::ProtocolConfig;
use crate::{id::UID, BFC_SYSTEM_ADDRESS, BFC_SYSTEM_STATE_OBJECT_ID};
use enum_dispatch::enum_dispatch;
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::balance::Balance;
use crate::collection_types::{VecMap, Bag};
use crate::dao::Dao;
use crate::proposal::ProposalStatus;

const BFC_SYSTEM_STATE_WRAPPER_STRUCT_NAME: &IdentStr = ident_str!("BfcSystemState");

pub const BFC_ROUND_FUNCTION_NAME: &IdentStr = ident_str!("bfc_round");
pub const BFC_ROUND_SAFE_MODE_FUNCTION_NAME: &IdentStr = ident_str!("bfc_round_safe_mode");
pub const BFC_SYSTEM_MODULE_NAME: &IdentStr = ident_str!("bfc_system");
pub const STABLE_COIN_TO_BFC_FUNCTION_NAME: &IdentStr = ident_str!("inner_stablecoin_to_bfc");
pub const DEPOSIT_TO_TREASURY_FUNCTION_NAME: &IdentStr = ident_str!("deposit_to_treasury_pool_no_entry");

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Treasury {
    pub id: UID,
    pub bfc_balance: Balance,
    supplies: Bag,
    index: u64,
    time_interval: u32,
    updated_at: u64,
    init: bool,
    total_bfc_supply: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TreasuryPool {
    pub id: UID,
    pub balance: Balance,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BfcSystemStateWrapper {
    pub id: UID,
    pub version: u64,
}

impl BfcSystemStateWrapper {
    pub fn type_() -> StructTag {
        StructTag {
            address: BFC_SYSTEM_ADDRESS,
            name: BFC_SYSTEM_STATE_WRAPPER_STRUCT_NAME.to_owned(),
            module: BFC_SYSTEM_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn bfc_round_safe_mode(
        &self,
        object_store: &dyn ObjectStore,
        protocol_config: &ProtocolConfig,
    ) -> Object {
        let id = self.id.id.bytes;
        let mut field_object = get_dynamic_field_object_from_store(object_store, id, &self.version)
            .expect("Dynamic field object of wrapper should always be present in the object store");
        let move_object = field_object
            .data
            .try_as_move_mut()
            .expect("Dynamic field object must be a Move object");
        match self.version {
            1 => {
                Self::bfc_round_safe_mode_impl::<BfcSystemStateInnerV1>(
                    move_object,
                    protocol_config,
                );
            }
            _ => unreachable!(),
        }
        field_object
    }

    fn bfc_round_safe_mode_impl<T>(
        move_object: &mut MoveObject,
        protocol_config: &ProtocolConfig,
    ) where
        T: Serialize + DeserializeOwned + BfcSystemStateTrait,
    {
        let mut field: Field<u64, T> =
            bcs::from_bytes(move_object.contents()).expect("bcs deserialization should never fail");
        tracing::info!(
            "bfc round safe mode: current round: {}",
            field.value.round(),
        );
        field.value.bfc_round_safe_mode();
        tracing::info!(
            "Safe mode activated. New epoch: {}",
            field.value.round(),
        );
        let new_contents = bcs::to_bytes(&field).expect("bcs serialization should never fail");
        move_object
            .update_contents(new_contents, protocol_config)
            .expect("Update bfc system object content cannot fail since it should be small");
    }
}

/// This is the standard API that all inner system state object type should implement.
#[enum_dispatch]
pub trait BfcSystemStateTrait {
    fn round(&self) -> u64;
    fn bfc_round_safe_mode(&mut self);
}


#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BfcSystemStateInnerV1 {
    pub round: u64,
    pub stable_base_points: u64,
    pub reward_rate: u64,
    pub dao: Dao,
    pub treasury: Treasury,
    pub treasury_pool: TreasuryPool,
    pub rate_map: VecMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[enum_dispatch(BfcSystemStateTrait)]
pub enum BFCSystemState {
    V1(BfcSystemStateInnerV1),
}

impl BFCSystemState {
    pub fn inner_state(self) -> BfcSystemStateInnerV1 {
        match self {
            BFCSystemState::V1(inner) => inner,
        }
    }
}

impl BfcSystemStateTrait for BfcSystemStateInnerV1 {
    fn round(&self) -> u64 {
        0
    }

    fn bfc_round_safe_mode(&mut self) {}
}

pub fn get_stable_rate_map(object_store: &dyn ObjectStore) -> Result<VecMap<String, u64>, SuiError> {
    match get_bfc_system_state(object_store) {
        Ok(BFCSystemState::V1(bfc_system_state)) => {
            Ok(bfc_system_state.rate_map)
        }
        Err(e) => Err(e),
    }
}

pub fn get_stable_rate_with_base_point(object_store: &dyn ObjectStore) -> Result<(VecMap<String, u64>, u64), SuiError> {
    match get_bfc_system_state(object_store) {
        Ok(BFCSystemState::V1(bfc_system_state)) => {
            Ok((bfc_system_state.rate_map, bfc_system_state.stable_base_points))
        }
        Err(e) => Err(e),
    }
}

pub fn get_stable_rate_and_reward_rate(object_store: &dyn ObjectStore) -> Result<(VecMap<String, u64>, u64), SuiError> {
    match get_bfc_system_state(object_store) {
        Ok(BFCSystemState::V1(bfc_system_state)) => {
            Ok((bfc_system_state.rate_map, bfc_system_state.reward_rate))
        }
        Err(e) => Err(e),
    }
}

pub fn get_bfc_system_state_wrapper(
    object_store: &dyn ObjectStore,
) -> Result<BfcSystemStateWrapper, SuiError> {
    let wrapper = object_store
        .get_object(&BFC_SYSTEM_STATE_OBJECT_ID)?
        // Don't panic here on None because object_store is a generic store.
        .ok_or_else(|| {
            SuiError::SuiSystemStateReadError("BfcSystemStateWrapper object not found".to_owned())
        })?;
    let move_object = wrapper.data.try_as_move().ok_or_else(|| {
        SuiError::SuiSystemStateReadError(
            "BfcSystemStateWrapper object must be a Move object".to_owned(),
        )
    })?;

    let result = bcs::from_bytes::<BfcSystemStateWrapper>(move_object.contents())
        .map_err(|err| SuiError::SuiSystemStateReadError(err.to_string()))?;
    Ok(result)
}

pub fn get_bfc_system_state(object_store: &dyn ObjectStore) -> Result<BFCSystemState, SuiError> {
    let wrapper = get_bfc_system_state_wrapper(object_store)?;
    let id = wrapper.id.id.bytes;
    let result = match wrapper.version {
        1 => {
            let result: BfcSystemStateInnerV1 =
                get_dynamic_field_from_store(object_store, id, &wrapper.version).map_err(
                    |err| {
                        SuiError::DynamicFieldReadError(format!(
                            "Failed to load bfc system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;
            Ok(BFCSystemState::V1(result))
        }
        _ => Err(SuiError::BfcSystemStateReadError(format!(
            "Unsupported BfcSystemState version: {}",
            wrapper.version
        ))),
    };

    #[cfg(msim)]
        let result = bfc_get_stable_rate_result_injection::maybe_modify_result(result);

    result
}

pub fn get_bfc_system_proposal_state_map(object_store: &dyn ObjectStore) -> Result<VecMap<u64, ProposalStatus>, SuiError> {
    let wrapper = get_bfc_system_state_wrapper(object_store)?;
    let id = wrapper.id.id.bytes;
    match wrapper.version {
        1 => {
            let result: BfcSystemStateInnerV1 =
                get_dynamic_field_from_store(object_store, id, &wrapper.version).map_err(
                    |err| {
                        SuiError::DynamicFieldReadError(format!(
                            "Failed to load bfc system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;

            Ok(result.dao.current_proposal_status)
        }
        _ => Err(SuiError::SuiSystemStateReadError(format!(
            "Unsupported BfcSystemState version: {}",
            wrapper.version
        ))),
    }
}


#[cfg(msim)]
pub mod bfc_get_stable_rate_result_injection {
    use std::cell::RefCell;
    use crate::bfc_system_state::BFCSystemState;
    use crate::error::SuiError;

    thread_local! {
        static OVERRIDE: RefCell<Option<bool>>  = RefCell::new(None);
    }

    pub fn set_result_error(value: Option<bool>) {
        OVERRIDE.with(|o| *o.borrow_mut() = value);
    }

    pub fn maybe_modify_result(
        result: Result<BFCSystemState, SuiError>,
    ) -> Result<BFCSystemState, SuiError> {
        if let Some(enabled) = OVERRIDE.with(|o| *o.borrow()) {
            if enabled {
                return Err::<BFCSystemState, SuiError>(
                    SuiError::BfcSystemStateReadError("Unsupported BfcSystemState version: test mode".to_string()),
                );
            }
        }

        result
    }
}
