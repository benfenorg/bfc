// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

pub use checked::*;

#[sui_macros::with_checked_arithmetic]
mod checked {

    use move_binary_format::CompiledModule;

    use move_vm_runtime::move_vm::MoveVM;
    use std::{
        collections::{HashSet},
        sync::Arc,
    };
    use crate::{temporary_store::TemporaryStore};
    use sui_types::gas::{calculate_reward_rate, calculate_add};
    use sui_types::gas_coin::GAS;

    use sui_types::balance::{
        BALANCE_CREATE_REWARDS_FUNCTION_NAME, BALANCE_DESTROY_REBATES_FUNCTION_NAME,
        BALANCE_MODULE_NAME,
    };
    use sui_types::execution_mode::{self, ExecutionMode};
    use sui_types::messages_checkpoint::CheckpointTimestamp;
    use sui_types::metrics::LimitsMetrics;
    use sui_types::object::OBJECT_START_VERSION;
    use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
    use sui_types::randomness_state::{
        RANDOMNESS_MODULE_NAME, RANDOMNESS_STATE_CREATE_FUNCTION_NAME,
        RANDOMNESS_STATE_UPDATE_FUNCTION_NAME,
    };
    use sui_types::SUI_RANDOMNESS_STATE_OBJECT_ID;
    use tracing::{info, instrument, trace, warn};

    use crate::programmable_transactions;
    use crate::type_layout_resolver::TypeLayoutResolver;
    //use move_binary_format::access::ModuleAccess;
    use crate::{gas_charger::GasCharger};
    use sui_protocol_config::{check_limit_by_meter, LimitThresholdCrossed, ProtocolConfig};
    use sui_types::authenticator_state::{
        AUTHENTICATOR_STATE_CREATE_FUNCTION_NAME, AUTHENTICATOR_STATE_EXPIRE_JWKS_FUNCTION_NAME,
        AUTHENTICATOR_STATE_MODULE_NAME, AUTHENTICATOR_STATE_UPDATE_FUNCTION_NAME,
    };
    use sui_types::clock::{CLOCK_MODULE_NAME, CONSENSUS_COMMIT_PROLOGUE_FUNCTION_NAME};
    use sui_types::committee::EpochId;
    use sui_types::deny_list::{DENY_LIST_CREATE_FUNC, DENY_LIST_MODULE};
    use sui_types::effects::TransactionEffects;
    use sui_types::error::{ExecutionError, ExecutionErrorKind};
    use sui_types::execution::is_certificate_denied;
    use sui_types::execution_config_utils::to_binary_config;
    use sui_types::execution_status::ExecutionStatus;
    use sui_types::gas::GasCostSummary;
    use sui_types::gas::SuiGasStatus;
    use sui_types::inner_temporary_store::InnerTemporaryStore;
    use sui_types::storage::BackingStore;
    #[cfg(msim)]
    use sui_types::sui_system_state::advance_epoch_result_injection::maybe_modify_result;
    use sui_types::sui_system_state::{AdvanceEpochParams, ADVANCE_EPOCH_SAFE_MODE_FUNCTION_NAME, ChangeObcRoundParams};
    use sui_types::transaction::{
        Argument, AuthenticatorStateExpire, AuthenticatorStateUpdate, CallArg, ChangeEpoch,
        Command, EndOfEpochTransactionKind, GenesisTransaction, ObjectArg, ProgrammableTransaction,
        TransactionKind,
    };
    use sui_types::transaction::{CheckedInputObjects, RandomnessStateUpdate};
    use sui_types::{
        base_types::{ObjectRef, SuiAddress, TransactionDigest, TxContext},
        object::{Object, ObjectInner},
        sui_system_state::{ADVANCE_EPOCH_FUNCTION_NAME, SUI_SYSTEM_MODULE_NAME},
        bfc_system_state::{BFC_SYSTEM_MODULE_NAME, BFC_ROUND_FUNCTION_NAME},
        SUI_AUTHENTICATOR_STATE_OBJECT_ID, SUI_FRAMEWORK_ADDRESS,
        SUI_SYSTEM_PACKAGE_ID,SUI_FRAMEWORK_PACKAGE_ID
    };

    use sui_types::{BFC_SYSTEM_PACKAGE_ID};
    use sui_types::bfc_system_state::{DEPOSIT_TO_TREASURY_FUNCTION_NAME, STABLE_COIN_TO_BFC_FUNCTION_NAME};

    /// If a transaction digest shows up in this list, when executing such transaction,
    /// we will always return `ExecutionError::CertificateDenied` without executing it (but still do
    /// gas smashing). Because this list is not gated by protocol version, there are a few important
    /// criteria for adding a digest to this list:
    /// 1. The certificate must be causing all validators to either panic or hang forever deterministically.
    /// 2. If we ever ship a fix to make it no longer panic or hang when executing such transaction,
    /// we must make sure the transaction is already in this list. Otherwise nodes running the newer version
    /// without these transactions in the list will generate forked result.
    /// Below is a scenario of when we need to use this list:
    /// 1. We detect that a specific transaction is causing all validators to either panic or hang forever deterministically.
    /// 2. We push a CertificateDenyConfig to deny such transaction to all validators asap.
    /// 3. To make sure that all fullnodes are able to sync to the latest version, we need to add the transaction digest
    /// to this list as well asap, and ship this binary to all fullnodes, so that they can sync past this transaction.
    /// 4. We then can start fixing the issue, and ship the fix to all nodes.
    /// 5. Unfortunately, we can't remove the transaction digest from this list, because if we do so, any future
    /// node that sync from genesis will fork on this transaction. We may be able to remove it once
    /// we have stable snapshots and the binary has a minimum supported protocol version past the epoch.
    // pub fn get_denied_certificates() -> &'static HashSet<TransactionDigest> {
    //     static DENIED_CERTIFICATES: Lazy<HashSet<TransactionDigest>> =
    //         Lazy::new(|| HashSet::from([]));
    //     Lazy::force(&DENIED_CERTIFICATES)
    // }
    //
    // fn is_certificate_denied(
    //     transaction_digest: &TransactionDigest,
    //     certificate_deny_set: &HashSet<TransactionDigest>,
    // ) -> bool {
    //     certificate_deny_set.contains(transaction_digest)
    //         || get_denied_certificates().contains(transaction_digest)
    // }

