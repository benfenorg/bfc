use std::{sync::Arc, time::Duration};

use mysten_metrics::spawn_logged_monitored_task;
use shared_crypto::intent::{Intent, IntentMessage};
use sui_json_rpc_types::{SuiExecutionStatus, SuiTransactionBlockEffectsAPI, SuiTransactionBlockResponse};
use sui_types::{base_types::{ObjectID, ObjectRef, SuiAddress}, crypto::{Signature, SuiKeyPair}, digests::TransactionDigest, gas_coin::GasCoin, object::Owner, transaction::{ObjectArg, Transaction}};
use tracing::{error, info};

use crate::{action_executor::{BridgeActionExecutionWrapper, CHANNEL_SIZE, submit_to_executor}, aml::{check_aml_risk_score, check_aml_risk_score_solana}, fast_path::FastPathSelector, metrics::BridgeMetrics, storage::BridgeOrchestratorTables, sui_client::SuiClientInner, sui_transaction_builder::build_token_send_back_transaction, types::{BridgeAction, BridgeActionStatus}};
use crate::sui_client::SuiClient;

#[derive(Debug)]
pub struct AMLCheckerWrapper(pub BridgeAction, pub u64);

pub trait AMLCheckerTrait {
    fn run(
        self,
        executor_sender: mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
    ) -> (
        Vec<tokio::task::JoinHandle<()>>,
        mysten_metrics::metered_channel::Sender<AMLCheckerWrapper>,
    );
}

pub struct AMLChecker<P> {
    store: Arc<BridgeOrchestratorTables>,
    sui_client: Arc<SuiClient<P>>,
    sui_address: SuiAddress,
    gas_object_id: ObjectID,
    bridge_object_arg: ObjectArg,
    metrics: Arc<BridgeMetrics>,
    key: SuiKeyPair,
    aml_key: String,
}

impl<P> AMLCheckerTrait for AMLChecker<P>
where
    P: SuiClientInner + 'static,
{
    fn run(self,executor_sender: mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>) -> (
        Vec<tokio::task::JoinHandle<()>>,
        mysten_metrics::metered_channel::Sender<AMLCheckerWrapper>,
    ) {
        let (sender, receiver) = mysten_metrics::metered_channel::channel(
            CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["aml_checker_queue"]),
        );
        let executor_sender_clone = executor_sender.clone();
        let store_clone = self.store.clone();
        let mut tasks = vec![];
        tasks.push(spawn_logged_monitored_task!(
            Self::run_inner(&self.sui_client, receiver, &store_clone, executor_sender_clone.clone(), &self.metrics,self.sui_address,self.gas_object_id,self.bridge_object_arg,&self.key,self.aml_key.clone())
        ));
        tasks.push(spawn_logged_monitored_task!(
            Self::resubmit_pending_actions(&self.store,executor_sender)
        ));
        (tasks, sender)
    }
}

