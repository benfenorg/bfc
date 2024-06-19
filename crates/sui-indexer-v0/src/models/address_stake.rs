use diesel::{Insertable, Queryable};
use move_core_types::{
    account_address::AccountAddress, ident_str, language_storage::StructTag,
    parser::parse_struct_tag,
};
use serde::{Deserialize, Serialize};
use sui_types::{id::ID, SUI_FRAMEWORK_ADDRESS, SUI_SYSTEM_ADDRESS};

use crate::{errors::IndexerError, schema::address_stakes};

use super::{checkpoints::Checkpoint, events::Event, objects::Object};

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = address_stakes)]
pub struct AddressStake {
    pub staked_object_id: String,
    pub staker_address: String,
    pub pool_id: String,
    pub validator_address: String,
    pub stake_coin: String,

    pub principal_epoch: i64,
    pub principal_amount: i64,
    pub principal_timestamp_ms: i64,

    pub estimated_reward: i64,
    pub estimated_at_epoch: i64,

    pub stake_activation_epoch: i64,

    pub unstaking_epoch: Option<i64>,
    pub unstaking_amount: Option<i64>,
    pub unstaking_timestamp_ms: Option<i64>,
    pub unstaking_reward_amount: Option<i64>,

    pub timestamp_ms: i64,
}

fn staked_type() -> (StructTag, StructTag) {
    (
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            module: ident_str!("staking_pool").into(),
            name: ident_str!("StakedBfc").into(),
            type_params: vec![],
        },
        StructTag {
            address: SUI_SYSTEM_ADDRESS,
            module: ident_str!("stable_pool").into(),
            name: ident_str!("StakedStable").into(),
            type_params: vec![],
        },
    )
}

pub enum ExtractedAddressStake {
    Staking(ExtractedStaking),
    Unstaking(ExtractedUnstaking),
}

pub struct ExtractedStaking(String, String, Event);
pub struct ExtractedUnstaking(Event);

pub fn extract(objects: &[Object], events: &[Event]) -> Option<ExtractedAddressStake> {
    if let Some(v) = extract_staking(objects, events) {
        return Some(ExtractedAddressStake::Staking(v));
    }
    if let Some(v) = extract_unstaking(events) {
        return Some(ExtractedAddressStake::Unstaking(v));
    }
    None
}

pub fn extract_staking(objects: &[Object], events: &[Event]) -> Option<ExtractedStaking> {
    let event_type: StructTag = StructTag {
        address: SUI_SYSTEM_ADDRESS,
        module: ident_str!("validator").into(),
        name: ident_str!("StakingRequestEvent").into(),
        type_params: vec![],
    };
    let event = events.iter().find(|x| {
        if let Ok(type_) = parse_struct_tag(&x.event_type) {
            type_ == event_type
        } else {
            false
        }
    })?;
    let object = filter_staked_object(objects)?;
    Some(ExtractedStaking(
        object.object_id.clone(),
        object.object_type.clone(),
        event.clone(),
    ))
}

pub fn extract_unstaking(events: &[Event]) -> Option<ExtractedUnstaking> {
    let event_type: StructTag = StructTag {
        address: SUI_SYSTEM_ADDRESS,
        module: ident_str!("validator").into(),
        name: ident_str!("UnstakingRequestEvent").into(),
        type_params: vec![],
    };
    let event = events.iter().find(|x| {
        if let Ok(type_) = parse_struct_tag(&x.event_type) {
            type_ == event_type
        } else {
            false
        }
    })?;
    Some(ExtractedUnstaking(event.clone()))
}

pub fn filter_staked_object(objects: &[Object]) -> Option<&Object> {
    objects.iter().find(|x| is_staked_object(&x.object_type))
}

fn is_staked_object(type_: &str) -> bool {
    let (bfc_type, stable_type) = staked_type();
    if let Ok(type_) = parse_struct_tag(type_) {
        type_ == bfc_type
            || (type_.address == stable_type.address
                && type_.module == stable_type.module
                && type_.name == stable_type.name)
    } else {
        false
    }
}

