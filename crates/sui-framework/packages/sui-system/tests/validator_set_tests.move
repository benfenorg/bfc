// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module sui_system::validator_set_tests {
    use sui::balance;
    use sui::coin;
    use sui_system::validator::{Self, Validator, staking_pool_id, rate_vec_map};
    use sui_system::validator_set::{Self, ValidatorSet};
    use sui::test_scenario::{Self, Scenario};
    use sui::address;
    use bfc_system::bars::BARS;
    use bfc_system::baud::BAUD;
    use bfc_system::bbrl::BBRL;
    use bfc_system::bcad::BCAD;
    use bfc_system::beur::BEUR;
    use bfc_system::bgbp::BGBP;
    use bfc_system::bidr::BIDR;
    use bfc_system::binr::BINR;
    use bfc_system::bjpy::BJPY;
    use bfc_system::bkrw::BKRW;
    use bfc_system::bmxn::BMXN;
    use bfc_system::brub::BRUB;
    use bfc_system::bsar::BSAR;
    use bfc_system::btry::BTRY;
    use bfc_system::busd::BUSD;
    use bfc_system::bzar::BZAR;
    use bfc_system::mgg::MGG;
    use sui::test_utils::{Self, assert_eq};
    use sui::vec_map;
    use std::debug;
    use std::ascii::string;

    const MIST_PER_SUI: u64 = 1_000_000_000; // used internally for stakes.

    #[test]
    fun test_validator_set_flow() {
        // Create 4 validators, with stake 100, 200, 300, 400. Only the first validator is an initial validator.
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        let validator1 = create_validator(@0x1, 1, 1, true, ctx);
        let validator2 = create_validator(@0x2, 2, 1, false, ctx);
        let validator3 = create_validator(@0x3, 3, 1, false, ctx);
        let validator4 = create_validator(@0x4, 4, 1, false, ctx);

        // Create a validator set with only the first validator in it.
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert!(validator_set::total_stake(&validator_set) == 100 * MIST_PER_SUI, 0);

        // Add the other 3 validators one by one.
        add_and_activate_validator(
        &mut validator_set,
        validator2,
        scenario
        );
        // Adding validator during the epoch should not affect stake and quorum threshold.
        assert!(validator_set::total_stake(&validator_set) == 100 * MIST_PER_SUI, 0);

        add_and_activate_validator(
        &mut validator_set,
        validator3,
        scenario
        );
        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        {
        let ctx1 = test_scenario::ctx(scenario);
        let stake = validator_set::request_add_stake(
        &mut validator_set,
        @0x1,
        coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1)),
        ctx1,
        );
        transfer::public_transfer(stake, @0x1);
        // Adding stake to existing active validator during the epoch
        // should not change total stake.
        assert!(validator_set::total_stake(&validator_set) == 100 * MIST_PER_SUI, 0);
        };

        add_and_activate_validator(
        &mut validator_set,
        validator4,
        scenario
        );

        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        // Total stake for these should be the starting stake + the 500 staked with validator 1 in addition to the starting stake.
        assert!(validator_set::total_stake(&validator_set) == 1500 * MIST_PER_SUI, 0);

        test_scenario::next_tx(scenario, @0x1);
        {
        let ctx1 = test_scenario::ctx(scenario);

        validator_set::request_remove_validator(
        &mut validator_set,
        ctx1,
        );
        };

        // Total validator candidate count changes, but total stake remains during epoch.
        assert!(validator_set::total_stake(&validator_set) == 1500 * MIST_PER_SUI, 0);
        assert!(validator_set.total_stake() == 1500 * MIST_PER_SUI);
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        // Validator1 is gone. This removes its stake (100) + the 500 staked with it.
        assert!(validator_set::total_stake(&validator_set) == 900 * MIST_PER_SUI, 0);

        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_validator_set_flow_with_stable() {
        // Create 1 validator  with stake 100, which is an initial validator.
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        let validator1 = create_validator(@0x1, 1, 1, true, ctx);

        // Create a validator set with only the first validator in it.
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert!(validator_set::total_stake(&validator_set) == 100 * MIST_PER_SUI, 0);

        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        let staked = {
        let ctx1 = test_scenario::ctx(scenario);
        let stake = validator_set::request_add_stake(
        &mut validator_set,
        @0x1,
        coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1)),
        ctx1,
        );
        transfer::public_transfer(stake, @0x1);
        // Adding stake to existing active validator during the epoch
        // should not change total stake.
        assert!(validator_set::total_stake(&validator_set) == 100 * MIST_PER_SUI, 0);
        //add stable stake
        let new_stake = coin::into_balance(coin::mint_for_testing(300 * MIST_PER_SUI, ctx1));
        validator_set::request_add_stable_stake<BUSD>(&mut validator_set, @0x1, new_stake, ctx1)
        };


        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        {
        let ctx1 = test_scenario::ctx(scenario);
        let (withdraw, bfc) = validator_set::request_withdraw_stable_stake<BUSD>(&mut validator_set, staked, ctx1);
        transfer::public_transfer(coin::from_balance(withdraw, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc, ctx1), @0x1);

        };


        // Total stake for these should be the stable stake + init stake +
        // the 500 staked with validator 1 in addition to the starting stake.(300 + 100 + 500)
        assert!(validator_set::total_stake(&validator_set) == 900 * MIST_PER_SUI, 0);

        test_scenario::next_tx(scenario, @0x1);

        // Total validator candidate count changes, but total stake remains during epoch.
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        // Validator1 is gone. This removes its is 300 stable staked with it.
        assert!(validator_set::total_stake(&validator_set) == 600 * MIST_PER_SUI, 0);

        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = validator::EInvalidCoinType)]
    fun test_validator_set_flow_with_other_stable() {
        // Create 1 validator  with stake 100, which is an initial validator.
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        let validator1 = create_validator(@0x1, 1, 1, true, ctx);

        // Create a validator set with only the first validator in it.
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert!(validator_set::total_stake(&validator_set) == 100 * MIST_PER_SUI, 0);

        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        let (staked1, staked2, staked3, staked4, staked5, staked6, staked7, staked8, staked9, staked10, staked11, staked12, staked13, staked14, staked15, staked16, staked17) = {
        let ctx1 = test_scenario::ctx(scenario);
        let stake = validator_set::request_add_stake(
        &mut validator_set,
        @0x1,
        coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1)),
        ctx1,
        );
        transfer::public_transfer(stake, @0x1);
        // Adding stake to existing active validator during the epoch
        // should not change total stake.
        assert!(validator_set::total_stake(&validator_set) == 100 * MIST_PER_SUI, 1);
        //add stable stake
        let new_stake1 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked1 = validator_set::request_add_stable_stake<BUSD>(&mut validator_set, @0x1, new_stake1, ctx1);
        let new_stake2 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked2 = validator_set::request_add_stable_stake<BARS>(&mut validator_set, @0x1, new_stake2, ctx1);
        let new_stake3 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked3 = validator_set::request_add_stable_stake<BBRL>(&mut validator_set, @0x1, new_stake3, ctx1);
        let new_stake4 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked4 = validator_set::request_add_stable_stake<BAUD>(&mut validator_set, @0x1, new_stake4, ctx1);
        let new_stake = coin::into_balance(coin::mint_for_testing(300 * MIST_PER_SUI, ctx1));
        let staked5 = validator_set::request_add_stable_stake<BJPY>(&mut validator_set, @0x1, new_stake, ctx1);
        let new_stake6 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked6 = validator_set::request_add_stable_stake<BCAD>(&mut validator_set, @0x1, new_stake6, ctx1);
        let new_stake7 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked7 =validator_set::request_add_stable_stake<BEUR>(&mut validator_set, @0x1, new_stake7, ctx1);
        let new_stake8 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked8 = validator_set::request_add_stable_stake<BGBP>(&mut validator_set, @0x1, new_stake8, ctx1);
        let new_stake9 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked9 = validator_set::request_add_stable_stake<BINR>(&mut validator_set, @0x1, new_stake9, ctx1);
        let new_stake10 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked10 = validator_set::request_add_stable_stake<BIDR>(&mut validator_set, @0x1, new_stake10, ctx1);
        let new_stake11 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked11 = validator_set::request_add_stable_stake<BKRW>(&mut validator_set, @0x1, new_stake11, ctx1);
        let new_stake12 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked12 = validator_set::request_add_stable_stake<BMXN>(&mut validator_set, @0x1, new_stake12, ctx1);
        let new_stake13 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked13 = validator_set::request_add_stable_stake<BRUB>(&mut validator_set, @0x1, new_stake13, ctx1);
        let new_stake14 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked14 = validator_set::request_add_stable_stake<BTRY>(&mut validator_set, @0x1, new_stake14, ctx1);
        let new_stake15 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked15 = validator_set::request_add_stable_stake<BZAR>(&mut validator_set, @0x1, new_stake15, ctx1);
        let new_stake16 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked16 = validator_set::request_add_stable_stake<MGG>(&mut validator_set, @0x1, new_stake16, ctx1);
        let new_stake17 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
        let staked17 =validator_set::request_add_stable_stake<BSAR>(&mut validator_set, @0x1, new_stake17, ctx1);
        (staked1, staked2, staked3, staked4, staked5, staked6, staked7, staked8, staked9, staked10, staked11, staked12, staked13, staked14, staked15, staked16, staked17)
        };


        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        {
        let ctx1 = test_scenario::ctx(scenario);
        let (withdraw1, bfc1) = validator_set::request_withdraw_stable_stake<BUSD>(&mut validator_set, staked1, ctx1);
        let (withdraw2, bfc2) = validator_set::request_withdraw_stable_stake<BARS>(&mut validator_set, staked2, ctx1);
        let (withdraw3, bfc3) = validator_set::request_withdraw_stable_stake<BBRL>(&mut validator_set, staked3, ctx1);
        let (withdraw4, bfc4) = validator_set::request_withdraw_stable_stake<BAUD>(&mut validator_set, staked4, ctx1);
        let (withdraw5, bfc5) = validator_set::request_withdraw_stable_stake<BJPY>(&mut validator_set, staked5, ctx1);
        let (withdraw6, bfc6) = validator_set::request_withdraw_stable_stake<BCAD>(&mut validator_set, staked6, ctx1);
        let (withdraw7, bfc7) = validator_set::request_withdraw_stable_stake<BEUR>(&mut validator_set, staked7, ctx1);
        let (withdraw8, bfc8) = validator_set::request_withdraw_stable_stake<BGBP>(&mut validator_set, staked8, ctx1);
        let (withdraw9, bfc9) = validator_set::request_withdraw_stable_stake<BINR>(&mut validator_set, staked9, ctx1);
        let (withdraw10, bfc10) = validator_set::request_withdraw_stable_stake<BIDR>(&mut validator_set, staked10, ctx1);
        let (withdraw11, bfc11) = validator_set::request_withdraw_stable_stake<BKRW>(&mut validator_set, staked11, ctx1);
        let (withdraw12, bfc12) = validator_set::request_withdraw_stable_stake<BMXN>(&mut validator_set, staked12, ctx1);
        let (withdraw13, bfc13) = validator_set::request_withdraw_stable_stake<BRUB>(&mut validator_set, staked13, ctx1);
        let (withdraw14, bfc14) = validator_set::request_withdraw_stable_stake<BTRY>(&mut validator_set, staked14, ctx1);
        let (withdraw15, bfc15) = validator_set::request_withdraw_stable_stake<BZAR>(&mut validator_set, staked15, ctx1);
        let (withdraw16, bfc16) = validator_set::request_withdraw_stable_stake<MGG>(&mut validator_set, staked16, ctx1);
        let (withdraw17, bfc17) = validator_set::request_withdraw_stable_stake<BSAR>(&mut validator_set, staked17, ctx1);
        transfer::public_transfer(coin::from_balance(withdraw1, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw2, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw3, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw4, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw5, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw6, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw7, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw8, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw9, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw10, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw11, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw12, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw13, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw14, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw15, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw16, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(withdraw17, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc1, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc2, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc3, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc4, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc5, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc6, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc7, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc8, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc9, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc10, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc11, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc12, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc13, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc14, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc15, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc16, ctx1), @0x1);
        transfer::public_transfer(coin::from_balance(bfc17, ctx1), @0x1);
        };


        // Total stake for these should be the stable stake + init stake +
        // the 500 staked with validator 1 in addition to the starting stake.(300 + 100 + 500)
        assert!(validator_set::total_stake(&validator_set) == 8900 * MIST_PER_SUI, 2);

        test_scenario::next_tx(scenario, @0x1);

        // Total validator candidate count changes, but total stake remains during epoch.
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        // Validator1 is gone. This removes its is 300 stable staked with it.
        assert!(validator_set::total_stake(&validator_set) == 600 * MIST_PER_SUI, 3);

        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_reference_gas_price_derivation() {
        // Create 5 validators with different stakes and different gas prices.
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        let v1 = create_validator(@0x1, 1, 45, true, ctx);
        let v2 = create_validator(@0x2, 2, 42, false, ctx);
        let v3 = create_validator(@0x3, 3, 40, false, ctx);
        let v4 = create_validator(@0x4, 4, 41, false, ctx);
        let v5 = create_validator(@0x5, 10, 43, false, ctx);
        // Create a validator set with only the first validator in it.
        let mut validator_set = validator_set::new(vector[v1], ctx);

        assert_eq(validator_set::derive_reference_gas_price(&validator_set), 45);

        add_and_activate_validator(&mut validator_set, v2, scenario);
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);

        assert_eq(validator_set::derive_reference_gas_price(&validator_set), 45);

        add_and_activate_validator(
        &mut validator_set,
        v3,
        scenario
        );
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);

        assert_eq(validator_set::derive_reference_gas_price(&validator_set), 42);

        add_and_activate_validator(
        &mut validator_set,
        v4,
        scenario
        );
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);

        assert_eq(validator_set::derive_reference_gas_price(&validator_set), 42);

        add_and_activate_validator(
        &mut validator_set,
        v5,
        scenario
        );
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);

        assert_eq(validator_set::derive_reference_gas_price(&validator_set), 43);

        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = validator_set::EStakingBelowThreshold)]
    fun test_staking_below_threshold() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        let validator1 = create_validator(@0x1, 1, 1, true, ctx);
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert_eq(validator_set::total_stake(&validator_set), 100 * MIST_PER_SUI);
        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        let ctx1 = test_scenario::ctx(scenario);

        let stake = validator_set::request_add_stake(
        &mut validator_set,
        @0x1,
        balance::create_for_testing(MIST_PER_SUI - 1), // 1 MIST lower than the threshold
        ctx1,
        );
        transfer::public_transfer(stake, @0x1);
        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = validator_set::EStakingBelowThreshold)]
    fun test_staking_below_threshold_with_stable() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        let validator1 = create_validator(@0x1, 1, 1, true, ctx);
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert_eq(validator_set::total_stake(&validator_set), 100 * MIST_PER_SUI);
        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        let ctx1 = test_scenario::ctx(scenario);
        let stake = validator_set::request_add_stable_stake<BUSD>(
        &mut validator_set,
        @0x1,
        balance::create_for_testing(MIST_PER_SUI - 1), // 1 MIST lower than the threshold
        ctx1,
        );
        transfer::public_transfer(stake, @0x1);
        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_staking_min_threshold_with_stable() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        let validator1 = create_validator(@0x1, 1, 1, true, ctx);
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert_eq(validator_set::total_stake(&validator_set), 100 * MIST_PER_SUI);
        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        let ctx1 = test_scenario::ctx(scenario);
        let stake = validator_set::request_add_stable_stake<BUSD>(
        &mut validator_set,
        @0x1,
        balance::create_for_testing(MIST_PER_SUI), // min possible stake
        ctx1,
        );
        transfer::public_transfer(stake, @0x1);

        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        assert!(validator_set::total_stake(&validator_set) == 101 * MIST_PER_SUI, 0);

        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = validator_set::EMinJoiningStakeNotReached)]
    fun add_validator_failure_below_min_voting_power() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        // Create 2 validators, with voting power 9_998 and 2
        let validator1 = create_validator_with_initial_stake(@0x1, 1, 9_998, true, scenario.ctx());
        let insufficient_stake = 2; // need at least 3 voting power to join, won't work
        let validator2 = create_validator_with_initial_stake(
        @0x2,
        2,
        insufficient_stake,
        false,
        scenario.ctx(),
        );
        // Create a validator set with only the first validator in it
        let mut validator_set = validator_set::new(vector[validator1], scenario.ctx());
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        skip_to_min_stake_v2_final_thresholds(scenario);
        scenario.next_tx(@0x2);

        // Try to add a validator with less than the min voting power
        validator_set.request_add_validator_candidate(validator2, scenario.ctx());
        validator_set.request_add_validator(scenario.ctx());
        test_utils::destroy(validator_set);
        scenario_val.end();
    }

    // get 10 validators with equal stake
    fun get_10_validators(ctx: &mut TxContext): vector<Validator> {
        vector[
            create_validator_with_initial_stake(@0x1, 1, 1000, true, ctx),
            create_validator_with_initial_stake(@0x2, 2, 1000, true, ctx),
            create_validator_with_initial_stake(@0x3, 3, 1000, true, ctx),
            create_validator_with_initial_stake(@0x4, 4, 1000, true, ctx),
            create_validator_with_initial_stake(@0x5, 5, 1000, true, ctx),
            create_validator_with_initial_stake(@0x6, 6, 1000, true, ctx),
            create_validator_with_initial_stake(@0x7, 7, 1000, true, ctx),
            create_validator_with_initial_stake(@0x8, 8, 1000, true, ctx),
            create_validator_with_initial_stake(@0x9, 9, 1000, true, ctx),
            create_validator_with_initial_stake(@0xA, 10, 1000, true, ctx),
        ]
    }

    // skip to the final values for voting power thresholds
    fun skip_to_min_stake_v2_final_thresholds(scenario: &mut Scenario) {
        let min_stake_v2_phase_length = 14;
        let num_phases = 3;
        let epoch = scenario.ctx().epoch();
        scenario.skip_to_epoch(epoch + min_stake_v2_phase_length * num_phases)
    }

    #[test]
    fun add_validator_with_min_voting_power() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        // Create 2 validators, with stake 9_997 and stake 3
        let validator1 = create_validator_with_initial_stake(@0x1, 1, 9_997, true, scenario.ctx());
        let min_stake = 3; // need at least 3 voting power to join
        let validator2 = create_validator_with_initial_stake(@0x2, 2, min_stake, false, scenario.ctx());
        // Create a validator set with only the first validator in it
        let mut validator_set = validator_set::new(vector[validator1], scenario.ctx());
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        skip_to_min_stake_v2_final_thresholds(scenario);
        scenario.next_tx(@0x2);
        let num_validators = validator_set.active_validators().length();

        let _pool_id_2 = staking_pool_id(&validator2);
        // Try to add a validator with the min voting power. it should work
        validator_set.request_add_validator_candidate(validator2, scenario.ctx());
        assert!(validator_set.is_validator_candidate(@0x2));

        validator_set.request_add_validator(scenario.ctx());
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        assert!(validator_set.is_active_validator(@0x1));
        assert!(validator_set.is_active_validator(@0x2));
        assert!(validator_set.total_stake() == 10_000 * MIST_PER_SUI);
        // epoch change should emit one ValidatorEpochInfoEvent per validator and one ValidatorJoinEvent for the new validator
        let effects = scenario.next_tx(@0xB);
        assert_eq(effects.num_user_events(), num_validators + 1);

        test_utils::destroy(validator_set);
        scenario_val.end();
    }


    #[test]
    fun add_candidate_then_remove() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = scenario.ctx();

        // Create 2 validators, with stake 100 and 200.
        let validator1 = create_validator(@0x1, 1, 1, true, ctx);
        let validator2 = create_validator(@0x2, 2, 1, false, ctx);

        let pool_id_2 = staking_pool_id(&validator2);

        // Create a validator set with only the first validator in it.
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert_eq(validator_set.total_stake(), 100 * MIST_PER_SUI);
        scenario_val.end();

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        let ctx1 = scenario.ctx();
        // Add the second one as a candidate.
        validator_set.request_add_validator_candidate(validator2, ctx1);
        assert!(validator_set.is_validator_candidate(@0x2));
        assert_eq(validator_set.validator_address_by_pool_id(&pool_id_2), @0x2);

        scenario.next_tx(@0x2);
        // Then remove its candidacy.
        validator_set.request_remove_validator_candidate(scenario.ctx());
        assert!(!validator_set.is_validator_candidate(@0x2));
        assert!(validator_set.is_inactive_validator(pool_id_2));
        assert_eq(validator_set.validator_address_by_pool_id(&pool_id_2), @0x2);

        test_utils::destroy(validator_set);
        scenario_val.end();
    }

    #[test]
    #[expected_failure(abort_code = validator_set::ENotAValidator)]
    fun request_add_then_pull_stake() {
        // get enough stake to be added, then pull it before the epoch change
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        // Create 2 validators, with stake 9_997 and stake 3
        let validator1 = create_validator_with_initial_stake(@0x1, 1, 9_997, true, scenario.ctx());
        let new_v = create_validator_with_initial_stake(@0xA, 0, 0, false, scenario.ctx());
        // Create a validator set with only the first validator in it
        let mut validator_set = validator_set::new(vector[validator1], scenario.ctx());
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        skip_to_min_stake_v2_final_thresholds(scenario);
        scenario.next_tx(@0xA);

        validator_set.request_add_validator_candidate(new_v, scenario.ctx());
        let stake = validator_set.request_add_stake(
        @0xA,
        balance::create_for_testing(3 * MIST_PER_SUI),
        scenario.ctx(),
        );
        validator_set.request_add_validator(scenario.ctx()); // can be admitted
        let bal = validator_set.request_withdraw_stake(stake, scenario.ctx()); // should fail here with ENotAValidator

        test_utils::destroy(validator_set);
        test_utils::destroy(bal);
        scenario_val.end();
    }

    #[test]
    fun withdraw_all() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        // create 10 validators with equal-ish stake so we don't run up against max voting power limits
        let init_validators = get_10_validators(scenario.ctx());
        let new_v = create_validator_with_initial_stake(@0xB, 11, 0, false, scenario.ctx());
        let mut validator_set = validator_set::new(init_validators, scenario.ctx());
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        skip_to_min_stake_v2_final_thresholds(scenario);
        scenario.next_tx(@0xB);

        validator_set.request_add_validator_candidate(new_v, scenario.ctx());
        let stake = validator_set.request_add_stake(
        @0xB,
        balance::create_for_testing(4 * MIST_PER_SUI),
        scenario.ctx(),
        );
        validator_set.request_add_validator(scenario.ctx()); // can be admitted
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        assert!(validator_set.is_active_validator(@0xB));
        assert!(validator_set.find_for_testing(@0xB).voting_power() == 3);
        let num_validators = validator_set.active_validators().length();
        // withdraw all the stake. validator will now have voting power 0 + must be kicked out immediately
        let bal = validator_set.request_withdraw_stake(stake, scenario.ctx());
        // use "low stake" grace period of 10 epochs to ensure that validator actually did hit the "very low stake" threshold
        advance_epoch_with_low_stake_grace_period(&mut validator_set, 10, scenario);
        assert!(!validator_set.is_active_validator(@0xB));
        // epoch change should emit one ValidatorEpochInfoEvent per validator and one ValidatorLeaveEvent for the departed validator
        let effects = scenario.next_tx(@0xB);
        assert_eq(effects.num_user_events(), num_validators + 1);

        test_utils::destroy(validator_set);
        test_utils::destroy(bal);
        scenario_val.end();
    }

    #[test]
    fun very_low_voting_power_departure() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        // create 10 validators with equal-ish stake so we don't run up against max voting power limits
        let init_validators = get_10_validators(scenario.ctx());
        let new_v = create_validator_with_initial_stake(@0xB, 12, 0, false, scenario.ctx());
        let mut validator_set = validator_set::new(init_validators, scenario.ctx());
        debug::print(&string(b"wubin here voting_power"));
        debug::print(&validator_set);

        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        skip_to_min_stake_v2_final_thresholds(scenario);
        scenario.next_tx(@0xB);

        let grace_period = 3;
        validator_set.request_add_validator_candidate(new_v, scenario.ctx());
        let mut stake = validator_set.request_add_stake(
        @0xB,
        balance::create_for_testing(4 * MIST_PER_SUI),
        scenario.ctx(),
        );
        validator_set.request_add_validator(scenario.ctx()); // can be admitted
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(validator_set.is_active_validator(@0xB));
        assert!(validator_set.find_for_testing(@0xB).voting_power() == 3);
        let num_validators = validator_set.active_validators().length();
        // withdraw most of the stake. validator will now have voting power 1 and will be kicked out immediately
        let bal = validator_set.request_withdraw_stake(
        stake.split(3 * MIST_PER_SUI, scenario.ctx()),
        scenario.ctx(),
        );
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(!validator_set.is_active_validator(@0xB));
        // epoch change should emit one ValidatorEpochInfoEvent per validator and one ValidatorLeaveEvent for the departed validator
        let effects = scenario.next_tx(@0xB);
        assert_eq(effects.num_user_events(), num_validators + 1);

        test_utils::destroy(validator_set);
        test_utils::destroy(bal);
        test_utils::destroy(stake);
        scenario_val.end();
    }

    #[test]
    fun low_voting_power_departure() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        // create 10 validators with equal-ish stake so we don't run up against max voting power limits
        let init_validators = get_10_validators(scenario.ctx());
        let new_v = create_validator_with_initial_stake(@0xB, 11, 0, false, scenario.ctx());
        let mut validator_set = validator_set::new(init_validators, scenario.ctx());
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        skip_to_min_stake_v2_final_thresholds(scenario);
        scenario.next_tx(@0xB);

        let grace_period = 3;
        validator_set.request_add_validator_candidate(new_v, scenario.ctx());
        let mut stake = validator_set.request_add_stake(
        @0xB,
        balance::create_for_testing(4 * MIST_PER_SUI),
        scenario.ctx(),
        );
        validator_set.request_add_validator(scenario.ctx()); // can be admitted
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(validator_set.is_active_validator(@0xB));
        assert!(validator_set.find_for_testing(@0xB).voting_power() == 3);
        let num_validators = validator_set.active_validators().length();
        // withdraw part of the stake. validator will now have voting power 2 and is now at risk
        let bal = validator_set.request_withdraw_stake(
        stake.split(2 * MIST_PER_SUI, scenario.ctx()),
        scenario.ctx(),
        );
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(validator_set.is_active_validator(@0xB));
        assert!(validator_set.find_for_testing(@0xB).voting_power() == 1);
        assert_eq(validator_set.find_for_testing(@0xB).voting_power(), 1);
        assert!(validator_set.is_at_risk_validator(@0xB));

        // ... 1 epoch goes by. still in the grace period
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(validator_set.is_active_validator(@0xB));
        assert!(validator_set.is_at_risk_validator(@0xB));
        assert!(validator_set.find_for_testing(@0xB).voting_power() == 1);
        // ... 2 epochs go by. still in the grace period
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(validator_set.is_active_validator(@0xB));
        assert!(validator_set.is_at_risk_validator(@0xB));
        assert!(validator_set.find_for_testing(@0xB).voting_power() == 1);
        // ... 3 epochs go by, grace period over. validator is kicked out
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(!validator_set.is_active_validator(@0xB));
        // epoch change should emit one ValidatorEpochInfoEvent per validator and one ValidatorLeaveEvent for the departed validator
        let effects = scenario.next_tx(@0xB);
        assert_eq(effects.num_user_events(), num_validators + 1);

        test_utils::destroy(validator_set);
        test_utils::destroy(bal);
        test_utils::destroy(stake);
        scenario_val.end();
    }

    #[test]
    fun low_voting_power_recovery() {
        // a validator drops below the low voting power threshold, then recovers as stake is added back
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        // create 10 validators with equal-ish stake so we don't run up against max voting power limits
        let init_validators = get_10_validators(scenario.ctx());
        let new_v = create_validator_with_initial_stake(@0xB, 12, 0, false, scenario.ctx());
        let mut validator_set = validator_set::new(init_validators, scenario.ctx());
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        skip_to_min_stake_v2_final_thresholds(scenario);
        scenario.next_tx(@0xB);

        let grace_period = 3;
        validator_set.request_add_validator_candidate(new_v, scenario.ctx());
        let mut stake1 = validator_set.request_add_stake(
        @0xB,
        balance::create_for_testing(4 * MIST_PER_SUI),
        scenario.ctx(),
        );
        validator_set.request_add_validator(scenario.ctx()); // can be admitted
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(validator_set.is_active_validator(@0xB));
        assert!(validator_set.find_for_testing(@0xB).voting_power() == 3);
        // withdraw part of the stake. validator will now have voting power 2 and is now at risk
        let bal = validator_set.request_withdraw_stake(
        stake1.split(2 * MIST_PER_SUI, scenario.ctx()),
        scenario.ctx(),
        );
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(validator_set.is_active_validator(@0xB));
        assert!(validator_set.find_for_testing(@0xB).voting_power() == 1);
        assert!(validator_set.is_at_risk_validator(@0xB));

        // add back the stake and get the validator above the threshold. should no longer be at risk
        let stake2 = validator_set.request_add_stake(@0xB, bal, scenario.ctx());
        advance_epoch_with_low_stake_grace_period(&mut validator_set, grace_period, scenario);
        assert!(validator_set.is_active_validator(@0xB));
        assert!(validator_set.find_for_testing(@0xB).voting_power() == 3);
        assert!(!validator_set.is_at_risk_validator(@0xB));

        test_utils::destroy(validator_set);
        test_utils::destroy(stake1);
        test_utils::destroy(stake2);
        scenario_val.end();
    }

    #[test]
    fun add_then_increase_stake_of_others() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        // start with 10 validators that have stake 1000
        let init_validators = get_10_validators(scenario.ctx());
        let new_v = create_validator_with_initial_stake(@0xB, 11, 0, false, scenario.ctx());
        let mut validator_set = validator_set::new(init_validators, scenario.ctx());
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        skip_to_min_stake_v2_final_thresholds(scenario);
        scenario.next_tx(@0xB);
        // add an 11th that has the same stake as the others
        validator_set.request_add_validator_candidate(new_v, scenario.ctx());
        let stake = validator_set.request_add_stake(
        @0xB,
        balance::create_for_testing(1000 * MIST_PER_SUI),
        scenario.ctx(),
        );
        test_utils::destroy(stake);
        validator_set.request_add_validator(scenario.ctx()); // can be admitted

        let mut new_total_stake = validator_set.total_stake();
        let num_validators: u64 = validator_set.active_validators().length();
        // add 10,000,000 to all existing validators
        num_validators.do!(|i| {
        let to_add = 10_000_000 * MIST_PER_SUI;
        let stake = validator_set.request_add_stake(
        address::from_u256((i + 1 as u256)),
        balance::create_for_testing(to_add),
        scenario.ctx(),
        );
        new_total_stake = new_total_stake + to_add;
        test_utils::destroy(stake);
        });

        // advance epoch, new guy should no longer be added
        advance_epoch_with_dummy_rewards(&mut validator_set, scenario);
        assert!(!validator_set.is_active_validator(@0xB));
        assert!(validator_set.active_validators().length() == num_validators);
        assert!(validator_set.total_stake() == new_total_stake);
        // epoch change should emit one ValidatorEpochInfoEvent per validator, but no ValidatorJoinEvent or ValidatorLeaveEvent
        let effects = scenario.next_tx(@0xB);
        assert_eq(effects.num_user_events(), num_validators); // epoch changes hould not emit ValidatorJoinEvent or ValidatorLeaveEvent

        test_utils::destroy(validator_set);
        scenario_val.end();
    }

    fun create_validator(
        addr: address,
        hint: u8,
        gas_price: u64,
        is_initial_validator: bool,
        ctx: &mut TxContext,
    ): Validator {
        let stake_value = hint as u64 * 100 * MIST_PER_SUI;
        let name = hint_to_ascii(hint);
        let validator = validator::new_for_testing(
            addr,
            vector[hint],
            vector[hint],
            vector[hint],
            vector[hint],
            name,
            name,
            name,
            name,
            vector[hint],
            vector[hint],
            vector[hint],
            vector[hint],
            option::some(balance::create_for_testing(stake_value)),
            gas_price,
            0,
            is_initial_validator,
            ctx,
        );
        validator
    }

    fun create_validator_with_initial_stake(
        addr: address,
        hint: u8,
        initial_stake: u64,
        is_initial_validator: bool,
        ctx: &mut TxContext,
    ): Validator {
        let name = hint_to_ascii(hint);
        let validator = validator::new_for_testing(
            addr,
            vector[hint],
            vector[hint],
            vector[hint],
            vector[hint],
            copy name,
            copy name,
            copy name,
            name,
            vector[hint],
            vector[hint],
            vector[hint],
            vector[hint],
            if (initial_stake != 0) {
                option::some(balance::create_for_testing(initial_stake * MIST_PER_SUI))
            } else { option::none() },
            1,
            0,
            is_initial_validator,
            ctx,
        );
        validator
    }


    fun hint_to_ascii(hint: u8): vector<u8> {
        let ascii_bytes = vector[hint / 100 + 65, hint % 100 / 10 + 65, hint % 10 + 65];
        ascii_bytes.to_ascii_string().into_bytes()
    }

    fun advance_epoch_with_dummy_rewards(validator_set: &mut ValidatorSet, scenario: &mut Scenario) {
        advance_epoch_with_low_stake_grace_period(validator_set, 0, scenario);
    }

    fun advance_epoch_with_low_stake_grace_period(
        validator_set: &mut ValidatorSet,
        low_stake_grace_period: u64,
        scenario: &mut Scenario,
    ) {
        scenario.next_epoch(@0x0);
        let mut dummy_computation_reward = balance::zero();
        let mut dummy_storage_fund_reward = balance::zero();
        let rate_map = rate_vec_map();

        validator_set.advance_epoch(
        &mut dummy_computation_reward,
        &mut dummy_storage_fund_reward,
        &mut vec_map::empty(),
        0, // reward_slashing_rate
        low_stake_grace_period,
        rate_map,
        scenario.ctx(),
        );

        dummy_computation_reward.destroy_zero();
        dummy_storage_fund_reward.destroy_zero();
    }


    fun add_and_activate_validator(validator_set: &mut ValidatorSet, validator: Validator, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, validator::sui_address(&validator));
        let ctx = test_scenario::ctx(scenario);
        validator_set::request_add_validator_candidate(validator_set, validator, ctx);
        validator_set::request_add_validator(validator_set, ctx);
    }
}
