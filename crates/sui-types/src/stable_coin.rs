// Copyright (c) Openblock Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    ident_str,
    identifier::IdentStr,
    language_storage::{StructTag, TypeTag},
    value::MoveStructLayout,
};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};

use crate::{
    balance::Balance,
    base_types::{ObjectID, SequenceNumber},
    coin::Coin,
    error::{ExecutionError, ExecutionErrorKind},
    id::UID,
    object::{Data, MoveObject, Object},
};

/// The number of Mist per Sui token
pub const MIST_PER_SUI: u64 = 1_000_000_000;

/// Total supply denominated in Sui
pub const TOTAL_SUPPLY_SUI: u64 = 10_000_000_000;

// Note: cannot use checked arithmetic here since `const unwrap` is still unstable.
/// Total supply denominated in Mist
pub const TOTAL_SUPPLY_MIST: u64 = TOTAL_SUPPLY_SUI * MIST_PER_SUI;

pub const GAS_MODULE_NAME: &IdentStr = ident_str!("usd");
pub const GAS_STRUCT_NAME: &IdentStr = ident_str!("USD");

pub use checked::*;

#[sui_macros::with_checked_arithmetic]
mod checked {
    use crate::BFC_SYSTEM_ADDRESS;
    use super::*;

    pub struct STABLE {}
    impl STABLE {
        pub fn type_() -> StructTag {
            StructTag {
                address: BFC_SYSTEM_ADDRESS,
                name: GAS_STRUCT_NAME.to_owned(),
                module: GAS_MODULE_NAME.to_owned(),
                type_params: Vec::new(),
            }
        }

        pub fn type_tag() -> TypeTag {
            TypeTag::Struct(Box::new(Self::type_()))
        }

        pub fn is_gas(other: &StructTag) -> bool {
            &Self::type_() == other
        }

        pub fn is_gas_type(other: &TypeTag) -> bool {
            match other {
                TypeTag::Struct(s) => Self::is_gas(s),
                _ => false,
            }
        }
    }

    /// Rust version of the Move sui::coin::Coin<STABLE> type
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StableCoin(pub Coin);

    impl StableCoin {
        pub fn new(id: ObjectID, value: u64) -> Self {
            Self(Coin::new(UID::new(id), value))
        }

        pub fn value(&self) -> u64 {
            self.0.value()
        }

        pub fn type_() -> StructTag {
            Coin::type_(TypeTag::Struct(Box::new(STABLE::type_())))
        }

        /// Return `true` if `s` is the type of a gas coin (i.e., 0x2::coin::Coin<0x2::sui::SUI>)
        pub fn is_gas_coin(s: &StructTag) -> bool {
            Coin::is_coin(s) && s.type_params.len() == 1 && STABLE::is_gas_type(&s.type_params[0])
        }

        /// Return `true` if `s` is the type of a gas balance (i.e., 0x2::balance::Balance<0x2::sui::SUI>)
        pub fn is_gas_balance(s: &StructTag) -> bool {
            Balance::is_balance(s)
                && s.type_params.len() == 1
                && STABLE::is_gas_type(&s.type_params[0])
        }

        pub fn id(&self) -> &ObjectID {
            self.0.id()
        }

        pub fn to_bcs_bytes(&self) -> Vec<u8> {
            bcs::to_bytes(&self).unwrap()
        }

        pub fn to_object(&self, version: SequenceNumber) -> MoveObject {
            MoveObject::new_stable_coin(version, *self.id(), self.value())
        }

        pub fn layout() -> MoveStructLayout {
            Coin::layout(TypeTag::Struct(Box::new(STABLE::type_())))
        }

        #[cfg(test)]
        pub fn new_for_testing(value: u64) -> Self {
            Self::new(ObjectID::random(), value)
        }
    }

    impl TryFrom<&MoveObject> for StableCoin {
        type Error = ExecutionError;

        fn try_from(value: &MoveObject) -> Result<StableCoin, ExecutionError> {
            if !value.type_().is_gas_coin() {
                return Err(ExecutionError::new_with_source(
                    ExecutionErrorKind::InvalidGasObject,
                    format!("Gas object type is not a stable gas coin: {}", value.type_()),
                ));
            }
            let gas_coin: StableCoin = bcs::from_bytes(value.contents()).map_err(|err| {
                ExecutionError::new_with_source(
                    ExecutionErrorKind::InvalidGasObject,
                    format!("Unable to deserialize stable gas object: {:?}", err),
                )
            })?;
            Ok(gas_coin)
        }
    }

    impl TryFrom<&Object> for StableCoin {
        type Error = ExecutionError;

        fn try_from(value: &Object) -> Result<StableCoin, ExecutionError> {
            match &value.data {
                Data::Move(obj) => obj.try_into(),
                Data::Package(_) => Err(ExecutionError::new_with_source(
                    ExecutionErrorKind::InvalidGasObject,
                    format!("Gas object type is not a stable gas coin: {:?}", value),
                )),
            }
        }
    }

    impl Display for StableCoin {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Coin {{ id: {}, value: {} }}", self.id(), self.value())
        }
    }
}
