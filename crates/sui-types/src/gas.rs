// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use tracing::warn;
pub use checked::*;

#[sui_macros::with_checked_arithmetic]
pub mod checked {

    use crate::gas_model::gas_predicates::gas_price_too_high;
    use crate::{
        effects::{TransactionEffects, TransactionEffectsAPI},
        error::{ExecutionError, SuiResult, UserInputError, UserInputResult},
        gas_model::{gas_v2::SuiGasStatus as SuiGasStatusV2, tables::GasStatus},
        object::Object,
        sui_serde::{BigInt, Readable},
        transaction::ObjectReadResult,
        ObjectID,
    };
    use enum_dispatch::enum_dispatch;
    use itertools::MultiUnzip;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use sui_protocol_config::ProtocolConfig;
    use crate::gas::{calculate_bfc_to_stable_cost_with_base_point};

    pub const BASE_RATE:u64 = 1_000_000_000u64;
    pub const DEFAULT_BASE_POINT_FOR_STABLE_COINS:u64 = 10u64;
    pub const DEFAULT_BASE_POINT_FOR_BFC:u64 = 0u64;

    pub const ONE_HUNDRED: u64 = 100;

    #[enum_dispatch]
    pub trait SuiGasStatusAPI {
        fn is_unmetered(&self) -> bool;
        fn move_gas_status(&self) -> &GasStatus;
        fn move_gas_status_mut(&mut self) -> &mut GasStatus;
        fn bucketize_computation(&mut self) -> Result<(), ExecutionError>;
        fn summary(&self) -> GasCostSummary;
        fn gas_budget(&self) -> u64;
        fn storage_gas_units(&self) -> u64;
        fn storage_rebate(&self) -> u64;
        fn unmetered_storage_rebate(&self) -> u64;
        fn gas_used(&self) -> u64;
        fn reset_storage_cost_and_rebate(&mut self);
        fn charge_storage_read(&mut self, size: usize) -> Result<(), ExecutionError>;
        fn charge_publish_package(&mut self, size: usize) -> Result<(), ExecutionError>;
        fn track_storage_mutation(
            &mut self,
            object_id: ObjectID,
            new_size: usize,
            storage_rebate: u64,
        ) -> u64;
        fn charge_storage_and_rebate(&mut self) -> Result<(), ExecutionError>;
        fn adjust_computation_on_out_of_gas(&mut self);
        fn stable_rate(&self) -> Option<u64>;
        fn base_points(&self) -> Option<u64>;
        fn has_adjust_computation_on_out_of_gas(&self) -> bool;
    }

    /// Version aware enum for gas status.
    #[enum_dispatch(SuiGasStatusAPI)]
    #[derive(Debug)]
    pub enum SuiGasStatus {
        // V1 does not exists any longer as it was a pre mainnet version.
        // So we start the enum from V2
        V2(SuiGasStatusV2),
    }

    impl SuiGasStatus {
        pub fn new(
            gas_budget: u64,
            gas_price: u64,
            reference_gas_price: u64,
            config: &ProtocolConfig,
            stable_rate: Option<u64>,
            base_points: Option<u64>,
        ) -> SuiResult<Self> {
            // Common checks. We may pull them into version specific status as needed, but they
            // are unlikely to change.

            // gas price must be bigger or equal to reference gas price
            if gas_price < reference_gas_price {
                return Err(UserInputError::GasPriceUnderRGP {
                    gas_price,
                    reference_gas_price,
                }
                .into());
            }

            if gas_price_too_high(config.gas_model_version()) && gas_price >= config.max_gas_price()
            {
                return Err(UserInputError::GasPriceTooHigh {
                    max_gas_price: config.max_gas_price(),
                }
                .into());
            }

            Ok(Self::V2(SuiGasStatusV2::new_with_budget(
                gas_budget,
                gas_price,
                reference_gas_price,
                config,
                stable_rate,
                base_points,
            )))
        }

