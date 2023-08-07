// Copyright (c) OpenBlock Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::coin::Coin;
use crate::gas_coin_strategy::GasCoinExchange;

#[derive(Default)]
pub struct InnerSwap {}

impl InnerSwap {
    pub fn new() -> Self {
        Self {}
    }
}

impl GasCoinExchange for InnerSwap {

    fn exchange(any_coin: Coin, amount: u64) -> u64 {
        //todo
        0
    }
}