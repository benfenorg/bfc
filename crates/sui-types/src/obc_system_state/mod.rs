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
use crate::{id::UID,OBC_SYSTEM_ADDRESS, OBC_SYSTEM_PACKAGE_ID};
use enum_dispatch::enum_dispatch;
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

const OBC_SYSTEM_STATE_WRAPPER_STRUCT_NAME: &IdentStr = ident_str!("OBCSystemState");

pub const OBC_ROUND_FUNCTION_NAME: &IdentStr = ident_str!("obc_round");
pub const OBC_ROUND_SAFE_MODE_FUNCTION_NAME: &IdentStr = ident_str!("obc_round_safe_mode");
pub const OBC_SYSTEM_MODULE_NAME: &IdentStr = ident_str!("obc_system");


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObcSystemStateWrapper {
    pub id: UID,
    pub version: u64,
}

impl ObcSystemStateWrapper {
    pub fn type_() -> StructTag {
        StructTag {
            address: OBC_SYSTEM_ADDRESS,
            name: OBC_SYSTEM_STATE_WRAPPER_STRUCT_NAME.to_owned(),
            module: OBC_SYSTEM_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn obc_round_safe_mode(
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
                Self::obc_round_safe_mode_impl::<ObcSystemStateInnerV1>(
                    move_object,
                    protocol_config
                );
            }
            _ => unreachable!(),
        }
        field_object
    }

    fn obc_round_safe_mode_impl<T>(
        move_object: &mut MoveObject,
        protocol_config: &ProtocolConfig,
    ) where
        T: Serialize + DeserializeOwned + ObcSystemStateTrait,
    {
        let mut field: Field<u64, T> =
            bcs::from_bytes(move_object.contents()).expect("bcs deserialization should never fail");
        tracing::info!(
            "obc round safe mode: current round: {}",
            field.value.round(),
        );
        field.value.obc_round_safe_mode();
        tracing::info!(
            "Safe mode activated. New epoch: {}",
            field.value.round(),
        );
        let new_contents = bcs::to_bytes(&field).expect("bcs serialization should never fail");
        move_object
            .update_contents(new_contents,protocol_config)
            .expect("Update sui system object content cannot fail since it should be small");
    }
}

/// This is the standard API that all inner system state object type should implement.
#[enum_dispatch]
pub trait ObcSystemStateTrait {
    fn round(&self) -> u64;
    fn obc_round_safe_mode(&mut self);
}


#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ObcSystemStateInnerV1 {
    pub round: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[enum_dispatch(ObcSystemStateTrait)]
pub enum ObcSystemState {
    V1(ObcSystemStateInnerV1),
}

impl ObcSystemStateTrait for ObcSystemStateInnerV1{
    fn round(&self) -> u64{
        return 0;
    }

    fn obc_round_safe_mode(&mut self){

    }

}

pub fn get_obc_system_state_wrapper(
    object_store: &dyn ObjectStore,
) -> Result<ObcSystemStateWrapper, SuiError> {
    let wrapper = object_store
        .get_object(&OBC_SYSTEM_PACKAGE_ID)?
        // Don't panic here on None because object_store is a generic store.
        .ok_or_else(|| {
            SuiError::SuiSystemStateReadError("SuiSystemStateWrapper object not found".to_owned())
        })?;
    let move_object = wrapper.data.try_as_move().ok_or_else(|| {
        SuiError::SuiSystemStateReadError(
            "SuiSystemStateWrapper object must be a Move object".to_owned(),
        )
    })?;
    let result = bcs::from_bytes::<ObcSystemStateWrapper>(move_object.contents())
        .map_err(|err| SuiError::SuiSystemStateReadError(err.to_string()))?;
    Ok(result)
}

pub fn get_obc_system_state(object_store: &dyn ObjectStore) -> Result<ObcSystemState, SuiError> {
    let wrapper = get_obc_system_state_wrapper(object_store)?;
    let id = wrapper.id.id.bytes;
    match wrapper.version {
        1 => {
            let result: ObcSystemStateInnerV1 =
                get_dynamic_field_from_store(object_store, id, &wrapper.version).map_err(
                    |err| {
                        SuiError::DynamicFieldReadError(format!(
                            "Failed to load sui system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;
            Ok(ObcSystemState::V1(result))
        }
        _ => Err(SuiError::SuiSystemStateReadError(format!(
            "Unsupported SuiSystemState version: {}",
            wrapper.version
        ))),
    }
}

