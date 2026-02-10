// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tracing::info;
use sui_types::Identifier;

use sui_types::event::EventID;
use typed_store::rocks::{DBMap, MetricConf};
use typed_store::DBMapUtils;
use typed_store::Map;

use crate::error::{BridgeError, BridgeResult};
use crate::fast_path::FastPathSelector;
use crate::types::{BridgeAction, BridgeActionDigest};

//(address,chain_id,fast_path_enabled)
//chain_id is the chain id of the evm chain,not the bridge chain id
pub type EthSyncerCursorsKey = (ethers::types::Address, u64,FastPathSelector);

#[derive(DBMapUtils)]
pub struct BridgeOrchestratorTables {
    /// pending BridgeActions that orchestrator received but not yet executed
    pub(crate) pending_actions: DBMap<BridgeActionDigest, BridgeAction>,
    /// module identifier to the last processed EventID
    pub(crate) sui_syncer_cursors: DBMap<Identifier, EventID>,
    /// contract address to the last processed block
    pub(crate) eth_syncer_cursors: DBMap<EthSyncerCursorsKey, u64>,
    /// Solana contract address to the last processed signature
    pub(crate) solana_signature_cursors: DBMap<String, String>,
    /// Solana contract address to the last processed slot (for resilience if signature is pruned)
    pub(crate) solana_slot_cursors: DBMap<String, u64>,
    /// pending actions that are waiting for aml check
    pub(crate) pending_aml_checked_actions: DBMap<BridgeActionDigest, BridgeAction>,
}

impl BridgeOrchestratorTables {
    pub fn new(path: &Path) -> Arc<Self> {
        Arc::new(Self::open_tables_read_write(
            path.to_path_buf(),
            MetricConf::new("bridge"),
            None,
            None,
        ))
    }

