// Copyright (c) Openblock Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};

use crate::{
    base_types::{SequenceNumber},
    coin::Coin,
    error::{ExecutionError, ExecutionErrorKind},
    id::UID,
    object::{Data, MoveObject, Object},
};
use crate::stable_coin::stable::checked::{STABLE, STABLE::BUSD};

pub mod stable;

pub use checked::*;

#[sui_macros::with_checked_arithmetic]
mod checked {
    use move_core_types::language_storage::{StructTag, TypeTag};
    use move_core_types::annotated_value::MoveStructLayout;
    use move_core_types::ident_str;
    use crate::balance::Balance;
    use crate::base_types::ObjectID;
    use crate::BFC_SYSTEM_ADDRESS;
    use super::*;

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

        pub fn busd_type_() -> StructTag {
            Coin::type_(TypeTag::Struct(Box::new(BUSD.type_())))
        }
        pub fn type_with_tag(tag: TypeTag) -> StructTag {
            Coin::type_(tag)
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

        pub fn invalid_gas_coin_type() -> StructTag {
            let struc_tag= StructTag {
                address: BFC_SYSTEM_ADDRESS,
                name: ident_str!("usdx").to_owned(),
                module: ident_str!("usdx").to_owned(),
                type_params: Vec::new(),
            };

            Coin::type_(TypeTag::Struct(Box::new(struc_tag)))
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
                    format!(
                        "Gas object type is not a stable gas coin: {}",
                        value.type_()
                    ),
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
