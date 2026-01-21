// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use clap::*;
use ethers::providers::Middleware;
use ethers::types::Address as EthAddress;
use ethers::types::U256;
use fastcrypto::encoding::Encoding;
use fastcrypto::encoding::Hex;
use fastcrypto::hash::{HashFunction, Keccak256};
use move_core_types::ident_str;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use sui_bridge::abi::EthBridgeCommittee;
use sui_bridge::abi::{eth_sui_bridge, EthSuiBridge};
use sui_bridge::crypto::BridgeAuthorityPublicKeyBytes;
use sui_bridge::error::BridgeResult;
use sui_bridge::sui_client::{SuiBridgeClient, SuiClientInner};
use sui_bridge::types::BridgeAction;
use sui_bridge::types::FastPathLimitUpdateAction;
use sui_bridge::types::RefundAdminAction;
use sui_bridge::types::{
    AddTokensOnEvmAction, AddTokensOnSuiAction, AssetPriceUpdateAction, BlocklistCommitteeAction,
    BlocklistType, EmergencyAction, EmergencyActionType, EvmContractUpgradeAction,
    LimitUpdateAction, AddExternalCoinAdminAction, RemoveExternalCoinAdminAction,
    AddExternalCoinWitnessAction,RemoveExternalCoinWitnessAction,
    AddExternalCoinTargetAction,RemoveExternalCoinTargetAction,
    AddTokenOnTokenListAction,RemoveTokenOnTokenListAction,
    SingleTransferLimitUpdateAction,UpdateBridgeFeeOnCrossOutAction,
    UpdateBridgeFeeOnCrossInAction, WithdrawBridgeFeeAction,
    AddLpTokenIdAction, UpdateInvestAddressAction,
    AddTokenOnSolanaAction,ExtendProgramOnSolanaAction,
    UpgradeProgramOnSolanaAction,
};
use sui_bridge::utils::{get_eth_signer_client, EthSigner, SolanaSigner};
use sui_config::Config;
use sui_json_rpc_types::SuiObjectDataOptions;
use sui_keys::keypair_file::read_key;
use sui_sdk::SuiClientBuilder;
use sui_types::base_types::SuiAddress;
use sui_types::base_types::{ObjectID, ObjectRef};
use sui_types::bridge::{BridgeChainId, BRIDGE_MODULE_NAME};
use sui_types::crypto::{Signature, SuiKeyPair};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{CallArg, ObjectArg, Transaction, TransactionData};
use sui_types::{TypeTag, BRIDGE_PACKAGE_ID};
use tracing::info;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Signer;

