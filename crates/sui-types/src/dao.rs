// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;
use crate::collection_types::VecMap;
use crate::base_types::ObjectID;
use crate::error::SuiError;
use crate::id::{ID, UID};
use crate::object::{Data, Object};
use serde::Deserialize;
use crate::SUI_SYSTEM_ADDRESS;
use serde::Serialize;
use schemars::JsonSchema;
use crate::proposal::{BFCDaoAction, ProposalInfo, ProposalStatus};
use std::collections::BTreeMap;

pub const DAO_MODULE_NAME: &IdentStr = ident_str!("Dao");
pub const DAO_STRUCT_NAME: &IdentStr = ident_str!("Dao");

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct VotingPool{
    pub id: UID,
    /// The total number of BFC tokens in this pool,
    pub bfc_balance: u64,
    /// Total number of pool tokens issued by the pool.
    pub pool_token_balance: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Dao  {
    pub id: UID,
    pub admin: ID,
    pub config: DaoConfig,
    pub info: DaoGlobalInfo,

    pub proposal_record: VecMap<u64, ProposalInfo>,  //pid -> proposal address
    pub action_record: VecMap<u64, BFCDaoAction>,    //actionId -> action address
    pub votes_record: VecMap<u64, u64>,  //pid -> vote count
    pub voting_pool: VotingPool,
    pub current_proposal_status:  VecMap<u64, ProposalStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct DaoRPC  {
    pub id: UID,
    pub admin: ID,
    pub config: DaoConfig,
    pub info: DaoGlobalInfo,
    pub proposal_record: Vec<ProposalInfo>,
    pub action_record: BTreeMap<u64, BFCDaoAction>,
    pub votes_record: BTreeMap<u64, u64>,  //pid -> vote count
    pub voting_pool: VotingPool,
    pub current_proposal_status:  BTreeMap<u64, ProposalStatus>,
}

impl DaoRPC {
    pub fn type_() -> StructTag {
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            module: DAO_MODULE_NAME.to_owned(),
            name: DAO_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn id(&self) -> ObjectID {
        self.id.id.bytes
    }
}

impl TryFrom<&Object> for DaoRPC {
    type Error = SuiError;
    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match &object.data {
            Data::Move(o) => {
                return bcs::from_bytes(o.contents()).map_err(|err| SuiError::TypeError {
                    error: format!("Unable to deserialize dao object: {:?}", err),
                });
            }
            Data::Package(_) => {}
        }

        Err(SuiError::TypeError {
            error: format!("Object type is not a dao: {:?}", object),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
pub struct DaoConfig{
    /// after proposal created, how long use should wait before he can vote (in milliseconds)
    pub voting_delay: u64,
    /// how long the voting window is (in milliseconds).
    pub voting_period: u64,
    /// the quorum rate to agree on the proposal.
    /// if 50% votes needed, then the voting_quorum_rate should be 50.
    /// it should between (0, 100].
    pub voting_quorum_rate: u8,
    /// how long the proposal should wait before it can be executed (in milliseconds).
    pub min_action_delay: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
/// global DAO info of the specified token type `Token`.
pub struct DaoGlobalInfo{
    //pub id: UID,
    // next proposal id.
    pub next_proposal_id: u64,

    // next action id
    pub next_action_id: u64,

    // proposal creating event.
    //pub proposal_create_event: ProposalCreatedEvent,
    // voting event.
    //pub vote_changed_event: VoteChangedEvent,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
/// emitted when user vote/revoke_vote.
pub struct VoteChangedEvent{
    /// the proposal id.
    pub proposal_id: u64,
    /// the voter.
    pub voter: ID,
    /// creator of the proposal.
    pub proposer: ID,
    /// agree with the proposal or not
    pub agree: bool,
    /// latest vote count of the voter.
    pub vote: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
/// emitted when proposal created.
pub struct ProposalCreatedEvent{
    /// the proposal id.
    pub proposal_id: u64,
    /// proposer is the user who create the proposal.
    pub proposer: ID,
}