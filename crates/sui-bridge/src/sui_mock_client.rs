// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! A mock implementation of Sui JSON-RPC client.

use crate::error::{BridgeError, BridgeResult};
use crate::test_utils::DUMMY_MUTALBE_BRIDGE_OBJECT_ARG;
use async_trait::async_trait;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};
use sui_json_rpc_types::{ SuiTransactionBlockResponse};
use sui_json_rpc_types::{EventFilter, EventPage, SuiEvent};
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::base_types::ObjectRef;
use sui_types::bridge::{BridgeCommitteeSummary, BridgeSummary, BridgeTokenMetadata, BridgeTreasurySummary, MoveTypeParsedDefiTransferOutMessage, MoveTypeParsedTokenTransferMessageV2};
use sui_types::digests::TransactionDigest;
use sui_types::event::EventID;
use sui_types::gas_coin::GasCoin;
use sui_types::object::Owner;
use sui_types::transaction::ObjectArg;
use sui_types::transaction::Transaction;
use sui_types::{Identifier, TypeTag};

use crate::sui_client::SuiClientInner;
use crate::types::{BridgeAction, BridgeActionStatus, IsBridgePaused};

/// Mock client used in test environments.
#[allow(clippy::type_complexity)]
#[derive(Clone, Debug)]
pub struct SuiMockClient {
    // the top two fields do not change during tests so we don't need them to be Arc<Mutex>>
    chain_identifier: String,
    latest_checkpoint_sequence_number: Arc<AtomicU64>,
    events: Arc<Mutex<HashMap<(ObjectID, Identifier, Option<EventID>), EventPage>>>,
    past_event_query_params: Arc<Mutex<VecDeque<(ObjectID, Identifier, Option<EventID>)>>>,
    events_by_tx_digest:
        Arc<Mutex<HashMap<TransactionDigest, Result<Vec<SuiEvent>, sui_sdk::error::Error>>>>,
    transaction_responses:
        Arc<Mutex<HashMap<TransactionDigest, BridgeResult<SuiTransactionBlockResponse>>>>,
    wildcard_transaction_response: Arc<Mutex<Option<BridgeResult<SuiTransactionBlockResponse>>>>,
    get_object_info: Arc<Mutex<HashMap<ObjectID, (GasCoin, ObjectRef, Owner)>>>,
    onchain_status: Arc<Mutex<HashMap<(u8, u64), BridgeActionStatus>>>,
    bridge_committee_summary: Arc<Mutex<Option<BridgeCommitteeSummary>>>,
    is_paused: Arc<Mutex<Option<IsBridgePaused>>>,
    requested_transactions_tx: tokio::sync::broadcast::Sender<TransactionDigest>,
}

impl SuiMockClient {
    pub fn default() -> Self {
        Self {
            chain_identifier: "".to_string(),
            latest_checkpoint_sequence_number: Arc::new(AtomicU64::new(0)),
            events: Default::default(),
            past_event_query_params: Default::default(),
            events_by_tx_digest: Default::default(),
            transaction_responses: Default::default(),
            wildcard_transaction_response: Default::default(),
            get_object_info: Default::default(),
            onchain_status: Default::default(),
            bridge_committee_summary: Default::default(),
            is_paused: Default::default(),
            requested_transactions_tx: tokio::sync::broadcast::channel(10000).0,
        }
    }

    pub fn add_event_response(
        &self,
        package: ObjectID,
        module: Identifier,
        cursor: EventID,
        events: EventPage,
    ) {
        self.events
            .lock()
            .unwrap()
            .insert((package, module, Some(cursor)), events);
    }

    pub fn add_events_by_tx_digest(&self, tx_digest: TransactionDigest, events: Vec<SuiEvent>) {
        self.events_by_tx_digest
            .lock()
            .unwrap()
            .insert(tx_digest, Ok(events));
    }

    pub fn add_events_by_tx_digest_error(&self, tx_digest: TransactionDigest) {
        self.events_by_tx_digest.lock().unwrap().insert(
            tx_digest,
            Err(sui_sdk::error::Error::DataError("".to_string())),
        );
    }

    pub fn add_transaction_response(
        &self,
        tx_digest: TransactionDigest,
        response: BridgeResult<SuiTransactionBlockResponse>,
    ) {
        self.transaction_responses
            .lock()
            .unwrap()
            .insert(tx_digest, response);
    }

    pub fn set_action_onchain_status(&self, action: &BridgeAction, status: BridgeActionStatus) {
        self.onchain_status
            .lock()
            .unwrap()
            .insert((action.chain_id() as u8, action.seq_number()), status);
    }

    pub fn set_bridge_committee(&self, committee: BridgeCommitteeSummary) {
        self.bridge_committee_summary
            .lock()
            .unwrap()
            .replace(committee);
    }

    pub fn set_is_bridge_paused(&self, value: IsBridgePaused) {
        self.is_paused.lock().unwrap().replace(value);
    }

    pub fn set_wildcard_transaction_response(
        &self,
        response: BridgeResult<SuiTransactionBlockResponse>,
    ) {
        *self.wildcard_transaction_response.lock().unwrap() = Some(response);
    }