pub const SEPOLIA_BRIDGE_PROXY_ADDR: &str = "0xAE68F87938439afEEDd6552B0E83D2CbC2473623";

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Args {
    #[clap(subcommand)]
    pub command: BridgeCommand,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Network {
    Testnet,
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum BridgeCommand {
    #[clap(name = "create-bridge-validator-key")]
    CreateBridgeValidatorKey { path: PathBuf },
    #[clap(name = "create-bridge-client-key")]
    CreateBridgeClientKey {
        path: PathBuf,
        #[clap(long = "use-ecdsa", default_value = "false")]
        use_ecdsa: bool,
    },
    /// Read bridge key from a file and print related information
    /// If `is-validator-key` is true, the key must be a secp256k1 key
    #[clap(name = "examine-key")]
    ExamineKey {
        path: PathBuf,
        #[clap(long = "is-validator-key")]
        is_validator_key: bool,
    },
    #[clap(name = "create-bridge-node-config-template")]
    CreateBridgeNodeConfigTemplate {
        path: PathBuf,
        #[clap(long = "run-client")]
        run_client: bool,
    },
    /// Governance client to facilitate and execute Bridge governance actions
    #[clap(name = "governance")]
    Governance {
        /// Path of BridgeCliConfig
        #[clap(long = "config-path")]
        config_path: PathBuf,
        #[clap(long = "chain-id")]
        chain_id: u8,
        #[clap(subcommand)]
        cmd: GovernanceClientCommands,
        /// If true, only collect signatures but not execute on chain
        #[clap(long = "dry-run")]
        dry_run: bool,
    },
    /// View current status of Eth bridge
    #[clap(name = "view-eth-bridge")]
    ViewEthBridge {
        #[clap(long = "network")]
        network: Option<Network>,
        #[clap(long = "bridge-proxy")]
        bridge_proxy: Option<EthAddress>,
        #[clap(long = "eth-rpc-url")]
        eth_rpc_url: String,
    },
    /// View current list of registered validators
    #[clap(name = "view-bridge-registration")]
    ViewBridgeRegistration {
        #[clap(long = "sui-rpc-url")]
        sui_rpc_url: String,
    },
    /// View current status of Sui bridge
    #[clap(name = "view-sui-bridge")]
    ViewSuiBridge {
        #[clap(long = "sui-rpc-url")]
        sui_rpc_url: String,
        #[clap(long, default_value = "false")]
        hex: bool,
        #[clap(long, default_value = "false")]
        ping: bool,
    },
    /// Client to facilitate and execute Bridge actions
    #[clap(name = "client")]
    Client {
        /// Path of BridgeCliConfig
        #[clap(long = "config-path")]
        config_path: PathBuf,
        #[clap(subcommand)]
        cmd: BridgeClientCommands,
    },
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum GovernanceClientCommands {
    #[clap(name = "emergency-button")]
    EmergencyButton {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "action-type", long)]
        action_type: EmergencyActionType,
    },
    #[clap(name = "update-committee-blocklist")]
    UpdateCommitteeBlocklist {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "blocklist-type", long)]
        blocklist_type: BlocklistType,
        #[clap(name = "pubkey-hex", use_value_delimiter = true, long)]
        pubkeys_hex: Vec<BridgeAuthorityPublicKeyBytes>,
    },
    #[clap(name = "update-refund-admin")]
    UpdateRefundAdmin {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "op-type", long)]
        op_type: u8,
        #[clap(name = "sui-address", long)]
        sui_address: String,
    },
    #[clap(name = "update-fast-path-limit")]
    UpdateFastPathLimit {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "chain-id", long)]
        chain_id_evm: u8,
        #[clap(name = "token-id", long)]
        token_id: u64,
        #[clap(name = "amount", long)]
        amount: u64,
    },
    #[clap(name = "update-limit")]
    UpdateLimit {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "sending-chain", long)]
        sending_chain: u8,
        #[clap(name = "new-usd-limit", long)]
        new_usd_limit: u64,
    },
    #[clap(name = "update-single-transfer-limit")]
    UpdateSingleTransferLimit {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "sending-chain", long)]
        sending_chain: u8,
        #[clap(name = "new-usd-limit", long)]
        new_usd_limit: u64,
    },
    #[clap(name = "update-asset-price")]
    UpdateAssetPrice {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "token-id", long)]
        token_id: u64,
        #[clap(name = "new-usd-price", long)]
        new_usd_price: u64,
    },
    #[clap(name = "add-external-coin-admin")]
    AddExternalCoinAdmin {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "coin-type", long)]
        coin_type: String,
        #[clap(name = "admin-address", long)]
        admin_address: String,
    },
    #[clap(name = "remove-external-coin-admin")]
    RemoveExternalCoinAdmin {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "coin-type", long)]
        coin_type: String,
        #[clap(name = "admin-address", long)]
        admin_address: String,
    },
    #[clap(name = "add-external-coin-witness")]
    AddExternalCoinWitness {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "coin-type", long)]
        coin_type: String,
        #[clap(name = "witness-address", long)]
        witness_address: EthAddress,
    },
    #[clap(name = "remove-external-coin-witness")]
    RemoveExternalCoinWitness {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "coin-type", long)]
        coin_type: String,
        #[clap(name = "witness-address", long)]
        witness_address: EthAddress,
    },
    #[clap(name = "add-external-coin-target")]
    AddExternalCoinTarget {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "coin-type", long)]
        coin_type: String,
        #[clap(name = "target-address", long)]
        target_address: String,
    },
    #[clap(name = "remove-external-coin-target")]
    RemoveExternalCoinTarget {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "coin-type", long)]
        coin_type: String,
        #[clap(name = "target-address", long)]
        target_address: String,
    },
    #[clap(name = "add-token-on-token-list")]
    AddTokenOnTokenList {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "from-chain-id", long)]
        from_chain_id: u8,
        #[clap(name = "to-chain-id", long)]
        to_chain_id: u8,
        #[clap(name = "token-id", long)]
        token_id: u64,
    },
    #[clap(name = "remove-token-on-token-list")]
    RemoveTokenOnTokenList {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "from-chain-id", long)]
        from_chain_id: u8,
        #[clap(name = "to-chain-id", long)]
        to_chain_id: u8,
        #[clap(name = "token-id", long)]
        token_id: u64,
    },
    #[clap(name = "set-bridge-fee-on-cross-out")]
    SetBridgeFeeOnCrossOut {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "to-chain-id", long)]
        to_chain_id: u8,
        #[clap(name = "token-id", long)]
        token_id: u64,
        #[clap(name = "mode", long)]
        mode: u64,
        #[clap(name = "amount", long)]
        amount: u64,
    },
    #[clap(name = "set-bridge-fee-on-cross-in")]
    SetBridgeFeeOnCrossIn {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "from-chain-id", long)]
        from_chain_id: u8,
        #[clap(name = "token-id", long)]
        token_id: u64,
        #[clap(name = "mode", long)]
        mode: u64,
        #[clap(name = "amount", long)]
        amount: u64,
    },
    #[clap(name = "withdraw-bridge-fee")]
    WithdrawBridgeFee {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "recipient", long)]
        recipient: SuiAddress,
        #[clap(name = "coin-type", long)]
        coin_type: String,
        #[clap(name = "amount", long)]
        amount: u64,
    },
    #[clap(name = "add-tokens-on-sui")]
    AddTokensOnSui {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "token-ids", use_value_delimiter = true, long)]
        token_ids: Vec<u64>,
        #[clap(name = "token-type-names", use_value_delimiter = true, long)]
        token_type_names: Vec<TypeTag>,
        #[clap(name = "token-prices", use_value_delimiter = true, long)]
        token_prices: Vec<u64>,
    },
    #[clap(name="add-token-on-solana")]
    AddTokenOnSolana {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "token-id", long)]
        token_id: u64,
        #[clap(name = "token-mint-address", long)]
        token_mint_address: String,
        #[clap(name = "token-price", long)]
        token_price: u64,

        #[clap(name = "benfen-decimal", long)]
        benfen_decimal: u8,
    },
    #[clap(name = "add-tokens-on-evm")]
    AddTokensOnEvm {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "token-ids", use_value_delimiter = true, long)]
        token_ids: Vec<u64>,
        #[clap(name = "token-type-names", use_value_delimiter = true, long)]
        token_addresses: Vec<EthAddress>,
        #[clap(name = "token-prices", use_value_delimiter = true, long)]
        token_prices: Vec<u64>,
        #[clap(name = "token-sui-decimals", use_value_delimiter = true, long)]
        token_sui_decimals: Vec<u8>,
    },
    #[clap(name = "add-lp-token-id")]
    AddLpTokenId {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "protocol-type", long)]
        protocol_type: u64,
        #[clap(name = "token-id", long)]
        token_id: u64,
        #[clap(name = "lp-token-id", long)]
        lp_token_id: u64,
    },
    #[clap(name = "upgrade-evm-contract")]
    UpgradeEVMContract {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "proxy-address", long)]
        proxy_address: EthAddress,
        /// The address of the new implementation contract
        #[clap(name = "implementation-address", long)]
        implementation_address: EthAddress,
        /// Function selector with params types, e.g. `foo(uint256,bool,string)`
        #[clap(name = "function-selector", long)]
        function_selector: Option<String>,
        /// Params to be passed to the function, e.g. `420,false,hello`
        #[clap(name = "params", use_value_delimiter = true, long)]
        params: Vec<String>,
    },
    #[clap(name="update-invest-address")]
    UpdateInvestAddress {
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "invest-address", long)]
        invest_address: EthAddress,
    },
    #[clap(name = "extend-program-on-solana")]
    ExtendProgramOnSolana{
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "program-id", long)]
        program_id: String,
        #[clap(name = "size", long)]
        size: u32,
    },

    #[clap(name = "upgrade-program-on-solana")]
    UpgradeProgramOnSolana{
        #[clap(name = "nonce", long)]
        nonce: u64,
        #[clap(name = "proxy", long)]
        proxy: String,
        #[clap(name = "implementation", long)]
        implementation: String,
        #[clap(name = "version", long)]
        version: u8,
    }
}

