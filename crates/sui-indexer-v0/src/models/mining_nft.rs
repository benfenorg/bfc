use crate::{
    benfen,
    schema::{mining_nft_history_profits, mining_nft_staking, mining_nfts},
};
use chrono::Utc;
use diesel::prelude::*;
use move_core_types::{
    account_address::{account_address_util::convert_to_evm_address, AccountAddress},
    ident_str,
    parser::parse_struct_tag,
};
use serde::{Deserialize, Serialize};
use sui_json_rpc_types::{SuiMiningNFT, SuiMiningNFTStatus};
use sui_types::{base_types::ObjectID, id::ID};

use super::{checkpoints::Checkpoint, events::Event, objects::Object};

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = mining_nfts)]
pub struct MiningNFT {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub owner: String,
    pub miner_id: String,
    pub miner_url: String,
    pub miner_name: String,
    pub token_id: String,
    pub power: i64,
    pub mint_at: i64,
    pub earliest_held_at: i64,
    pub mint_duration: i64,
    pub mining_ticket_id: Option<String>,
    pub mining_started_at: i64,
    pub total_mint_bfc: i64,
    pub yesterday_mint_bfc: i64,
    pub yesterday_dt_ms: i64,
    pub miner_redeem: bool,
    pub transfered_at: i64,
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = mining_nft_staking)]
pub struct MiningNFTStaking {
    pub owner: String,
    pub ticket_id: String,
    pub miner_id: String,
    pub staked_at: i64,
    pub unstaked_at: Option<i64>,
    pub total_mint_bfc: i64,
}

#[derive(Queryable, Insertable, Clone, Debug, Default)]
#[diesel(table_name = mining_nft_history_profits)]
pub struct MiningNFTHistoryProfit {
    pub owner: String,
    pub miner_id: String,
    pub dt_timestamp_ms: i64,
    pub mint_bfc: i64,
    pub mint_usd: i64,
    pub pending_reward: i64,
    pub claimed_reward: i64,
}

#[derive(Debug, Clone)]
pub enum ExtractedMiningNFT {
    Minted(MintNFTEvent),
    Transfer(Object),
}

