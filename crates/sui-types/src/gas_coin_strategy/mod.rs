// Copyright (c) OpenBlock Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::gas_coin::{GasCoin, GAS};
use anyhow::{format_err, Result};
use std::fmt::{self};
use core::str::FromStr;
use num_enum::{IntoPrimitive, TryFromPrimitive};


pub mod price_oracle;
pub mod inner_swap;

#[derive(
Clone,
Copy,
Debug,
Eq,
Hash,
PartialEq,
PartialOrd,
Ord,
IntoPrimitive,
TryFromPrimitive,
)]
#[repr(u8)]
pub enum GasCoinStrategy {
    PriceOracle = 0,
    InnerSwap = 1,
}


impl GasCoinStrategy {
    pub fn value(self) -> u8 {
        self.into()
    }
}

impl Default for GasCoinStrategy {
    fn default() -> Self {
        GasCoinStrategy::PriceOracle
    }
}

impl fmt::Display for GasCoinStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GasCoinStrategy::PriceOracle => write!(f, "price_oracle"),
            GasCoinStrategy::InnerSwap => write!(f, "inner_swap"),
        }
    }
}

impl FromStr for GasCoinStrategy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "price_oracle" => Ok(GasCoinStrategy::PriceOracle),
            "inner_swap" => Ok(GasCoinStrategy::InnerSwap),
            s => Err(format_err!("Unknown GasCoinStrategy: {}", s)),
        }
    }
}


pub trait GasCoinExchange {
    /// Exchange any coin to default platform coin
    fn exchange(any_coin: GasCoin, amount: u64) -> GAS;
}