pub fn make_action(chain_id: BridgeChainId, cmd: &GovernanceClientCommands) -> BridgeAction {
    match cmd {
        GovernanceClientCommands::EmergencyButton { nonce, action_type } => {
            BridgeAction::EmergencyAction(EmergencyAction {
                nonce: *nonce,
                chain_id,
                action_type: *action_type,
            })
        }
        GovernanceClientCommands::UpdateCommitteeBlocklist {
            nonce,
            blocklist_type,
            pubkeys_hex,
        } => BridgeAction::BlocklistCommitteeAction(BlocklistCommitteeAction {
            nonce: *nonce,
            chain_id,
            blocklist_type: *blocklist_type,
            members_to_update: pubkeys_hex.clone(),
        }),
        GovernanceClientCommands::UpdateRefundAdmin {
            nonce,
            op_type,
            sui_address,
        } => BridgeAction::RefundAdminAction(RefundAdminAction {
            nonce: *nonce,
            chain_id,
            op_type: *op_type,
            sui_address: sui_address.clone(),
        }),
        GovernanceClientCommands::UpdateFastPathLimit {
            nonce,
            chain_id_evm,
            token_id,
            amount,
        } => {
            let chain_id_evm = BridgeChainId::try_from(*chain_id_evm).expect("Invalid chain id");
            BridgeAction::FastPathLimitUpdateAction(FastPathLimitUpdateAction {
                nonce: *nonce,
                chain_id,
                token_id: *token_id,
                amount: *amount,
                chain_id_evm,
            })
        }
        GovernanceClientCommands::UpdateLimit {
            nonce,
            sending_chain,
            new_usd_limit,
        } => {
            let sending_chain_id =
                BridgeChainId::try_from(*sending_chain).expect("Invalid sending chain id");
            BridgeAction::LimitUpdateAction(LimitUpdateAction {
                nonce: *nonce,
                chain_id,
                sending_chain_id,
                new_usd_limit: *new_usd_limit,
            })
        }
        GovernanceClientCommands::UpdateSingleTransferLimit {
            nonce,
            sending_chain,
            new_usd_limit,
        } => {
            let sending_chain_id =
                BridgeChainId::try_from(*sending_chain).expect("Invalid sending chain id");
            BridgeAction::SingleTransferLimitUpdateAction(SingleTransferLimitUpdateAction {
                nonce: *nonce,
                chain_id,
                sending_chain_id,
                new_usd_limit: *new_usd_limit,
            })
        }
        GovernanceClientCommands::UpdateAssetPrice {
            nonce,
            token_id,
            new_usd_price,
        } => BridgeAction::AssetPriceUpdateAction(AssetPriceUpdateAction {
            nonce: *nonce,
            chain_id,
            token_id: *token_id,
            new_usd_price: *new_usd_price,
        }),
        GovernanceClientCommands::AddExternalCoinAdmin {
            nonce,
            coin_type,
            admin_address,
        } => BridgeAction::AddExternalCoinAdminAction(AddExternalCoinAdminAction {
            nonce: *nonce,
            chain_id,
            coin_type: coin_type.clone(),
            admin_address: admin_address.clone(),
        }),
        GovernanceClientCommands::RemoveExternalCoinAdmin {
            nonce,
            coin_type,
            admin_address,
        } => BridgeAction::RemoveExternalCoinAdminAction(RemoveExternalCoinAdminAction {
            nonce: *nonce,
            chain_id,
            coin_type: coin_type.clone(),
            admin_address: admin_address.clone(),
        }),
        GovernanceClientCommands::AddExternalCoinWitness {
            nonce,
            coin_type,
            witness_address,
        } => BridgeAction::AddExternalCoinWitnessAction(AddExternalCoinWitnessAction {
            nonce: *nonce,
            chain_id,
            coin_type: coin_type.clone(),
            witness_address: *witness_address,
        }),
        GovernanceClientCommands::RemoveExternalCoinWitness {
            nonce,
            coin_type,
            witness_address,
        } => BridgeAction::RemoveExternalCoinWitnessAction(RemoveExternalCoinWitnessAction {
            nonce: *nonce,
            chain_id,
            coin_type: coin_type.clone(),
            witness_address: *witness_address,
        }),
        GovernanceClientCommands::AddExternalCoinTarget {
            nonce,
            coin_type,
            target_address,
        } => BridgeAction::AddExternalCoinTargetAction(AddExternalCoinTargetAction {
            nonce: *nonce,
            chain_id,
            coin_type: coin_type.clone(),
            target_address: target_address.clone(),
        }),
        GovernanceClientCommands::RemoveExternalCoinTarget {
            nonce,
            coin_type,
            target_address,
        } => BridgeAction::RemoveExternalCoinTargetAction(RemoveExternalCoinTargetAction {
            nonce: *nonce,
            chain_id,
            coin_type: coin_type.clone(),
            target_address: target_address.clone(),
        }),
        GovernanceClientCommands::AddTokenOnTokenList {
            nonce,
            from_chain_id,
            to_chain_id,
            token_id,
        } => {
            let from_chain_id = BridgeChainId::try_from(*from_chain_id).expect("Invalid chain id");
            let to_chain_id = BridgeChainId::try_from(*to_chain_id).expect("Invalid chain id");
            BridgeAction::AddTokenOnTokenListAction(AddTokenOnTokenListAction {
                nonce: *nonce,
                chain_id,
                from_chain_id,
                to_chain_id,
                token_id: *token_id,
            })
        }
        GovernanceClientCommands::RemoveTokenOnTokenList {
            nonce,
            from_chain_id,
            to_chain_id,
            token_id,
        } => {
            let from_chain_id = BridgeChainId::try_from(*from_chain_id).expect("Invalid chain id");
            let to_chain_id = BridgeChainId::try_from(*to_chain_id).expect("Invalid chain id");
            BridgeAction::RemoveTokenOnTokenListAction(RemoveTokenOnTokenListAction {
                nonce: *nonce,
                chain_id,
                from_chain_id,
                to_chain_id,
                token_id: *token_id,
            })
        }
        GovernanceClientCommands::SetBridgeFeeOnCrossOut {
            nonce,
            to_chain_id,
            token_id,
            mode,
            amount,
        } => {
            let to_chain_id = BridgeChainId::try_from(*to_chain_id).expect("Invalid chain id");
            BridgeAction::UpdateBridgeFeeOnCrossOutAction(UpdateBridgeFeeOnCrossOutAction {
                nonce: *nonce,
                chain_id,
                to_chain_id,
                token_id: *token_id,
                mode: *mode,
                amount: *amount,
            })
        }
        GovernanceClientCommands::SetBridgeFeeOnCrossIn {
            nonce,
            from_chain_id,
            token_id,
            mode,
            amount,
        } => {
            let from_chain_id = BridgeChainId::try_from(*from_chain_id).expect("Invalid chain id");
            BridgeAction::UpdateBridgeFeeOnCrossInAction(UpdateBridgeFeeOnCrossInAction {
                nonce: *nonce,
                chain_id,
                from_chain_id,
                token_id: *token_id,
                mode: *mode,
                amount: *amount,
            })
        }

        GovernanceClientCommands::WithdrawBridgeFee {
            nonce,
            recipient,
            coin_type,
            amount,
        } => BridgeAction::WithdrawBridgeFeeAction(WithdrawBridgeFeeAction {
            nonce: *nonce,
            chain_id,
            addr: *recipient,
            coin_type: coin_type.clone(),
            amount: *amount,
        }),

        GovernanceClientCommands::AddTokensOnSui {
            nonce,
            token_ids,
            token_type_names,
            token_prices,
        } => {
            assert_eq!(token_ids.len(), token_type_names.len());
            assert_eq!(token_ids.len(), token_prices.len());
            BridgeAction::AddTokensOnSuiAction(AddTokensOnSuiAction {
                nonce: *nonce,
                chain_id,
                native: false, // only foreign tokens are supported now
                token_ids: token_ids.clone(),
                token_type_names: token_type_names.clone(),
                token_prices: token_prices.clone(),
            })
        }


        //  nonce: *nonce,
        //         native: true, // only eth native tokens are supported now
        //         chain_id,
        //         token_ids: token_ids.clone(),
        //         token_addresses: token_addresses.clone(),
        //         token_prices: token_prices.clone(),
        //         token_sui_decimals: token_sui_decimals.clone(),

        GovernanceClientCommands::AddTokenOnSolana {
            nonce,
            token_id,
            token_mint_address,
            token_price,
            benfen_decimal,
        } => BridgeAction::AddTokenOnSolanaAction(AddTokenOnSolanaAction {
            nonce: *nonce,
            native: true,
            chain_id,
            token_id: *token_id,
            token_address: Pubkey::from_str(token_mint_address).expect("Invalid token address"),
            benfen_decimal: *benfen_decimal,
            token_price: *token_price,
        }),

        GovernanceClientCommands::ExtendProgramOnSolana { nonce, program_id, size } => {
            BridgeAction::ExtendProgramOnSolanaAction(ExtendProgramOnSolanaAction {
                nonce: *nonce,
                chain_id,
                program_id: Pubkey::from_str(program_id).expect("Invalid program id"),
                size: *size,
            })
        }

        GovernanceClientCommands::UpgradeProgramOnSolana { nonce, proxy, implementation, version } => {
            BridgeAction::UpgradeProgramOnSolanaAction(UpgradeProgramOnSolanaAction {
                nonce: *nonce,
                chain_id,
                proxy: Pubkey::from_str(proxy).expect("Invalid proxy address"),
                implementation: Pubkey::from_str(implementation).expect("Invalid implementation address"),
                version: *version,
            })
        }
        GovernanceClientCommands::AddTokensOnEvm {
            nonce,
            token_ids,
            token_addresses,
            token_prices,
            token_sui_decimals,
        } => {
            assert_eq!(token_ids.len(), token_addresses.len());
            assert_eq!(token_ids.len(), token_prices.len());
            assert_eq!(token_ids.len(), token_sui_decimals.len());
            BridgeAction::AddTokensOnEvmAction(AddTokensOnEvmAction {
                nonce: *nonce,
                native: true, // only eth native tokens are supported now
                chain_id,
                token_ids: token_ids.clone(),
                token_addresses: token_addresses.clone(),
                token_prices: token_prices.clone(),
                token_sui_decimals: token_sui_decimals.clone(),
            })
        }
        GovernanceClientCommands::AddLpTokenId {
            nonce,
            protocol_type,
            token_id,
            lp_token_id,
        } => {
            BridgeAction::AddLpTokenIdAction(AddLpTokenIdAction {
                nonce: *nonce,
                chain_id,
                protocol_type: *protocol_type,
                token_id: *token_id,
                lp_token_id: *lp_token_id,
            })
        },

        GovernanceClientCommands::UpdateInvestAddress {
            nonce,
            invest_address,
        } => {
            BridgeAction::UpdateInvestAddressAction(UpdateInvestAddressAction {
                nonce: *nonce,
                chain_id,
                invest_address: *invest_address,
            })
        },
        GovernanceClientCommands::UpgradeEVMContract {
            nonce,
            proxy_address,
            implementation_address,
            function_selector,
            params,
        } => {
            let call_data = match function_selector {
                Some(function_selector) => encode_call_data(function_selector, params),
                None => vec![],
            };
            BridgeAction::EvmContractUpgradeAction(EvmContractUpgradeAction {
                nonce: *nonce,
                chain_id,
                proxy_address: *proxy_address,
                new_impl_address: *implementation_address,
                call_data,
            })
        }
    }
}

