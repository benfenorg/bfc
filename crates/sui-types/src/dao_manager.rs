// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use crate::error::SuiError;
use crate::id::UID;
use crate::object::{Data, Object};
use serde::Deserialize;
use serde::Serialize;
use schemars::JsonSchema;

pub const MANAGE_MODULE_NAME: &IdentStr = ident_str!("BFCDaoManageKey");
pub const MANAGE_STRUCT_NAME: &IdentStr = ident_str!("BFCDaoManageKey");

#[derive(Debug, Serialize, JsonSchema, Deserialize, Clone, Eq, PartialEq)]
struct BFCDaoManageKey {
    id: UID,
}

impl BFCDaoManageKey {}

impl TryFrom<&Object> for BFCDaoManageKey {
    type Error = SuiError;
    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match &object.data {
            Data::Move(o) => {
                return bcs::from_bytes(o.contents()).map_err(|err| SuiError::TypeError {
                    error: format!("Unable to deserialize BFCDaoManageKey object: {:?}", err),
                });
            }
            Data::Package(_) => {}
        }

        Err(SuiError::TypeError {
            error: format!("Object type is not a BFCDaoManageKey: {:?}", object),
        })
    }
}