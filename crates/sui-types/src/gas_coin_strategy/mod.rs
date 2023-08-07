// Copyright (c) OpenBlock Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::gas_coin::{GasCoin};
use anyhow::{format_err, Result};
use std::fmt::{self};
use core::str::FromStr;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::base_types::SuiAddress;
use crate::collection_types::VecMap;
use serde::{Deserialize, Serialize};
use crate::coin::Coin;


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
    fn exchange(any_coin: Coin, amount: u64) -> u64;
}

/// Rust version of the Move sui_system::gas_coin_map::GasCoinMap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GasCoinMap {
    pub active_gas_coins: VecMap<SuiAddress, GasCoinEntity>,
}

/// Rust version of the Move sui_system::gas_coin_map::GasCoinEntity type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GasCoinEntity {
    pub id_address: SuiAddress
}