fn encode_call_data(function_selector: &str, params: &[String]) -> Vec<u8> {
    let left = function_selector
        .find('(')
        .expect("Invalid function selector, no left parentheses");
    let right = function_selector
        .find(')')
        .expect("Invalid function selector, no right parentheses");
    let param_types = function_selector[left + 1..right]
        .split(',')
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    assert_eq!(param_types.len(), params.len(), "Invalid number of params");

    let mut call_data = Keccak256::digest(function_selector).digest[0..4].to_vec();
    let mut tokens = vec![];
    for (param, param_type) in params.iter().zip(param_types.iter()) {
        match param_type.to_lowercase().as_str() {
            "uint256" => {
                tokens.push(ethers::abi::Token::Uint(
                    ethers::types::U256::from_dec_str(param).expect("Invalid U256"),
                ));
            }
            "bool" => {
                tokens.push(ethers::abi::Token::Bool(match param.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => panic!("Invalid bool in params"),
                }));
            }
            "string" => {
                tokens.push(ethers::abi::Token::String(param.clone()));
            }
            // TODO: need to support more types if needed
            _ => panic!("Invalid param type"),
        }
    }
    if !tokens.is_empty() {
        call_data.extend(ethers::abi::encode(&tokens));
    }
    call_data
}

pub fn select_contract_address(
    config: &LoadedBridgeCliConfig,
    cmd: &GovernanceClientCommands,
) -> EthAddress {
    match cmd {
        GovernanceClientCommands::EmergencyButton { .. } => config.eth_bridge_proxy_address,
        GovernanceClientCommands::UpdateCommitteeBlocklist { .. } => {
            config.eth_bridge_committee_proxy_address
        }
        GovernanceClientCommands::UpdateLimit { .. } => config.eth_bridge_limiter_proxy_address,
        GovernanceClientCommands::UpdateSingleTransferLimit { .. } => {
            config.eth_bridge_limiter_proxy_address
        }
        GovernanceClientCommands::UpdateAssetPrice { .. } => config.eth_bridge_config_proxy_address,
        GovernanceClientCommands::UpgradeEVMContract { proxy_address, .. } => *proxy_address,
        GovernanceClientCommands::AddExternalCoinAdmin { .. } => unreachable!(),
        GovernanceClientCommands::RemoveExternalCoinAdmin { .. } => unreachable!(),
        GovernanceClientCommands::AddExternalCoinWitness { .. } => unreachable!(),
        GovernanceClientCommands::RemoveExternalCoinWitness { .. } => unreachable!(),
        GovernanceClientCommands::AddTokenOnTokenList { .. } => unreachable!(),
        GovernanceClientCommands::RemoveTokenOnTokenList { .. } => unreachable!(),
        GovernanceClientCommands::SetBridgeFeeOnCrossIn { .. } => unreachable!(),
        GovernanceClientCommands::SetBridgeFeeOnCrossOut { .. } => unreachable!(),
        GovernanceClientCommands::WithdrawBridgeFee { .. } => unreachable!(),
        GovernanceClientCommands::AddExternalCoinTarget { .. } => unreachable!(),
        GovernanceClientCommands::RemoveExternalCoinTarget { .. } => unreachable!(),
        GovernanceClientCommands::AddTokensOnSui { .. } => unreachable!(),
        GovernanceClientCommands::AddTokenOnSolana { .. } => unreachable!(),
        GovernanceClientCommands::AddTokensOnEvm { .. } => config.eth_bridge_config_proxy_address,
        GovernanceClientCommands::UpdateRefundAdmin { .. } => config.eth_bridge_config_proxy_address,
        GovernanceClientCommands::UpdateFastPathLimit { .. } => config.eth_bridge_config_proxy_address,
        GovernanceClientCommands::AddLpTokenId { .. } => config.eth_bridge_config_proxy_address,
        GovernanceClientCommands::UpdateInvestAddress { .. } => config.eth_bridge_proxy_address,
        GovernanceClientCommands::ExtendProgramOnSolana { .. } => unreachable!(),
        GovernanceClientCommands::UpgradeProgramOnSolana { .. } => unreachable!(),
    }
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeCliConfig {
    /// Rpc url for Sui fullnode, used for query stuff and submit transactions.
    pub sui_rpc_url: String,
    /// Rpc url for Eth fullnode, used for query stuff.
    pub eth_rpc_url: String,
    /// Proxy address for SuiBridge deployed on Eth
    pub eth_bridge_proxy_address: EthAddress,

    pub solana_rpc_url: Option<String>,

    pub solana_bridge_program_id: Option<String>,

    /// Path of the file where private key is stored. The content could be any of the following:
    /// - Base64 encoded `flag || privkey` for ECDSA key
    /// - Base64 encoded `privkey` for Raw key
    /// - Hex encoded `privkey` for Raw key
    /// At leaset one of `sui_key_path` or `eth_key_path` must be provided.
    /// If only one is provided, it will be used for both Sui and Eth.
    pub sui_key_path: Option<PathBuf>,
    /// See `sui_key_path`. Must be Secp256k1 key.
    pub eth_key_path: Option<PathBuf>,
    pub solana_key_path: Option<PathBuf>,
}

impl Config for BridgeCliConfig {}

pub struct LoadedBridgeCliConfig {
    /// Rpc url for Sui fullnode, used for query stuff and submit transactions.
    pub sui_rpc_url: String,
    /// Rpc url for Eth fullnode, used for query stuff.
    pub eth_rpc_url: String,

    pub solana_rpc_url: Option<String>,

    pub solana_bridge_program_id: Option<Pubkey>,

    /// Proxy address for SuiBridge deployed on Eth
    pub eth_bridge_proxy_address: EthAddress,
    /// Proxy address for BridgeCommittee deployed on Eth
    pub eth_bridge_committee_proxy_address: EthAddress,
    /// Proxy address for BridgeConfig deployed on Eth
    pub eth_bridge_config_proxy_address: EthAddress,
    /// Proxy address for BridgeLimiter deployed on Eth
    pub eth_bridge_limiter_proxy_address: EthAddress,
    /// Key pair for Sui operations
    sui_key: SuiKeyPair,
    /// Key pair for Eth operations, must be Secp256k1 key
    eth_signer: EthSigner,

    solana_signer: SolanaSigner,
}

impl LoadedBridgeCliConfig {
    pub async fn load(cli_config: BridgeCliConfig) -> anyhow::Result<Self> {
        if cli_config.eth_key_path.is_none() && cli_config.sui_key_path.is_none() {
            return Err(anyhow!(
                "At least one of `sui_key_path` or `eth_key_path` must be provided"
            ));
        }
        let sui_key = if let Some(sui_key_path) = &cli_config.sui_key_path {
            Some(read_key(sui_key_path, false)?)
        } else {
            None
        };
        let eth_key = if let Some(eth_key_path) = &cli_config.eth_key_path {
            let eth_key = read_key(eth_key_path, true)?;
            Some(eth_key)
        } else {
            None
        };
        let (eth_key, sui_key) = {
            if eth_key.is_none() {
                let sui_key = sui_key.unwrap();
                if !matches!(sui_key, SuiKeyPair::Secp256k1(_)) {
                    return Err(anyhow!("Eth key must be an ECDSA key"));
                }
                (sui_key.copy(), sui_key)
            } else if sui_key.is_none() {
                let eth_key = eth_key.unwrap();
                (eth_key.copy(), eth_key)
            } else {
                (eth_key.unwrap(), sui_key.unwrap())
            }
        };

        let solana_key = if let Some(solana_key_path) = &cli_config.solana_key_path {
            read_keypair_file(solana_key_path.to_str().unwrap()).expect("Failed to read solana key pair file")
        } else {
            return Err(anyhow!("`solana_key_path` must be provided"));
        };

        let provider = Arc::new(
            ethers::prelude::Provider::<ethers::providers::Http>::try_from(&cli_config.eth_rpc_url)
                .unwrap()
                .interval(std::time::Duration::from_millis(2000)),
        );
        let private_key = Hex::encode(eth_key.to_bytes_no_flag());
        let eth_signer = get_eth_signer_client(&cli_config.eth_rpc_url, &private_key).await?;
        let solana_signer = Arc::new(solana_key);
        let sui_bridge = EthSuiBridge::new(cli_config.eth_bridge_proxy_address, provider.clone());
        let eth_bridge_committee_proxy_address: EthAddress = sui_bridge.committee().call().await?;
        let eth_bridge_limiter_proxy_address: EthAddress = sui_bridge.limiter().call().await?;
        let eth_committee =
            EthBridgeCommittee::new(eth_bridge_committee_proxy_address, provider.clone());
        let eth_bridge_committee_proxy_address: EthAddress = sui_bridge.committee().call().await?;
        let eth_bridge_config_proxy_address: EthAddress = eth_committee.config().call().await?;

        let eth_address = eth_signer.address();
        let eth_chain_id = provider.get_chainid().await?;
        let sui_address = SuiAddress::from(&sui_key.public());
        println!("Using Sui address: {:?}", sui_address);
        println!("Using Eth address: {:?}", eth_address);
        println!("Using Solana address: {:?}", solana_signer.pubkey());
        println!("Using Eth chain: {:?}", eth_chain_id);

        let solana_rpc_url = if let Some(solana_rpc_url) = &cli_config.solana_rpc_url {
            Some(solana_rpc_url.clone())
        } else {
            None
        };

        let solana_bridge_program_id = if let Some(solana_bridge_program_id) = &cli_config.solana_bridge_program_id {
            Some(Pubkey::from_str(solana_bridge_program_id)?)
        } else {
            None
        };

        Ok(Self {
            sui_rpc_url: cli_config.sui_rpc_url,
            eth_rpc_url: cli_config.eth_rpc_url,
            solana_rpc_url,
            solana_bridge_program_id,
            eth_bridge_proxy_address: cli_config.eth_bridge_proxy_address,
            eth_bridge_committee_proxy_address,
            eth_bridge_limiter_proxy_address,
            eth_bridge_config_proxy_address,
            sui_key,
            eth_signer,
            solana_signer,
        })
    }
}

impl LoadedBridgeCliConfig {
    pub fn eth_signer(self: &LoadedBridgeCliConfig) -> &EthSigner {
        &self.eth_signer
    }

    pub fn solana_signer(self: &LoadedBridgeCliConfig)->&SolanaSigner {
        &self.solana_signer
    }

    pub async fn get_sui_account_info(
        self: &LoadedBridgeCliConfig,
    ) -> anyhow::Result<(SuiKeyPair, SuiAddress, ObjectRef)> {
        let pubkey = self.sui_key.public();
        let sui_client_address = SuiAddress::from(&pubkey);
        let sui_sdk_client = SuiClientBuilder::default()
            .build(self.sui_rpc_url.clone())
            .await?;
        let gases = sui_sdk_client
            .coin_read_api()
            .get_coins(sui_client_address, None, None, None)
            .await?
            .data;
        // TODO: is 5 Sui a good number?
        let gas = gases
            .into_iter()
            .find(|coin| coin.balance >= 5_000_000_000)
            .ok_or(anyhow!(
                "Did not find gas object with enough balance for {}",
                sui_client_address
            ))?;
        println!("Using Gas object: {}", gas.coin_object_id);
        Ok((self.sui_key.copy(), sui_client_address, gas.object_ref()))
    }
}
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum BridgeClientCommands {
    #[clap(name = "deposit-native-ether-on-eth")]
    DepositNativeEtherOnEth {
        #[clap(long)]
        ether_amount: f64,
        #[clap(long)]
        target_chain: u8,
        #[clap(long)]
        sui_recipient_address: SuiAddress,
    },
    #[clap(name = "deposit-on-sui")]
    DepositOnSui {
        #[clap(long)]
        coin_object_id: ObjectID,
        #[clap(long)]
        coin_type: String,
        #[clap(long)]
        token_id_expect: u64,
        #[clap(long)]
        target_chain: u8,
        #[clap(long)]
        recipient_address: EthAddress,
    },
    #[clap(name = "claim-on-eth")]
    ClaimOnEth {
        #[clap(long)]
        seq_num: u64,
        #[clap(long, default_value_t = true, action = clap::ArgAction::Set)]
        dry_run: bool,
    },
    #[clap(name = "set-mint-busd-limit")]
    SetMintBusdLimit {
        #[clap(name = "modify-cap-id", long)]
        modify_cap_id: ObjectID,
        #[clap(name = "new-limit", long)]
        new_limit: u64,
    },
}

impl BridgeClientCommands {
    pub async fn handle(
        self,
        config: &LoadedBridgeCliConfig,
        sui_bridge_client: SuiBridgeClient,
    ) -> anyhow::Result<()> {
        match self {
            BridgeClientCommands::DepositNativeEtherOnEth {
                ether_amount,
                target_chain,
                sui_recipient_address,
            } => {
                let eth_sui_bridge = EthSuiBridge::new(
                    config.eth_bridge_proxy_address,
                    Arc::new(config.eth_signer().clone()),
                );
                // Note: even with f64 there may still be loss of precision even there are a lot of 0s
                let int_part = ether_amount.trunc() as u64;
                let frac_part = ether_amount.fract();
                let int_wei = U256::from(int_part) * U256::exp10(18);
                let frac_wei = U256::from((frac_part * 1_000_000_000_000_000_000f64) as u64);
                let amount = int_wei + frac_wei;
                let eth_tx = eth_sui_bridge
                    .bridge_eth(sui_recipient_address.to_vec().into(), target_chain)
                    .value(amount);
                let pending_tx = eth_tx.send().await.unwrap();
                let tx_receipt = pending_tx.await.unwrap().unwrap();
                info!(
                    "Deposited {ether_amount} Ethers to {:?} (target chain {target_chain}). Receipt: {:?}", sui_recipient_address, tx_receipt,
                );
                Ok(())
            }
            BridgeClientCommands::ClaimOnEth { seq_num, dry_run } => {
                claim_on_eth(seq_num, config, sui_bridge_client, dry_run)
                    .await
                    .map_err(|e| anyhow!("{:?}", e))
            }
            BridgeClientCommands::DepositOnSui {
                coin_object_id,
                coin_type,
                token_id_expect,
                target_chain,
                recipient_address,
            } => {
                let target_chain = BridgeChainId::try_from(target_chain).expect("Invalid chain id");
                let coin_type = TypeTag::from_str(&coin_type).expect("Invalid coin type");
                deposit_on_sui(
                    coin_object_id,
                    coin_type,
                    token_id_expect,
                    target_chain,
                    recipient_address,
                    config,
                    sui_bridge_client,
                )
                .await
            }
            BridgeClientCommands::SetMintBusdLimit {
                modify_cap_id,
                new_limit,
            } => set_busd_limit(modify_cap_id, new_limit, config, sui_bridge_client).await,
        }
    }
}

async fn set_busd_limit(
    modify_cap_id: ObjectID,
    new_limit: u64,
    config: &LoadedBridgeCliConfig,
    sui_bridge_client: SuiBridgeClient,
) -> anyhow::Result<()> {
    let sui_client = sui_bridge_client.sui_client();
    let bridge_object_arg = sui_bridge_client
        .get_mutable_bridge_object_arg_must_succeed()
        .await;
    let bfc_system_modify_cap = sui_client.get_cap_object_ref(modify_cap_id).await?;
    let rgp = sui_client
        .governance_api()
        .get_reference_gas_price()
        .await
        .unwrap();
    let sender = SuiAddress::from(&config.sui_key.public());
    let gas_obj_ref = sui_client
        .coin_read_api()
        .select_coins(sender, None, 1_000_000_000, vec![])
        .await?
        .first()
        .ok_or(anyhow!("No coin found for address {}", sender))?
        .object_ref();
    let mut builder = ProgrammableTransactionBuilder::new();
    let arg_bridge = builder.obj(bridge_object_arg)?;
    let system_obj = builder.input(CallArg::BFC_SYSTEM_MUT)?;
    let cap_obj = builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(
        bfc_system_modify_cap,
    )))?;
    let new_limit = builder.pure(new_limit)?;

    CallArg::Object(ObjectArg::ImmOrOwnedObject(bfc_system_modify_cap));
    builder.programmable_move_call(
        BRIDGE_PACKAGE_ID,
        ident_str!("bridge").to_owned(),
        ident_str!("set_max_mint_busd_amount").to_owned(),
        vec![],
        vec![arg_bridge, system_obj, cap_obj, new_limit],
    );
    let pt = builder.finish();
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_obj_ref], pt, 100_000_000, rgp);
    let sig = Signature::new_secure(
        &IntentMessage::new(Intent::sui_transaction(), tx_data.clone()),
        &config.sui_key,
    );
    let signed_tx = Transaction::from_data(tx_data, vec![sig]);
    let tx_digest = *signed_tx.digest();
    info!(?tx_digest, "set busd limit amount.");
    let resp = sui_bridge_client
        .execute_transaction_block_with_effects(signed_tx)
        .await
        .expect("Failed to execute set busd transaction block");
    if !resp.status_ok().unwrap() {
        return Err(anyhow!("Transaction {:?} failed: {:?}", tx_digest, resp));
    }
    let events = resp.events.unwrap();
    info!(?tx_digest, "Transaction succeeded. Events: {:?}", events);
    Ok(())
}

