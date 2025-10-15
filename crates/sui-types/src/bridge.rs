// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::base_types::ObjectID;
use crate::base_types::SequenceNumber;
use crate::collection_types::LinkedTableNode;
use crate::collection_types::VecSet;
use crate::dynamic_field::{get_dynamic_field_from_store, Field};
use crate::error::SuiResult;
use crate::object::Owner;
use crate::storage::ObjectStore;
use crate::sui_serde::BigInt;
use crate::sui_serde::Readable;
use crate::versioned::Versioned;
use crate::SUI_BRIDGE_OBJECT_ID;
use crate::{
    base_types::SuiAddress,
    collection_types::{Bag, LinkedTable, VecMap},
    error::SuiError,
    id::UID,
};
use enum_dispatch::enum_dispatch;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use num_enum::TryFromPrimitive;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use strum_macros::Display;

pub type BridgeInnerDynamicField = Field<u64, BridgeInnerV1>;
pub type BridgeRecordDyanmicField = Field<
    MoveTypeBridgeMessageKey,
    LinkedTableNode<MoveTypeBridgeMessageKey, MoveTypeBridgeRecord>,
>;

pub type ExternalBridgeRecordDyanmicField = Field<
    MoveTypeExternalBridgeMessageKey,
    LinkedTableNode<MoveTypeExternalBridgeMessageKey, MoveTypeExternalBridgeRecord>,
>;

pub const BRIDGE_MODULE_NAME: &IdentStr = ident_str!("bridge");
pub const BRIDGE_TREASURY_MODULE_NAME: &IdentStr = ident_str!("treasury");
pub const BRIDGE_LIMITER_MODULE_NAME: &IdentStr = ident_str!("limiter");
pub const BRIDGE_COMMITTEE_MODULE_NAME: &IdentStr = ident_str!("committee");
pub const BRIDGE_MESSAGE_MODULE_NAME: &IdentStr = ident_str!("message");
pub const BRIDGE_CREATE_FUNCTION_NAME: &IdentStr = ident_str!("create");
pub const BRIDGE_INIT_COMMITTEE_FUNCTION_NAME: &IdentStr = ident_str!("init_bridge_committee");
pub const BRIDGE_REGISTER_FOREIGN_TOKEN_FUNCTION_NAME: &IdentStr =
    ident_str!("register_foreign_token");
pub const BRIDGE_CREATE_ADD_TOKEN_ON_SUI_MESSAGE_FUNCTION_NAME: &IdentStr =
    ident_str!("create_add_tokens_on_sui_message");
pub const BRIDGE_EXECUTE_SYSTEM_MESSAGE_FUNCTION_NAME: &IdentStr =
    ident_str!("execute_system_message");
pub const BRIDGE_ADD_TOKENLIST_FUNCTION_NAME: &IdentStr = ident_str!("init_token_list");
pub const BRIDGE_ADD_CENTER_TOKENLIST_FUNCTION_NAME: &IdentStr = ident_str!("migrate");

pub const BRIDGE_SUPPORTED_ASSET: &[&str] = &["btc", "eth", "usdc", "usdt"];

pub const BRIDGE_COMMITTEE_MINIMAL_VOTING_POWER: u64 = 7500; // out of 10000 (75%)
pub const BRIDGE_COMMITTEE_MAXIMAL_VOTING_POWER: u64 = 10000; // (100%)

