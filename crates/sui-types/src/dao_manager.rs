// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use crate::base_types::ObjectID;
use crate::error::SuiError;
use crate::id::UID;
use crate::object::{Data, Object};
use serde::Deserialize;
use crate::SUI_SYSTEM_ADDRESS;
use serde::Serialize;
use schemars::JsonSchema;

pub const MANAGE_MODULE_NAME: &IdentStr = ident_str!("OBCDaoManageKey");
pub const MANAGE_STRUCT_NAME: &IdentStr = ident_str!("OBCDaoManageKey");

#[derive(Debug, Serialize, JsonSchema, Deserialize, Clone, Eq, PartialEq)]
struct OBCDaoManageKey {
    id: UID,
}

impl OBCDaoManageKey {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            module: MANAGE_MODULE_NAME.to_owned(),
            name: MANAGE_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn id(&self) -> ObjectID {
        self.id.id.bytes
    }
}

impl TryFrom<&Object> for OBCDaoManageKey {
    type Error = SuiError;
    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match &object.data {
            Data::Move(o) => {
                return bcs::from_bytes(o.contents()).map_err(|err| SuiError::TypeError {
                    error: format!("Unable to deserialize OBCDaoManageKey object: {:?}", err),
                });
            }
            Data::Package(_) => {}
        }

        Err(SuiError::TypeError {
            error: format!("Object type is not a OBCDaoManageKey: {:?}", object),
        })
    }
}