// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module sui_system::voting_power_tests {
    use sui_system::governance_test_utils as gtu;
    use sui_system::voting_power;
    use sui::test_scenario;
    use sui::test_utils;
    use sui::tx_context::TxContext;
    use std::vector;
    use sui_system::validator::{Self, Validator, rate_vec_map};

    const TOTAL_VOTING_POWER: u64 = 10_000;
    const MAX_VOTING_POWER: u64 = 1_000;
    const MIST_PER_SUI: u64 = 1_000_000_000;

    fun check(stakes: vector<u64>, voting_power: vector<u64>, ctx: &mut TxContext) {
        let validators = gtu::create_validators_with_stakes(stakes, ctx);
        let rate_map = rate_vec_map();
        voting_power::set_voting_power(&mut validators, rate_map);
        test_utils::assert_eq(get_voting_power(&validators), voting_power);
        test_utils::destroy(validators);
    }

    fun check_with_stable(stakes: vector<u64>, stables:vector<u64>, voting_power: vector<u64>, ctx: &mut TxContext) {
        let validators = gtu::create_validators_with_stakes(stakes, ctx);
        gtu::stake_stables(&mut validators, stables, ctx);
        let rate_map = rate_vec_map();
        voting_power::set_voting_power(&mut validators, rate_map);
        test_utils::assert_eq(get_voting_power(&validators), voting_power);
        test_utils::destroy(validators);
    }

    #[test]
    fun test_small_validator_sets() {
        let scenario = test_scenario::begin(@0x0);
        let ctx = test_scenario::ctx(&mut scenario);
        check(vector[1], vector[TOTAL_VOTING_POWER], ctx);
        check(vector[77], vector[TOTAL_VOTING_POWER], ctx);
        check(vector[TOTAL_VOTING_POWER * 93], vector[TOTAL_VOTING_POWER], ctx);
        check(vector[1, 1], vector[5_000, 5_000], ctx);
        check(vector[1, 2], vector[5_000, 5_000], ctx);
        check(vector[1, 1, 1], vector[3_333, 3_333, 3_334], ctx);
        check(vector[1, 1, 2], vector[3_333, 3_333, 3_334], ctx);
        check(vector[1, 1, 1, 1], vector[2_500, 2_500, 2_500, 2_500], ctx);
        check(vector[1, 1, 1, 1, 1, 1], vector[1666, 1666, 1667, 1667, 1667, 1667], ctx);
        check(vector[1, 1, 1, 1, 1, 1, 1], vector[1428, 1428, 1428, 1429, 1429, 1429, 1429], ctx);
        check(vector[1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1111, 1111, 1111, 1111, 1111, 1111, 1111, 1111, 1112], ctx);
        // different stake distributions that all lead to 10 validators, all with max voting power
        check(vector[1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000], ctx);
        check(vector[2, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000], ctx);
        check(vector[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vector[1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000], ctx);
        // This tests the scenario where we have validators whose stakes are only slightly different.
        // Make sure that the order is preserved correctly and the leftover voting power goes to the right validators.
        check(vector[10000, 10001, 10000], vector[3333, 3334, 3333], ctx);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_small_stable_validator_sets() {
        let scenario = test_scenario::begin(@0x0);
        let ctx = test_scenario::ctx(&mut scenario);
        check_with_stable(vector[1],vector[1 * MIST_PER_SUI], vector[TOTAL_VOTING_POWER], ctx);
        check_with_stable(vector[1], vector[10_000 * MIST_PER_SUI],vector[TOTAL_VOTING_POWER], ctx);
        check_with_stable(vector[1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[5_000, 5_000], ctx);
        check_with_stable(vector[1, 1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[3_333, 3_333, 3_334], ctx);
        check_with_stable(vector[1, 1, 2], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[3_333, 3_333, 3_334], ctx);
        check_with_stable(vector[1, 1, 1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[2_500, 2_500, 2_500, 2_500], ctx);
        check_with_stable(vector[1, 1, 1, 1, 1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[1666, 1666, 1667, 1667, 1667, 1667], ctx);
        check_with_stable(vector[1, 1, 1, 1, 1, 1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[1428, 1428, 1428, 1429, 1429, 1429, 1429], ctx);
        check_with_stable(vector[1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[1111, 1111, 1111, 1111, 1111, 1111, 1111, 1111, 1112], ctx);
        check_with_stable(vector[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000], ctx);
        check_with_stable(vector[2, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000, 1_000], ctx);
        check_with_stable(vector[10000, 10001, 10000], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI],vector[3333, 3334, 3333], ctx);
        check_with_stable(vector[10000, 1, 10000], vector[10000 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI],vector[3334, 3332, 3334], ctx);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_medium_validator_sets() {
        let scenario = test_scenario::begin(@0x0);
        let ctx = test_scenario::ctx(&mut scenario);
        // >10 validators. now things get a bit more interesting because we can redistribute stake away from the max validators
        check(vector[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[909, 909, 909, 909, 909, 909, 909, 909, 909, 909, 910], ctx);
        check(vector[2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1000, 900, 900, 900, 900, 900, 900, 900, 900, 900, 900], ctx);
        check(vector[2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1000, 1000, 888, 889, 889, 889, 889, 889, 889, 889, 889], ctx);
        check(vector[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], vector[522, 674, 826, 978, 1000, 1000, 1000, 1000, 1000, 1000, 1000], ctx);

        test_scenario::end(scenario);
    }

    #[test]
    fun test_medium_stable_validator_sets() {
        let scenario = test_scenario::begin(@0x0);
        let ctx = test_scenario::ctx(&mut scenario);
        // >10 validators. now things get a bit more interesting because we can redistribute stake away from the max validators
        check_with_stable(vector[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[909, 909, 909, 909, 909, 909, 909, 909, 909, 909, 910], ctx);
        check_with_stable(vector[2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[1000, 900, 900, 900, 900, 900, 900, 900, 900, 900, 900], ctx);
        check_with_stable(vector[2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[1000, 1000, 888, 889, 889, 889, 889, 889, 889, 889, 889], ctx);
        check_with_stable(vector[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], vector[1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI, 1 * MIST_PER_SUI], vector[555, 685, 815, 945, 1000, 1000, 1000, 1000, 1000, 1000, 1000], ctx);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_medium_validator_sets_2() {
        let scenario = test_scenario::begin(@0x0);
        let ctx = test_scenario::ctx(&mut scenario);

        // more validators, harder to reach max
        check(vector[2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[953, 953, 476, 476, 476, 476, 476, 476, 476, 476, 476, 476, 476, 476, 476, 476, 476, 477, 477], ctx);
        check(vector[4, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vector[1000, 951, 951, 951, 639, 639, 639, 325, 325, 325, 325, 325, 325, 325, 325, 326, 326, 326, 326, 326], ctx);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_medium_stable_validator_sets_2() {
        let scenario = test_scenario::begin(@0x0);
        let ctx = test_scenario::ctx(&mut scenario);
        let stable = 1 * MIST_PER_SUI;
        // more validators, harder to reach max
        check_with_stable(vector[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vector[stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable],
            vector[526, 526, 526, 526, 526, 526, 526, 526, 526, 526, 526, 526, 526, 527, 527, 527, 527, 527, 527], ctx);
        check_with_stable(vector[2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vector[stable * 2,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable,stable, stable],
            vector[889, 667, 667, 667, 444, 444, 444, 444, 444, 444, 444, 444, 444, 444, 445, 445, 445, 445, 445, 445], ctx);
        test_scenario::end(scenario);
    }

    fun get_voting_power(validators: &vector<Validator>): vector<u64> {
        let result = vector[];
        let i = 0;
        let len = vector::length(validators);
        while (i < len) {
            let voting_power = validator::voting_power(vector::borrow(validators, i));
            vector::push_back(&mut result, voting_power);
            i = i + 1;
        };
        result
    }
}