async fn deposit_on_sui(
    coin_object_id: ObjectID,
    coin_type: TypeTag,
    token_id_expect: u64,
    target_chain: BridgeChainId,
    recipient_address: EthAddress,
    config: &LoadedBridgeCliConfig,
    sui_bridge_client: SuiBridgeClient,
) -> anyhow::Result<()> {
    let target_chain = target_chain as u8;
    let sui_client = sui_bridge_client.sui_client();
    let bridge_object_arg = sui_bridge_client
        .get_mutable_bridge_object_arg_must_succeed()
        .await;
    let rgp = sui_client
        .governance_api()
        .get_reference_gas_price()
        .await
        .unwrap();
    let sender = SuiAddress::from(&config.sui_key.public());
    let gas_obj_ref = sui_client
        .coin_read_api()
        .select_coins(sender, None, 1_000_000_000, vec![])
        .await?
        .first()
        .ok_or(anyhow!("No coin found for address {}", sender))?
        .object_ref();
    let coin_obj_ref = sui_client
        .read_api()
        .get_object_with_options(coin_object_id, SuiObjectDataOptions::default())
        .await?
        .data
        .unwrap()
        .object_ref();

    let mut builder = ProgrammableTransactionBuilder::new();
    let arg_target_chain = builder.pure(target_chain).unwrap();
    let arg_target_address = builder.pure(recipient_address.as_bytes()).unwrap();
    let arg_token = builder
        .obj(ObjectArg::ImmOrOwnedObject(coin_obj_ref))
        .unwrap();
    let arg_bridge = builder.obj(bridge_object_arg).unwrap();
    let arg_token_id_expect = builder.pure(token_id_expect).unwrap();
    builder.programmable_move_call(
        BRIDGE_PACKAGE_ID,
        BRIDGE_MODULE_NAME.to_owned(),
        ident_str!("send_token").to_owned(),
        vec![coin_type],
        vec![
            arg_bridge,
            arg_target_chain,
            arg_target_address,
            arg_token,
            arg_token_id_expect,
        ],
    );
    let pt = builder.finish();
    let tx_data =
        TransactionData::new_programmable(sender, vec![gas_obj_ref], pt, 500_000_000, rgp);
    let sig = Signature::new_secure(
        &IntentMessage::new(Intent::sui_transaction(), tx_data.clone()),
        &config.sui_key,
    );
    let signed_tx = Transaction::from_data(tx_data, vec![sig]);
    let tx_digest = *signed_tx.digest();
    info!(?tx_digest, "Sending deposit transction to Sui.");
    let resp = sui_bridge_client
        .execute_transaction_block_with_effects(signed_tx)
        .await
        .expect("Failed to execute transaction block");
    if !resp.status_ok().unwrap() {
        return Err(anyhow!("Transaction {:?} failed: {:?}", tx_digest, resp));
    }
    let events = resp.events.unwrap();
    info!(
        ?tx_digest,
        "Deposit transaction succeeded. Events: {:?}", events
    );
    Ok(())
}