#[derive(Debug, Clone)]
pub enum MiningNFTOperation {
    StakingStake(StakeEvent),
    StakingUnstake(UnstakeEvent),
    StakingEmergencyUnstake(EmergencyUnstakeEvent),
    StakingTransferReward(TransferRewardEvent),
    BurnNFT(BurnNFTEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintNFTEvent {
    pub nft: NFTInfo,
    pub recipient: AccountAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnNFTEvent {
    pub nft: NFTInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeEvent {
    pub ticket_id: ID,
    pub nft: NFTInfo,              // 质押的 nft id
    pub recipient: AccountAddress, // ticket 接收者地址
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnstakeEvent {
    pub ticket_id: ID,
    pub nft: NFTInfo,              // 质押的 nft id
    pub recipient: AccountAddress, // ticket 接收者地址
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyUnstakeEvent {
    pub ticket_id: ID,
    pub nft: NFTInfo,              // 质押的 nft id
    pub recipient: AccountAddress, // ticket 接收者地址
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRewardEvent {
    pub ticket_id: ID,
    pub nft: NFTInfo, // 质押的 nft id
    pub reward: u64,
    pub recipient: AccountAddress, // ticket 接收者地址
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTInfo {
    pub id: ID,           // nft id
    pub token_id: String, // nft token id
    pub created_at: u64,  // nft 创建时间
    pub power: u64,
}

pub fn extract_from_events(
    contract: &str,
    events: &[Event],
) -> Result<Vec<ExtractedMiningNFT>, bcs::Error> {
    let address = AccountAddress::from_hex_literal(&convert_to_evm_address(contract.to_owned()))
        .unwrap_or(AccountAddress::ZERO);
    let mut results = vec![];
    for x in events.iter() {
        if let Ok(type_) = parse_struct_tag(&x.event_type) {
            if type_.address != address {
                continue;
            }
            if type_.module != ident_str!("event").into() {
                continue;
            }
            if type_.name == ident_str!("MintNFTEvent").into() {
                results.push(ExtractedMiningNFT::Minted(bcs::from_bytes::<MintNFTEvent>(
                    &x.event_bcs,
                )?))
            }
        }
    }
    Ok(results)
}

pub fn is_miner(contract: &str, value: &Object) -> bool {
    let address = AccountAddress::from_hex_literal(&convert_to_evm_address(contract.to_owned()))
        .unwrap_or(AccountAddress::ZERO);
    if let Ok(tag) = parse_struct_tag(&value.object_type) {
        // "0x00000000000000000000000000000000000000000000000000000000000000c8::bfc_dao::Proposal"
        return tag.address == address
            && tag.module.as_str() == "nft"
            && tag.name.as_str() == "Miner";
    }
    false
}

impl From<Object> for ExtractedMiningNFT {
    fn from(value: Object) -> Self {
        Self::Transfer(value)
    }
}

pub fn extract_operations_from_events(
    contract: &str,
    events: &[Event],
) -> Result<Vec<MiningNFTOperation>, bcs::Error> {
    let address = AccountAddress::from_hex_literal(&convert_to_evm_address(contract.to_owned()))
        .unwrap_or(AccountAddress::ZERO);
    let mut results = vec![];
    for x in events.iter() {
        if let Ok(type_) = parse_struct_tag(&x.event_type) {
            if type_.address != address {
                continue;
            }

            if type_.module != ident_str!("event").into() {
                continue;
            }
            if type_.name == ident_str!("StakeEvent").into() {
                results.push(MiningNFTOperation::StakingStake(bcs::from_bytes::<
                    StakeEvent,
                >(
                    &x.event_bcs
                )?));
            } else if type_.name == ident_str!("UnstakeEvent").into() {
                results.push(MiningNFTOperation::StakingUnstake(bcs::from_bytes::<
                    UnstakeEvent,
                >(
                    &x.event_bcs
                )?));
            } else if type_.name == ident_str!("EmergencyUnstakeEvent").into() {
                results.push(MiningNFTOperation::StakingEmergencyUnstake(
                    bcs::from_bytes::<EmergencyUnstakeEvent>(&x.event_bcs)?,
                ));
            } else if type_.name == ident_str!("TransferReward").into() {
                results.push(MiningNFTOperation::StakingTransferReward(
                    bcs::from_bytes::<TransferRewardEvent>(&x.event_bcs)?,
                ));
            } else if type_.name == ident_str!("BurnNFTEvent").into() {
                results.push(MiningNFTOperation::BurnNFT(
                    bcs::from_bytes::<BurnNFTEvent>(&x.event_bcs)?,
                ))
            }
        }
    }
    Ok(results)
}

impl From<(Checkpoint, ExtractedMiningNFT)> for MiningNFT {
    fn from(values: (Checkpoint, ExtractedMiningNFT)) -> Self {
        let (cp, value) = values;
        match value {
            ExtractedMiningNFT::Minted(v) => Self {
                id: None,
                owner: v.recipient.to_hex_literal(),
                miner_id: v.nft.id.bytes.to_hex_literal(),
                power: v.nft.power as i64,
                mint_at: v.nft.created_at as i64,
                earliest_held_at: cp.timestamp_ms,
                mint_duration: 0,
                mining_started_at: 0,
                mining_ticket_id: None,
                total_mint_bfc: 0,
                yesterday_mint_bfc: 0,
                yesterday_dt_ms: benfen::get_yesterday_started_at(),
                token_id: v.nft.token_id,
                miner_url: String::new(),
                miner_name: String::new(),
                miner_redeem: false,
                transfered_at: 0,
            },
            ExtractedMiningNFT::Transfer(object) => Self {
                id: None,
                owner: object
                    .owner_address
                    .unwrap_or(AccountAddress::ZERO.to_hex_literal()),
                miner_id: object.object_id,
                miner_url: String::new(),
                miner_name: String::new(),
                token_id: String::new(),
                power: 0,
                mint_at: cp.timestamp_ms / 1000,
                earliest_held_at: cp.timestamp_ms,
                mint_duration: 0,
                mining_ticket_id: None,
                mining_started_at: 0,
                total_mint_bfc: 0,
                yesterday_mint_bfc: 0,
                yesterday_dt_ms: benfen::get_yesterday_started_at(),
                miner_redeem: false,
                transfered_at: 0,
            },
        }
    }
}

impl Into<SuiMiningNFT> for MiningNFT {
    fn into(self) -> SuiMiningNFT {
        let mut mint_duration = self.mint_duration as u64;
        if self.mining_started_at > 0 {
            mint_duration += (Utc::now().timestamp() - self.mining_started_at) as u64;
        }
        SuiMiningNFT {
            owner: AccountAddress::from_hex_literal(&self.owner)
                .unwrap_or(AccountAddress::ZERO)
                .into(),
            miner_id: ObjectID::from_hex_literal(&self.miner_id).unwrap_or(ObjectID::ZERO),
            token_id: self.token_id,
            power: self.power as u64,
            mining_started_at: self.mining_started_at as u64,
            mint_at: self.mint_at as u64 * 1_000,
            mint_duration: mint_duration * 1_000,
            total_mined_bfc: self.total_mint_bfc as u64,
            status: if self.miner_redeem {
                // TODO(wanghui): change field
                SuiMiningNFTStatus::Redeem
            } else if self.mining_started_at > 0 {
                SuiMiningNFTStatus::Mining
            } else {
                SuiMiningNFTStatus::Idle
            },
            ticket_id: self
                .mining_ticket_id
                .map(|x| ObjectID::from_hex_literal(&x).unwrap_or(ObjectID::ZERO)),
            miner_url: self.miner_url,
            miner_name: self.miner_name,
        }
    }
}