impl TryFrom<(Checkpoint, ExtractedStaking)> for AddressStake {
    type Error = IndexerError;
    fn try_from(value: (Checkpoint, ExtractedStaking)) -> Result<Self, Self::Error> {
        let (checkpoint, ExtractedStaking(object_id, object_type, event)) = value;
        let object_type = parse_struct_tag(&object_type)?;
        let request_event: StakingRequestEvent = bcs::from_bytes(&event.event_bcs)?;
        let stake_coin = get_stake_coin(&object_type);
        Ok(AddressStake {
            staked_object_id: object_id,
            staker_address: request_event.staker_address.to_hex_literal(),
            pool_id: request_event.pool_id.bytes.to_string(),
            validator_address: request_event.validator_address.to_hex_literal(),
            stake_coin,
            principal_amount: request_event.amount as i64,
            principal_epoch: request_event.epoch as i64,
            principal_timestamp_ms: checkpoint.timestamp_ms as i64,
            estimated_at_epoch: request_event.epoch as i64,
            estimated_reward: 0,
            stake_activation_epoch: request_event.epoch as i64 + 1,
            unstaking_epoch: None,
            unstaking_amount: None,
            unstaking_timestamp_ms: None,
            unstaking_reward_amount: None,
            timestamp_ms: cached::instant::now() as i64,
        })
    }
}

impl TryFrom<(Checkpoint, Object, ExtractedUnstaking)> for AddressStake {
    type Error = IndexerError;

    fn try_from(value: (Checkpoint, Object, ExtractedUnstaking)) -> Result<Self, Self::Error> {
        let (checkpoint, object, ExtractedUnstaking(event)) = value;
        let object_id = object.object_id;
        let object_type = parse_struct_tag(&object.object_type)?;
        let request_event: UnstakingRequestEvent = bcs::from_bytes(&event.event_bcs)?;
        let stake_coin = get_stake_coin(&object_type);
        let withdraw_amount = request_event.principal_amount + request_event.reward_amount;

        Ok(AddressStake {
            staked_object_id: object_id,
            staker_address: request_event.staker_address.to_hex_literal(),
            pool_id: request_event.pool_id.bytes.to_string(),
            validator_address: request_event.validator_address.to_hex_literal(),
            stake_coin,
            principal_amount: request_event.principal_amount as i64,
            principal_epoch: (request_event.stake_activation_epoch - 1) as i64,
            principal_timestamp_ms: checkpoint.timestamp_ms as i64,
            estimated_at_epoch: request_event.unstaking_epoch as i64,
            estimated_reward: request_event.reward_amount as i64,
            stake_activation_epoch: request_event.stake_activation_epoch as i64,
            unstaking_epoch: Some(request_event.unstaking_epoch as i64),
            unstaking_timestamp_ms: Some(checkpoint.timestamp_ms as i64),
            unstaking_amount: Some(withdraw_amount as i64),
            unstaking_reward_amount: Some(request_event.reward_amount as i64),
            timestamp_ms: cached::instant::now() as i64,
        })
    }
}

fn get_stake_coin(object_type: &StructTag) -> String {
    let (bfc_type, _) = staked_type();
    if object_type.module == bfc_type.module {
        native_coin().to_string()
    } else {
        object_type.type_params[0].to_string()
    }
}

pub fn native_coin() -> StructTag {
    StructTag {
        address: SUI_FRAMEWORK_ADDRESS,
        module: ident_str!("bfc").into(),
        name: ident_str!("BFC").into(),
        type_params: vec![],
    }
}

// Rust types re-defined in sui-framework/packages/sui-system/sources/validator.move
#[derive(Debug, Serialize, Deserialize, Clone)]
struct StakingRequestEvent {
    pool_id: ID,
    validator_address: AccountAddress,
    staker_address: AccountAddress,
    epoch: u64,
    amount: u64,
}

// Rust types re-defined in sui-framework/packages/sui-system/sources/validator.move
#[derive(Debug, Serialize, Deserialize, Clone)]
struct UnstakingRequestEvent {
    pool_id: ID,
    validator_address: AccountAddress,
    staker_address: AccountAddress,
    stake_activation_epoch: u64,
    unstaking_epoch: u64,
    principal_amount: u64,
    reward_amount: u64,
}