    #[instrument(name = "tx_execute_to_effects", level = "debug", skip_all)]
    pub fn execute_transaction_to_effects<Mode: ExecutionMode>(
        store: &dyn BackingStore,
        input_objects: CheckedInputObjects,
        gas_coins: Vec<ObjectRef>,
        gas_status: SuiGasStatus,
        transaction_kind: TransactionKind,
        transaction_signer: SuiAddress,
        transaction_digest: TransactionDigest,
        move_vm: &Arc<MoveVM>,
        epoch_id: &EpochId,
        epoch_timestamp_ms: u64,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        enable_expensive_checks: bool,
        certificate_deny_set: &HashSet<TransactionDigest>,
    ) -> (
        InnerTemporaryStore,
        SuiGasStatus,
        TransactionEffects,
        Result<Mode::ExecutionResults, ExecutionError>,
    ) {
        let input_objects = input_objects.into_inner();
        let mutable_inputs = if enable_expensive_checks {
            input_objects.mutable_inputs().keys().copied().collect()
        } else {
            HashSet::new()
        };
        let shared_object_refs = input_objects.filter_shared_objects();
        let receiving_objects = transaction_kind.receiving_objects();
        let mut transaction_dependencies = input_objects.transaction_dependencies();
        let contains_deleted_input = input_objects.contains_deleted_objects();

        let mut temporary_store = TemporaryStore::new(
            store,
            input_objects,
            receiving_objects,
            transaction_digest,
            protocol_config,
        );

        let mut gas_charger =
            GasCharger::new(transaction_digest, gas_coins, gas_status, protocol_config);

        let mut tx_ctx = TxContext::new_from_components(
            &transaction_signer,
            &transaction_digest,
            epoch_id,
            epoch_timestamp_ms,
        );

        //let is_epoch_change: bool = matches!(transaction_kind, TransactionKind::ChangeEpoch(_));
        let is_epoch_change = transaction_kind.is_end_of_epoch_tx();

        let deny_cert = is_certificate_denied(&transaction_digest, certificate_deny_set);
        let (gas_cost_summary, execution_result) = execute_transaction::<Mode>(
            &mut temporary_store,
            transaction_kind,
            &mut gas_charger,
            &mut tx_ctx,
            move_vm,
            protocol_config,
            metrics,
            enable_expensive_checks,
            deny_cert,
            contains_deleted_input,
        );

        let status = if let Err(error) = &execution_result {
            // Elaborate errors in logs if they are unexpected or their status is terse.
            use ExecutionErrorKind as K;
            match error.kind() {
                K::InvariantViolation | K::VMInvariantViolation => {
                    #[skip_checked_arithmetic]
                    tracing::error!(
                        kind = ?error.kind(),
                        tx_digest = ?transaction_digest,
                        "INVARIANT VIOLATION! Source: {:?}",
                        error.source(),
                    );
                }

                K::SuiMoveVerificationError | K::VMVerificationOrDeserializationError => {
                    #[skip_checked_arithmetic]
                    tracing::debug!(
                        kind = ?error.kind(),
                        tx_digest = ?transaction_digest,
                        "Verification Error. Source: {:?}",
                        error.source(),
                    );
                }

                K::PublishUpgradeMissingDependency | K::PublishUpgradeDependencyDowngrade => {
                    #[skip_checked_arithmetic]
                    tracing::debug!(
                        kind = ?error.kind(),
                        tx_digest = ?transaction_digest,
                        "Publish/Upgrade Error. Source: {:?}",
                        error.source(),
                    )
                }

                _ => (),
            };

            let (status, command) = error.to_execution_status();
            ExecutionStatus::new_failure(status, command)
        } else {
            ExecutionStatus::Success
        };

        #[skip_checked_arithmetic]
        trace!(
            tx_digest = ?transaction_digest,
            computation_gas_cost = gas_cost_summary.computation_cost,
            storage_gas_cost = gas_cost_summary.storage_cost,
            storage_gas_rebate = gas_cost_summary.storage_rebate,
            "Finished execution of transaction with status {:?}",
            status
        );

        // Genesis writes a special digest to indicate that an object was created during
        // genesis and not written by any normal transaction - remove that from the
        // dependencies
        transaction_dependencies.remove(&TransactionDigest::genesis_marker());

        if enable_expensive_checks && !Mode::allow_arbitrary_function_calls() {
            temporary_store
                .check_ownership_invariants(
                    &transaction_signer,
                    &mut gas_charger,
                    &mutable_inputs,
                    is_epoch_change,
                )
                .unwrap()
        } // else, in dev inspect mode and anything goes--don't check

        let (inner, effects) = temporary_store.into_effects(
            shared_object_refs,
            &transaction_digest,
            transaction_dependencies,
            gas_cost_summary,
            status,
            &mut gas_charger,
            *epoch_id,
        );

        (
            inner,
            gas_charger.into_gas_status(),
            effects,
            execution_result,
        )
    }

    pub fn execute_genesis_state_update(
        store: &dyn BackingStore,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        move_vm: &Arc<MoveVM>,
        tx_context: &mut TxContext,
        input_objects: CheckedInputObjects,
        pt: ProgrammableTransaction,
    ) -> Result<InnerTemporaryStore, ExecutionError> {
        info!("Executing genesis state update in latest version...");
        let input_objects = input_objects.into_inner();
        let mut temporary_store = TemporaryStore::new(
            store,
            input_objects,
            vec![],
            tx_context.digest(),
            protocol_config,
        );
        let mut gas_charger = GasCharger::new_unmetered(tx_context.digest());
        programmable_transactions::execution::execute::<execution_mode::Genesis>(
            protocol_config,
            metrics,
            move_vm,
            &mut temporary_store,
            tx_context,
            &mut gas_charger,
            pt,
        )?;
        temporary_store.update_object_version_and_prev_tx();
        Ok(temporary_store.into_inner())
    }