    pub fn set_latest_checkpoint_sequence_number(&self, value: u64) {
        self.latest_checkpoint_sequence_number
            .store(value, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn add_gas_object_info(&self, gas_coin: GasCoin, object_ref: ObjectRef, owner: Owner) {
        self.get_object_info
            .lock()
            .unwrap()
            .insert(object_ref.0, (gas_coin, object_ref, owner));
    }

    pub fn subscribe_to_requested_transactions(
        &self,
    ) -> tokio::sync::broadcast::Receiver<TransactionDigest> {
        self.requested_transactions_tx.subscribe()
    }
    #[allow(unused)]
    async fn get_cap_object_ref(
        &self,
        _cap_id: ObjectID,
    ) -> anyhow::Result<ObjectRef> {
        //todo: @suoyuan
        Err(anyhow::anyhow!("Not implemented"))
    }

}

#[async_trait]
impl SuiClientInner for SuiMockClient {
    type Error = sui_sdk::error::Error;

    async fn notify_something_done(&self) {
        self.requested_transactions_tx.send(TransactionDigest::random()).unwrap();
    }
    async fn get_object_for_cap(&self,
                                _address: SuiAddress,
                                _filter_tag: &str) -> Result<ObjectArg, Self::Error> {
        Ok(DUMMY_MUTALBE_BRIDGE_OBJECT_ARG)
    }


    // Unwraps in this function: We assume the responses are pre-populated
    // by the test before calling into this function.
    async fn query_events(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
    ) -> Result<EventPage, Self::Error> {
        let events = self.events.lock().unwrap();
        match query {
            EventFilter::MoveEventModule { package, module } => {
                self.past_event_query_params.lock().unwrap().push_back((
                    package,
                    module.clone(),
                    cursor,
                ));
                Ok(events
                    .get(&(package, module.clone(), cursor))
                    .cloned()
                    .unwrap_or_else(|| {
                        panic!(
                            "No preset events found for package: {:?}, module: {:?}, cursor: {:?}",
                            package, module, cursor
                        )
                    }))
            }
            _ => unimplemented!(),
        }
    }

    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<Vec<SuiEvent>, Self::Error> {
        let events = self.events_by_tx_digest.lock().unwrap();

        match events
            .get(&tx_digest)
            .unwrap_or_else(|| panic!("No preset events found for tx_digest: {:?}", tx_digest))
        {
            Ok(events) => Ok(events.clone()),
            // sui_sdk::error::Error is not Clone
            Err(_) => Err(sui_sdk::error::Error::DataError("".to_string())),
        }
    }

    async fn get_chain_identifier(&self) -> Result<String, Self::Error> {
        Ok(self.chain_identifier.clone())
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, Self::Error> {
        Ok(self
            .latest_checkpoint_sequence_number
            .load(std::sync::atomic::Ordering::Relaxed))
    }

    async fn get_mutable_bridge_object_arg(&self) -> Result<ObjectArg, Self::Error> {
        Ok(DUMMY_MUTALBE_BRIDGE_OBJECT_ARG)
    }

    async fn get_reference_gas_price(&self) -> Result<u64, Self::Error> {
        Ok(1000)
    }

    async fn get_bridge_summary(&self) -> Result<BridgeSummary, Self::Error> {
        Ok(BridgeSummary {
            bridge_version: 0,
            message_version: 0,
            chain_id: 0,
            sequence_nums: vec![],
            bridge_records_id: ObjectID::random(),
            is_frozen: self.is_paused.lock().unwrap().unwrap_or_default(),
            limiter: Default::default(),
            committee: self
                .bridge_committee_summary
                .lock()
                .unwrap()
                .clone()
                .unwrap_or_default(),
                treasury: BridgeTreasurySummary {
                    external_coin_target_address: vec![
                        ("0x11c6be44f809a2a017d2e5580b2ceab5cd3e20582da1e615c92127222470ac75".to_string(),
                            vec![
                                "n1sfLwoLTnLFxj2BT8kNETsLDM8xMecYn3".to_string(),
                                "123".to_string(),
                            ],
                        ),
                    ],
                    supported_tokens: vec![("11c6be44f809a2a017d2e5580b2ceab5cd3e20582da1e615c92127222470ac75::btc::BTC".to_string(), BridgeTokenMetadata{
                        id: 1,
                        decimal_multiplier: 100_000_000,
                        notional_value: 50_000 ,
                        native_token: false,
                    }), ("12c6be44f809a2a017d2e5580b2ceab5cd3e20582da1e615c92127222470ac75::eth::ETH".to_string(), BridgeTokenMetadata{
                        id: 2,
                        decimal_multiplier: 100_000_000,
                        notional_value: 3_000 ,
                        native_token: false,
                    }), ("13c6be44f809a2a017d2e5580b2ceab5cd3e20582da1e615c92127222470ac75::usdc::USDC".to_string(), BridgeTokenMetadata{
                        id: 3,
                        decimal_multiplier: 1_000_000,
                        notional_value: 1,
                        native_token: false,
                    }), ("14c6be44f809a2a017d2e5580b2ceab5cd3e20582da1e615c92127222470ac75::usdt::USDT".to_string(), BridgeTokenMetadata{
                        id: 4,
                        decimal_multiplier: 1_000_000,
                        notional_value: 1,
                        native_token: false,
                    })],
                    id_token_type_map: vec![(1,"11c6be44f809a2a017d2e5580b2ceab5cd3e20582da1e615c92127222470ac75::usdc::USDC".to_string()), (2,"12c6be44f809a2a017d2e5580b2ceab5cd3e20582da1e615c92127222470ac75::eth::ETH".to_string()), (3,"13c6be44f809a2a017d2e5580b2ceab5cd3e20582da1e615c92127222470ac75::usdc::USDC".to_string()), (4,"14c6be44f809a2a017d2e5580b2ceab5cd3e20582da1e615c92127222470ac75::usdt::USDT".to_string())],
                },
        })
    }

    async fn get_token_transfer_action_onchain_status(
        &self,
        _bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<BridgeActionStatus, BridgeError> {
        Ok(self
            .onchain_status
            .lock()
            .unwrap()
            .get(&(source_chain_id, seq_number))
            .cloned()
            .unwrap_or(BridgeActionStatus::Pending))
    }

    async fn get_defi_transfer_action_onchain_status(
        &self,
        _bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<BridgeActionStatus, BridgeError> {
        Ok(self
            .onchain_status
            .lock()
            .unwrap()
            .get(&(source_chain_id, seq_number))
            .cloned()
            .unwrap_or(BridgeActionStatus::Pending))
    }

    async fn get_eth_to_sui_limit(
        &self,
        _bridge_object_arg: ObjectArg,
        _source_chain_id: u8,
        _token_type:u64,
        _ca_token_type_map:HashMap<u64,TypeTag>
    ) -> Result<u128, BridgeError> {
        Ok(1000)
    }

    async fn get_cross_out_fee_amount(
        &self,
        _bridge_object_arg: ObjectArg,
        _chain_id: u64,
        _amount: u64,
        _token_type:u64,
        _ca_token_type_map:HashMap<u64,TypeTag>
    ) -> Result<u64, BridgeError> {
        Ok(1000)
    }

    async fn get_external_token_transfer_action_onchain_status(
        &self,
        _bridge_object_arg: ObjectArg,
        _source_chain: u8,
        _source_address: &Vec<u8>,
        _target_address: &Vec<u8>,
        _amount: u64,
        _tx_hash: String,
    ) -> Result<BridgeActionStatus, BridgeError> {
        Ok(BridgeActionStatus::NotFound)
    }

    async fn get_defi_transfer_action_status(
        &self,
        _bridge_object_arg: ObjectArg,
        _source_chain: u8,
        _seq_number: u64,
    ) -> Result<BridgeActionStatus, BridgeError> {
        Ok(BridgeActionStatus::NotFound)
    }

    async fn get_send_back_onchain_status(
        &self,
        _bridge_object_arg: ObjectArg,
        _tx_hash: Vec<u8>,
    ) -> Result<BridgeActionStatus, BridgeError> {
        Ok(BridgeActionStatus::NotFound)
    }

    async fn get_token_transfer_action_onchain_signatures(
        &self,
        _bridge_object_arg: ObjectArg,
        _source_chain_id: u8,
        _seq_number: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, BridgeError> {
        unimplemented!()
    }

    async fn get_defi_transfer_out_action_onchain_signatures(
        &self,
        _bridge_object_arg: ObjectArg,
        _source_chain_id: u8,
        _seq_number: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, BridgeError> {
        unimplemented!()
    }

    async fn get_parsed_token_transfer_message(
        &self,
        _bridge_object_arg: ObjectArg,
        _source_chain_id: u8,
        _seq_number: u64,
    ) -> Result<Option<MoveTypeParsedTokenTransferMessageV2>, BridgeError> {
        unimplemented!()
    }

    async fn get_parsed_defi_transfer_out_message(
        &self,
        _bridge_object_arg: ObjectArg,
        _source_chain_id: u8,
        _seq_number: u64,
    ) -> Result<Option<MoveTypeParsedDefiTransferOutMessage>, BridgeError> {
        unimplemented!()
    }

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<SuiTransactionBlockResponse, BridgeError> {
        self.requested_transactions_tx.send(*tx.digest()).unwrap();
        match self.transaction_responses.lock().unwrap().get(tx.digest()) {
            Some(response) => response.clone(),
            None => self
                .wildcard_transaction_response
                .lock()
                .unwrap()
                .clone()
                .unwrap_or_else(|| panic!("No preset transaction response found for tx: {:?}", tx)),
        }
    }

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        self.get_object_info
            .lock()
            .unwrap()
            .get(&gas_object_id)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "No preset gas object info found for gas_object_id: {:?}",
                    gas_object_id
                )
            })
    }

    async fn get_cap_object_ref(
        &self,
        _cap_id: ObjectID,
    ) -> anyhow::Result<ObjectRef> {
        //todo: @suoyuan
        Err(anyhow::anyhow!("Not implemented"))
    }
}