// Threshold for action to be approved by the committee (our of 10000)
pub const APPROVAL_THRESHOLD_TOKEN_TRANSFER: u64 = 3334;
pub const APPROVAL_THRESHOLD_EMERGENCY_PAUSE: u64 = 450;
pub const APPROVAL_THRESHOLD_EMERGENCY_UNPAUSE: u64 = 5001;
pub const APPROVAL_THRESHOLD_COMMITTEE_BLOCKLIST: u64 = 5001;
pub const APPROVAL_THRESHOLD_LIMIT_UPDATE: u64 = 5001;
pub const APPROVAL_THRESHOLD_ASSET_PRICE_UPDATE: u64 = 5001;
pub const APPROVAL_THRESHOLD_EVM_CONTRACT_UPGRADE: u64 = 5001;
pub const APPROVAL_THRESHOLD_ADD_TOKENS_ON_SUI: u64 = 5001;
pub const APPROVAL_THRESHOLD_ADD_TOKENS_ON_EVM: u64 = 5001;
pub const APPROVAL_THRESHOLD_REFUND_ADMIN: u64 = 5001;
pub const APPROVAL_THRESHOLD_FAST_PATH_LIMIT_UPDATE: u64 = 5001;
pub const APPROVAL_THRESHOLD_EXTERNAL_COIN_ADMIN: u64 = 5001;
pub const APPROVAL_THRESHOLD_EXTERNAL_COIN_WITNESS: u64 = 5001;
pub const APPROVAL_THRESHOLD_EXTERNAL_COIN_TARGET: u64 = 5001;
pub const APPROVAL_THRESHOLD_ADD_TOKEN_ON_TOKEN_LIST: u64 = 5001;
pub const APPROVAL_THRESHOLD_REMOVE_TOKEN_ON_TOKEN_LIST: u64 = 5001;
pub const APPROVAL_THRESHOLD_SINGLE_TRANSFER_LIMIT_UPDATE: u64=5001;
pub const APPROVAL_THRESHOLD_SET_CROSS_OUT_BRIDGE_FEE: u64 = 5001;
pub const APPROVAL_THRESHOLD_SET_CROSS_IN_BRIDGE_FEE: u64 = 5001;
pub const APPROVAL_THRESHOLD_WITHDRAW_BRIDGE_FEE: u64=5001;
pub const APPROVAL_THRESHOLD_UPDATE_INVEST_ADDRESS: u64 = 5001;
pub const APPROVAL_THRESHOLD_ADD_LP_TOKEN_ID: u64 = 5001;

// const for initial token ids for convenience
pub const TOKEN_ID_SUI: u64 = 0;
pub const TOKEN_ID_BTC: u64 = 1;
pub const TOKEN_ID_ETH: u64 = 2;
pub const TOKEN_ID_USDC: u64 = 3;
pub const TOKEN_ID_USDT: u64 = 4;
pub const TOKEN_ID_BUSD: u64 = 5;
pub const TOKEN_ID_BNB: u64 = 6;
pub const TOKEN_ID_OP: u64 = 7;
pub const TOKEN_ID_POL: u64 = 8;

// const for fast path
pub const FAST_PATH_THREASHOLD_LATEST_BUSD: u64 = 100_000_000_000;
pub const FAST_PATH_THREASHOLD_SAFE_BUSD: u64 = 10000_000_000_000;


#[derive(
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
    Copy,
    TryFromPrimitive,
    JsonSchema,
    Hash,
    Display,
    PartialOrd,
    Ord,
)]
#[repr(u8)]
pub enum BridgeChainId {
    SuiMainnet = 0,
    SuiTestnet = 1,
    SuiCustom = 2,

    EthMainnet = 10,
    EthSepolia = 11,
    EthCustom = 12,

    // benfen btc
    BtcMainnet = 20,
    BtcTestnet = 21,

    BscMainnet = 30,
    BscTestnet = 31,
    BscCustom = 32,

    OPMainnet = 33,
    OPTestnet = 34,
    OPCustom = 35,

    ArbMainnet = 36,
    ArbTestnet = 37,
    ArbCustom = 38,

    PolMainnet = 39,
    PolTestnet = 40,
    PolCustom = 41,

    BaseMainnet = 42,
    BaseTestnet = 43,
    BaseCustom = 44,

    AvaxMainnet = 45,
    AvaxTestnet = 46,
    AvaxCustom = 47,

    TronMainnet = 48,
    TronTestnet = 49,

