// Copyright (c) OpenBlock Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::gas_coin::{GasCoin, GAS};
use crate::gas_coin_strategy::GasCoinExchange;

#[derive(Default)]
pub struct PriceOracle {}

impl PriceOracle {
    pub fn new() -> Self {
        Self {}
    }
}

impl GasCoinExchange for PriceOracle {

    fn exchange(any_coin: GasCoin, amount: u64) -> GAS {
        //todo
        !todo!()
    }
}