    #[instrument(name = "tx_execute", level = "debug", skip_all)]
    fn execute_transaction<Mode: ExecutionMode>(
        temporary_store: &mut TemporaryStore<'_>,
        transaction_kind: TransactionKind,
        gas_charger: &mut GasCharger,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        enable_expensive_checks: bool,
        deny_cert: bool,
        contains_deleted_input: bool,
    ) -> (
        GasCostSummary,
        Result<Mode::ExecutionResults, ExecutionError>,
    ) {
        gas_charger.smash_gas(temporary_store);

        // At this point no charges have been applied yet
        debug_assert!(
            gas_charger.no_charges(),
            "No gas charges must be applied yet"
        );

        let is_genesis_tx = matches!(transaction_kind, TransactionKind::Genesis(_));
        let advance_epoch_gas_summary = transaction_kind.get_advance_epoch_tx_gas_summary();

        // We must charge object read here during transaction execution, because if this fails
        // we must still ensure an effect is committed and all objects versions incremented
        let result = gas_charger.charge_input_objects(temporary_store);
        let mut result = result.and_then(|()| {
            let mut execution_result = if deny_cert {
                Err(ExecutionError::new(
                    ExecutionErrorKind::CertificateDenied,
                    None,
                ))
            } else if contains_deleted_input {
                Err(ExecutionError::new(
                    ExecutionErrorKind::InputObjectDeleted,
                    None,
                ))
            } else {
                execution_loop::<Mode>(
                    temporary_store,
                    transaction_kind,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics.clone(),
                )
            };

            let meter_check = check_meter_limit(
                temporary_store,
                gas_charger,
                protocol_config,
                metrics.clone(),
            );
            if let Err(e) = meter_check {
                execution_result = Err(e);
            }

            if execution_result.is_ok() {
                let gas_check = check_written_objects_limit::<Mode>(
                    temporary_store,
                    gas_charger,
                    protocol_config,
                    metrics,
                );
                if let Err(e) = gas_check {
                    execution_result = Err(e);
                }
            }

            execution_result
        });

        let cost_summary = gas_charger.charge_gas(temporary_store, &mut result);
        // For advance epoch transaction, we need to provide epoch rewards and rebates as extra
        // information provided to check_sui_conserved, because we mint rewards, and burn
        // the rebates. We also need to pass in the unmetered_storage_rebate because storage
        // rebate is not reflected in the storage_rebate of gas summary. This is a bit confusing.
        // We could probably clean up the code a bit.
        // Put all the storage rebate accumulated in the system transaction
        // to the 0x5 object so that it's not lost.
        temporary_store.conserve_unmetered_storage_rebate(gas_charger.unmetered_storage_rebate());

        if let Err(e) = run_conservation_checks::<Mode>(
            temporary_store,
            gas_charger,
            tx_ctx,
            move_vm,
            protocol_config.simple_conservation_checks(),
            enable_expensive_checks,
            &cost_summary,
            is_genesis_tx,
            advance_epoch_gas_summary,
        ) {
            result = Err(e);
        }

        (cost_summary, result)
    }

    #[instrument(name = "run_conservation_checks", level = "debug", skip_all)]
    fn run_conservation_checks<Mode: ExecutionMode>(
        temporary_store: &mut TemporaryStore<'_>,
        gas_charger: &mut GasCharger,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        simple_conservation_checks: bool,
        enable_expensive_checks: bool,
        cost_summary: &GasCostSummary,
        is_genesis_tx: bool,
        advance_epoch_gas_summary: Option<(u64, u64)>,
    ) -> Result<(), ExecutionError> {
        let mut result: std::result::Result<(), sui_types::error::ExecutionError> = Ok(());
        if !is_genesis_tx && !Mode::skip_conservation_checks() {
            // ensure that this transaction did not create or destroy SUI, try to recover if the check fails
            let conservation_result = {
                temporary_store
                    .check_sui_conserved(simple_conservation_checks, cost_summary)
                    .and_then(|()| {
                        if enable_expensive_checks {
                            // ensure that this transaction did not create or destroy SUI, try to recover if the check fails
                            let mut layout_resolver =
                                TypeLayoutResolver::new(move_vm, Box::new(&*temporary_store));
                            temporary_store.check_sui_conserved_expensive(
                                cost_summary,
                                advance_epoch_gas_summary,
                                &mut layout_resolver,
                            )
                        } else {
                            Ok(())
                        }
                    })
            };
            if let Err(conservation_err) = conservation_result {
                // conservation violated. try to avoid panic by dumping all writes, charging for gas, re-checking
                // conservation, and surfacing an aborted transaction with an invariant violation if all of that works
                result = Err(conservation_err);
                gas_charger.reset(temporary_store);
                gas_charger.charge_gas(temporary_store, &mut result);
                // check conservation once more more
                if let Err(recovery_err) = {
                    temporary_store
                        .check_sui_conserved(simple_conservation_checks, cost_summary)
                        .and_then(|()| {
                            if enable_expensive_checks {
                                // ensure that this transaction did not create or destroy SUI, try to recover if the check fails
                                let mut layout_resolver =
                                    TypeLayoutResolver::new(move_vm, Box::new(&*temporary_store));
                                temporary_store.check_sui_conserved_expensive(
                                    cost_summary,
                                    advance_epoch_gas_summary,
                                    &mut layout_resolver,
                                )
                            } else {
                                Ok(())
                            }
                        })
                } {
                    // if we still fail, it's a problem with gas
                    // charging that happens even in the "aborted" case--no other option but panic.
                    // we will create or destroy SUI otherwise
                    panic!(
                        "SUI conservation fail in tx block {}: {}\nGas status is {}\nTx was ",
                        tx_ctx.digest(),
                        recovery_err,
                        gas_charger.summary()
                    )
                }
            }
        } // else, we're in the genesis transaction which mints the SUI supply, and hence does not satisfy SUI conservation, or
          // we're in the non-production dev inspect mode which allows us to violate conservation

        result
    }

