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
use crate::{id::UID, OBC_SYSTEM_ADDRESS, OBC_SYSTEM_STATE_OBJECT_ID};
use enum_dispatch::enum_dispatch;
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::balance::Balance;
use crate::base_types::ObjectID;
use crate::collection_types::VecMap;
use crate::gas_coin_strategy::GasCoinMap;
use crate::id::ID;

const OBC_SYSTEM_STATE_WRAPPER_STRUCT_NAME: &IdentStr = ident_str!("OBCSystemState");

pub const OBC_ROUND_FUNCTION_NAME: &IdentStr = ident_str!("obc_round");
pub const OBC_ROUND_SAFE_MODE_FUNCTION_NAME: &IdentStr = ident_str!("obc_round_safe_mode");
pub const OBC_SYSTEM_MODULE_NAME: &IdentStr = ident_str!("obc_system");

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
/// emitted when proposal created.
pub struct ProposalCreatedEvent{
    /// the proposal id.
    pub proposal_id: u64,
    /// proposer is the user who create the proposal.
    pub proposer: ID,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
/// global DAO info of the specified token type `Token`.
pub struct DaoGlobalInfo{
    pub id: UID,
    /// next proposal id.
    pub next_proposal_id: u64,

    // next action id
    pub next_action_id: u64,

    /// proposal creating event.
    pub proposal_create_event: ProposalCreatedEvent,
    /// voting event.
    pub vote_changed_event: VoteChangedEvent,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct OBCDaoAction{
    pub actionId: u64,
    /// Name for the action
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ProposalInfo {
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
    pub action: OBCDaoAction,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VotingPool{
    pub id: UID,
    /// The total number of OBC tokens in this pool,
    pub obc_balance: u64,
    /// Total number of pool tokens issued by the pool.
    pub pool_token_balance: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Dao  {
    pub id: UID,
    pub admin: ID,
    pub config: DaoConfig,
    pub info: DaoGlobalInfo,

    pub proposalRecord: VecMap<u64, ProposalInfo>,  //pid -> proposal address
    pub actionRecord: VecMap<u64, OBCDaoAction>,    //actionId -> action address
    pub votesRecord: VecMap<u64, u64>,  //pid -> vote count
    pub votingPool: VotingPool,
    pub curProposalStatus:  VecMap<u64, u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Bag {
    /// the ID of this bag
    id: UID,
    /// the number of key-value pairs in the bag
    size: u64,
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Treasury {
    pub id: UID,
    pub obc_balance: Balance,
    supplies: Bag,
    index: u64,
    time_interval: u32,
    updated_at: u64,
    init: bool,
}

#[derive(Debug)]
pub struct ObcRoundParams {
    pub round_id: u64,
}

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
            .expect("Update obc system object content cannot fail since it should be small");
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
    pub gas_coin_map: GasCoinMap,
    pub exchange_pool: ExchangePoolV1,
    pub dao: Dao,
    pub treasury: Treasury,
}

// Rust version of the Move obc_system::obc_system_state_inner::ExchangePool type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ExchangePoolV1 {
    pub id: ObjectID,
    pub activation_epoch: Option<u64>,
    pub obc_balance: u64,
    pub obc_pool: Balance,
    pub stable_token_balance: u64,
    pub stable_pool: Balance,
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[enum_dispatch(ObcSystemStateTrait)]
pub enum ObcSystemState {
    V1(ObcSystemStateInnerV1),
}

impl ObcSystemState {
    pub fn inner_state(self) -> ObcSystemStateInnerV1 {
        match self {
            ObcSystemState::V1(inner) => inner,
        }
    }
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
        .get_object(&OBC_SYSTEM_STATE_OBJECT_ID)?
        // Don't panic here on None because object_store is a generic store.
        .ok_or_else(|| {
            SuiError::SuiSystemStateReadError("ObcSystemStateWrapper object not found".to_owned())
        })?;
    let move_object = wrapper.data.try_as_move().ok_or_else(|| {
        SuiError::SuiSystemStateReadError(
            "ObcSystemStateWrapper object must be a Move object".to_owned(),
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
                            "Failed to load obc system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;
            Ok(ObcSystemState::V1(result))
        }
        _ => Err(SuiError::SuiSystemStateReadError(format!(
            "Unsupported ObcSystemState version: {}",
            wrapper.version
        ))),
    }
}

pub fn get_obc_system_proposal_state_map(object_store: &dyn ObjectStore) -> Result<VecMap<u64, u8>, SuiError> {
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

            Ok(result.dao.curProposalStatus)
        }
        _ => Err(SuiError::SuiSystemStateReadError(format!(
            "Unsupported SuiSystemState version: {}",
            wrapper.version
        ))),
    }
}