        pub fn new_unmetered() -> Self {
            Self::V2(SuiGasStatusV2::new_unmetered())
        }

        // This is the only public API on SuiGasStatus, all other gas related operations should
        // go through `GasCharger`
        pub fn check_gas_balance(
            &self,
            gas_objs: &[&ObjectReadResult],
            gas_budget: u64,
        ) -> UserInputResult {
            match self {
                Self::V2(status) => status.check_gas_balance(gas_objs, gas_budget),
            }
        }

        pub fn has_adjust_computation_on_out_of_gas(&self) ->bool {
            return match self {
                Self::V2(status) => status.has_adjust_computation_on_out_of_gas(),
            }
        }
    }

    /// Summary of the charges in a transaction.
    /// Storage is charged independently of computation.
    /// There are 3 parts to the storage charges:
    /// `storage_cost`: it is the charge of storage at the time the transaction is executed.
    ///                 The cost of storage is the number of bytes of the objects being mutated
    ///                 multiplied by a variable storage cost per byte
    /// `storage_rebate`: this is the amount a user gets back when manipulating an object.
    ///                   The `storage_rebate` is the `storage_cost` for an object minus fees.
    /// `non_refundable_storage_fee`: not all the value of the object storage cost is
    ///                               given back to user and there is a small fraction that
    ///                               is kept by the system. This value tracks that charge.
    ///
    /// When looking at a gas cost summary the amount charged to the user is
    /// `computation_cost + storage_cost - storage_rebate`
    /// and that is the amount that is deducted from the gas coins.
    /// `non_refundable_storage_fee` is collected from the objects being mutated/deleted
    /// and it is tracked by the system in storage funds.
    ///
    /// Objects deleted, including the older versions of objects mutated, have the storage field
    /// on the objects added up to a pool of "potential rebate". This rebate then is reduced
    /// by the "nonrefundable rate" such that:
    /// `potential_rebate(storage cost of deleted/mutated objects) =
    /// storage_rebate + non_refundable_storage_fee`

