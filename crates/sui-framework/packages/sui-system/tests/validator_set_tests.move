// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module sui_system::validator_set_tests {
    use sui::balance;
    use sui::coin;
    use sui_system::staking_pool::StakedBfc;
    use sui_system::validator::{Self, Validator, staking_pool_id, rate_vec_map};
    use sui_system::validator_set::{Self, ValidatorSet, active_validator_addresses};
    use sui::test_scenario::{Self, Scenario};
    use std::ascii;
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
    use sui_system::stable_pool::StakedStable;

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
            let (withdraw,bfc) = validator_set::request_withdraw_stable_stake<BUSD>(&mut validator_set, staked, ctx1);
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
        assert!(validator_set.total_stake() == 900 * MIST_PER_SUI);

        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
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
        let  (staked1, staked2, staked3, staked4, staked5, staked6, staked7, staked8, staked9, staked10, staked11, staked12, staked13, staked14, staked15, staked16, staked17) = {
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
            let staked2 =validator_set::request_add_stable_stake<BARS>(&mut validator_set, @0x1, new_stake2, ctx1);
            let new_stake3 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked3 =validator_set::request_add_stable_stake<BBRL>(&mut validator_set, @0x1, new_stake3, ctx1);
            let new_stake4 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked4 =validator_set::request_add_stable_stake<BAUD>(&mut validator_set, @0x1, new_stake4, ctx1);
            let new_stake = coin::into_balance(coin::mint_for_testing(300 * MIST_PER_SUI, ctx1));
            let staked5 = validator_set::request_add_stable_stake<BJPY>(&mut validator_set, @0x1, new_stake, ctx1);
            let new_stake6 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked6 =validator_set::request_add_stable_stake<BCAD>(&mut validator_set, @0x1, new_stake6, ctx1);
            let new_stake7 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked7 =validator_set::request_add_stable_stake<BEUR>(&mut validator_set, @0x1, new_stake7, ctx1);
            let new_stake8 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked8 =validator_set::request_add_stable_stake<BGBP>(&mut validator_set, @0x1, new_stake8, ctx1);
            let new_stake9 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked9 =validator_set::request_add_stable_stake<BINR>(&mut validator_set, @0x1, new_stake9, ctx1);
            let new_stake10 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked10 =validator_set::request_add_stable_stake<BIDR>(&mut validator_set, @0x1, new_stake10, ctx1);
            let new_stake11 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked11 =validator_set::request_add_stable_stake<BKRW>(&mut validator_set, @0x1, new_stake11, ctx1);
            let new_stake12 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked12 =validator_set::request_add_stable_stake<BMXN>(&mut validator_set, @0x1, new_stake12, ctx1);
            let new_stake13 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked13 =validator_set::request_add_stable_stake<BRUB>(&mut validator_set, @0x1, new_stake13, ctx1);
            let new_stake14 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked14 =validator_set::request_add_stable_stake<BTRY>(&mut validator_set, @0x1, new_stake14, ctx1);
            let new_stake15 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked15 =validator_set::request_add_stable_stake<BZAR>(&mut validator_set, @0x1, new_stake15, ctx1);
            let new_stake16 = coin::into_balance(coin::mint_for_testing(500 * MIST_PER_SUI, ctx1));
            let staked16 =validator_set::request_add_stable_stake<MGG>(&mut validator_set, @0x1, new_stake16, ctx1);
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
    fun test_add_validator_failure_below_min_stake() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        // Create 2 validators, with stake 100 and 200.
        let validator1 = create_validator(@0x1, 1, 1, true, ctx);
        let validator2 = create_validator(@0x2, 2, 1, false, ctx);

        // Create a validator set with only the first validator in it.
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert_eq(validator_set::total_stake(&validator_set), 100 * MIST_PER_SUI);
        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        let ctx1 = test_scenario::ctx(scenario);
        validator_set::request_add_validator_candidate(&mut validator_set, validator2, ctx1);

        test_scenario::next_tx(scenario, @0x42);
        {
            let ctx = test_scenario::ctx(scenario);
            let stake = validator_set::request_add_stake(
                &mut validator_set,
                @0x2,
                balance::create_for_testing(500 * MIST_PER_SUI),
                ctx,
            );
            transfer::public_transfer(stake, @0x42);
            // Adding stake to a preactive validator should not change total stake.
            assert_eq(validator_set::total_stake(&validator_set), 100 * MIST_PER_SUI);
        };

        test_scenario::next_tx(scenario, @0x2);
        // Validator 2 now has 700 SUI in stake but that's not enough because we need 701.
        validator_set::request_add_validator(&mut validator_set, 701 * MIST_PER_SUI, test_scenario::ctx(scenario));

        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_add_validator_with_nonzero_min_stake() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        // Create 2 validators, with stake 100 and 200.
        let validator1 = create_validator(@0x1, 1, 1, true, ctx);
        let validator2 = create_validator(@0x2, 2, 1, false, ctx);

        // Create a validator set with only the first validator in it.
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert_eq(validator_set::total_stake(&validator_set), 100 * MIST_PER_SUI);
        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        let ctx1 = test_scenario::ctx(scenario);
        validator_set::request_add_validator_candidate(&mut validator_set, validator2, ctx1);

        test_scenario::next_tx(scenario, @0x42);
        {
            let ctx = test_scenario::ctx(scenario);
            let stake = validator_set::request_add_stake(
                &mut validator_set,
                @0x2,
                balance::create_for_testing(500 * MIST_PER_SUI),
                ctx,
            );
            transfer::public_transfer(stake, @0x42);
            // Adding stake to a preactive validator should not change total stake.
            assert_eq(validator_set::total_stake(&validator_set), 100 * MIST_PER_SUI);
        };

        test_scenario::next_tx(scenario, @0x2);
        // Validator 2 now has 700 SUI in stake and that's just enough.
        validator_set::request_add_validator(&mut validator_set, 700 * MIST_PER_SUI, test_scenario::ctx(scenario));

        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_add_candidate_then_remove() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        // Create 2 validators, with stake 100 and 200.
        let validator1 = create_validator(@0x1, 1, 1, true, ctx);
        let validator2 = create_validator(@0x2, 2, 1, false, ctx);

        let pool_id_2 = staking_pool_id(&validator2);

        // Create a validator set with only the first validator in it.
        let mut validator_set = validator_set::new(vector[validator1], ctx);
        assert_eq(validator_set::total_stake(&validator_set), 100 * MIST_PER_SUI);
        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        let ctx1 = test_scenario::ctx(scenario);
        // Add the second one as a candidate.
        validator_set::request_add_validator_candidate(&mut validator_set, validator2, ctx1);
        assert!(validator_set::is_validator_candidate(&validator_set, @0x2), 0);

        test_scenario::next_tx(scenario, @0x2);
        // Then remove its candidacy.
        validator_set::request_remove_validator_candidate(&mut validator_set, test_scenario::ctx(scenario));
        assert!(!validator_set::is_validator_candidate(&validator_set, @0x2), 0);
        assert!(validator_set::is_inactive_validator(&validator_set, pool_id_2), 0);

        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_low_stake_departure() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        // Create 4 validators.
        let v1 = create_validator(@0x1, 1, 1, true, ctx); // 100 SUI of stake
        let v2 = create_validator(@0x2, 4, 1, true, ctx); // 400 SUI of stake
        let v3 = create_validator(@0x3, 10, 1, true, ctx); // 1000 SUI of stake
        let v4 = create_validator(@0x4, 4, 1, true, ctx); // 400 SUI of stake

        let mut validator_set = validator_set::new(vector[v1, v2, v3, v4], ctx);
        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        assert_eq(active_validator_addresses(&validator_set), vector[@0x1, @0x2, @0x3, @0x4]);

        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );

        // v1 is kicked out because their stake 100 is less than the very low stake threshold
        // which is 200.
        assert_eq(active_validator_addresses(&validator_set), vector[@0x2, @0x3, @0x4]);

        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x2, @0x3, @0x4]);

        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x2, @0x3, @0x4]);

        // Add some stake to @0x4 to get her out of the danger zone.
        test_scenario::next_tx(scenario, @0x42);
        {
            let ctx = test_scenario::ctx(scenario);
            let stake = validator_set::request_add_stake(
                &mut validator_set,
                @0x4,
                balance::create_for_testing(500 * MIST_PER_SUI),
                ctx,
            );
            transfer::public_transfer(stake, @0x42);
        };

        // So only @0x2 will be kicked out.
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3, @0x4]);

        // Withdraw the stake from @0x4.
        test_scenario::next_tx(scenario, @0x42);
        {
            let stake = test_scenario::take_from_sender<StakedBfc>(scenario);
            let ctx = test_scenario::ctx(scenario);
            let withdrawn_balance = validator_set::request_withdraw_stake(
                &mut validator_set,
                stake,
                ctx,
            );
            transfer::public_transfer(coin::from_balance(withdrawn_balance, ctx), @0x42);
        };

        // Now @0x4 gets kicked out after 3 grace days are used at the 4th epoch change.
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3, @0x4]);
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3, @0x4]);
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3, @0x4]);
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        // @0x4 was kicked out.
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3]);
        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_low_stake_departure_stable() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        // Create 4 validators.
        let v1 = create_validator(@0x1, 1, 1, true, ctx); // 100 SUI of stake
        let v2 = create_validator(@0x2, 4, 1, true, ctx); // 400 SUI of stake
        let v3 = create_validator(@0x3, 10, 1, true, ctx); // 1000 SUI of stake
        let v4 = create_validator(@0x4, 4, 1, true, ctx); // 400 SUI of stake

        let mut validator_set = validator_set::new(vector[v1, v2, v3, v4], ctx);
        test_scenario::end(scenario_val);

        let mut scenario_val = test_scenario::begin(@0x1);
        let scenario = &mut scenario_val;
        assert_eq(active_validator_addresses(&validator_set), vector[@0x1, @0x2, @0x3, @0x4]);

        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );

        // v1 is kicked out because their stake 100 is less than the very low stake threshold
        // which is 200.
        assert_eq(active_validator_addresses(&validator_set), vector[@0x2, @0x3, @0x4]);

        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x2, @0x3, @0x4]);

        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x2, @0x3, @0x4]);

        // Add some stake to @0x4 to get her out of the danger zone.
        test_scenario::next_tx(scenario, @0x42);
        {
            let ctx = test_scenario::ctx(scenario);
            let stake = validator_set::request_add_stable_stake<BUSD>(
                &mut validator_set,
                @0x4,
                balance::create_for_testing<BUSD>(500 * MIST_PER_SUI),
                ctx,
            );
            transfer::public_transfer(stake, @0x42);
        };

        // So only @0x2 will be kicked out.
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3, @0x4]);

        // Withdraw the stake from @0x4.
        test_scenario::next_tx(scenario, @0x42);
        {
            let stake = test_scenario::take_from_sender<StakedStable<BUSD>>(scenario);
            let ctx = test_scenario::ctx(scenario);
            let (withdrawn_balance_busd, withdrawn_balance_bfc) = validator_set::request_withdraw_stable_stake<BUSD>(
                &mut validator_set,
                stake,
                ctx,
            );
            transfer::public_transfer(coin::from_balance<BUSD>(withdrawn_balance_busd, ctx), @0x42);
            transfer::public_transfer(coin::from_balance(withdrawn_balance_bfc, ctx), @0x42);
        };

        // Now @0x4 gets kicked out after 3 grace days are used at the 4th epoch change.
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3, @0x4]);
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3, @0x4]);
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3, @0x4]);
        advance_epoch_with_low_stake_params(
            &mut validator_set, 500, 200, 3, scenario
        );
        // @0x4 was kicked out.
        assert_eq(active_validator_addresses(&validator_set), vector[@0x3]);
        test_utils::destroy(validator_set);
        test_scenario::end(scenario_val);
    }

    fun create_validator(addr: address, hint: u8, gas_price: u64, is_initial_validator: bool, ctx: &mut TxContext): Validator {
        let stake_value = (hint as u64) * 100 * MIST_PER_SUI;
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
            option::some(balance::create_for_testing(stake_value)),
            gas_price,
            0,
            is_initial_validator,
            ctx
        );
        validator
    }

    fun hint_to_ascii(hint: u8): vector<u8> {
        let ascii_bytes = vector[hint / 100 + 65, hint % 100 / 10 + 65, hint % 10 + 65];
        ascii::into_bytes(ascii::string(ascii_bytes))
    }

    fun advance_epoch_with_dummy_rewards(validator_set: &mut ValidatorSet, scenario: &mut Scenario) {
        test_scenario::next_epoch(scenario, @0x0);
        let mut dummy_computation_reward = balance::zero();
        let mut dummy_storage_fund_reward = balance::zero();

        let rate_map = rate_vec_map();
        validator_set::advance_epoch(
            validator_set,
            &mut dummy_computation_reward,
            &mut dummy_storage_fund_reward,
            &mut vec_map::empty(),
            0, // reward_slashing_rate
            0, // low_stake_threshold
            0, // very_low_stake_threshold
            0, // low_stake_grace_period
            rate_map, //INIT_STABLE_EXCHANGE_RATE
            test_scenario::ctx(scenario)
        );

        balance::destroy_zero(dummy_computation_reward);
        balance::destroy_zero(dummy_storage_fund_reward);
    }

    fun advance_epoch_with_low_stake_params(
        validator_set: &mut ValidatorSet,
        low_stake_threshold: u64,
        very_low_stake_threshold: u64,
        low_stake_grace_period: u64,
        scenario: &mut Scenario
    ) {
        test_scenario::next_epoch(scenario, @0x0);
        let mut dummy_computation_reward = balance::zero();
        let mut dummy_storage_fund_reward = balance::zero();
        let rate_map = rate_vec_map();
        validator_set::advance_epoch(
            validator_set,
            &mut dummy_computation_reward,
            &mut dummy_storage_fund_reward,
            &mut vec_map::empty(),
            0, // reward_slashing_rate
            low_stake_threshold * MIST_PER_SUI,
            very_low_stake_threshold * MIST_PER_SUI,
            low_stake_grace_period,
            rate_map,
            test_scenario::ctx(scenario)
        );

        balance::destroy_zero(dummy_computation_reward);
        balance::destroy_zero(dummy_storage_fund_reward);
    }

    fun add_and_activate_validator(validator_set: &mut ValidatorSet, validator: Validator, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, validator::sui_address(&validator));
        let ctx = test_scenario::ctx(scenario);
        validator_set::request_add_validator_candidate(validator_set, validator, ctx);
        validator_set::request_add_validator(validator_set, 0, ctx);
    }
}