    SolanaMainnet = 50,
    SolanaTestnet = 51,

    LTCMainnet = 52,
    LTCTestnet = 53,

    DogeMainnet = 54,
    DogeTestnet = 55,

    SuiOfficialMainnet = 56,
    SuiOfficialTestnet = 57,

    AptosMainnet = 58,
    AptosTestnet = 59,
}

impl BridgeChainId {
    pub fn is_sui_chain(&self) -> bool {
        matches!(
            self,
            BridgeChainId::SuiMainnet | BridgeChainId::SuiTestnet | BridgeChainId::SuiCustom
        )
    }
    pub fn is_bsc_chain(&self) -> bool {
        matches!(
            self,
            BridgeChainId::BscMainnet | BridgeChainId::BscTestnet | BridgeChainId::BscCustom
        )
    }
    pub fn is_eth_chain(&self) -> bool {
        matches!(
            self,
            BridgeChainId::EthMainnet | BridgeChainId::EthSepolia | BridgeChainId::EthCustom
        )
    }

    pub fn is_op_chain(&self) -> bool {
        matches!(
            self,
            BridgeChainId::OPMainnet | BridgeChainId::OPTestnet | BridgeChainId::OPCustom
        )
    }

    pub fn is_arb_chain(&self) -> bool {
        matches!(
            self,
            BridgeChainId::ArbMainnet | BridgeChainId::ArbTestnet | BridgeChainId::ArbCustom
        )
    }

    pub fn is_pol_chain(&self) -> bool {
        matches!(
            self,
            BridgeChainId::PolMainnet | BridgeChainId::PolTestnet | BridgeChainId::PolCustom
        )
    }

    pub fn is_base_chain(&self) -> bool {
        matches!(
            self,
            BridgeChainId::BaseMainnet | BridgeChainId::BaseTestnet | BridgeChainId::BaseCustom
        )
    }

    pub fn is_optimism_chain(&self) -> bool {
        matches!(
            self,
            BridgeChainId::OPMainnet | BridgeChainId::OPTestnet | BridgeChainId::OPCustom
        )
    }

    pub fn is_avax_chain(&self) -> bool {
        matches!(
            self,
            BridgeChainId::AvaxMainnet | BridgeChainId::AvaxTestnet | BridgeChainId::AvaxCustom
        )
    }

    pub fn is_eth_by_id(id: u8) -> bool {
        let eth_mainnet = BridgeChainId::EthMainnet as u8;
        let eth_sepolia = BridgeChainId::EthSepolia as u8;
        let eth_custom = BridgeChainId::EthCustom as u8;
        id == eth_mainnet || id == eth_sepolia || id == eth_custom
    }
    pub fn is_bsc_by_id(id: u8) -> bool {
        let bsc_mainnet = BridgeChainId::BscMainnet as u8;
        let bsc_testnet = BridgeChainId::BscTestnet as u8;
        let bsc_custom = BridgeChainId::BscCustom as u8;
        id == bsc_mainnet || id == bsc_testnet || id == bsc_custom
    }
    pub fn is_op_by_id(id: u8) -> bool {
        let op_mainnet = BridgeChainId::OPMainnet as u8;
        let op_testnet = BridgeChainId::OPTestnet as u8;
        let op_custom = BridgeChainId::OPCustom as u8;
        id == op_mainnet || id == op_testnet || id == op_custom
    }
    pub fn is_base_by_id(id: u8) -> bool {
        let base_mainnet = BridgeChainId::BaseMainnet as u8;
        let base_testnet = BridgeChainId::BaseTestnet as u8;
        let base_custom = BridgeChainId::BaseCustom as u8;
        id == base_mainnet || id == base_testnet || id == base_custom
    }