    #[instrument(name = "check_meter_limit", level = "debug", skip_all)]
    fn check_meter_limit(
        temporary_store: &mut TemporaryStore<'_>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let effects_estimated_size = temporary_store.estimate_effects_size_upperbound();

        // Check if a limit threshold was crossed.
        // For metered transactions, there is not soft limit.
        // For system transactions, we allow a soft limit with alerting, and a hard limit where we terminate
        match check_limit_by_meter!(
            !gas_charger.is_unmetered(),
            effects_estimated_size,
            protocol_config.max_serialized_tx_effects_size_bytes(),
            protocol_config.max_serialized_tx_effects_size_bytes_system_tx(),
            metrics.excessive_estimated_effects_size
        ) {
            LimitThresholdCrossed::None => Ok(()),
            LimitThresholdCrossed::Soft(_, limit) => {
                warn!(
                    effects_estimated_size = effects_estimated_size,
                    soft_limit = limit,
                    "Estimated transaction effects size crossed soft limit",
                );
                Ok(())
            }
            LimitThresholdCrossed::Hard(_, lim) => Err(ExecutionError::new_with_source(
                ExecutionErrorKind::EffectsTooLarge {
                    current_size: effects_estimated_size as u64,
                    max_size: lim as u64,
                },
                "Transaction effects are too large",
            )),
        }
    }

