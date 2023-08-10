// Copyright (c) OpenBlock Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::coin::Coin;
use sui_types::gas_coin::GasCoin;
use sui_types::transaction::{CallArg, ObjectArg};
use crate::gas_coin_commands::get_object_ref;
use crate::validator_commands::{call_0x5, write_transaction_response};

const DEFAULT_GAS_BUDGET: u64 = 100_000_000; // 0.1 SUI

#[derive(Default)]
pub struct InnerSwap {}

impl InnerSwap {
    pub fn new() -> Self {
        Self {}
    }
}

impl InnerSwap {
    pub async fn exchange_obc(context: &mut WalletContext, any_coin: Coin) -> GasCoin {
        let coin_id_ref = get_object_ref(context, *any_coin.id()).await.unwrap();
        let args = vec![
            CallArg::Object(ObjectArg::ImmOrOwnedObject(coin_id_ref))
        ];
        let response =
            call_0x5(context, "request_swap_obc", args, DEFAULT_GAS_BUDGET).await.unwrap();
        let result = write_transaction_response(&response).unwrap();
        println!("{:?}", result);
        GasCoin::new(ObjectID::random(), 1000)
    }

    pub async fn exchange(context: &mut WalletContext, any_coin_x: Coin, any_coin_y: Coin) {
        let coin_x_ref = get_object_ref(context, *(any_coin_x.id())).await.unwrap();
        let coin_y_ref = get_object_ref(context, *(any_coin_y.id())).await.unwrap();

        let args = vec![
            CallArg::Object(ObjectArg::ImmOrOwnedObject(coin_x_ref)),
            CallArg::Object(ObjectArg::ImmOrOwnedObject(coin_y_ref))
        ];
        let response =
            call_0x5(context, "request_swap", args, DEFAULT_GAS_BUDGET).await.unwrap();
        let result = write_transaction_response(&response).unwrap();
        println!("{:?}", result);
    }

    pub fn price(_any_coin_x: Coin, _any_coin_y: Coin) {
        todo!()
    }
}