    pub fn is_custom_chain_by_id(id: u8) -> bool {
        id == BridgeChainId::BscCustom as u8
            || id == BridgeChainId::SuiCustom as u8
            || id == BridgeChainId::EthCustom as u8
            || id == BridgeChainId::OPCustom as u8
            || id == BridgeChainId::ArbCustom as u8
            || id == BridgeChainId::PolCustom as u8
            || id == BridgeChainId::BaseCustom as u8
            || id == BridgeChainId::AvaxCustom as u8
    }
}

pub fn get_bridge_obj_initial_shared_version(
    object_store: &dyn ObjectStore,
) -> SuiResult<Option<SequenceNumber>> {
    Ok(object_store
        .get_object(&SUI_BRIDGE_OBJECT_ID)
        .map(|obj| match obj.owner {
            Owner::Shared {
                initial_shared_version,
            } => initial_shared_version,
            _ => unreachable!("Bridge object must be shared"),
        }))
}

/// Bridge provides an abstraction over multiple versions of the inner BridgeInner object.
/// This should be the primary interface to the bridge object in Rust.
/// We use enum dispatch to dispatch all methods defined in BridgeTrait to the actual
/// implementation in the inner types.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_dispatch(BridgeTrait)]
pub enum Bridge {
    V1(BridgeInnerV1),
}

/// Rust version of the Move sui::bridge::Bridge type
/// This repreents the object with 0x9 ID.
/// In Rust, this type should be rarely used since it's just a thin
/// wrapper used to access the inner object.
/// Within this module, we use it to determine the current version of the bridge inner object type,
/// so that we could deserialize the inner object correctly.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BridgeWrapper {
    pub id: UID,
    pub version: Versioned,
}

/// This is the standard API that all bridge inner object type should implement.
#[enum_dispatch]
pub trait BridgeTrait {
    fn bridge_version(&self) -> u64;
    fn message_version(&self) -> u8;
    fn chain_id(&self) -> u8;
    fn sequence_nums(&self) -> &VecMap<u8, u64>;
    fn committee(&self) -> &MoveTypeBridgeCommittee;
    fn treasury(&self) -> &MoveTypeBridgeTreasury;
    fn bridge_records(&self) -> &LinkedTable<MoveTypeBridgeMessageKey>;
    fn frozen(&self) -> bool;
    fn try_into_bridge_summary(self, external_limiter: MoveTypeBridgeExternalLimiter) -> SuiResult<BridgeSummary>;
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BridgeSummary {
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "Readable<BigInt<u64>, _>")]
    pub bridge_version: u64,
    // Message version
    pub message_version: u8,
    /// Self Chain ID
    pub chain_id: u8,
    /// Sequence numbers of all message types
    #[schemars(with = "Vec<(u8, BigInt<u64>)>")]
    #[serde_as(as = "Vec<(_, Readable<BigInt<u64>, _>)>")]
    pub sequence_nums: Vec<(u8, u64)>,
    pub committee: BridgeCommitteeSummary,
    /// Summary of the treasury
    pub treasury: BridgeTreasurySummary,
    /// Object ID of bridge Records (dynamic field)
    pub bridge_records_id: ObjectID,
    /// Summary of the limiter
    pub limiter: BridgeLimiterSummary,
    /// Whether the bridge is currently frozen or not
    pub is_frozen: bool,
    // TODO: add treasury
}

impl Default for BridgeSummary {
    fn default() -> Self {
        BridgeSummary {
            bridge_version: 1,
            message_version: 1,
            chain_id: 1,
            sequence_nums: vec![],
            committee: BridgeCommitteeSummary::default(),
            treasury: BridgeTreasurySummary::default(),
            bridge_records_id: ObjectID::random(),
            limiter: BridgeLimiterSummary::default(),
            is_frozen: false,
        }
    }
}

