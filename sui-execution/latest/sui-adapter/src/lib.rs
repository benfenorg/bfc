// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate sui_types;

use tracing::warn;

pub mod adapter;
pub mod error;
pub mod execution_engine;
pub mod gas_charger;
pub mod programmable_transactions;
pub mod type_layout_resolver;
pub mod temporary_store;

#[cfg(test)]
mod tests;


pub fn calculate_bfc_to_stable_cost(cost: u64, rate: u64) -> u64 {
    if rate == 0 {
        warn!("rate is zero, cost: {}, rate: {}", cost, rate);
        return cost;
    }
    //参考合约中的处理：将bfc换成stable采用舍去小数：checked_div_round
    ((cost as u128 * 1000000000u128) / rate as u128) as u64
}

pub fn calculate_stable_to_bfc_cost(cost: u64, rate: u64) -> u64 {
    if rate == 0 {
        warn!("rate is zero, cost: {}, rate: {}", cost, rate);
        return cost;
    }
    let num = cost as u128 * rate as u128;
    let denom = 1000000000u128;
    let quotient = num / denom;
    let remained = num % denom;
    if remained > 0 {
        (quotient + 1) as u64
    } else {
        quotient as u64
    }
}