impl<P> AMLChecker<P>
where
    P: SuiClientInner + 'static,{
    pub async fn new(
        store: Arc<BridgeOrchestratorTables>,
        sui_client: Arc<SuiClient<P>>,
        sui_address: SuiAddress,
        gas_object_id: ObjectID,
        key: SuiKeyPair,
        metrics: Arc<BridgeMetrics>,
        aml_key: String,
    ) -> Self {
        let bridge_object_arg = sui_client
            .get_mutable_bridge_object_arg_must_succeed()
            .await;
        Self {
            store,
            sui_client,
            sui_address,
            gas_object_id,
            bridge_object_arg,
            key,
            metrics,
            aml_key,
        }
    }

    async fn resubmit_pending_actions(store: &Arc<BridgeOrchestratorTables>,executor_sender: mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>){
            loop{
                //sleep first to fix simtest:test_onchain_execution_loop
                tokio::time::sleep(Duration::from_secs(60*60)).await;
                let pending_actions = store.get_all_pending_actions().into_values().collect::<Vec<_>>();
                info!("Resubmitting {} pending actions", pending_actions.len());
                for action in pending_actions {
                    submit_to_executor(&executor_sender, action,false)
                        .await
                        .expect("Submit to executor should not fail");
                }
            }
    }

    async fn run_inner(sui_client: &Arc<SuiClient<P>>,mut receiver: mysten_metrics::metered_channel::Receiver<AMLCheckerWrapper>, store: &Arc<BridgeOrchestratorTables>, executor_sender: mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,metrics: &Arc<BridgeMetrics>,sui_address: SuiAddress,gas_object_id: ObjectID,bridge_object_arg: ObjectArg,key: &SuiKeyPair,aml_key: String){
        info!("[DEBUG] AMLChecker run_inner started");

        while let Some(action) = receiver.recv().await {
            let AMLCheckerWrapper(bridge_action, _) = action;
            info!("[DEBUG]  AMLChecker received action: {:?}", bridge_action);
            // Only token transfer action should reach here
            match &bridge_action {
                BridgeAction::SuiToEthBridgeAction(_)
                | BridgeAction::SuiToSolanaBridgeAction(_)
                | BridgeAction::EthToSuiBridgeAction(_)
                | BridgeAction::SuiToEthDefiBridgeAction(_)
                | BridgeAction::EthToSuiDefiBridgeAction(_)
                | BridgeAction::SolanaToSuiBridgeAction(_) => (),
                _ => unreachable!("Non token transfer action should not reach here"),
            };
            match &bridge_action {
                BridgeAction::EthToSuiBridgeAction(action_inner) => {
                    let eth_address = action_inner.eth_bridge_event.eth_address;
                    let skip_aml_check = if action_inner.eth_event_index > u8::MAX as u16 {
                        true
                    } else {
                        false
                    };

                    let is_passed = check_aml_risk_score(
                        action_inner.eth_bridge_event.eth_chain_id,
                        action_inner.eth_bridge_event.token_id,
                        eth_address,
                        aml_key.clone()
                    ).await;

                    info!("aml checker eth address:{:?} is_passed: {:?} tx_hash: {:?}", &eth_address, &is_passed, &action_inner.eth_tx_hash);
                    if skip_aml_check || is_passed {
                        store.insert_pending_actions(&[bridge_action.clone()]).unwrap_or_else(|e| {
                            panic!("Write to DB should not fail: {:?}", e);
                        });
                        submit_to_executor(&executor_sender, bridge_action.clone(),true).await.expect("Submit to executor should not fail");
                        store.remove_pending_aml_checked_actions(&[bridge_action.digest()]).unwrap_or_else(|e| {
                            panic!("Write to DB should not fail: {:?}", e);
                        });
                        sui_client.notify_something_done().await;
                    }else{
                        // only finalized fast path selector will be sent back
                        if action_inner.eth_bridge_event.fast_path_selector == FastPathSelector::Finalized {
                            Self::send_back(bridge_action.clone(), store, key, metrics,sui_client,sui_address,gas_object_id,bridge_object_arg).await;
                        }else{
                            store.remove_pending_aml_checked_actions(&[bridge_action.digest()]).unwrap_or_else(|e| {
                                panic!("remove from DB should not fail: {:?}", e);
                            });
                            info!("fast path selector is not finalized, skipping send back address:{:?} tx_hash:{:?}", &eth_address, &action_inner.eth_tx_hash);
                        }
                    }
                },
                BridgeAction::EthToSuiDefiBridgeAction(_) => {
                    store.insert_pending_actions(&[bridge_action.clone()]).unwrap_or_else(|e| {
                        panic!("Write to DB should not fail: {:?}", e);
                    });
                    submit_to_executor(&executor_sender, bridge_action.clone(),true).await.expect("Submit to executor should not fail");
                    store.remove_pending_aml_checked_actions(&[bridge_action.digest()]).unwrap_or_else(|e| {
                        panic!("Write to DB should not fail: {:?}", e);
                    });
                    sui_client.notify_something_done().await;
                },
                BridgeAction::SolanaToSuiBridgeAction(action_inner) => {
                    let solana_address = action_inner.solana_bridge_event.solana_address;

                    let is_passed = check_aml_risk_score_solana(
                        action_inner.solana_bridge_event.solana_chain_id,
                        action_inner.solana_bridge_event.token_id,
                        solana_address,
                        aml_key.clone()).await;
                    info!("aml checker solana address:{:?} is_passed: {:?} tx_hash: {:?}", &solana_address, &is_passed, &action_inner.solana_tx_signature);
                    if is_passed {
                        store.insert_pending_actions(&[bridge_action.clone()]).unwrap_or_else(|e| {
                            panic!("Write to DB should not fail: {:?}", e);
                        });
                        submit_to_executor(&executor_sender, bridge_action.clone(),true).await.expect("Submit to executor should not fail");
                        store.remove_pending_aml_checked_actions(&[bridge_action.digest()]).unwrap_or_else(|e| {
                            panic!("Write to DB should not fail: {:?}", e);
                        });
                        sui_client.notify_something_done().await;
                    }else{
                        // only finalized fast path selector will be sent back
                        if action_inner.solana_bridge_event.fast_path_selector == FastPathSelector::Finalized {
                            Self::send_back(bridge_action.clone(), store, key, metrics,sui_client,sui_address,gas_object_id,bridge_object_arg).await;
                        }else{
                            store.remove_pending_aml_checked_actions(&[bridge_action.digest()]).unwrap_or_else(|e| {
                                panic!("remove from DB should not fail: {:?}", e);
                            });
                            info!("fast path selector is not finalized, skipping send back address:{:?} tx_hash:{:?}", &solana_address, &action_inner.solana_tx_signature);
                        }
                    }
                },
                _ => {
                    continue;
                }
            }

        }
    }

    async fn send_back(action:BridgeAction,store: &Arc<BridgeOrchestratorTables>, sui_key: &SuiKeyPair, metrics: &Arc<BridgeMetrics>,sui_client: &Arc<SuiClient<P>>,sui_address: SuiAddress,gas_object_id: ObjectID,bridge_object_arg: ObjectArg){
        let (_gas_coin, gas_object_ref) = Self::get_gas_data_assert_ownership(sui_address, gas_object_id, &sui_client).await;
        let rgp = sui_client.get_reference_gas_price_until_success().await;
        let tx_data =match build_token_send_back_transaction(sui_address, &gas_object_ref, action.clone(), bridge_object_arg, rgp){
            Ok(tx_data) => tx_data,
            Err(err) => {
                metrics.err_build_sui_transaction.inc();
                error!(
                        "Manual intervention is required. Failed to build transaction for action {:?}: {:?}",
                        action, err
                    );
                // This should not happen, but in case it does, we do not want to
                // panic, instead we log here for manual intervention.
                return;
            }
        };
        let sig = Signature::new_secure(
            &IntentMessage::new(Intent::sui_transaction(), &tx_data),
            sui_key,
        );
        let signed_tx = Transaction::from_data(tx_data, vec![sig]);
        let tx_digest = *signed_tx.digest();

        // Check twice: If the action is already processed, skip it.
        if Self::handle_already_processed_send_back_maybe(
            &sui_client.clone(), &action, store, &metrics,
        )
            .await
        {
            info!("Action already processed, skipping");
            store.remove_pending_aml_checked_actions(&[action.digest()]).unwrap_or_else(|e| {
                panic!("remove from DB should not fail: {:?}", e);
            });
            return;
        }

        info!(?tx_digest, ?gas_object_ref, "Sending transaction to Sui");
        match sui_client
            .execute_transaction_block_with_effects(signed_tx)
            .await
        {
            Ok(resp) => {
                info!("Sui transaction executed successfully resp:{:?}",resp);
                Self::handle_execution_effects(tx_digest, resp, store, &action, &metrics).await
            }

            // If the transaction did not go through, retry up to a certain times.
            Err(_err) => {
                info!("Sui transaction failed at signing err:{:?}",_err);
                //todo fix errors
                // error!(
                //     ?action_key,
                //     ?tx_digest,
                //     "Sui transaction failed at signing: {err:?}"
                // );
                // metrics.err_sui_transaction_submission.inc();
                // let metrics_clone = metrics.clone();
                // // Do this in a separate task so we won't deadlock here
                // let sender_clone = execution_queue_sender.clone();
                // spawn_logged_monitored_task!(async move {
                //     // If it fails for too many times, log and ask for manual intervention.
                //     if attempt_times >= MAX_EXECUTION_ATTEMPTS {
                //         metrics_clone
                //             .err_sui_transaction_submission_too_many_failures
                //             .inc();
                //         error!("Manual intervention is required. Failed to collect execute transaction for bridge action after {MAX_EXECUTION_ATTEMPTS} attempts: {:?}", err);
                //         return;
                //     }
                //     delay(attempt_times).await;
                //     sender_clone
                //         .send(CertifiedBridgeActionExecutionWrapper(
                //             certificate,
                //             attempt_times + 1,
                //         ))
                //         .await
                //         .unwrap_or_else(|e| {
                //             panic!("Sending to execution queue should not fail: {:?}", e);
                //         });
                //     info!("Re-enqueued certificate for execution");
                // }.instrument(tracing::debug_span!("reenqueue_execution_task", action_key=?action_key)));
            }
        }


        store.remove_pending_aml_checked_actions(&[action.digest()]).unwrap_or_else(|e| {
            panic!("Write to DB should not fail: {:?}", e);
        });
    }

    // Checks if the action is already processed on chain.
    // If yes, skip this action and remove it from the pending log.
    // Returns true if the action is already processed.
    async fn handle_already_processed_send_back_maybe(
        sui_client: &Arc<SuiClient<P>>,
        action: &BridgeAction,
        store: &Arc<BridgeOrchestratorTables>,
        metrics: &Arc<BridgeMetrics>,
    ) -> bool {
        let tx_hash = match action {
            BridgeAction::EthToSuiBridgeAction(a) => {
                a.eth_tx_hash.as_bytes().to_vec()
            }
            _ => unreachable!(),
        };
        let status = sui_client
            .get_send_back_onchain_status_until_success(
                tx_hash.clone()
            )
            .await;
        match status {
            BridgeActionStatus::Approved | BridgeActionStatus::Claimed | BridgeActionStatus::Pending => {
                info!(
                    "Action already approved or claimed or pending, removing action from pending logs: {:?}",
                    action
                );
                metrics.action_executor_already_processed_actions.inc();
                store
                    .remove_pending_aml_checked_actions(&[action.digest()])
                    .unwrap_or_else(|e| {
                        panic!("Write to DB should not fail: {:?}", e);
                    });
                true
            }
            // Although theoretically a legit SuiToEthBridgeAction should not have
            // status `NotFound`
            BridgeActionStatus::NotFound => false,
        }
    }

    async fn get_gas_data_assert_ownership(
        sui_address: SuiAddress,
        gas_object_id: ObjectID,
        sui_client: &SuiClient<P>,
    ) -> (GasCoin, ObjectRef) {
        let (gas_coin, gas_obj_ref, owner) = sui_client
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await;

        // TODO: when we add multiple gas support in the future we could discard
        // transferred gas object instead.
        assert_eq!(
            owner,
            Owner::AddressOwner(sui_address),
            "Gas object {:?} is no longer owned by address {}",
            gas_object_id,
            sui_address
        );
        (gas_coin, gas_obj_ref)
    }

    // TODO: do we need a mechanism to periodically read pending actions from DB?
    async fn handle_execution_effects(
        tx_digest: TransactionDigest,
        response: SuiTransactionBlockResponse,
        store: &Arc<BridgeOrchestratorTables>,
        action: &BridgeAction,
        metrics: &Arc<BridgeMetrics>,
    ) {
        let effects = response
            .effects
            .clone()
            .expect("We requested effects but got None.");
        let status = effects.status();
        match status {
            SuiExecutionStatus::Success => {
                let events = response.events.expect("We requested events but got None.");
                // If the transaction is successful, there must be either
                // TokenTransferAlreadyClaimed or TokenTransferClaimed event.
                assert!(events
                            .data
                            .iter()
                            .any(|e| {
                                e.type_.name.as_str() == "TokenSendBackEvent" || e.type_.name.as_str() == "TokenSendBackEventV2"}),
                        "Expected TokenSendBackEvent or TokenSendBackEventV2 event but got: {:?}",
                        events,
                );
                info!(?tx_digest, "send back transaction executed successfully");
                store
                    .remove_pending_aml_checked_actions(&[action.digest()])
                    .unwrap_or_else(|e| {
                        panic!("Write to DB should not fail: {:?}", e);
                    })
            }
            SuiExecutionStatus::Failure { error } => {
                // In practice the transaction could fail because of running out of gas, but really
                // should not be due to other reasons.
                // This means manual intervention is needed. So we do not push them back to
                // the execution queue because retries are mostly likely going to fail anyway.
                // After human examination, the node should be restarted and fetch them from WAL.

                metrics.err_sui_transaction_execution.inc();
                error!(?tx_digest, "Manual intervention is needed. Sui transaction executed and failed with error: {error:?}");
            }
        }
    }
}