pub fn get_bridge_wrapper(object_store: &dyn ObjectStore) -> Result<BridgeWrapper, SuiError> {
    let wrapper = object_store
        .get_object(&SUI_BRIDGE_OBJECT_ID)
        // Don't panic here on None because object_store is a generic store.
        .ok_or_else(|| SuiError::SuiBridgeReadError("BridgeWrapper object not found".to_owned()))?;
    let move_object = wrapper.data.try_as_move().ok_or_else(|| {
        SuiError::SuiBridgeReadError("BridgeWrapper object must be a Move object".to_owned())
    })?;
    let result = bcs::from_bytes::<BridgeWrapper>(move_object.contents())
        .map_err(|err| SuiError::SuiBridgeReadError(err.to_string()))?;
    Ok(result)
}

pub fn get_bridge(object_store: &dyn ObjectStore) -> Result<Bridge, SuiError> {
    let wrapper = get_bridge_wrapper(object_store)?;
    let id = wrapper.version.id.id.bytes;
    let version = wrapper.version.version;
    match version {
        1 => {
            let result: BridgeInnerV1 = get_dynamic_field_from_store(object_store, id, &version)
                .map_err(|err| {
                    SuiError::SuiBridgeReadError(format!(
                        "Failed to load bridge inner object with ID {:?} and version {:?}: {:?}",
                        id, version, err
                    ))
                })?;
            Ok(Bridge::V1(result))
        }
        _ => Err(SuiError::SuiBridgeReadError(format!(
            "Unsupported SuiBridge version: {}",
            version
        ))),
    }
}

pub fn get_bridge_external_limiter(
    object_store: &dyn ObjectStore,
) -> Result<MoveTypeBridgeExternalLimiter, SuiError> {
    let wrapper = get_bridge_wrapper(object_store)?;
    let id = wrapper.id.id.bytes;
    let key = "bridge_external_limits";
    let limiter: MoveTypeBridgeExternalLimiter = get_dynamic_field_from_store(
        object_store,
        id,
        &key.to_string().into_bytes(),
    ).map_err(|err| SuiError::SuiBridgeReadError(
        format!(
            "Failed to load bridge external limiter with ID {:?} and key {:?}: {:?}",
            id, key, err
        )
    ))?;
    Ok(limiter)
}

/// Rust version of the Move bridge::BridgeInner type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BridgeInnerV1 {
    pub bridge_version: u64,
    pub message_version: u8,
    pub chain_id: u8,
    pub sequence_nums: VecMap<u8, u64>,
    pub committee: MoveTypeBridgeCommittee,
    pub treasury: MoveTypeBridgeTreasury,
    pub bridge_records: LinkedTable<MoveTypeBridgeMessageKey>,
    pub external_bridge_records: LinkedTable<MoveTypeExternalBridgeMessageKey>,
    pub pre_deposit_multi_signature_records: LinkedTable<MoveTypeExternalBridgeMessageKey>,
    pub limiter: MoveTypeBridgeTransferLimiter,
    pub frozen: bool,
    pub refund_records: LinkedTable<MoveTypeRefundMessageKey>,
    pub refund_admins: VecSet<String>,
    // pub bfc_system_id: UID,
    pub defi_holders: LinkedTable<SuiAddress>,
}

impl BridgeTrait for BridgeInnerV1 {
    fn bridge_version(&self) -> u64 {
        self.bridge_version
    }

    fn message_version(&self) -> u8 {
        self.message_version
    }

    fn chain_id(&self) -> u8 {
        self.chain_id
    }

    fn sequence_nums(&self) -> &VecMap<u8, u64> {
        &self.sequence_nums
    }

    fn committee(&self) -> &MoveTypeBridgeCommittee {
        &self.committee
    }

    fn treasury(&self) -> &MoveTypeBridgeTreasury {
        &self.treasury
    }

    fn bridge_records(&self) -> &LinkedTable<MoveTypeBridgeMessageKey> {
        &self.bridge_records
    }

    fn frozen(&self) -> bool {
        self.frozen
    }