    #[serde_as]
    #[derive(Eq, PartialEq, Clone, Debug, Default,Hash, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct GasCostSummary {
        pub base_point:u64,
        pub rate:u64,
        /// Cost of computation/execution
        #[schemars(with = "BigInt<u64>")]
        #[serde_as(as = "Readable<BigInt<u64>, _>")]
        pub computation_cost: u64,
        /// Storage cost, it's the sum of all storage cost for all objects created or mutated.
        #[schemars(with = "BigInt<u64>")]
        #[serde_as(as = "Readable<BigInt<u64>, _>")]
        pub storage_cost: u64,
        /// The amount of storage cost refunded to the user for all objects deleted or mutated in the
        /// transaction.
        #[schemars(with = "BigInt<u64>")]
        #[serde_as(as = "Readable<BigInt<u64>, _>")]
        pub storage_rebate: u64,
        /// The fee for the rebate. The portion of the storage rebate kept by the system.
        #[schemars(with = "BigInt<u64>")]
        #[serde_as(as = "Readable<BigInt<u64>, _>")]
        pub non_refundable_storage_fee: u64,
    }

    #[serde_as]
    #[derive(Eq, PartialEq, Clone, Debug, Default,Hash, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct GasCostSummaryAdjusted{
        pub gas_by_bfc:GasCostSummary,
        pub gas_by_stable: GasCostSummary,
    }

    impl GasCostSummary {
        pub fn new(
            computation_cost: u64,
            storage_cost: u64,
            storage_rebate: u64,
            non_refundable_storage_fee: u64,
        ) -> GasCostSummary {
            GasCostSummary {
                base_point: DEFAULT_BASE_POINT_FOR_BFC,
                rate: BASE_RATE,
                computation_cost,
                storage_cost,
                storage_rebate,
                non_refundable_storage_fee,
            }
        }

        pub fn gas_used(&self) -> u64 {
            self.computation_cost + self.storage_cost
        }

        pub fn gas_used_improved(&self) -> u64 {
            let computation_cost = calculate_bfc_to_stable_cost_with_base_point(self.computation_cost, self.rate, self.base_point);
            let storage_cost = calculate_bfc_to_stable_cost_with_base_point(self.storage_cost, self.rate, self.base_point);
            computation_cost + storage_cost
        }

        /// Portion of the storage rebate that gets passed on to the transaction sender. The remainder
        /// will be burned, then re-minted + added to the storage fund at the next epoch change
        pub fn sender_rebate(&self, storage_rebate_rate: u64) -> u64 {
            // we round storage rebate such that `>= x.5` goes to x+1 (rounds up) and
            // `< x.5` goes to x (truncates). We replicate `f32/64::round()`
            const BASIS_POINTS: u128 = 10000;
            (((self.storage_rebate as u128 * storage_rebate_rate as u128)
                + (BASIS_POINTS / 2)) // integer rounding adds half of the BASIS_POINTS (denominator)
                / BASIS_POINTS) as u64
        }

        /// Get net gas usage, positive number means used gas; negative number means refund.
        pub fn net_gas_usage(&self) -> i64 {
            self.gas_used() as i64 - self.storage_rebate as i64
        }

        pub fn new_from_txn_effects<'a>(
            transactions: impl Iterator<Item=&'a TransactionEffects>,
        ) -> GasCostSummary {
            let (storage_costs, computation_costs, storage_rebates, non_refundable_storage_fee): (
                Vec<u64>,
                Vec<u64>,
                Vec<u64>,
                Vec<u64>,
            ) = transactions
                .map(|e| {
                    (
                        e.gas_cost_summary().storage_cost,
                        e.gas_cost_summary().computation_cost,
                        e.gas_cost_summary().storage_rebate,
                        e.gas_cost_summary().non_refundable_storage_fee,
                    )
                })
                .multiunzip();

            GasCostSummary {
                base_point: DEFAULT_BASE_POINT_FOR_BFC,
                rate: BASE_RATE,
                storage_cost: storage_costs.iter().sum(),
                computation_cost: computation_costs.iter().sum(),
                storage_rebate: storage_rebates.iter().sum(),
                non_refundable_storage_fee: non_refundable_storage_fee.iter().sum(),
            }
        }

        pub fn into_rpc(&self) -> GasCostSummary  {
            GasCostSummary {
                base_point:self.base_point,
                rate:self.rate,
                storage_cost:calculate_bfc_to_stable_cost_with_base_point(self.storage_cost, self.rate, self.base_point),
                computation_cost: calculate_bfc_to_stable_cost_with_base_point(self.computation_cost, self.rate, self.base_point),
                storage_rebate: calculate_bfc_to_stable_cost_with_base_point(self.storage_rebate, self.rate, self.base_point),
                non_refundable_storage_fee: calculate_bfc_to_stable_cost_with_base_point(self.non_refundable_storage_fee, self.rate, self.base_point),
            }
        }
        pub fn net_gas_usage_improved(&self) -> i64 {
            let computation_cost = calculate_bfc_to_stable_cost_with_base_point(self.computation_cost, self.rate, self.base_point);
            let storage_cost = calculate_bfc_to_stable_cost_with_base_point(self.storage_cost, self.rate, self.base_point);
            let storage_rebate = calculate_bfc_to_stable_cost_with_base_point(self.storage_rebate, self.rate, self.base_point);
            (computation_cost + storage_cost) as i64 - (storage_rebate as i64)
        }

        pub fn storage_gas_usage_abs_improved(&self) -> u64 {
            let storage_cost = calculate_bfc_to_stable_cost_with_base_point(self.storage_cost, self.rate, self.base_point);
            let storage_rebate = calculate_bfc_to_stable_cost_with_base_point(self.storage_rebate, self.rate, self.base_point);
            if storage_cost >= storage_rebate {
                storage_cost - storage_rebate
            } else {
                storage_rebate - storage_cost
            }
        }

        pub fn storage_gas_usage_abs(&self) -> u64 {
            if self.storage_cost >= self.storage_rebate {
                self.storage_cost - self.storage_rebate
            } else {
                self.storage_rebate - self.storage_cost
            }
        }
    }

    impl std::fmt::Display for GasCostSummary {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "computation_cost: {}, storage_cost: {},  storage_rebate: {}, non_refundable_storage_fee: {}",
                self.computation_cost, self.storage_cost, self.storage_rebate, self.non_refundable_storage_fee,
            )
        }
    }

    impl std::ops::AddAssign<&Self> for GasCostSummary {
        fn add_assign(&mut self, other: &Self) {
            self.computation_cost += other.computation_cost;
            self.storage_cost += other.storage_cost;
            self.storage_rebate += other.storage_rebate;
            self.non_refundable_storage_fee += other.non_refundable_storage_fee;
        }
    }

    impl std::ops::AddAssign<Self> for GasCostSummary {
        fn add_assign(&mut self, other: Self) {
            self.add_assign(&other)
        }
    }

    //
    // Helper functions to deal with gas coins operations.
    //

    pub fn deduct_gas(gas_object: &mut Object, charge_or_rebate: i64) {
        // The object must be a gas coin as we have checked in transaction handle phase.
        let gas_coin = gas_object.data.try_as_move_mut().unwrap();
        let balance = gas_coin.get_coin_value_unsafe();
        let new_balance = if charge_or_rebate < 0 {
            balance + (-charge_or_rebate as u64)
        } else {
            assert!(balance >= charge_or_rebate as u64);
            balance - charge_or_rebate as u64
        };
        gas_coin.set_coin_value_unsafe(new_balance)
    }

    pub fn get_gas_balance(gas_object: &Object) -> UserInputResult<u64> {
        if let Some(move_obj) = gas_object.data.try_as_move() {
            let move_type = move_obj.type_();
            if move_type.is_gas_coin() || move_type.is_stable_gas_coin(){
                Ok(move_obj.get_coin_value_unsafe())
            }else {
                Err(UserInputError::InvalidGasObject {
                    object_id: gas_object.id(),
                })
            }
        } else {
            Err(UserInputError::InvalidGasObject {
                object_id: gas_object.id(),
            })
        }
    }
}
pub fn calculate_divide_rate(val: u64, rate_option: Option<u64>) -> u64 {
    if let Some(rate) = rate_option {
        if rate == 0 {
            return val;
        }
        let result =  (val as u128)  * BASE_RATE as u128/ (rate as u128);
        result as u64
    }else {
        val
    }
}