    pub(crate) fn insert_pending_actions(&self, actions: &[BridgeAction]) -> BridgeResult<()> {
        for action in actions {
            match action {
                BridgeAction::EthSendBackBridgeAction(a) => {
                    info!("[DEBUG] insert_pending_actions EthSendBackBridgeAction: {:#?}", a);
                }
                BridgeAction::EthToSuiBridgeAction(a) => {
                    info!("[DEBUG] insert_pending_actions EthToSuiBridgeAction: {:#?}", a);
                }
                _ => (),
            };
        }

        let mut batch = self.pending_actions.batch();
        batch
            .insert_batch(
                &self.pending_actions,
                actions.iter().map(|a| (a.digest(), a)),
            )
            .map_err(|e| {
                BridgeError::StorageError(format!("Couldn't insert into pending_actions: {:?}", e))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub(crate) fn remove_pending_actions(
        &self,
        actions: &[BridgeActionDigest],
    ) -> BridgeResult<()> {
        let mut batch = self.pending_actions.batch();
        batch
            .delete_batch(&self.pending_actions, actions)
            .map_err(|e| {
                BridgeError::StorageError(format!("Couldn't delete from pending_actions: {:?}", e))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub(crate) fn insert_pending_aml_checked_actions(
        &self,
        actions: &[BridgeAction],
    ) -> BridgeResult<()> {
        let mut batch = self.pending_aml_checked_actions.batch();
        batch
            .insert_batch(
                &self.pending_aml_checked_actions,
                actions.iter().map(|a| (a.digest(), a)),
            )
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Couldn't insert into pending_aml_checked_actions: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub(crate) fn remove_pending_aml_checked_actions(
        &self,
        actions: &[BridgeActionDigest],
    ) -> BridgeResult<()> {
        let mut batch = self.pending_aml_checked_actions.batch();
        batch
            .delete_batch(&self.pending_aml_checked_actions, actions)
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Couldn't delete from pending_aml_checked_actions: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }


    pub(crate) fn update_sui_event_cursor(
        &self,
        module: Identifier,
        cursor: EventID,
    ) -> BridgeResult<()> {
        let mut batch = self.sui_syncer_cursors.batch();

        batch
            .insert_batch(&self.sui_syncer_cursors, [(module, cursor)])
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Coudln't insert into sui_syncer_cursors: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub(crate) fn update_eth_event_cursor(
        &self,
        key: EthSyncerCursorsKey,
        cursor: u64,
    ) -> BridgeResult<()> {
        // let (_, chain_id) = key.clone();
        // if (chain_id == 11155420) {
        //     info!("[DEBUG]  update_eth_event_cursor: key: {:?}, cursor: {}, current:{:?}", key, cursor, &self.get_eth_event_cursors(&[key.clone()]));
        // }

        let mut batch = self.eth_syncer_cursors.batch();

        batch
            .insert_batch(&self.eth_syncer_cursors, [(key, cursor)])
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Coudln't insert into eth_syncer_cursors: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub fn get_all_pending_actions(&self) -> HashMap<BridgeActionDigest, BridgeAction> {
        self.pending_actions
            .safe_iter()
            .collect::<Result<HashMap<_, _>, _>>()
            .expect("failed to get all pending actions");

        self.pending_actions.safe_iter().filter_map(
            |result| {
                match result {
                    Ok((digest, action)) => {
                        // 在这里使用 digest 和 action
                        if let BridgeAction::ExternalDepositStartBridgeAction(ref external_action) = action {
                            let tx_hash = &external_action.sui_bridge_event.tx_hash;
                            if tx_hash == "abb26e297b0d347834a99b9fdf43d40c828532740b4c643b607192a83dd86340#result-2" {
                                info!("filter pending actions by tx_hash for hard code fix bug, sui_hash {} tx_hash: {}",
                        &external_action.sui_tx_digest, tx_hash);
                                return None; // 过滤掉这个元素
                            }
                        }
                        info!("[DEBUG] pending_actions: {:#?}", action);
                        Some((digest, action)) // 保留这个元素
                    }
                    Err(e) => {
                        // 处理错误情况
                        eprintln!("Error reading from storage: {:?}", e);
                        None
                    }
                }
            }
        ).collect()
    }

    pub fn get_all_pending_actions_4_aml(&self) -> HashMap<BridgeActionDigest, BridgeAction> {
        self.pending_aml_checked_actions.safe_iter().filter_map(
            |result| {
                match result {
                    Ok((digest, action)) => {
                        info!("[DEBUG]  get_all_pending_actions_4_aml: {:#?}", action);
                        Some((digest, action))
                    }
                    Err(e) => {
                        eprintln!("Error reading from storage: {:?}", e);
                        None
                    }
                }
            }
        ).collect()
    }

    pub fn get_sui_event_cursors(
        &self,
        identifiers: &[Identifier],
    ) -> BridgeResult<Vec<Option<EventID>>> {
        self.sui_syncer_cursors.multi_get(identifiers).map_err(|e| {
            BridgeError::StorageError(format!("Couldn't get sui_syncer_cursors: {:?}", e))
        })
    }

    pub fn get_eth_event_cursors(
        &self,
        keys: &[EthSyncerCursorsKey],
    ) -> BridgeResult<Vec<Option<u64>>> {
        self.eth_syncer_cursors
            .multi_get(keys)
            .map_err(|e| {
                BridgeError::StorageError(format!("Couldn't get sui_syncer_cursors: {:?}", e))
            })
    }

    pub(crate) fn update_solana_signature_cursor(
        &self,
        key: String,
        sig: String,
    ) -> BridgeResult<()> {
        let mut batch = self.solana_signature_cursors.batch();
        batch
            .insert_batch(&self.solana_signature_cursors, [(key, sig)])
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Coudln't insert into solana_signature_cursors: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub fn get_solana_signature_cursors(
        &self,
        keys: &[String],
    ) -> BridgeResult<Vec<Option<String>>> {
        self.solana_signature_cursors.multi_get(keys).map_err(|e| {
            BridgeError::StorageError(format!("Couldn't get solana_signature_cursors: {:?}", e))
        })
    }

    pub(crate) fn update_solana_slot_cursor(
        &self,
        key: String,
        slot: u64,
    ) -> BridgeResult<()> {
        let mut batch = self.solana_slot_cursors.batch();
        batch
            .insert_batch(&self.solana_slot_cursors, [(key, slot)])
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Couldn't insert into solana_slot_cursors: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub fn get_solana_slot_cursors(
        &self,
        keys: &[String],
    ) -> BridgeResult<Vec<Option<u64>>> {
        self.solana_slot_cursors.multi_get(keys).map_err(|e| {
            BridgeError::StorageError(format!("Couldn't get solana_slot_cursors: {:?}", e))
        })
    }

    /// Atomically insert pending AML actions and update Solana cursors.
    /// This ensures that either all operations succeed or none do,
    /// preventing inconsistent state during crash recovery.
    pub(crate) fn insert_pending_aml_actions_and_update_solana_cursors(
        &self,
        actions: &[BridgeAction],
        address: String,
        signature: Option<String>,
        slot: Option<u64>,
    ) -> BridgeResult<()> {
        let mut batch = self.pending_aml_checked_actions.batch();

        // Insert pending AML actions
        if !actions.is_empty() {
            batch
                .insert_batch(
                    &self.pending_aml_checked_actions,
                    actions.iter().map(|a| (a.digest(), a)),
                )
                .map_err(|e| {
                    BridgeError::StorageError(format!(
                        "Couldn't insert into pending_aml_checked_actions: {:?}",
                        e
                    ))
                })?;
        }

        // Update signature cursor
        if let Some(sig) = signature {
            batch
                .insert_batch(&self.solana_signature_cursors, [(address.clone(), sig)])
                .map_err(|e| {
                    BridgeError::StorageError(format!(
                        "Couldn't insert into solana_signature_cursors: {:?}",
                        e
                    ))
                })?;
        }

        // Update slot cursor
        if let Some(s) = slot {
            batch
                .insert_batch(&self.solana_slot_cursors, [(address, s)])
                .map_err(|e| {
                    BridgeError::StorageError(format!(
                        "Couldn't insert into solana_slot_cursors: {:?}",
                        e
                    ))
                })?;
        }

        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    /// Atomically update Solana cursors only (for empty event batches).
    pub(crate) fn update_solana_cursors(
        &self,
        address: String,
        signature: Option<String>,
        slot: Option<u64>,
    ) -> BridgeResult<()> {
        let mut batch = self.solana_signature_cursors.batch();

        // Update signature cursor
        if let Some(sig) = signature {
            batch
                .insert_batch(&self.solana_signature_cursors, [(address.clone(), sig)])
                .map_err(|e| {
                    BridgeError::StorageError(format!(
                        "Couldn't insert into solana_signature_cursors: {:?}",
                        e
                    ))
                })?;
        }

        // Update slot cursor
        if let Some(s) = slot {
            batch
                .insert_batch(&self.solana_slot_cursors, [(address, s)])
                .map_err(|e| {
                    BridgeError::StorageError(format!(
                        "Couldn't insert into solana_slot_cursors: {:?}",
                        e
                    ))
                })?;
        }

        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use sui_types::bridge::BridgeChainId;
    use sui_types::digests::TransactionDigest;

    use crate::test_utils::get_test_sui_to_eth_bridge_action;

    use super::*;

    // async: existing runtime is required with typed-store
    #[tokio::test]
    async fn test_bridge_storage_basic() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let action1 = get_test_sui_to_eth_bridge_action(
            None,
            Some(0),
            Some(99),
            Some(10000),
            None,
            None,
            None,
        );

        let action2 = get_test_sui_to_eth_bridge_action(
            None,
            Some(2),
            Some(100),
            Some(10000),
            None,
            None,
            None,
        );

        // in the beginning it's empty
        let actions = store.get_all_pending_actions();
        assert!(actions.is_empty());

        // remove non existing entry is ok
        store.remove_pending_actions(&[action1.digest()]).unwrap();

        store
            .insert_pending_actions(&vec![action1.clone(), action2.clone()])
            .unwrap();

        let actions = store.get_all_pending_actions();
        assert_eq!(
            actions,
            HashMap::from_iter(vec![
                (action1.digest(), action1.clone()),
                (action2.digest(), action2.clone())
            ])
        );

        // insert an existing action is ok
        store.insert_pending_actions(&[action1.clone()]).unwrap();
        let actions = store.get_all_pending_actions();
        assert_eq!(
            actions,
            HashMap::from_iter(vec![
                (action1.digest(), action1.clone()),
                (action2.digest(), action2.clone())
            ])
        );

        // remove action 2
        store.remove_pending_actions(&[action2.digest()]).unwrap();
        let actions = store.get_all_pending_actions();
        assert_eq!(
            actions,
            HashMap::from_iter(vec![(action1.digest(), action1.clone())])
        );

        // remove action 1
        store.remove_pending_actions(&[action1.digest()]).unwrap();
        let actions = store.get_all_pending_actions();
        assert!(actions.is_empty());

        // update eth event cursor
        let eth_contract_address = ethers::types::Address::random();
        let eth_block_num = 199999u64;
        assert!(store
            .get_eth_event_cursors(&[(eth_contract_address, BridgeChainId::EthCustom as u64,FastPathSelector::Finalized)])
            .unwrap()[0]
            .is_none());
        store
            .update_eth_event_cursor((eth_contract_address, BridgeChainId::EthCustom as u64,FastPathSelector::Finalized), eth_block_num)
            .unwrap();
        assert_eq!(
            store
                .get_eth_event_cursors(&[(eth_contract_address, BridgeChainId::EthCustom as u64,FastPathSelector::Finalized)])
                .unwrap()[0]
                .unwrap(),
            eth_block_num
        );

        // update sui event cursor
        let sui_module = Identifier::from_str("test").unwrap();
        let sui_cursor = EventID {
            tx_digest: TransactionDigest::random(),
            event_seq: 1,
        };
        assert!(store.get_sui_event_cursors(&[sui_module.clone()]).unwrap()[0].is_none());
        store
            .update_sui_event_cursor(sui_module.clone(), sui_cursor)
            .unwrap();
        assert_eq!(
            store.get_sui_event_cursors(&[sui_module.clone()]).unwrap()[0].unwrap(),
            sui_cursor
        );
    }

    #[tokio::test]
    async fn test_solana_atomic_operations() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let action1 = get_test_sui_to_eth_bridge_action(
            None,
            Some(0),
            Some(99),
            Some(10000),
            None,
            None,
            None,
        );

        let action2 = get_test_sui_to_eth_bridge_action(
            None,
            Some(2),
            Some(100),
            Some(10000),
            None,
            None,
            None,
        );

        let address = "SolanaAddress123".to_string();
        let signature = "sig_abc123".to_string();
        let slot = 12345u64;

        // Test atomic insert with actions and cursors
        store
            .insert_pending_aml_actions_and_update_solana_cursors(
                &[action1.clone(), action2.clone()],
                address.clone(),
                Some(signature.clone()),
                Some(slot),
            )
            .unwrap();

        // Verify actions were inserted
        let pending_actions = store.get_all_pending_actions_4_aml();
        assert_eq!(pending_actions.len(), 2);
        assert!(pending_actions.contains_key(&action1.digest()));
        assert!(pending_actions.contains_key(&action2.digest()));

        // Verify cursors were updated
        assert_eq!(
            store.get_solana_signature_cursors(&[address.clone()]).unwrap()[0],
            Some(signature.clone())
        );
        assert_eq!(
            store.get_solana_slot_cursors(&[address.clone()]).unwrap()[0],
            Some(slot)
        );

        // Test update_solana_cursors (without actions)
        let new_signature = "sig_def456".to_string();
        let new_slot = 12346u64;
        store
            .update_solana_cursors(
                address.clone(),
                Some(new_signature.clone()),
                Some(new_slot),
            )
            .unwrap();

        // Verify cursors were updated
        assert_eq!(
            store.get_solana_signature_cursors(&[address.clone()]).unwrap()[0],
            Some(new_signature)
        );
        assert_eq!(
            store.get_solana_slot_cursors(&[address.clone()]).unwrap()[0],
            Some(new_slot)
        );

        // Test with empty actions (should still update cursors)
        let final_signature = "sig_ghi789".to_string();
        let final_slot = 12347u64;
        store
            .insert_pending_aml_actions_and_update_solana_cursors(
                &[],
                address.clone(),
                Some(final_signature.clone()),
                Some(final_slot),
            )
            .unwrap();

        // Verify cursors were updated even with empty actions
        assert_eq!(
            store.get_solana_signature_cursors(&[address.clone()]).unwrap()[0],
            Some(final_signature)
        );
        assert_eq!(
            store.get_solana_slot_cursors(&[address.clone()]).unwrap()[0],
            Some(final_slot)
        );

        // Verify actions count unchanged
        let pending_actions = store.get_all_pending_actions_4_aml();
        assert_eq!(pending_actions.len(), 2);
    }

    #[tokio::test]
    async fn test_solana_cursors_partial_update() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let address = "SolanaAddress456".to_string();

        // Test with only signature (no slot)
        store
            .update_solana_cursors(
                address.clone(),
                Some("sig_only".to_string()),
                None,
            )
            .unwrap();

        assert_eq!(
            store.get_solana_signature_cursors(&[address.clone()]).unwrap()[0],
            Some("sig_only".to_string())
        );
        assert_eq!(
            store.get_solana_slot_cursors(&[address.clone()]).unwrap()[0],
            None
        );

        // Test with only slot (no signature)
        let address2 = "SolanaAddress789".to_string();
        store
            .update_solana_cursors(
                address2.clone(),
                None,
                Some(99999),
            )
            .unwrap();

        assert_eq!(
            store.get_solana_signature_cursors(&[address2.clone()]).unwrap()[0],
            None
        );
        assert_eq!(
            store.get_solana_slot_cursors(&[address2.clone()]).unwrap()[0],
            Some(99999)
        );
    }
}