    fn try_into_bridge_summary(self, external_limiter: MoveTypeBridgeExternalLimiter) -> SuiResult<BridgeSummary> {
        let transfer_limit = self
            .limiter
            .transfer_limit
            .contents
            .into_iter()
            .map(|e| {
                let source = BridgeChainId::try_from(e.key.source).map_err(|_e| {
                    SuiError::GenericBridgeError {
                        error: format!("Unrecognized chain id: {}", e.key.source),
                    }
                })?;
                let destination = BridgeChainId::try_from(e.key.destination).map_err(|_e| {
                    SuiError::GenericBridgeError {
                        error: format!("Unrecognized chain id: {}", e.key.destination),
                    }
                })?;
                Ok((source, destination, e.value))
            })
            .collect::<SuiResult<Vec<_>>>()?;
        let transfer_out_limits = external_limiter
            .transfer_out_limits
            .contents
            .into_iter()
            .map(|e| {
                let source = BridgeChainId::try_from(e.key.source).map_err(|_e| {
                    SuiError::GenericBridgeError {
                        error: format!("Unrecognized chain id: {}", e.key.source),
                    }
                })?;
                let destination = BridgeChainId::try_from(e.key.destination).map_err(|_e| {
                    SuiError::GenericBridgeError {
                        error: format!("Unrecognized chain id: {}", e.key.destination),
                    }
                })?;
                Ok((source, destination, e.value))
            })
            .collect::<SuiResult<Vec<_>>>()?;

        let external_coin_target_address = self
            .treasury
            .external_coin_target_address
            .contents
            .into_iter()
            .map(|e| (e.key, e.value.contents))
            .collect::<Vec<_>>();

        let supported_tokens = self
            .treasury
            .supported_tokens
            .contents
            .into_iter()
            .map(|e| (e.key, e.value))
            .collect::<Vec<_>>();
        let id_token_type_map = self
            .treasury
            .id_token_type_map
            .contents
            .into_iter()
            .map(|e| (e.key, e.value))
            .collect::<Vec<_>>();
        let transfer_records = self
            .limiter
            .transfer_records
            .contents
            .into_iter()
            .map(|e| {
                let source = BridgeChainId::try_from(e.key.source).map_err(|_e| {
                    SuiError::GenericBridgeError {
                        error: format!("Unrecognized chain id: {}", e.key.source),
                    }
                })?;
                let destination = BridgeChainId::try_from(e.key.destination).map_err(|_e| {
                    SuiError::GenericBridgeError {
                        error: format!("Unrecognized chain id: {}", e.key.destination),
                    }
                })?;
                Ok((source, destination, e.value))
            })
            .collect::<SuiResult<Vec<_>>>()?;
        let max_mint_busd_limit = self.limiter.max_mint_busd_limit;
        let limiter = BridgeLimiterSummary {
            transfer_limit,
            transfer_records,
            max_mint_busd_limit,
            transfer_out_limits,
        };
        Ok(BridgeSummary {
            bridge_version: self.bridge_version,
            message_version: self.message_version,
            chain_id: self.chain_id,
            sequence_nums: self
                .sequence_nums
                .contents
                .into_iter()
                .map(|e| (e.key, e.value))
                .collect(),
            committee: BridgeCommitteeSummary {
                members: self
                    .committee
                    .members
                    .contents
                    .into_iter()
                    .map(|e| (e.key, e.value))
                    .collect(),
                member_registration: self
                    .committee
                    .member_registrations
                    .contents
                    .into_iter()
                    .map(|e| (e.key, e.value))
                    .collect(),
                last_committee_update_epoch: self.committee.last_committee_update_epoch,
            },
            bridge_records_id: self.bridge_records.id,
            limiter,
            treasury: BridgeTreasurySummary {
                external_coin_target_address,
                supported_tokens,
                id_token_type_map,
            },
            is_frozen: self.frozen,
        })
    }
}