async fn claim_on_eth(
    seq_num: u64,
    config: &LoadedBridgeCliConfig,
    sui_bridge_client: SuiBridgeClient,
    dry_run: bool,
) -> BridgeResult<()> {
    let sui_chain_id = sui_bridge_client.get_bridge_summary().await?.chain_id;
    let parsed_message = sui_bridge_client
        .get_parsed_token_transfer_message(sui_chain_id, seq_num)
        .await?;
    if parsed_message.is_none() {
        println!("No record found for seq_num: {seq_num}, chain id: {sui_chain_id}");
        return Ok(());
    }
    let parsed_message = parsed_message.unwrap();
    let sigs = sui_bridge_client
        .get_token_transfer_action_onchain_signatures_until_success(sui_chain_id, seq_num)
        .await;
    if sigs.is_none() {
        println!("No signatures found for seq_num: {seq_num}, chain id: {sui_chain_id}");
        return Ok(());
    }
    let signatures = sigs
        .unwrap()
        .into_iter()
        .map(|sig: Vec<u8>| ethers::types::Bytes::from(sig))
        .collect::<Vec<_>>();

    let eth_sui_bridge = EthSuiBridge::new(
        config.eth_bridge_proxy_address,
        Arc::new(config.eth_signer().clone()),
    );
    let message = eth_sui_bridge::Message::from(parsed_message);
    let tx = eth_sui_bridge.transfer_bridged_tokens_with_signatures(signatures, message);
    if dry_run {
        let tx = tx.tx;
        let resp = config.eth_signer.estimate_gas(&tx, None).await;
        println!(
            "Sui to Eth bridge transfer claim dry run result: {:?}",
            resp
        );
    } else {
        let eth_claim_tx_receipt = tx.send().await.unwrap().await.unwrap().unwrap();
        println!(
            "Sui to Eth bridge transfer claimed: {:?}",
            eth_claim_tx_receipt
        );
    }
    Ok(())
}
#[allow(dead_code)]
async fn claim_defi_on_eth(
    seq_num: u64,
    config: &LoadedBridgeCliConfig,
    sui_bridge_client: SuiBridgeClient,
    dry_run: bool,
) -> BridgeResult<()> {
    let sui_chain_id = sui_bridge_client.get_bridge_summary().await?.chain_id;
    let parsed_message = sui_bridge_client
        .get_parsed_defi_transfer_out_message(sui_chain_id, seq_num)
        .await?;
    if parsed_message.is_none() {
        println!("No record found for seq_num: {seq_num}, chain id: {sui_chain_id}");
        return Ok(());
    }
    let parsed_message = parsed_message.unwrap();
    let sigs = sui_bridge_client
        .get_defi_transfer_out_action_onchain_signatures_until_success(sui_chain_id, seq_num)
        .await;
    if sigs.is_none() {
        println!("No signatures found for seq_num: {seq_num}, chain id: {sui_chain_id}");
        return Ok(());
    }
    let signatures = sigs
        .unwrap()
        .into_iter()
        .map(|sig: Vec<u8>| ethers::types::Bytes::from(sig))
        .collect::<Vec<_>>();

    let eth_sui_bridge = EthSuiBridge::new(
        config.eth_bridge_proxy_address,
        Arc::new(config.eth_signer().clone()),
    );
    let message = eth_sui_bridge::Message::from(parsed_message);
    let tx = eth_sui_bridge.invest_bridged_tokens_with_signatures(signatures, message);
    if dry_run {
        let tx = tx.tx;
        let resp = config.eth_signer.estimate_gas(&tx, None).await;
        println!(
            "Sui to Eth bridge transfer claim dry run result: {:?}",
            resp
        );
    } else {
        let eth_claim_tx_receipt = tx.send().await.unwrap().await.unwrap().unwrap();
        println!(
            "Sui to Eth bridge transfer claimed: {:?}",
            eth_claim_tx_receipt
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use ethers::abi::FunctionExt;

    use super::*;

    #[tokio::test]
    async fn test_encode_call_data() {
        let abi_json =
            std::fs::read_to_string("../sui-bridge/abi/tests/mock_sui_bridge_v2.json").unwrap();
        let abi: ethers::abi::Abi = serde_json::from_str(&abi_json).unwrap();

        let function_selector = "initializeV2Params(uint256,bool,string)";
        let params = vec!["420".to_string(), "false".to_string(), "hello".to_string()];
        let call_data = encode_call_data(function_selector, &params);

        let function = abi
            .functions()
            .find(|f| {
                let selector = f.selector();
                call_data.starts_with(selector.as_ref())
            })
            .expect("Function not found");

        // Decode the data excluding the selector
        let tokens = function.decode_input(&call_data[4..]).unwrap();
        assert_eq!(
            tokens,
            vec![
                ethers::abi::Token::Uint(ethers::types::U256::from_dec_str("420").unwrap()),
                ethers::abi::Token::Bool(false),
                ethers::abi::Token::String("hello".to_string())
            ]
        )
    }
}
