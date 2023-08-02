// Copyright (c) OpenBlock Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::gas_coin::{GasCoin};
use crate::gas_coin_strategy::GasCoinExchange;
use anyhow::Result;

trait PriceOracle {
    fn get_price(&self,first_coin: GasCoin,second_coin:GasCoin) -> Result<u64>;
}

#[derive(Default)]
pub struct SwapPriceOracle {}

impl SwapPriceOracle {
    pub fn new() -> Self {
        Self {}
    }
}

impl GasCoinExchange for SwapPriceOracle {

    fn exchange(any_coin: GasCoin, amount: u64) -> u64 {
        //todo
        !todo!()
    }
}

impl PriceOracle for SwapPriceOracle {
    fn get_price(&self,first_coin: GasCoin,second_coin:GasCoin) -> Result<u64>{
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
    fn get_price(&self,first_coin: GasCoin,second_coin:GasCoin) -> Result<u64>{
        Ok(1)
    }
}