/// Rust version of the Move treasury::BridgeTreasury type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveTypeBridgeTreasury {
    pub external_coin_admin_address: VecMap<String, VecSet<String>>,
    pub external_coin_target_address: VecMap<String, VecSet<String>>,
    pub external_coin_witness_address: VecMap<String, VecSet<Vec<u8>>>,
    pub treasuries: Bag,
    pub supported_tokens: VecMap<String, BridgeTokenMetadata>,
    // Mapping token id to type name
    pub id_token_type_map: VecMap<u64, String>,
    // Bag for storing potential new token waiting to be approved
    pub waiting_room: Bag,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BridgeTokenMetadata {
    pub id: u64,
    pub decimal_multiplier: u64,
    pub notional_value: u64,
    pub native_token: bool,
}

/// Rust version of the Move committee::BridgeCommittee type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveTypeBridgeCommittee {
    pub members: VecMap<Vec<u8>, MoveTypeCommitteeMember>,
    pub member_registrations: VecMap<SuiAddress, MoveTypeCommitteeMemberRegistration>,
    pub last_committee_update_epoch: u64,
}

/// Rust version of the Move committee::CommitteeMemberRegistration type.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MoveTypeCommitteeMemberRegistration {
    pub sui_address: SuiAddress,
    pub bridge_pubkey_bytes: Vec<u8>,
    pub http_rest_url: Vec<u8>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct BridgeCommitteeSummary {
    pub members: Vec<(Vec<u8>, MoveTypeCommitteeMember)>,
    pub member_registration: Vec<(SuiAddress, MoveTypeCommitteeMemberRegistration)>,
    pub last_committee_update_epoch: u64,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct BridgeLimiterSummary {
    pub transfer_limit: Vec<(BridgeChainId, BridgeChainId, u64)>,
    pub transfer_records: Vec<(BridgeChainId, BridgeChainId, MoveTypeBridgeTransferRecord)>,
    pub max_mint_busd_limit: u64,
    pub transfer_out_limits: Vec<(BridgeChainId, BridgeChainId, u64)>, //from external limiter,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct BridgeTreasurySummary {
    pub external_coin_target_address: Vec<(String, Vec<String>)>,
    pub supported_tokens: Vec<(String, BridgeTokenMetadata)>,
    pub id_token_type_map: Vec<(u64, String)>,
}

/// Rust version of the Move committee::CommitteeMember type.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Default, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MoveTypeCommitteeMember {
    pub sui_address: SuiAddress,
    pub bridge_pubkey_bytes: Vec<u8>,
    pub voting_power: u64,
    pub http_rest_url: Vec<u8>,
    pub blocklisted: bool,
}

/// Rust version of the Move message::BridgeMessageKey type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeBridgeMessageKey {
    pub source_chain: u8,
    pub message_type: u8,
    pub bridge_seq_num: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeExternalBridgeMessageKey {
    pub source_chain: u8,
    pub source_address: Vec<u8>,
    pub target_address: Vec<u8>,
    pub amount: u64,
    pub tx_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeExternalBridgeRecord {
    pub source_chain: u8,
    pub target_chain: u8,
    pub source_address: Vec<u8>,
    pub target_address: Vec<u8>,
    pub amount: u64,
    pub verified_signatures: Option<Vec<Vec<u8>>>,
    pub claimed: bool,
}

/// Rust version of the Move message::BridgeMessageKey type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeRefundMessageKey {
    pub tx_hash: Vec<u8>,
}

/// Rust version of the Move bridge::DefiProtocolKey type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeDefiProtocolKey {
    pub protocol_type: u64,
    pub protocol_version: u64,
    pub protocol_token_id: u64,
    pub chain_id: u8,
}

/// Rust version of the Move bridge::DefiHolderInfo type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeDefiHolderInfo {
    pub amount: u64,
    pub lp_token_amount: u64,
}

/// Rust version of the Move limiter::TransferLimiter type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveTypeBridgeTransferLimiter {
    pub transfer_limit: VecMap<MoveTypeBridgeRoute, u64>,
    pub transfer_records: VecMap<MoveTypeBridgeRoute, MoveTypeBridgeTransferRecord>,
    pub max_mint_busd_limit: u64,
}

