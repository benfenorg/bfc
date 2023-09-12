// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use move_core_types::account_address::AccountAddress;

use crate::balance::Balance;
use crate::base_types::ObjectID;
use crate::committee::EpochId;
use crate::error::SuiError;
use crate::gas_coin::MIST_PER_SUI;
use crate::id::{ID, UID};
use crate::object::{Data, Object};
use serde::Deserialize;
use crate::SUI_SYSTEM_ADDRESS;
use serde::Serialize;
use schemars::{schema_for, JsonSchema};

pub const PROPOSAL_MODULE_NAME: &IdentStr = ident_str!("Proposal");
pub const PROPOSAL_STRUCT_NAME: &IdentStr = ident_str!("Proposal");

#[derive(Debug, Serialize, JsonSchema, Deserialize, Clone, Eq, PartialEq)]
pub struct OBCDaoAction{
    actionId: u64,
    /// Name for the action
    name: String,
}

#[derive(Debug, Serialize, JsonSchema, Deserialize, Clone, Eq, PartialEq)]
pub struct ProposalInfo{
    pid: u64,
    /// creator of the proposal
    proposer: ID,
    /// when voting begins.
    start_time: u64,
    /// when voting ends.
    end_time: u64,
    /// count of voters who agree with the proposal
    for_votes: u64,
    /// count of voters who're against the proposal
    against_votes: u64,
    /// executable after this time.
    eta: u64,
    /// after how long, the agreed proposal can be executed.
    action_delay: u64,
    /// how many votes to reach to make the proposal pass.
    quorum_votes: u64,
    /// proposal action.
    action: OBCDaoAction,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct Proposal{
    /// id of the proposal
    id: UID,
    proposal: ProposalInfo,
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

    pub fn is_proposal(s: &StructTag) -> bool {
        true
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
                    error: format!("Unable to deserialize propodal object: {:?}", err),
                });
            }
            Data::Package(_) => {}
        }

        Err(SuiError::TypeError {
            error: format!("Object type is not a propodal: {:?}", object),
        })
    }
}