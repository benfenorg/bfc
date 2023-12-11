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
pub const TOTAL_SUPPLY_SUI: u64 = 1_000_000_000;

// Note: cannot use checked arithmetic here since `const unwrap` is still unstable.
/// Total supply denominated in Mist
pub const TOTAL_SUPPLY_MIST: u64 = TOTAL_SUPPLY_SUI * MIST_PER_SUI;

pub use checked::*;

#[sui_macros::with_checked_arithmetic]
mod checked {
    use super::*;
    use crate::BFC_SYSTEM_ADDRESS;

    pub enum STABLE {
        BUSD,
        BARS,
        BAUD,
        BBRL,
        BCAD,
        BEUR,
        BGBP,
        BIDR,
        BINR,
        BJPY,
        BKRW,
        BMXN,
        BRUB,
        BSAR,
        BTRY,
        BZAR,
    }

    impl STABLE {
        pub fn type_(&self) -> StructTag {
            let (module_name, struct_name) = match self {
                STABLE::BARS => (ident_str!("bars"), ident_str!("BARS")),
                STABLE::BAUD => (ident_str!("baud"), ident_str!("BAUD")),
                STABLE::BZAR => (ident_str!("bzar"), ident_str!("BZAR")),
                STABLE::BUSD => (ident_str!("busd"), ident_str!("BUSD")),
                STABLE::BBRL => (ident_str!("bbrl"), ident_str!("BBRL")),
                STABLE::BCAD => (ident_str!("bcad"), ident_str!("BCAD")),
                STABLE::BEUR => (ident_str!("beur"), ident_str!("BEUR")),
                STABLE::BGBP => (ident_str!("bgbp"), ident_str!("BGBP")),
                STABLE::BIDR => (ident_str!("bidr"), ident_str!("BIDR")),
                STABLE::BINR => (ident_str!("binr"), ident_str!("BINR")),
                STABLE::BJPY => (ident_str!("bjpy"), ident_str!("BJPY")),
                STABLE::BKRW => (ident_str!("bkrw"), ident_str!("BKRW")),
                STABLE::BMXN => (ident_str!("bmxn"), ident_str!("BMXN")),
                STABLE::BRUB => (ident_str!("brub"), ident_str!("BRUB")),
                STABLE::BSAR => (ident_str!("bsar"), ident_str!("BSAR")),
                STABLE::BTRY => (ident_str!("btry"), ident_str!("BTRY")),
                STABLE::BZAR => (ident_str!("bzar"), ident_str!("BZAR")),
            };

            StructTag {
                address: BFC_SYSTEM_ADDRESS,
                name: struct_name.to_owned(),
                module: module_name.to_owned(),
                type_params: Vec::new(),
            }
        }

        pub fn type_tag(&self) -> TypeTag {
            TypeTag::Struct(Box::new(self.type_()))
        }

        pub fn is_gas_type(other: &TypeTag) -> bool {
            [   STABLE::BARS,
                STABLE::BAUD,
                STABLE::BZAR,
                STABLE::BUSD,
                STABLE::BBRL,
                STABLE::BCAD,
                STABLE::BEUR,
                STABLE::BGBP,
                STABLE::BIDR,
                STABLE::BINR,
                STABLE::BJPY,
                STABLE::BKRW,
                STABLE::BMXN,
                STABLE::BRUB,
                STABLE::BSAR,
                STABLE::BTRY,
             ]
                .iter()
                .map(|stable_type| stable_type.type_tag())
                .any(|stable_tag| &stable_tag == other)
        }

    }


    impl From<StructTag> for STABLE {
        fn from(s: StructTag) -> Self {
            match (s.module.as_str(), s.name.as_str()) {
                ("bars", "BARS") => STABLE::BARS,
                ("baud", "BAUD") => STABLE::BAUD,
                ("bbrl", "BBRL") => STABLE::BBRL,
                ("bcad", "BCAD") => STABLE::BCAD,
                ("beur", "BEUR") => STABLE::BEUR,
                ("bgbp", "BGBP") => STABLE::BGBP,
                ("bidr", "BIDR") => STABLE::BIDR,
                ("binr", "BINR") => STABLE::BINR,
                ("bjpy", "BJPY") => STABLE::BJPY,
                ("bkrw", "BKRW") => STABLE::BKRW,
                ("bmxn", "BMXN") => STABLE::BMXN,
                ("brub", "BRUB") => STABLE::BRUB,
                ("bsar", "BSAR") => STABLE::BSAR,
                ("btry", "BTRY") => STABLE::BTRY,
                ("bzar", "BZAR") => STABLE::BZAR,
                // 其他情况
                _ => panic!("unknown StructTag: {:?}, {:?}", s.module, s.name),
            }
        }
    }
    impl From<TypeTag> for STABLE {
        fn from(s: TypeTag) -> Self {
            match s {
                TypeTag::Struct(s) => STABLE::from(*s),
                _ => panic!("unknown TypeTag: {:?}", s),
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
            //default busd
            Coin::type_(TypeTag::Struct(Box::new(STABLE::type_(&STABLE::BUSD))))
        }

        pub fn types_(stable: STABLE) -> StructTag {
            crate::coin::Coin::type_(TypeTag::Struct(Box::new(stable.type_())))
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

        pub fn layout(s: &StructTag) -> MoveStructLayout {
            Coin::layout(TypeTag::Struct(Box::new(STABLE::from(s.clone()).type_())))
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