const REWARD_BASIS_POINTS: u128 = 100;

pub fn calculate_reward_rate(reward: u64, reward_rate: u64) -> u64 {
    if reward_rate == 0 {
        warn!("reward rate is zero, reward: {}", reward);
        return reward;
    }
    // 将u64转换为u128
    let reward_u128 = reward as u128;
    let rate_u128 = reward_rate as u128;

    // 使用checked_mul和checked_div
    match reward_u128.checked_mul(rate_u128) {
        Some(reward_) => {
            (reward_ / REWARD_BASIS_POINTS) as u64
        },
        None => {
            warn!("Multiplication overflow: reward: {}, reward_rate: {}", reward, reward_rate);
            return reward;
        }
    }
}

pub fn calculate_add(base: u64, add: u64) -> u64 {
    base.checked_add(add).unwrap_or_else(|| {
        warn!("Addition overflow: base: {}, add: {}", base, add);
        base
    })
}

const BFC_STABLE_BASIS_POINTS_U64: u64 = 100;
const BFC_PRECISION: u128 = 1000000000;

pub fn calculate_bfc_to_stable_cost_with_base_point(cost: u64, rate: u64, base_point: u64) -> u64 {
    if rate == 0 || rate == 1 {
        return cost;
    }
    //参考合约中的处理：将bfc换成stable采用舍去小数：checked_div_round
    ((cost as u128 * BFC_PRECISION * (BFC_STABLE_BASIS_POINTS_U64 + base_point) as u128) / (rate * BFC_STABLE_BASIS_POINTS_U64) as u128) as u64
}