/// Rust version of the Move limiter::ExternalLimiter type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveTypeBridgeExternalLimiter {
    pub transfer_out_limits: VecMap<MoveTypeBridgeRoute, u64>,
    pub external: Bag,
}

impl Default for MoveTypeBridgeExternalLimiter {
    fn default() -> Self {
        Self {
            transfer_out_limits: VecMap { contents: vec![] },
            external: Bag::default(),
        }
    }
}

/// Rust version of the Move chain_ids::BridgeRoute type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveTypeBridgeRoute {
    pub source: u8,
    pub destination: u8,
}

/// Rust version of the Move limiter::TransferRecord type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct MoveTypeBridgeTransferRecord {
    hour_head: u64,
    hour_tail: u64,
    per_hour_amounts: Vec<u64>,
    total_amount: u64,
}

impl MoveTypeBridgeTransferRecord {
    pub fn total_amount(&self) -> u64 {
        self.total_amount
    }
}

/// Rust version of the Move message::BridgeMessage type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTypeBridgeMessage {
    pub message_type: u8,
    pub message_version: u8,
    pub seq_num: u64,
    pub source_chain: u8,
    pub payload: Vec<u8>,
}

/// Rust version of the Move message::BridgeMessage type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTypeBridgeRecord {
    pub message: MoveTypeBridgeMessage,
    pub verified_signatures: Option<Vec<Vec<u8>>>,
    pub claimed: bool,
}

pub fn is_bridge_committee_initiated(object_store: &dyn ObjectStore) -> SuiResult<bool> {
    match get_bridge(object_store) {
        Ok(bridge) => Ok(!bridge.committee().members.contents.is_empty()),
        Err(SuiError::SuiBridgeReadError(..)) => Ok(false),
        Err(other) => Err(other),
    }
}

/// Rust version of the Move message::TokenTransferPayload type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeTokenTransferPayload {
    pub sender_address: Vec<u8>,
    pub target_chain: u8,
    pub target_address: Vec<u8>,
    pub token_type: u64,
    pub amount: u64,
    pub tx_hash: Vec<u8>,
    pub event_idx: u8,
}

/// Rust version of the Move message::TokenTransferPayloadV2 type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeTokenTransferPayloadV2 {
    pub sender_address: Vec<u8>,
    pub target_chain: u8,
    pub target_address: Vec<u8>,
    pub token_type: u64,
    pub amount: u64,
    pub tx_hash: Vec<u8>,
    pub event_idx: u16,
}

/// Rust version of the Move message::DefiTransferOutPayload type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeDefiTransferOutPayload {
    pub sender_address: Vec<u8>,
    pub target_chain: u8,
    pub amount: u64,
    pub tx_hash: Vec<u8>,
    pub event_idx: u16,
    pub protocol_type: u64,
    pub protocol_version: u64,
    pub protocol_token_id: u64,
    pub action_type: u8,
}

/// Rust version of the Move message::ParsedTokenTransferMessage type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeParsedTokenTransferMessage {
    pub message_version: u8,
    pub seq_num: u64,
    pub source_chain: u8,
    pub payload: Vec<u8>,
    pub parsed_payload: MoveTypeTokenTransferPayload,
}

/// Rust version of the Move message::ParsedTokenTransferMessageV2 type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeParsedTokenTransferMessageV2 {
    pub message_version: u8,
    pub seq_num: u64,
    pub source_chain: u8,
    pub payload: Vec<u8>,
    pub parsed_payload: MoveTypeTokenTransferPayloadV2,
}

/// Rust version of the Move message::ParsedDefiTransferOutMessage type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveTypeParsedDefiTransferOutMessage {
    pub message_version: u8,
    pub seq_num: u64,
    pub source_chain: u8,
    pub payload: Vec<u8>,
    pub parsed_payload: MoveTypeDefiTransferOutPayload,
}