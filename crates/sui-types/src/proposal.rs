// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use crate::base_types::ObjectID;
use crate::error::SuiError;
use crate::id::{ID, UID};
use crate::object::{Data, Object};
use serde::Deserialize;
use crate::SUI_SYSTEM_ADDRESS;
use serde::Serialize;
use schemars::JsonSchema;

pub const PROPOSAL_MODULE_NAME: &IdentStr = ident_str!("Proposal");
pub const PROPOSAL_STRUCT_NAME: &IdentStr = ident_str!("Proposal");

#[derive(Debug, Serialize, JsonSchema, Deserialize, Clone, Eq, PartialEq)]
pub struct ProposalStatus{
    pub version_id : u64,
    pub status : u8,
}

#[derive(Debug, Serialize, JsonSchema, Deserialize, Clone, Eq, PartialEq)]
pub struct BFCDaoAction{
    pub action_id: u64,
    /// Name for the action
    pub name: String,
    pub status: bool,
}

#[derive(Debug, Serialize, JsonSchema, Deserialize, Clone, Eq, PartialEq)]
pub struct ProposalInfo{
    pub proposal_uid: ID,
    pub pid: u64,
    /// creator of the proposal
    pub proposer: ID,
    /// when voting begins.
    pub start_time: u64,
    /// when voting ends.
    pub end_time: u64,
    /// count of voters who agree with the proposal
    pub for_votes: u64,
    /// count of voters who're against the proposal
    pub against_votes: u64,
    /// executable after this time.
    pub eta: u64,
    /// after how long, the agreed proposal can be executed.
    pub action_delay: u64,
    /// how many votes to reach to make the proposal pass.
    pub quorum_votes: u64,
    /// proposal action.
    pub action: BFCDaoAction,
    pub version_id: u64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct Proposal{
    /// id of the proposal
    pub id: UID,
    pub proposal: ProposalInfo,
}

impl Proposal {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            module: PROPOSAL_MODULE_NAME.to_owned(),
            name: PROPOSAL_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn id(&self) -> ObjectID {
        self.id.id.bytes
    }
}

impl TryFrom<&Object> for Proposal {
    type Error = SuiError;
    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match &object.data {
            Data::Move(o) => {
                return bcs::from_bytes(o.contents()).map_err(|err| SuiError::TypeError {
                    error: format!("Unable to deserialize proposal object: {:?}", err),
                });
            }
            Data::Package(_) => {}
        }

        Err(SuiError::TypeError {
            error: format!("Object type is not a proposal: {:?}", object),
        })
    }
}