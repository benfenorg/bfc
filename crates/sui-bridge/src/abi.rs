// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::encoding::{BridgeMessageEncoding, ADD_TOKENS_ON_EVM_MESSAGE_VERSION, ASSET_PRICE_UPDATE_MESSAGE_VERSION, EVM_CONTRACT_UPGRADE_MESSAGE_VERSION, LIMIT_UPDATE_MESSAGE_VERSION, SINGLE_TRANSFER_LIMIT_UPDATE_MESSAGE_VERSION, TOKEN_TRANSFER_MESSAGE_VERSION_V3};
use crate::encoding::{
    COMMITTEE_BLOCKLIST_MESSAGE_VERSION, EMERGENCY_BUTTON_MESSAGE_VERSION,
};
use crate::error::{BridgeError, BridgeResult};
use crate::fast_path::FastPathSelector;
use crate::types::{
    AddTokensOnEvmAction, AssetPriceUpdateAction, BlocklistCommitteeAction, BridgeAction, BridgeActionType, EmergencyAction, EthLog, EthToSuiBridgeAction, EthToSuiDefiBridgeAction, EvmContractUpgradeAction, LimitUpdateAction, SuiToEthBridgeAction
};
use crate::types::{
    ParsedTokenTransferMessage, ParsedTokenTransferMessageV2, SingleTransferLimitUpdateAction,
};
use ethers::types::Log;
use ethers::{
    abi::RawLog,
    contract::{abigen, EthLogDecode},
    types::Address as EthAddress,
};
use serde::{Deserialize, Serialize};
use sui_types::base_types::SuiAddress;
use sui_types::bridge::{BridgeChainId, TOKEN_ID_BUSD, TOKEN_ID_USDC, TOKEN_ID_USDT};

macro_rules! gen_eth_events {
    ($($contract:ident, $contract_event:ident, $abi_path:literal),* $(,)?) => {
        $(
            abigen!(
                $contract,
                $abi_path,
                event_derives(serde::Deserialize, serde::Serialize)
            );
        )*

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
        pub enum EthBridgeEvent {
            $(
                $contract_event($contract_event),
            )*
        }

        impl EthBridgeEvent {
            pub fn try_from_eth_log(log: &EthLog) -> Option<EthBridgeEvent> {
                Self::try_from_log(&log.log)
            }

            pub fn try_from_log(log: &Log) -> Option<EthBridgeEvent> {
                let raw_log = RawLog {
                    topics: log.topics.clone(),
                    data: log.data.to_vec(),
                };

                $(
                    if let Ok(decoded) = $contract_event::decode_log(&raw_log) {
                        return Some(EthBridgeEvent::$contract_event(decoded));
                    }
                )*

                None
            }
        }
    };

    // For contracts that don't have Events
    ($($contract:ident, $abi_path:literal),* $(,)?) => {
        $(
            abigen!(
                $contract,
                $abi_path,
                event_derives(serde::Deserialize, serde::Serialize)
            );
        )*
    };
}

#[rustfmt::skip]
gen_eth_events!(
    EthSuiBridge, EthSuiBridgeEvents, "abi/sui_bridge.json",
    EthBridgeCommittee, EthBridgeCommitteeEvents, "abi/bridge_committee.json",
    EthBridgeLimiter, EthBridgeLimiterEvents, "abi/bridge_limiter.json",
    EthBridgeConfig, EthBridgeConfigEvents, "abi/bridge_config.json",
    EthCommitteeUpgradeableContract, EthCommitteeUpgradeableContractEvents, "abi/bridge_committee_upgradeable.json"
);

gen_eth_events!(EthBridgeVault, "abi/bridge_vault.json");