pub fn calculate_bfc_to_stable_cost(cost: u64, rate: u64) -> u64 {
    if rate == 0 || rate == 1 {
        return cost;
    }
    //参考合约中的处理：将bfc换成stable采用舍去小数：checked_div_round
    ((cost as u128 * BFC_PRECISION) / (rate as u128)) as u64
}

pub fn calculate_stable_net_used_with_base_point(summary :&GasCostSummary) -> i64 {
    let computation_cost = calculate_bfc_to_stable_cost_with_base_point(summary.computation_cost, summary.rate, summary.base_point);
    let storage_cost = calculate_bfc_to_stable_cost_with_base_point(summary.storage_cost, summary.rate, summary.base_point);
    let storage_rebate = calculate_bfc_to_stable_cost_with_base_point(summary.storage_rebate, summary.rate, summary.base_point);
    (computation_cost + storage_cost) as i64 - (storage_rebate as i64)
}

pub fn calculate_stable_to_bfc_cost_with_base_point(stable_cost: u64, rate: u64, base_point: u64) -> u64 {
    stable_cost * (rate * BFC_STABLE_BASIS_POINTS_U64) / (BFC_PRECISION * (BFC_STABLE_BASIS_POINTS_U64 + base_point) as u128) as u64
}

pub fn calculate_stable_to_bfc_cost(cost: u64, rate: u64) -> u64 {
    if rate == 0 {
        warn!("rate is zero, cost: {}, rate: {}", cost, rate);
        return cost;
    }
    let cost_u128 = cost as u128;
    let rate_u128 = rate as u128;

    // 使用checked_mul和checked_div
    match cost_u128.checked_mul(rate_u128) {
        Some(num) => {
            let quotient = num / BFC_PRECISION;
            let remainder = num % BFC_PRECISION;
            if remainder > 0 {
                match quotient.checked_add(1) {
                    Some(result) => result as u64,
                    None => {
                        // 处理加法溢出
                        warn!("Addition overflow: quotient: {}", quotient);
                        cost
                    }
                }
            } else {
                quotient as u64
            }
        },
        None => {
            // 处理乘法溢出
            warn!("Multiplication overflow: cost: {}, rate: {}", cost, rate);
            cost
        }
    }
}

#[cfg(test)]
mod test{
    use crate::gas::{calculate_bfc_to_stable_cost_with_base_point};
    #[test]
    fn test_calculate_stable_rate() {
        let cost = 132240;
        let rate = 1000000032u64;
        let result = calculate_bfc_to_stable_cost_with_base_point(cost, rate, 0);
        println!("result: {}", result);
        let cost = -87119i64;
        let rate = 999999999u64;
        let result = calculate_bfc_to_stable_cost_with_base_point(cost.abs() as u64, rate, 10);
        println!("result: {}", result);
    }

    #[test]
    fn test_calculate_stable_rate_() {
        let cost = 100000+2645;
        let rate = 10000995385;
        let mut result = calculate_bfc_to_stable_cost_with_base_point(cost, rate, 10);

        println!("result: {}", result);

        let cost = 264480;
        let rate = 10000995385;
        println!("storage is {:?}",calculate_bfc_to_stable_cost_with_base_point(cost, rate, 10));
        result += calculate_bfc_to_stable_cost_with_base_point(cost, rate, 10);

        println!("result: {}", result);

        let cost = 2645;
        let rate = 10000995385;//291
        result = calculate_bfc_to_stable_cost_with_base_point(cost, rate, 10);

        println!("result: {}", result);

    }
}