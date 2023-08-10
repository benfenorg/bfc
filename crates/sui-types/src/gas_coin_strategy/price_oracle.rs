// Copyright (c) OpenBlock Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::gas_coin::{GasCoin};
use crate::gas_coin_strategy::GasCoinExchange;
use anyhow::Result;
use crate::coin::Coin;

trait PriceOracle {
    fn get_price(&self,first_coin: Coin, second_coin:Coin) -> Result<u64>;
}

#[derive(Default)]
pub struct SwapPriceOracle {}

impl SwapPriceOracle {
    pub fn new() -> Self {
        Self {}
    }
}

impl GasCoinExchange for SwapPriceOracle {
    fn exchange_obc(_any_coin: Coin) -> GasCoin {
        todo!()
    }

    fn exchange(_any_coin_x: Coin, _any_coin_y: Coin) {
        todo!()
    }

    fn price(_any_coin_x: Coin, _any_coin_y: Coin) {
        todo!()
    }
}

impl PriceOracle for SwapPriceOracle {
    fn get_price(&self, _first_coin: Coin, _second_coin:Coin) -> Result<u64>{
        Ok(1)
    }
}

#[derive(Default)]
pub struct ExternalPriceOracle {}

impl ExternalPriceOracle {
    pub fn new() -> Self {
        Self {}
    }
}

impl PriceOracle for ExternalPriceOracle {
    fn get_price(&self, _first_coin: Coin, _second_coin:Coin) -> Result<u64>{
        Ok(1)
    }
}