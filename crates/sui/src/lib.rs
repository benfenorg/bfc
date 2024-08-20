// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use move_core_types::ident_str;
use shared_crypto::intent::Intent;
use sui_json_rpc_types::{SuiObjectDataOptions, SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions};
use sui_sdk::wallet_context::WalletContext;
use sui_types::transaction::{CallArg, Transaction, TransactionData};

pub mod client_commands;
#[macro_use]
pub mod client_ptb;
mod clever_error_rendering;
pub mod console;
pub mod displays;
pub mod fire_drill;
pub mod genesis_ceremony;
pub mod genesis_inspector;
pub mod key_identity;
pub mod keytool;
pub mod shell;
pub mod sui_commands;
pub mod validator_commands;
mod verifier_meter;
pub mod zklogin_commands_util;

use sui_keys::keystore::AccountKeystore;
use sui_types::base_types::{ObjectID, ObjectRef, ObjectType, SuiAddress};
use sui_types::SUI_SYSTEM_PACKAGE_ID;
use crate::fire_drill::get_gas_obj_ref;


/// Get gas coin object ref.
pub async fn get_object_ref(
    context: &mut WalletContext,
    coin_id: ObjectID,
) -> Result<ObjectRef> {
    get_object_ref_with_type(context, coin_id)
        .await
        .map(|(obj_ref, _)| obj_ref)
}

/// Get gas coin object ref and type.
pub async fn get_object_ref_with_type(
    context: &mut WalletContext,
    coin_id: ObjectID,
) -> Result<(ObjectRef, ObjectType) > {
    let sui_client = context.get_client().await?;
    let gas_obj = sui_client
        .read_api()
        .get_object_with_options(
            coin_id,
            SuiObjectDataOptions::default()
                .with_owner()
                .with_type(),
        )
        .await?
        .into_object()?;
    Ok((gas_obj.object_ref(), gas_obj.object_type()?))
}

pub async fn call_system_txn_with(
    context: &mut WalletContext,
    sender: SuiAddress,
    txn_data: TransactionData,
) -> anyhow::Result<SuiTransactionBlockResponse> {
    let signature =
        context
            .config
            .keystore
            .sign_secure(&sender, &txn_data, Intent::sui_transaction())?;
    let transaction = Transaction::from_data(txn_data, vec![signature]);
    let sui_client = context.get_client().await?;
    sui_client
        .quorum_driver_api()
        .execute_transaction_block(
            transaction,
            SuiTransactionBlockResponseOptions::new()
                .with_input()
                .with_effects(),
            Some(sui_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))
}
/// Common packaging of call system interface.
pub async fn call_system_txn(
    context: &mut WalletContext,
    txn_data: TransactionData,
) -> anyhow::Result<SuiTransactionBlockResponse> {
    let sender = context.active_address()?;
    let signature =
        context
            .config
            .keystore
            .sign_secure(&sender, &txn_data, Intent::sui_transaction())?;
    let transaction = Transaction::from_data(txn_data, vec![signature]);
    let sui_client = context.get_client().await?;
    sui_client
        .quorum_driver_api()
        .execute_transaction_block(
            transaction,
            SuiTransactionBlockResponseOptions::new()
                .with_input()
                .with_effects(),
            Some(sui_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))
}

/// Common packaging of call system interface.
pub async fn call_0x5(
    context: &mut WalletContext,
    function: &'static str,
    call_args: Vec<CallArg>,
    gas_budget: u64,
) -> anyhow::Result<SuiTransactionBlockResponse> {
    let sender = context.active_address()?;
    let tx_data =
        construct_unsigned_0x5_txn(context, sender, function, call_args, gas_budget).await?;
    call_system_txn(context, tx_data).await
}

pub async fn call_0x200(
    context: &mut WalletContext,
    function: &'static str,
    call_args: Vec<CallArg>,
    gas_budget: u64,
) -> anyhow::Result<SuiTransactionBlockResponse> {
    let sender = context.active_address()?;
    let tx_data =
        construct_unsigned_0x200_txn(context, sender, function, call_args, gas_budget).await?;
    call_system_txn(context, tx_data).await
}

pub async fn call_0x200_with(
    context: &mut WalletContext,
    sender: SuiAddress,
    function: &'static str,
    call_args: Vec<CallArg>,
    gas_budget: u64,
) -> anyhow::Result<SuiTransactionBlockResponse> {
    let tx_data =
        construct_unsigned_0x200_txn(context, sender, function, call_args, gas_budget).await?;
    call_system_txn_with(context, sender, tx_data).await
}

async fn construct_unsigned_0x200_txn(
    context: &mut WalletContext,
    sender: SuiAddress,
    function: &'static str,
    call_args: Vec<CallArg>,
    gas_budget: u64,
) -> anyhow::Result<TransactionData> {
    let mut args = vec![CallArg::BFC_SYSTEM_MUT];
    args.extend(call_args);
    construct_unsigned_system_txn(
        context,
        sender,
        "bfc_system",
        function,
        args,
        gas_budget
    ).await
}
async fn construct_unsigned_0x5_txn(
    context: &mut WalletContext,
    sender: SuiAddress,
    function: &'static str,
    call_args: Vec<CallArg>,
    gas_budget: u64,
) -> anyhow::Result<TransactionData> {
    let mut args = vec![CallArg::SUI_SYSTEM_MUT];
    args.extend(call_args);
    construct_unsigned_system_txn(
        context,
        sender,
        "sui_system",
        function,
        args,
        gas_budget
    ).await
}
async fn construct_unsigned_system_txn(
    context: &mut WalletContext,
    sender: SuiAddress,
    module: &'static str,
    function: &'static str,
    call_args: Vec<CallArg>,
    gas_budget: u64,
) -> anyhow::Result<TransactionData> {
    let sui_client = context.get_client().await?;
    let rgp = sui_client
        .governance_api()
        .get_reference_gas_price()
        .await?;

    let gas_obj_ref = get_gas_obj_ref(sender, &sui_client, gas_budget).await?;
    TransactionData::new_move_call(
        sender,
        SUI_SYSTEM_PACKAGE_ID,
        ident_str!(module).to_owned(),
        ident_str!(function).to_owned(),
        vec![],
        gas_obj_ref,
        call_args,
        gas_budget,
        rgp,
    )
}