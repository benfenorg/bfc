// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::ident_str;
use shared_crypto::intent::Intent;
use sui_json_rpc_types::{SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions};
use sui_sdk::wallet_context::WalletContext;
use sui_types::transaction::{CallArg, Transaction, TransactionData};

pub mod client_commands;
pub mod console;
pub mod fire_drill;
pub mod inner_swap;
pub mod keytool;
pub mod shell;
pub mod sui_commands;
pub mod validator_commands;
pub mod gas_coin_commands;

pub mod genesis_ceremony;
pub mod genesis_inspector;
use sui_keys::keystore::AccountKeystore;
use sui_types::base_types::SuiAddress;
use sui_types::SUI_SYSTEM_PACKAGE_ID;
use crate::fire_drill::get_gas_obj_ref;


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
    let signature =
        context
            .config
            .keystore
            .sign_secure(&sender, &tx_data, Intent::sui_transaction())?;
    let transaction = Transaction::from_data(tx_data, Intent::sui_transaction(), vec![signature]);
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

async fn construct_unsigned_0x5_txn(
    context: &mut WalletContext,
    sender: SuiAddress,
    function: &'static str,
    call_args: Vec<CallArg>,
    gas_budget: u64,
) -> anyhow::Result<TransactionData> {
    let sui_client = context.get_client().await?;
    let mut args = vec![CallArg::SUI_SYSTEM_MUT];
    args.extend(call_args);
    let rgp = sui_client
        .governance_api()
        .get_reference_gas_price()
        .await?;

    let gas_obj_ref = get_gas_obj_ref(sender, &sui_client, gas_budget).await?;
    TransactionData::new_move_call(
        sender,
        SUI_SYSTEM_PACKAGE_ID,
        ident_str!("sui_system").to_owned(),
        ident_str!(function).to_owned(),
        vec![],
        gas_obj_ref,
        args,
        gas_budget,
        rgp,
    )
}