abigen!(
    EthERC20,
    "abi/erc20.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

impl EthBridgeEvent {
    pub fn try_into_bridge_action(
        self,
        eth_tx_hash: ethers::types::H256,
        eth_event_index: u16,
    ) -> BridgeResult<Option<BridgeAction>> {
        Ok(match self {
            EthBridgeEvent::EthSuiBridgeEvents(event) => {
                match event {
                    EthSuiBridgeEvents::TokensDepositedFilter(event) => {
                        let bridge_event = match EthToSuiTokenBridgeV1::try_from(&event) {
                            Ok(mut bridge_event) => {
                                if bridge_event.sui_adjusted_amount == 0 {
                                    return Err(BridgeError::ZeroValueBridgeTransfer(format!(
                                        "Manual intervention is required: {}",
                                        eth_tx_hash
                                    )));
                                }
                                bridge_event.set_tx_hash(eth_tx_hash.as_bytes().to_vec());
                                bridge_event.set_event_idx(eth_event_index);
                                bridge_event
                            }
                            // This only happens when solidity code does not align with rust code.
                            // When this happens in production, there is a risk of stuck bridge transfers.
                            // We log error here.
                            // TODO: add metrics and alert
                            Err(e) => {
                                return Err(BridgeError::Generic(format!("Manual intervention is required. Failed to convert TokensDepositedFilter log to EthToSuiTokenBridgeV1. This indicates incorrect parameters or a bug in the code: {:?}. Err: {:?}", event, e)));
                            }
                        };

                        Some(BridgeAction::EthToSuiBridgeAction(EthToSuiBridgeAction {
                            eth_tx_hash,
                            eth_event_index,
                            eth_bridge_event: bridge_event,
                        }))
                    }
                    EthSuiBridgeEvents::TokensStakedFilter(_event) => None,
                    EthSuiBridgeEvents::TokensUnStakedFilter(event) => {
                        let bridge_event = match EthToSuiDefiBridgeV1::try_from(&event) {
                            Ok(mut bridge_event) => {
                                bridge_event.set_tx_hash(eth_tx_hash.as_bytes().to_vec());
                                bridge_event.set_event_idx(eth_event_index);
                                bridge_event
                            }
                            // This only happens when solidity code does not align with rust code.
                            // When this happens in production, there is a risk of stuck bridge transfers.
                            // We log error here.
                            // TODO: add metrics and alert
                            Err(e) => {
                                return Err(BridgeError::Generic(format!("Manual intervention is required. Failed to convert TokensUnStakedFilter log to EthToSuiDefiBridgeV1. This indicates incorrect parameters or a bug in the code: {:?}. Err: {:?}", event, e)));
                            }
                        };
                        Some(BridgeAction::EthToSuiDefiBridgeAction(EthToSuiDefiBridgeAction {  
                            eth_tx_hash,
                            eth_event_index,
                            eth_bridge_event: bridge_event,
                        }))
                    },
                    EthSuiBridgeEvents::UpdateInvestAddressFilter(_event) => None,
                    EthSuiBridgeEvents::TokensClaimedFilter(_event) => None,
                    EthSuiBridgeEvents::PausedFilter(_event) => None,
                    EthSuiBridgeEvents::UnpausedFilter(_event) => None,
                    EthSuiBridgeEvents::UpgradedFilter(_event) => None,
                    EthSuiBridgeEvents::InitializedFilter(_event) => None,
                    EthSuiBridgeEvents::ContractUpgradedFilter(_event) => None,
                    EthSuiBridgeEvents::EmergencyOperationFilter(_event) => None,
                }
            }
            EthBridgeEvent::EthBridgeCommitteeEvents(event) => match event {
                EthBridgeCommitteeEvents::BlocklistUpdatedFilter(_event) => None,
                EthBridgeCommitteeEvents::InitializedFilter(_event) => None,
                EthBridgeCommitteeEvents::UpgradedFilter(_event) => None,
                EthBridgeCommitteeEvents::BlocklistUpdatedV2Filter(_event) => None,
                EthBridgeCommitteeEvents::ContractUpgradedFilter(_event) => None,
            },
            EthBridgeEvent::EthBridgeLimiterEvents(event) => match event {
                EthBridgeLimiterEvents::LimitUpdatedFilter(_event) => None,
                EthBridgeLimiterEvents::SingleTransferLimitUpdateFilter(_event) => None,
                EthBridgeLimiterEvents::InitializedFilter(_event) => None,
                EthBridgeLimiterEvents::UpgradedFilter(_event) => None,
                EthBridgeLimiterEvents::HourlyTransferAmountUpdatedFilter(_event) => None,
                EthBridgeLimiterEvents::OwnershipTransferredFilter(_event) => None,
                EthBridgeLimiterEvents::ContractUpgradedFilter(_event) => None,
                EthBridgeLimiterEvents::LimitUpdatedV2Filter(_event) => None,
            },
            EthBridgeEvent::EthBridgeConfigEvents(event) => match event {
                EthBridgeConfigEvents::LpTokenIdAddedFilter(_event) => None,
                EthBridgeConfigEvents::InitializedFilter(_event) => None,
                EthBridgeConfigEvents::UpgradedFilter(_event) => None,
                EthBridgeConfigEvents::TokenAddedFilter(_event) => None,
                EthBridgeConfigEvents::TokenPriceUpdatedFilter(_event) => None,
                EthBridgeConfigEvents::ContractUpgradedFilter(_event) => None,
                EthBridgeConfigEvents::TokenPriceUpdatedV2Filter(_event) => None,
                EthBridgeConfigEvents::TokensAddedV2Filter(_event) => None,
            },
            EthBridgeEvent::EthCommitteeUpgradeableContractEvents(event) => match event {
                EthCommitteeUpgradeableContractEvents::InitializedFilter(_event) => None,
                EthCommitteeUpgradeableContractEvents::UpgradedFilter(_event) => None,
            },
        })
    }
}

/// The event emitted when tokens are deposited into the bridge on Ethereum.
/// Sanity checked version of TokensDepositedFilter
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EthToSuiTokenBridgeV1 {
    pub nonce: u64,
    pub sui_chain_id: BridgeChainId,
    pub eth_chain_id: BridgeChainId,
    pub sui_address: SuiAddress,
    pub eth_address: EthAddress,
    pub token_id: u64,
    pub sui_adjusted_amount: u64,
    pub tx_hash: Vec<u8>,
    pub event_idx: u16,
    pub fast_path_selector: FastPathSelector,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EthToSuiDefiBridgeV1 {
    pub nonce: u64,
    pub sui_chain_id: BridgeChainId,
    pub eth_chain_id: BridgeChainId,
    pub sui_address: SuiAddress,
    pub eth_address: EthAddress,
    pub tx_hash: Vec<u8>,
    pub event_idx: u16,
    pub fast_path_selector: FastPathSelector,
    pub protocol_type: u64,
    pub protocol_version: u64,
    pub protocol_token_id: u64,
    pub action_type: u8,
    pub original_seq_num: u64,
    pub sui_adjusted_amount: u64,
    pub lp_token_amount: u64,
}

impl EthToSuiDefiBridgeV1 {
    pub fn set_tx_hash(&mut self, tx_hash: Vec<u8>) {
        self.tx_hash = tx_hash;
    }

    pub fn set_event_idx(&mut self, event_idx: u16) {
        self.event_idx = event_idx;
    }

    pub fn set_fast_path_selector(&mut self, fast_path_selector: FastPathSelector) {
        self.fast_path_selector = fast_path_selector;
    }
}

impl EthToSuiTokenBridgeV1 {
    pub fn set_tx_hash(&mut self, tx_hash: Vec<u8>) {
        self.tx_hash = tx_hash;
    }

    pub fn set_event_idx(&mut self, event_idx: u16) {
        self.event_idx = event_idx;
    }

    pub fn set_fast_path_selector(&mut self, fast_path_selector: FastPathSelector) {
        self.fast_path_selector = fast_path_selector;
    }
}

impl TryFrom<&TokensDepositedFilter> for EthToSuiTokenBridgeV1 {
    type Error = BridgeError;
    fn try_from(event: &TokensDepositedFilter) -> BridgeResult<Self> {
        Ok(Self {
            nonce: event.nonce,
            sui_chain_id: BridgeChainId::try_from(event.destination_chain_id)?,
            eth_chain_id: BridgeChainId::try_from(event.source_chain_id)?,
            sui_address: SuiAddress::from_bytes(event.recipient_address.as_ref())?,
            eth_address: event.sender_address,
            token_id: event.token_id,
            sui_adjusted_amount: event.sui_adjusted_amount,
            tx_hash: vec![],
            event_idx: 0,
            fast_path_selector: FastPathSelector::Finalized,
        })
    }
}

impl TryFrom<&TokensStakedFilter> for EthToSuiDefiBridgeV1 {
    type Error = BridgeError;
    fn try_from(event: &TokensStakedFilter) -> BridgeResult<Self> {
        Ok(Self {
            nonce: event.nonce,
            sui_chain_id: BridgeChainId::try_from(event.destination_chain_id)?,
            eth_chain_id: BridgeChainId::try_from(event.source_chain_id)?,
            sui_address: SuiAddress::from_bytes(event.sender_address.as_ref())?,
            eth_address: event.recipient_address,
            tx_hash: vec![],
            event_idx: 0,
            fast_path_selector: FastPathSelector::Finalized,
            protocol_type: event.protocol_type,
            protocol_version: event.protocol_version,
            protocol_token_id: event.protocol_token_id,
            action_type: event.action_type,
            original_seq_num: event.origin_nonce,
            sui_adjusted_amount: event.erc_20_adjusted_amount.as_u64(),
            lp_token_amount: event.erc_2_0lp_token_amount.as_u64(),
        })
    }
}

impl TryFrom<&TokensUnStakedFilter> for EthToSuiDefiBridgeV1 {
    type Error = BridgeError;
    fn try_from(event: &TokensUnStakedFilter) -> BridgeResult<Self> {
        Ok(Self {
            nonce: event.nonce,
            sui_chain_id: BridgeChainId::try_from(event.destination_chain_id)?,
            eth_chain_id: BridgeChainId::try_from(event.source_chain_id)?,
            sui_address: SuiAddress::from_bytes(event.recipient_address.as_ref())?,
            eth_address: event.sender_address,
            tx_hash: vec![],
            event_idx: 0,
            fast_path_selector: FastPathSelector::Finalized,
            protocol_type: event.protocol_type,
            protocol_version: event.protocol_version,
            protocol_token_id: event.protocol_token_id,
            action_type: event.action_type,
            original_seq_num: event.origin_nonce,
            sui_adjusted_amount: event.erc_20_adjusted_amount.as_u64(),
            lp_token_amount: event.erc_2_0lp_token_amount.as_u64(),
        })
    }
}

impl TryFrom<&EthToSuiTokenBridgeV1> for EthToSuiTokenBridgeV1 {
    type Error = BridgeError;
    fn try_from(msg: &EthToSuiTokenBridgeV1) -> BridgeResult<Self> {
        //only eth chain need to adjust
        let need_adjust = (msg.token_id == TOKEN_ID_USDC || msg.token_id == TOKEN_ID_USDT)
            && msg.eth_chain_id.is_eth_chain();
        Ok(Self {
            nonce: msg.nonce,
            sui_chain_id: msg.sui_chain_id,
            eth_chain_id: msg.eth_chain_id,
            sui_address: msg.sui_address,
            eth_address: msg.eth_address,
            token_id: if msg.token_id == TOKEN_ID_USDC || msg.token_id == TOKEN_ID_USDT {
                TOKEN_ID_BUSD
            } else {
                msg.token_id
            },
            sui_adjusted_amount: if need_adjust {
                msg.sui_adjusted_amount
                    .checked_mul(1000)
                    .unwrap_or(msg.sui_adjusted_amount)
            } else {
                msg.sui_adjusted_amount
            },
            tx_hash: msg.tx_hash.clone(),
            event_idx: msg.event_idx,
            fast_path_selector: msg.fast_path_selector,
        })
    }
}
////////////////////////////////////////////////////////////////////////
//                        Eth Message Conversion                      //
////////////////////////////////////////////////////////////////////////

impl From<SuiToEthBridgeAction> for eth_sui_bridge::Message {
    fn from(action: SuiToEthBridgeAction) -> Self {
        eth_sui_bridge::Message {
            message_type: BridgeActionType::TokenTransfer as u8,
            version: TOKEN_TRANSFER_MESSAGE_VERSION_V3,
            nonce: action.sui_bridge_event.nonce,
            chain_id: action.sui_bridge_event.sui_chain_id as u8,
            payload: action.as_payload_bytes().into(),
        }
    }
}

impl From<ParsedTokenTransferMessage> for eth_sui_bridge::Message {
    fn from(parsed_message: ParsedTokenTransferMessage) -> Self {
        eth_sui_bridge::Message {
            message_type: BridgeActionType::TokenTransfer as u8,
            version: parsed_message.message_version,
            nonce: parsed_message.seq_num,
            chain_id: parsed_message.source_chain as u8,
            payload: parsed_message.payload.into(),
        }
    }
}

impl From<ParsedTokenTransferMessageV2> for eth_sui_bridge::Message {
    fn from(parsed_message: ParsedTokenTransferMessageV2) -> Self {
        eth_sui_bridge::Message {
            message_type: BridgeActionType::TokenTransfer as u8,
            version: parsed_message.message_version,
            nonce: parsed_message.seq_num,
            chain_id: parsed_message.source_chain as u8,
            payload: parsed_message.payload.into(),
        }
    }
}

impl From<EmergencyAction> for eth_sui_bridge::Message {
    fn from(action: EmergencyAction) -> Self {
        eth_sui_bridge::Message {
            message_type: BridgeActionType::EmergencyButton as u8,
            version: EMERGENCY_BUTTON_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().into(),
        }
    }
}

impl From<BlocklistCommitteeAction> for eth_bridge_committee::Message {
    fn from(action: BlocklistCommitteeAction) -> Self {
        eth_bridge_committee::Message {
            message_type: BridgeActionType::UpdateCommitteeBlocklist as u8,
            version: COMMITTEE_BLOCKLIST_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().into(),
        }
    }
}

impl From<LimitUpdateAction> for eth_bridge_limiter::Message {
    fn from(action: LimitUpdateAction) -> Self {
        eth_bridge_limiter::Message {
            message_type: BridgeActionType::LimitUpdate as u8,
            version: LIMIT_UPDATE_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().into(),
        }
    }
}

impl From<SingleTransferLimitUpdateAction> for eth_bridge_limiter::Message {
    fn from(action: SingleTransferLimitUpdateAction) -> Self {
        eth_bridge_limiter::Message {
            message_type: BridgeActionType::SingleTransferLimitUpdate as u8,
            version: SINGLE_TRANSFER_LIMIT_UPDATE_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().into(),
        }
    }
}

impl From<AssetPriceUpdateAction> for eth_bridge_config::Message {
    fn from(action: AssetPriceUpdateAction) -> Self {
        eth_bridge_config::Message {
            message_type: BridgeActionType::AssetPriceUpdate as u8,
            version: ASSET_PRICE_UPDATE_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().into(),
        }
    }
}

impl From<AddTokensOnEvmAction> for eth_bridge_config::Message {
    fn from(action: AddTokensOnEvmAction) -> Self {
        eth_bridge_config::Message {
            message_type: BridgeActionType::AddTokensOnEvm as u8,
            version: ADD_TOKENS_ON_EVM_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().into(),
        }
    }
}

impl From<EvmContractUpgradeAction> for eth_committee_upgradeable_contract::Message {
    fn from(action: EvmContractUpgradeAction) -> Self {
        eth_committee_upgradeable_contract::Message {
            message_type: BridgeActionType::EvmContractUpgrade as u8,
            version: EVM_CONTRACT_UPGRADE_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        crypto::BridgeAuthorityPublicKeyBytes,
        types::{BlocklistType, EmergencyActionType},
    };
    use ethers::types::TxHash;
    use fastcrypto::encoding::{Encoding, Hex};
    use hex_literal::hex;
    use std::str::FromStr;
    use sui_types::{bridge::TOKEN_ID_ETH, crypto::ToFromBytes};

    #[test]
    fn test_eth_message_conversion_emergency_action_regression() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();

        let action = EmergencyAction {
            nonce: 2,
            chain_id: BridgeChainId::EthSepolia,
            action_type: EmergencyActionType::Pause,
        };
        let message: eth_sui_bridge::Message = action.into();
        assert_eq!(
            message,
            eth_sui_bridge::Message {
                message_type: BridgeActionType::EmergencyButton as u8,
                version: EMERGENCY_BUTTON_MESSAGE_VERSION,
                nonce: 2,
                chain_id: BridgeChainId::EthSepolia as u8,
                payload: vec![0].into(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_eth_message_conversion_update_blocklist_action_regression() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let pub_key_bytes = BridgeAuthorityPublicKeyBytes::from_bytes(
            &Hex::decode("02321ede33d2c2d7a8a152f275a1484edef2098f034121a602cb7d767d38680aa4")
                .unwrap(),
        )
        .unwrap();
        let action = BlocklistCommitteeAction {
            nonce: 0,
            chain_id: BridgeChainId::EthSepolia,
            blocklist_type: BlocklistType::Blocklist,
            members_to_update: vec![pub_key_bytes],
        };
        let message: eth_bridge_committee::Message = action.into();
        assert_eq!(
            message,
            eth_bridge_committee::Message {
                message_type: BridgeActionType::UpdateCommitteeBlocklist as u8,
                version: COMMITTEE_BLOCKLIST_MESSAGE_VERSION,
                nonce: 0,
                chain_id: BridgeChainId::EthSepolia as u8,
                payload: Hex::decode("000168b43fd906c0b8f024a18c56e06744f7c6157c65")
                    .unwrap()
                    .into(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_eth_message_conversion_update_limit_action_regression() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let action = LimitUpdateAction {
            nonce: 2,
            chain_id: BridgeChainId::EthSepolia,
            sending_chain_id: BridgeChainId::SuiTestnet,
            new_usd_limit: 4200000,
        };
        let message: eth_bridge_limiter::Message = action.into();
        assert_eq!(
            message,
            eth_bridge_limiter::Message {
                message_type: BridgeActionType::LimitUpdate as u8,
                version: LIMIT_UPDATE_MESSAGE_VERSION,
                nonce: 2,
                chain_id: BridgeChainId::EthSepolia as u8,
                payload: Hex::decode("010000000000401640").unwrap().into(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_eth_message_conversion_contract_upgrade_action_regression() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let action = EvmContractUpgradeAction {
            nonce: 2,
            chain_id: BridgeChainId::EthSepolia,
            proxy_address: EthAddress::repeat_byte(1),
            new_impl_address: EthAddress::repeat_byte(2),
            call_data: Vec::from("deadbeef"),
        };
        let message: eth_committee_upgradeable_contract::Message = action.into();
        assert_eq!(
            message,
            eth_committee_upgradeable_contract::Message {
                message_type: BridgeActionType::EvmContractUpgrade as u8,
                version: EVM_CONTRACT_UPGRADE_MESSAGE_VERSION,
                nonce: 2,
                chain_id: BridgeChainId::EthSepolia as u8,
                payload: Hex::decode("0x00000000000000000000000001010101010101010101010101010101010101010000000000000000000000000202020202020202020202020202020202020202000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000086465616462656566000000000000000000000000000000000000000000000000").unwrap().into(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_eth_message_conversion_update_price_action_regression() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let action = AssetPriceUpdateAction {
            nonce: 2,
            chain_id: BridgeChainId::EthSepolia,
            token_id: TOKEN_ID_ETH,
            new_usd_price: 80000000,
        };
        let message: eth_bridge_config::Message = action.into();
        assert_eq!(
            message,
            eth_bridge_config::Message {
                message_type: BridgeActionType::AssetPriceUpdate as u8,
                version: ASSET_PRICE_UPDATE_MESSAGE_VERSION,
                nonce: 2,
                chain_id: BridgeChainId::EthSepolia as u8,
                payload: Hex::decode("00000000000000020000000004c4b400")
                    .unwrap()
                    .into(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_eth_message_conversion_add_tokens_on_evm_action_regression() -> anyhow::Result<()> {
        let action = AddTokensOnEvmAction {
            nonce: 5,
            chain_id: BridgeChainId::EthCustom,
            native: true,
            token_ids: vec![99, 100, 101],
            token_addresses: vec![
                EthAddress::repeat_byte(1),
                EthAddress::repeat_byte(2),
                EthAddress::repeat_byte(3),
            ],
            token_sui_decimals: vec![5, 6, 7],
            token_prices: vec![1_000_000_000, 2_000_000_000, 3_000_000_000],
        };
        let message: eth_bridge_config::Message = action.into();
        assert_eq!(
            message,
            eth_bridge_config::Message {
                message_type: BridgeActionType::AddTokensOnEvm as u8,
                version: ADD_TOKENS_ON_EVM_MESSAGE_VERSION,
                nonce: 5,
                chain_id: BridgeChainId::EthCustom as u8,
                payload: Hex::decode("0103000000000000006300000000000000640000000000000065030101010101010101010101010101010101010101020202020202020202020202020202020202020203030303030303030303030303030303030303030305060703000000003b9aca00000000007735940000000000b2d05e00").unwrap().into(),
            }
        );
        Ok(())
    }

    #[test]
    fn test_token_deposit_eth_log_to_sui_bridge_event_regression() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let tx_hash = TxHash::random();
        let action = EthLog {
            block_number: 33,
            tx_hash,
            log_index_in_tx: 1,
            log: Log {
                address: EthAddress::repeat_byte(1),
                topics: vec![
                    hex!("5aeed19d0207dbc2897bec15241a138fed03931a5b2d8dfe2364b7c2ae33431b").into(),
                    hex!("0000000000000000000000000000000000000000000000000000000000000001").into(),
                    hex!("0000000000000000000000000000000000000000000000000000000000000010").into(),
                    hex!("000000000000000000000000000000000000000000000000000000000000000b").into(),
                ],
                data: ethers::types::Bytes::from(
                    Hex::decode("0x0000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000098968000000000000000000000000058ac1eab83ae5ad29e0f1f566770d57c64cd7229000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000208f7c7cafbc956fe578a44b1034eb42a8d974c9098889125cab8b86be9bcdcf63").unwrap(),
                ),
                block_hash: None,
                block_number: None,
                transaction_hash: Some(tx_hash),
                transaction_index: Some(ethers::types::U64::from(0)),
                log_index: Some(ethers::types::U256::from(0)),
                transaction_log_index: None,
                log_type: None,
                removed: Some(false),
            }
        };
        let event = EthBridgeEvent::try_from_eth_log(&action).unwrap();
        assert_eq!(
            event,
            EthBridgeEvent::EthSuiBridgeEvents(EthSuiBridgeEvents::TokensDepositedFilter(
                TokensDepositedFilter {
                    source_chain_id: 1,
                    nonce: 16,
                    destination_chain_id: 11,
                    token_id: 3,
                    sui_adjusted_amount: 10000000u64,
                    sender_address: EthAddress::from_str(
                        "0x58ac1eab83ae5ad29e0f1f566770d57c64cd7229"
                    )
                    .unwrap(),
                    recipient_address: ethers::types::Bytes::from(
                        Hex::decode(
                            "0x8f7c7cafbc956fe578a44b1034eb42a8d974c9098889125cab8b86be9bcdcf63"
                        )
                        .unwrap(),
                    ),
                }
            ))
        );
        Ok(())
    }

    #[test]
    fn test_0_sui_amount_conversion_for_eth_event() {
        let e = EthBridgeEvent::EthSuiBridgeEvents(EthSuiBridgeEvents::TokensDepositedFilter(
            TokensDepositedFilter {
                source_chain_id: BridgeChainId::EthSepolia as u8,
                nonce: 0,
                destination_chain_id: BridgeChainId::SuiTestnet as u8,
                token_id: 2,
                sui_adjusted_amount: 1,
                sender_address: EthAddress::random(),
                recipient_address: ethers::types::Bytes::from(
                    SuiAddress::random_for_testing_only().to_vec(),
                ),
            },
        ));
        assert!(e
            .try_into_bridge_action(TxHash::random(), 0)
            .unwrap()
            .is_some());

        let e = EthBridgeEvent::EthSuiBridgeEvents(EthSuiBridgeEvents::TokensDepositedFilter(
            TokensDepositedFilter {
                source_chain_id: BridgeChainId::EthSepolia as u8,
                nonce: 0,
                destination_chain_id: BridgeChainId::SuiTestnet as u8,
                token_id: 2,
                sui_adjusted_amount: 0, // <------------
                sender_address: EthAddress::random(),
                recipient_address: ethers::types::Bytes::from(
                    SuiAddress::random_for_testing_only().to_vec(),
                ),
            },
        ));
        match e.try_into_bridge_action(TxHash::random(), 0).unwrap_err() {
            BridgeError::ZeroValueBridgeTransfer(_) => {}
            e => panic!("Unexpected error: {:?}", e),
        }
    }
}