    #[instrument(name = "check_written_objects_limit", level = "debug", skip_all)]
    fn check_written_objects_limit<Mode: ExecutionMode>(
        temporary_store: &mut TemporaryStore<'_>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        if let (Some(normal_lim), Some(system_lim)) = (
            protocol_config.max_size_written_objects_as_option(),
            protocol_config.max_size_written_objects_system_tx_as_option(),
        ) {
            let written_objects_size = temporary_store.written_objects_size();

            match check_limit_by_meter!(
                !gas_charger.is_unmetered(),
                written_objects_size,
                normal_lim,
                system_lim,
                metrics.excessive_written_objects_size
            ) {
                LimitThresholdCrossed::None => (),
                LimitThresholdCrossed::Soft(_, limit) => {
                    warn!(
                        written_objects_size = written_objects_size,
                        soft_limit = limit,
                        "Written objects size crossed soft limit",
                    )
                }
                LimitThresholdCrossed::Hard(_, lim) => {
                    return Err(ExecutionError::new_with_source(
                        ExecutionErrorKind::WrittenObjectsTooLarge {
                            current_size: written_objects_size as u64,
                            max_size: lim as u64,
                        },
                        "Written objects size crossed hard limit",
                    ));
                }
            };
        }

        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    fn execution_loop<Mode: ExecutionMode>(
        temporary_store: &mut TemporaryStore<'_>,
        transaction_kind: TransactionKind,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<Mode::ExecutionResults, ExecutionError> {
        let result = match transaction_kind {
            TransactionKind::ChangeEpoch(change_epoch) => {
                let builder = ProgrammableTransactionBuilder::new();
                advance_epoch(
                    builder,
                    change_epoch,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )?;
                Ok(Mode::empty_results())
            }
            TransactionKind::Genesis(GenesisTransaction { objects }) => {
                if tx_ctx.epoch() != 0 {
                    panic!("BUG: Genesis Transactions can only be executed in epoch 0");
                }

                for genesis_object in objects {
                    match genesis_object {
                        sui_types::transaction::GenesisObject::RawObject { data, owner } => {
                            let object = ObjectInner {
                                data,
                                owner,
                                previous_transaction: tx_ctx.digest(),
                                storage_rebate: 0,
                            };
                            temporary_store.create_object(object.into());
                        }
                    }
                }
                Ok(Mode::empty_results())
            }
            TransactionKind::ConsensusCommitPrologue(prologue) => {
                setup_consensus_commit(
                    prologue.commit_timestamp_ms,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )
                .expect("ConsensusCommitPrologue cannot fail");
                Ok(Mode::empty_results())
            }
            TransactionKind::ConsensusCommitPrologueV2(prologue) => {
                setup_consensus_commit(
                    prologue.commit_timestamp_ms,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )
                .expect("ConsensusCommitPrologueV2 cannot fail");
                Ok(Mode::empty_results())
            }
            TransactionKind::ProgrammableTransaction(pt) => {
                programmable_transactions::execution::execute::<Mode>(
                    protocol_config,
                    metrics,
                    move_vm,
                    temporary_store,
                    tx_ctx,
                    gas_charger,
                    pt,
                )
            }
            TransactionKind::EndOfEpochTransaction(txns) => {
                let mut builder = ProgrammableTransactionBuilder::new();
                let len = txns.len();
                for (i, tx) in txns.into_iter().enumerate() {
                    match tx {
                        EndOfEpochTransactionKind::ChangeEpoch(change_epoch) => {
                            assert_eq!(i, len - 1);
                            advance_epoch(
                                builder,
                                change_epoch,
                                temporary_store,
                                tx_ctx,
                                move_vm,
                                gas_charger,
                                protocol_config,
                                metrics,
                            )?;
                            return Ok(Mode::empty_results());
                        }
                        EndOfEpochTransactionKind::AuthenticatorStateCreate => {
                            assert!(protocol_config.enable_jwk_consensus_updates());
                            builder = setup_authenticator_state_create(builder);
                        }
                        EndOfEpochTransactionKind::AuthenticatorStateExpire(expire) => {
                            assert!(protocol_config.enable_jwk_consensus_updates());

                            // TODO: it would be nice if a failure of this function didn't cause
                            // safe mode.
                            builder = setup_authenticator_state_expire(builder, expire);
                        }
                        EndOfEpochTransactionKind::RandomnessStateCreate => {
                            assert!(protocol_config.random_beacon());
                            builder = setup_randomness_state_create(builder);
                        }
                        EndOfEpochTransactionKind::DenyListStateCreate => {
                            assert!(protocol_config.enable_coin_deny_list());
                            builder = setup_coin_deny_list_state_create(builder);
                        }
                    }
                }
                unreachable!("EndOfEpochTransactionKind::ChangeEpoch should be the last transaction in the list")
            }
            TransactionKind::AuthenticatorStateUpdate(auth_state_update) => {
                setup_authenticator_state_update(
                    auth_state_update,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )?;
                Ok(Mode::empty_results())
            }
            TransactionKind::RandomnessStateUpdate(randomness_state_update) => {
                setup_randomness_state_update(
                    randomness_state_update,
                    temporary_store,
                    tx_ctx,
                    move_vm,
                    gas_charger,
                    protocol_config,
                    metrics,
                )?;
                Ok(Mode::empty_results())
            }
        }?;
        temporary_store.check_execution_results_consistency()?;
        Ok(result)
    }

    fn mint_epoch_rewards_in_pt(
        builder: &mut ProgrammableTransactionBuilder,
        params: &AdvanceEpochParams,
    ) -> (Argument, Argument) {
        // Create storage rewards.
        let storage_charge_arg = builder
            .input(CallArg::Pure(
                bcs::to_bytes(&params.storage_charge).unwrap(),
            ))
            .unwrap();
        let storage_rewards = builder.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            BALANCE_MODULE_NAME.to_owned(),
            BALANCE_CREATE_REWARDS_FUNCTION_NAME.to_owned(),
            vec![GAS::type_tag()],
            vec![storage_charge_arg],
        );

        // Create computation rewards.
        let computation_charge_arg = builder
            .input(CallArg::Pure(
                bcs::to_bytes(&params.computation_charge).unwrap(),
            ))
            .unwrap();
        let computation_rewards = builder.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            BALANCE_MODULE_NAME.to_owned(),
            BALANCE_CREATE_REWARDS_FUNCTION_NAME.to_owned(),
            vec![GAS::type_tag()],
            vec![computation_charge_arg],
        );
        (storage_rewards, computation_rewards)
    }

    pub fn construct_advance_epoch_pt(
        obc_params: &ChangeObcRoundParams,
        mut builder: ProgrammableTransactionBuilder,
        params: &AdvanceEpochParams,
        is_safe_mode: bool,
        discard: bool,
    ) -> Result<ProgrammableTransaction, ExecutionError> {
        // obc
        construct_bfc_round_pt(obc_params, &mut builder, is_safe_mode, discard)?;
        // Step 1: Create storage and computation rewards.
        let (storage_rewards, computation_rewards) = mint_epoch_rewards_in_pt(&mut builder, params);
        // Step 2: Advance the epoch.
        let mut arguments = vec![storage_rewards, computation_rewards];
        let call_arg_arguments = vec![
            CallArg::SUI_SYSTEM_MUT,
            CallArg::Pure(bcs::to_bytes(&params.epoch).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.next_protocol_version.as_u64()).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.storage_rebate).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.non_refundable_storage_fee).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.storage_fund_reinvest_rate).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.reward_slashing_rate).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.epoch_start_timestamp_ms).unwrap()),
        ]
        .into_iter()
        .map(|a| builder.input(a))
        .collect::<Result<_, _>>();

        assert_invariant!(
            call_arg_arguments.is_ok(),
            "Unable to generate args for advance_epoch transaction!"
        );

        arguments.append(&mut call_arg_arguments.unwrap());

        info!("Call arguments to advance_epoch transaction: {:?}", params);

        let storage_rebates = builder.programmable_move_call(
            SUI_SYSTEM_PACKAGE_ID,
            SUI_SYSTEM_MODULE_NAME.to_owned(),
            ADVANCE_EPOCH_FUNCTION_NAME.to_owned(),
            vec![],
            arguments,
        );

        // Step 3: Destroy the storage rebates.
        builder.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            BALANCE_MODULE_NAME.to_owned(),
            BALANCE_DESTROY_REBATES_FUNCTION_NAME.to_owned(),
            vec![GAS::type_tag()],
            vec![storage_rebates],
        );
        Ok(builder.finish())
    }

    pub fn construct_advance_epoch_safe_mode_pt(
        params: &AdvanceEpochParams,
        protocol_config: &ProtocolConfig,
    ) -> Result<ProgrammableTransaction, ExecutionError> {
        let mut builder = ProgrammableTransactionBuilder::new();
        // Step 1: Create storage and computation rewards.
        let (storage_rewards, computation_rewards) = mint_epoch_rewards_in_pt(&mut builder, params);

        // Step 2: Advance the epoch.
        let mut arguments = vec![storage_rewards, computation_rewards];

        let mut args = vec![
            CallArg::SUI_SYSTEM_MUT,
            CallArg::Pure(bcs::to_bytes(&params.epoch).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.next_protocol_version.as_u64()).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.storage_rebate).unwrap()),
            CallArg::Pure(bcs::to_bytes(&params.non_refundable_storage_fee).unwrap()),
        ];

        if protocol_config.get_advance_epoch_start_time_in_safe_mode() {
            args.push(CallArg::Pure(
                bcs::to_bytes(&params.epoch_start_timestamp_ms).unwrap(),
            ));
        }

        let call_arg_arguments = args
            .into_iter()
            .map(|a| builder.input(a))
            .collect::<Result<_, _>>();

        assert_invariant!(
            call_arg_arguments.is_ok(),
            "Unable to generate args for advance_epoch transaction!"
        );

        arguments.append(&mut call_arg_arguments.unwrap());

        info!("Call arguments to advance_epoch transaction: {:?}", params);

        builder.programmable_move_call(
            SUI_SYSTEM_PACKAGE_ID,
            SUI_SYSTEM_MODULE_NAME.to_owned(),
            ADVANCE_EPOCH_SAFE_MODE_FUNCTION_NAME.to_owned(),
            vec![],
            arguments,
        );

        Ok(builder.finish())
    }

    pub fn construct_bfc_round_pt(
        param: &ChangeObcRoundParams,
        builder: &mut ProgrammableTransactionBuilder,
        is_safe_mode: bool,
        discard: bool,
    ) -> Result<(), ExecutionError> {
        if !is_safe_mode { // if safe mode skip judge dao vote result
            let mut arguments = vec![];
            let args = vec![
                CallArg::BFC_SYSTEM_MUT,
                CallArg::Pure(bcs::to_bytes(&param.epoch).unwrap()),
                CallArg::Pure(bcs::to_bytes(&param.epoch_start_timestamp_ms).unwrap()),
            ].into_iter()
                .map(|a| builder.input(a))
                .collect::<Result<_, _>>();

        let mut builder = ProgrammableTransactionBuilder::new();
        let mut arguments = vec![];

        arguments.append(&mut args.unwrap());

        info!("Call arguments to bfc round transaction: {:?}",round_id);

            builder.programmable_move_call(
                BFC_SYSTEM_PACKAGE_ID,
                BFC_SYSTEM_MODULE_NAME.to_owned(),
                BFC_ROUND_FUNCTION_NAME.to_owned(),
                vec![],
                arguments,
            );
        }
        if discard {
            return Ok(());
        }


        for (type_tag, gas_cost_summary) in param.stable_gas_summarys.clone().into_iter() {
            // create rewards in stable coin

            let stable_charge_arg = builder
                .input(CallArg::Pure(
                    bcs::to_bytes(&calculate_add(
                        calculate_reward_rate(
                            gas_cost_summary.gas_by_stable.computation_cost, param.reward_rate), gas_cost_summary.gas_by_stable.storage_cost)).unwrap(),
                ))
                .unwrap();
            let rewards = builder.programmable_move_call(
                SUI_FRAMEWORK_PACKAGE_ID,
                BALANCE_MODULE_NAME.to_owned(),
                BALANCE_CREATE_REWARDS_FUNCTION_NAME.to_owned(),
                vec![type_tag.clone()],
                vec![stable_charge_arg],
            );

            //exchange stable coin to bfc
            let system_obj = builder.input(CallArg::BFC_SYSTEM_MUT).unwrap();
            let bfc_charge_arg = builder
                .input(CallArg::Pure(
                    bcs::to_bytes(&calculate_add(calculate_reward_rate(gas_cost_summary.gas_by_bfc.computation_cost, param.reward_rate), gas_cost_summary.gas_by_bfc.storage_cost)).unwrap(),                ))
                .unwrap();
            let rewards_bfc = builder.programmable_move_call(
                BFC_SYSTEM_PACKAGE_ID,
                BFC_SYSTEM_MODULE_NAME.to_owned(),
                STABLE_COIN_TO_BFC_FUNCTION_NAME.to_owned(),
                vec![type_tag.clone()],
                vec![system_obj, rewards, bfc_charge_arg],
            );

            // Destroy the rewards.
            builder.programmable_move_call(
                SUI_FRAMEWORK_PACKAGE_ID,
                BALANCE_MODULE_NAME.to_owned(),
                BALANCE_DESTROY_REBATES_FUNCTION_NAME.to_owned(),
                vec![GAS::type_tag()],
                vec![rewards_bfc],
            );

        }
        let storage_rebate_arg = builder
            .input(CallArg::Pure(
                bcs::to_bytes(&(
                    param.storage_rebate)).unwrap(),
            ))
            .unwrap();
        let storage_rebate = builder.programmable_move_call(
            SUI_FRAMEWORK_PACKAGE_ID,
            BALANCE_MODULE_NAME.to_owned(),
            BALANCE_CREATE_REWARDS_FUNCTION_NAME.to_owned(),
            vec![GAS::type_tag()],
            vec![storage_rebate_arg],
        );

        let system_obj = builder.input(CallArg::BFC_SYSTEM_MUT).unwrap();
        builder.programmable_move_call(
            BFC_SYSTEM_PACKAGE_ID,
            BFC_SYSTEM_MODULE_NAME.to_owned(),
            DEPOSIT_TO_TREASURY_FUNCTION_NAME.to_owned(),
            vec![],
            vec![system_obj,storage_rebate],
        );

        Ok(builder.finish())
    }

    fn bfc_round(
        _change_round: ChangeBfcRound,
        _temporary_store: &mut TemporaryStore<'_>,
        _tx_ctx: &mut TxContext,
        _move_vm: &Arc<MoveVM>,
        _gas_charger: &mut GasCharger,
        _protocol_config: &ProtocolConfig,
        _metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError>{
        // let _ = BfcRoundParams {
        //     round_id:change_round.bfc_round
        // };
        // let advance_epoch_pt = construct_bfc_round_pt(change_round.bfc_round)?;
        // let result = programmable_transactions::execution::execute::<execution_mode::System>(
        //     protocol_config,
        //     metrics.clone(),
        //     move_vm,
        //     temporary_store,
        //     tx_ctx,
        //     gas_charger,
        //     advance_epoch_pt,
        // );

        // #[cfg(msim)]
        // let _result = maybe_modify_result(result, change_round.bfc_round);

        // if result.is_err() {
        //     tracing::error!(
        //     "Failed to execute advance epoch transaction. Switching to safe mode. Error: {:?}. Input objects: {:?}.",
        //     result.as_ref().err(),
        //     temporary_store.objects(),
        // );
        //     temporary_store.drop_writes();
        //     // Must reset the storage rebate since we are re-executing.
        //     gas_charger.reset_storage_cost_and_rebate();
        //
        //     temporary_store.advance_bfc_round_mode(protocol_config);
        // }
        Ok(())
    }

    fn advance_epoch(
        builder: ProgrammableTransactionBuilder,
        change_epoch: ChangeEpoch,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let mut storage_rebate = 0u64;
        let mut non_refundable_storage_fee = 0u64;

        let mut storage_charge = 0u64;
        let mut computation_charge = 0u64;
        let mut deposit_computation_charge = 0u64;
        let mut discard = false;


        info!("change epoch: {:?}",change_epoch);
        let rate_result = temporary_store.get_stable_rate_map_and_reward_rate();
        if rate_result.is_err() {
            discard = true;
        }

        let is_safe_mode = temporary_store.is_safe_mode();
        let mut reward_rate= 0u64; // deposit all gas to treasury

        match rate_result {
            Ok((_,rate)) => {
                reward_rate = rate;
                for (_, gas_cost_summary) in &change_epoch.stable_gas_summarys {
                    let computation_reward = calculate_reward_rate(gas_cost_summary.gas_by_bfc.computation_cost, rate);

                    // check u64 overflow
                    if storage_rebate > u64::MAX - gas_cost_summary.gas_by_bfc.storage_rebate
                        || non_refundable_storage_fee > u64::MAX - gas_cost_summary.gas_by_bfc.non_refundable_storage_fee
                        || storage_charge > u64::MAX - gas_cost_summary.gas_by_bfc.storage_cost
                        || computation_charge > u64::MAX - computation_reward
                        || deposit_computation_charge > u64::MAX - (gas_cost_summary.gas_by_bfc.computation_cost - computation_reward)
                    {
                        storage_rebate = 0;
                        non_refundable_storage_fee = 0;
                        storage_charge = 0;
                        computation_charge = 0;
                        deposit_computation_charge = 0;

                        discard = true;
                        break;
                    }

                    storage_rebate += gas_cost_summary.gas_by_bfc.storage_rebate;
                    non_refundable_storage_fee += gas_cost_summary.gas_by_bfc.non_refundable_storage_fee;
                    computation_charge += computation_reward;
                    deposit_computation_charge += gas_cost_summary.gas_by_bfc.computation_cost - computation_reward;
                    storage_charge += gas_cost_summary.gas_by_bfc.storage_cost;
                }
            },
            Err(e) => {
                info!("Read reward_rate failed with err {:?}",e);
                discard = true;
            }
        }
        // check u64 overflow for advance_epoch_params
        if !discard && (storage_charge > u64::MAX - change_epoch.bfc_storage_charge
            || computation_charge > u64::MAX - change_epoch.bfc_computation_charge
            || storage_rebate > u64::MAX - change_epoch.bfc_storage_rebate
            || non_refundable_storage_fee > u64::MAX - change_epoch.bfc_non_refundable_storage_fee) {
            storage_charge = 0;
            computation_charge = 0;
            storage_rebate = 0;
            non_refundable_storage_fee = 0;

            discard = true;
        }

        let obc_params = ChangeObcRoundParams {
            epoch: change_epoch.epoch,
            stable_gas_summarys: change_epoch.stable_gas_summarys.clone(),
            bfc_computation_charge: computation_charge,
            bfc_deposit_computation_charge: deposit_computation_charge,
            epoch_start_timestamp_ms: change_epoch.epoch_start_timestamp_ms,
            reward_rate,
            storage_rebate,
        };
        let advance_epoch_storage_charge = change_epoch.bfc_storage_charge + storage_charge;
        let advance_epoch_computation_charge = change_epoch.bfc_computation_charge + computation_charge;
        let advance_epoch_storage_rebate = change_epoch.bfc_storage_rebate + storage_rebate;
        let advance_epoch_non_refundable_storage_fee = change_epoch.bfc_non_refundable_storage_fee + non_refundable_storage_fee;
        let mut params = AdvanceEpochParams {
            epoch: change_epoch.epoch,
            next_protocol_version: change_epoch.protocol_version,
            storage_charge: advance_epoch_storage_charge,
            computation_charge: advance_epoch_computation_charge,
            storage_rebate: advance_epoch_storage_rebate,
            non_refundable_storage_fee: advance_epoch_non_refundable_storage_fee,
            storage_fund_reinvest_rate: protocol_config.storage_fund_reinvest_rate(),
            reward_slashing_rate: protocol_config.reward_slashing_rate(),
            epoch_start_timestamp_ms: change_epoch.epoch_start_timestamp_ms,
        };

        let advance_epoch_pt = construct_advance_epoch_pt(&obc_params, builder, &params, is_safe_mode, discard)?;
        let result = programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics.clone(),
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            advance_epoch_pt,
        );

        #[cfg(msim)]
        let result = maybe_modify_result(result, change_epoch.epoch);

        if result.is_err() {
            tracing::error!(
            "Failed to execute advance epoch transaction. Switching to safe mode. Error: {:?}. Input objects: {:?}. Tx data: {:?}",
            result.as_ref().err(),
            temporary_store.objects(),
            change_epoch,
        );
            temporary_store.drop_writes();
            // Must reset the storage rebate since we are re-executing.
            gas_charger.reset_storage_cost_and_rebate();
            params.storage_charge = change_epoch.bfc_storage_charge;
            params.computation_charge = change_epoch.bfc_computation_charge;
            params.storage_rebate = change_epoch.bfc_storage_rebate;
            params.non_refundable_storage_fee = change_epoch.bfc_non_refundable_storage_fee;

            if protocol_config.get_advance_epoch_start_time_in_safe_mode() {
                temporary_store.advance_epoch_safe_mode(&params, protocol_config);
            } else {
                let advance_epoch_safe_mode_pt =
                    construct_advance_epoch_safe_mode_pt(&params, protocol_config)?;
                programmable_transactions::execution::execute::<execution_mode::System>(
                    protocol_config,
                    metrics.clone(),
                    move_vm,
                    temporary_store,
                    tx_ctx,
                    gas_charger,
                    advance_epoch_safe_mode_pt,
                )
                .expect("Advance epoch with safe mode must succeed");
            }
        }


        let binary_config = to_binary_config(protocol_config);
        for (version, modules, dependencies) in change_epoch.system_packages.into_iter() {
            let deserialized_modules: Vec<_> = modules
                .iter()
                .map(|m| CompiledModule::deserialize_with_config(m, &binary_config).unwrap())
                .collect();

            if version == OBJECT_START_VERSION {
                let package_id = deserialized_modules.first().unwrap().address();
                info!("adding new system package {package_id}");

                let publish_pt = {
                    let mut b = ProgrammableTransactionBuilder::new();
                    b.command(Command::Publish(modules, dependencies));
                    b.finish()
                };

                programmable_transactions::execution::execute::<execution_mode::System>(
                    protocol_config,
                    metrics.clone(),
                    move_vm,
                    temporary_store,
                    tx_ctx,
                    gas_charger,
                    publish_pt,
                )
                .expect("System Package Publish must succeed");
            } else {
                let mut new_package = Object::new_system_package(
                    &deserialized_modules,
                    version,
                    dependencies,
                    tx_ctx.digest(),
                );

                info!(
                    "upgraded system package {:?}",
                    new_package.compute_object_reference()
                );

                // Decrement the version before writing the package so that the store can record the
                // version growing by one in the effects.
                new_package
                    .data
                    .try_as_package_mut()
                    .unwrap()
                    .decrement_version();

                // upgrade of a previously existing framework module
                temporary_store.upgrade_system_package(new_package);
            }
        }

        Ok(())
    }

    /// Perform metadata updates in preparation for the transactions in the upcoming checkpoint:
    ///
    /// - Set the timestamp for the `Clock` shared object from the timestamp in the header from
    ///   consensus.
    fn setup_consensus_commit(
        consensus_commit_timestamp_ms: CheckpointTimestamp,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let pt = {
            let mut builder = ProgrammableTransactionBuilder::new();
            let res = builder.move_call(
                SUI_FRAMEWORK_ADDRESS.into(),
                CLOCK_MODULE_NAME.to_owned(),
                CONSENSUS_COMMIT_PROLOGUE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![
                    CallArg::CLOCK_MUT,
                    CallArg::Pure(bcs::to_bytes(&consensus_commit_timestamp_ms).unwrap()),
                ],
            );
            assert_invariant!(
                res.is_ok(),
                "Unable to generate consensus_commit_prologue transaction!"
            );
            builder.finish()
        };
        programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics,
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            pt,
        )
    }

    fn setup_authenticator_state_create(
        mut builder: ProgrammableTransactionBuilder,
    ) -> ProgrammableTransactionBuilder {
        builder
            .move_call(
                SUI_FRAMEWORK_ADDRESS.into(),
                AUTHENTICATOR_STATE_MODULE_NAME.to_owned(),
                AUTHENTICATOR_STATE_CREATE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![],
            )
            .expect("Unable to generate authenticator_state_create transaction!");
        builder
    }

    fn setup_randomness_state_create(
        mut builder: ProgrammableTransactionBuilder,
    ) -> ProgrammableTransactionBuilder {
        builder
            .move_call(
                SUI_FRAMEWORK_ADDRESS.into(),
                RANDOMNESS_MODULE_NAME.to_owned(),
                RANDOMNESS_STATE_CREATE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![],
            )
            .expect("Unable to generate randomness_state_create transaction!");
        builder
    }

    fn setup_authenticator_state_update(
        update: AuthenticatorStateUpdate,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let pt = {
            let mut builder = ProgrammableTransactionBuilder::new();
            let res = builder.move_call(
                SUI_FRAMEWORK_ADDRESS.into(),
                AUTHENTICATOR_STATE_MODULE_NAME.to_owned(),
                AUTHENTICATOR_STATE_UPDATE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![
                    CallArg::Object(ObjectArg::SharedObject {
                        id: SUI_AUTHENTICATOR_STATE_OBJECT_ID,
                        initial_shared_version: update.authenticator_obj_initial_shared_version,
                        mutable: true,
                    }),
                    CallArg::Pure(bcs::to_bytes(&update.new_active_jwks).unwrap()),
                ],
            );
            assert_invariant!(
                res.is_ok(),
                "Unable to generate authenticator_state_update transaction!"
            );
            builder.finish()
        };
        programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics,
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            pt,
        )
    }

    fn setup_authenticator_state_expire(
        mut builder: ProgrammableTransactionBuilder,
        expire: AuthenticatorStateExpire,
    ) -> ProgrammableTransactionBuilder {
        builder
            .move_call(
                SUI_FRAMEWORK_ADDRESS.into(),
                AUTHENTICATOR_STATE_MODULE_NAME.to_owned(),
                AUTHENTICATOR_STATE_EXPIRE_JWKS_FUNCTION_NAME.to_owned(),
                vec![],
                vec![
                    CallArg::Object(ObjectArg::SharedObject {
                        id: SUI_AUTHENTICATOR_STATE_OBJECT_ID,
                        initial_shared_version: expire.authenticator_obj_initial_shared_version,
                        mutable: true,
                    }),
                    CallArg::Pure(bcs::to_bytes(&expire.min_epoch).unwrap()),
                ],
            )
            .expect("Unable to generate authenticator_state_expire transaction!");
        builder
    }

    fn setup_randomness_state_update(
        update: RandomnessStateUpdate,
        temporary_store: &mut TemporaryStore<'_>,
        tx_ctx: &mut TxContext,
        move_vm: &Arc<MoveVM>,
        gas_charger: &mut GasCharger,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
    ) -> Result<(), ExecutionError> {
        let pt = {
            let mut builder = ProgrammableTransactionBuilder::new();
            let res = builder.move_call(
                SUI_FRAMEWORK_ADDRESS.into(),
                RANDOMNESS_MODULE_NAME.to_owned(),
                RANDOMNESS_STATE_UPDATE_FUNCTION_NAME.to_owned(),
                vec![],
                vec![
                    CallArg::Object(ObjectArg::SharedObject {
                        id: SUI_RANDOMNESS_STATE_OBJECT_ID,
                        initial_shared_version: update.randomness_obj_initial_shared_version,
                        mutable: true,
                    }),
                    CallArg::Pure(bcs::to_bytes(&update.randomness_round).unwrap()),
                    CallArg::Pure(bcs::to_bytes(&update.random_bytes).unwrap()),
                ],
            );
            assert_invariant!(
                res.is_ok(),
                "Unable to generate randomness_state_update transaction!"
            );
            builder.finish()
        };
        programmable_transactions::execution::execute::<execution_mode::System>(
            protocol_config,
            metrics,
            move_vm,
            temporary_store,
            tx_ctx,
            gas_charger,
            pt,
        )
    }

    fn setup_coin_deny_list_state_create(
        mut builder: ProgrammableTransactionBuilder,
    ) -> ProgrammableTransactionBuilder {
        builder
            .move_call(
                SUI_FRAMEWORK_ADDRESS.into(),
                DENY_LIST_MODULE.to_owned(),
                DENY_LIST_CREATE_FUNC.to_owned(),
                vec![],
                vec![],
            )
            .expect("Unable to generate coin_deny_list_create transaction!");
        builder
    }
}
