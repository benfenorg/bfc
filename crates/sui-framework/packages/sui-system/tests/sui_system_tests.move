// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// This file contains tests testing functionalities in `sui_system` that are not
// already tested by the other more themed tests such as `stake_tests` or
// `rewards_distribution_tests`.

#[test_only]
module sui_system::sui_system_tests {
    use sui::test_scenario::{Self, Scenario};
    use sui::bfc::BFC;
    use sui::coin::Self;
    use sui_system::governance_test_utils::{add_validator_full_flow, advance_epoch, remove_validator, set_up_sui_system_state, create_sui_system_state_for_testing, stake_with, unstake};

    use sui_system::sui_system::{Self, SuiSystemState};
    use sui_system::sui_system_state_inner;
    use sui_system::validator::{Self, Validator};
    use sui_system::validator_set::{Self,EInvalidCap};
    use sui_system::validator_cap::UnverifiedValidatorOperationCap;
    use sui::vec_set;
    use sui::table;
    use sui::balance;
    use sui::test_utils::{assert_eq, destroy};
    use sui::url;
    use std::string;
    use std::ascii;
    use bfc_system::bars::BARS;
    use bfc_system::bbrl::BBRL;
    use bfc_system::bjpy::BJPY;
    use bfc_system::busd::BUSD;
module sui_system::sui_system_tests;

use std::unit_test::assert_eq;
use sui_system::test_runner;
use sui_system::validator_builder;
use sui_system::validator_cap::UnverifiedValidatorOperationCap;

const MIST_PER_SUI: u64 = 1_000_000_000;

#[test]
// Scenario: perform a series of report and undo report operations on a validator.
// Guarantees that:
// - report records are persisted across epochs.
// - report records are removed when a validator is removed.
// - report records are removed when a validator leaves.
// - duplicate report operations are ignored.
fun report_validator() {
    let mut runner = test_runner::new()
        .validators(vector[
            validator_builder::new().sui_address(@1),
            validator_builder::new().sui_address(@2),
            validator_builder::new().sui_address(@3),
        ])
        .build();

    // Validator 1 reports validator 2
    runner.set_sender(@1).report_validator(@2);
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@2).into_keys(), vector[@1])
    });

        report_helper(@0x1, @0x2, false, scenario);
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1], 0);
        report_helper(@0x3, @0x2, false, scenario);
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1, @0x3], 0);
    // Validator 3 reports validator 2
    runner.set_sender(@3).report_validator(@2);
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@2).into_keys(), vector[@1, @3])
    });

        // Report again and result should stay the same.
        report_helper(@0x1, @0x2, false, scenario);
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1, @0x3], 0);
    // Report again and result should stay the same.
    runner.set_sender(@1).report_validator(@2);
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@2).into_keys(), vector[@1, @3])
    });

        // Undo the report.
        report_helper(@0x3, @0x2, true, scenario);
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1], 0);
    // Undo the report from Validator 3.
    runner.set_sender(@3).undo_report_validator(@2);
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@2).into_keys(), vector[@1])
    });

    runner.advance_epoch(option::none()).destroy_for_testing();

        // After an epoch ends, report records are still present.
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1], 0);
    // After an epoch ends, report records are still present.
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@2).into_keys(), vector[@1])
    });

        report_helper(@0x2, @0x1, false, scenario);
        assert!(get_reporters_of(@0x1, scenario) == vector[@0x2], 0);
    // Validator 2 reports validator 1.
    runner.set_sender(@2).report_validator(@1);
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@1).into_keys(), vector[@2])
    });

    // Validator 3 reports validator 2 again.
    runner.set_sender(@3).report_validator(@2);
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@2).into_keys(), vector[@1, @3])
    });

        report_helper(@0x3, @0x2, false, scenario);
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1, @0x3], 0);
    // After Validator 3 leaves, its reports are gone.
    runner.set_sender(@3).remove_validator();
    runner.advance_epoch(option::none()).destroy_for_testing();
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@2).into_keys(), vector[@1])
    });

        // After 0x3 leaves, its reports are gone
        remove_validator(@0x3, scenario);
        advance_epoch(scenario);
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1], 0);
    // Validator 1 leaves.
    runner.set_sender(@1).remove_validator();
    runner.advance_epoch(option::none()).destroy_for_testing();
    runner.system_tx!(|system, _| {
        assert!(system.get_reporters_of(@1).is_empty());
        assert!(system.get_reporters_of(@2).is_empty());
    });

        // After 0x1 leaves, both its reports and the reports on its name are gone
        remove_validator(@0x1, scenario);
        advance_epoch(scenario);
        assert!(vector::is_empty(&get_reporters_of(@0x1, scenario)), 0);
        assert!(vector::is_empty(&get_reporters_of(@0x2, scenario)), 0);
        test_scenario::end(scenario_val);
    }
    runner.finish();
}

#[random_test]
// Scenario: transfer the validator cap object to different addresses and check
//   that everything works as expected.
//
// TODO: discovered that pending validator does not set the gas price for the
//   first active epoch, but for an epoch after that. Confirm, that this is the
//   expected behavior.
fun report_validator_by_stakee_ok(initial_stake: u8) {
    let initial_stake = initial_stake.max(1) as u64;
    let mut runner = test_runner::new()
        .validators_initial_stake(initial_stake)
        .validators(vector[
            validator_builder::new().sui_address(@1),
            validator_builder::new().sui_address(@2),
        ])
        .build();

    let stakee = @0xbeef;

        // @0x1 transfers the cap object to stakee.
        let stakee_address = @0xbeef;
        test_scenario::next_tx(scenario, @0x1);
        let cap = test_scenario::take_from_sender<UnverifiedValidatorOperationCap>(scenario);
        transfer::public_transfer(cap, stakee_address);
    // @0x1 transfers the cap object to stakee.
    runner.set_sender(@1).owned_tx!<UnverifiedValidatorOperationCap>(|cap| {
        transfer::public_transfer(cap, stakee);
    });

        // With the cap object in hand, stakee could report validators on behalf of @0x1.
        report_helper(stakee_address, @0x2, false, scenario);
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1], 0);
    // With the cap object in hand, stakee could report validators on behalf of @0x1.
    runner.set_sender(stakee).report_validator(@2);
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@2).into_keys(), vector[@1]);
    });

        // stakee could also undo report.
        report_helper(stakee_address, @0x2, true, scenario);
        assert!(vector::is_empty(&get_reporters_of(@0x2, scenario)), 0);
    // stakee could also undo report.
    runner.set_sender(stakee).undo_report_validator(@2);
    runner.system_tx!(|system, _| {
        assert!(system.get_reporters_of(@2).is_empty());
    });

        test_scenario::next_tx(scenario, stakee_address);
        let cap = test_scenario::take_from_sender<UnverifiedValidatorOperationCap>(scenario);
        let new_stakee_address = @0xcafe;
        transfer::public_transfer(cap, new_stakee_address);
    let new_stakee = @0xcafe;

        // New stakee could report validators on behalf of @0x1.
        report_helper(new_stakee_address, @0x2, false, scenario);
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1], 0);
    // transfer the cap object from `stakee` to `new_stakee`.
    runner.set_sender(stakee).owned_tx!<UnverifiedValidatorOperationCap>(|cap| {
        transfer::public_transfer(cap, new_stakee);
    });

    // `new_stakee` could report validators on behalf of @0x1.
    runner.set_sender(new_stakee).report_validator(@2);
    runner.system_tx!(|system, _| {
        assert_eq!(system.get_reporters_of(@2).into_keys(), vector[@1]);
    });

    // `new_stakee` could also set reference gas price on behalf of @0x1.
    runner.set_sender(new_stakee).set_gas_price(666);

    // Add a new pending validator
    runner.set_sender(@0);
    let validator = validator_builder::preset().initial_stake(initial_stake).build(runner.ctx());
    let new_validator = validator.sui_address();
    runner.add_validator_candidate(validator);
    runner.set_sender(new_validator).add_validator();

    // Pending validator could set reference price as well
    runner.set_sender(new_validator).set_gas_price(777);

    // Check that the next epoch gas price is set correctly
    runner.system_tx!(|system, _| {
        assert_eq!(system.active_validator_by_address(@1).next_epoch_gas_price(), 666);
        assert_eq!(system.pending_validator_by_address(new_validator).next_epoch_gas_price(), 777);
    });

    runner.advance_epoch(option::none()).destroy_for_testing();

    // Check that the next epoch gas price is set correctly
    runner.system_tx!(|system, _| {
        assert_eq!(system.active_validator_by_address(@1).gas_price(), 666);
        assert_eq!(system.active_validator_by_address(new_validator).gas_price(), 1);
    });

        test_scenario::next_tx(scenario, new_stakee_address);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let validator = sui_system::active_validator_by_address(&mut system_state, @0x1);
        assert!(validator::next_epoch_gas_price(validator) == 666, 0);
        let pending_validator = sui_system::pending_validator_by_address(&mut system_state, new_validator_addr);
        assert!(validator::next_epoch_gas_price(pending_validator) == 777, 0);
        test_scenario::return_shared(system_state);
    runner.advance_epoch(option::none()).destroy_for_testing();

        test_scenario::end(scenario_val);
    }
    #[test]
    #[expected_failure(abort_code = EInvalidCap)]
    fun test_report_validator_by_stakee_revoked() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        set_up_sui_system_state(vector[@0x1, @0x2]);
    // Pending validator's gas price is only accounted in the next epoch after it becoming active.
    runner.system_tx!(|system, _| {
        assert_eq!(system.active_validator_by_address(new_validator).gas_price(), 777);
    });

    runner.finish();
}

#[test, expected_failure(abort_code = ::sui_system::validator_set::EInvalidCap)]
fun report_validator_by_stakee_revoked() {
    let mut runner = test_runner::new()
        .validators(vector[
            validator_builder::new().sui_address(@1),
            validator_builder::new().sui_address(@2),
        ])
        .build();

        // @0x1 transfers the cap object to stakee.
        let stakee_address = @0xbeef;
        test_scenario::next_tx(scenario, @0x1);
        let cap = test_scenario::take_from_sender<UnverifiedValidatorOperationCap>(scenario);
        transfer::public_transfer(cap, stakee_address);
    // @0x1 transfers the cap object to stakee.
    let stakee = @0xbeef;
    runner.set_sender(@1).owned_tx!<UnverifiedValidatorOperationCap>(|cap| {
        transfer::public_transfer(cap, stakee);
    });

        report_helper(stakee_address, @0x2, false, scenario);
        assert!(get_reporters_of(@0x2, scenario) == vector[@0x1], 0);
    // Confirm the stakee has permission to report validators.
    runner.set_sender(stakee).report_validator(@2);

    // Validator 1 revokes stakee's permission by creating a new cap object.
    runner.set_sender(@1).system_tx!(|system, ctx| {
        system.rotate_operation_cap(ctx);
    });

    // Stakee no longer has permission to report validators, here it aborts.
    runner.set_sender(stakee).undo_report_validator(@2);

        test_scenario::end(scenario_val);
    }
    abort
}

    #[test]
    #[expected_failure(abort_code = EInvalidCap)]
    fun test_set_reference_gas_price_by_stakee_revoked() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        set_up_sui_system_state(vector[@0x1, @0x2]);
#[test, expected_failure(abort_code = ::sui_system::validator_set::EInvalidCap)]
fun set_reference_gas_price_by_stakee_revoked() {
    let mut runner = test_runner::new()
        .validators(vector[
            validator_builder::new().sui_address(@1),
            validator_builder::new().sui_address(@2),
        ])
        .build();

        // @0x1 transfers the cap object to stakee.
        let stakee_address = @0xbeef;
        test_scenario::next_tx(scenario, @0x1);
        let cap = test_scenario::take_from_sender<UnverifiedValidatorOperationCap>(scenario);
        transfer::public_transfer(cap, stakee_address);
    // @0x1 transfers the cap object to stakee.
    let stakee = @0xbeef;
    runner.set_sender(@1).owned_tx!<UnverifiedValidatorOperationCap>(|cap| {
        transfer::public_transfer(cap, stakee);
    });

    // Confirm the stakee has permission to report validators.
    runner.set_sender(stakee).report_validator(@2);

        test_scenario::next_tx(scenario, stakee_address);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let validator = sui_system::active_validator_by_address(&mut system_state, @0x1);
        assert!(validator::next_epoch_gas_price(validator) == 888, 0);
        test_scenario::return_shared(system_state);
    // Validator 1 revokes stakee's permission by creating a new cap object.
    runner.set_sender(@1).system_tx!(|system, ctx| {
        system.rotate_operation_cap(ctx);
    });

    // Stakee no longer has permission to set gas price, here it aborts.
    runner.set_sender(stakee).set_gas_price(888);

        // stakee no longer has permission to report validators, here it aborts.
        set_gas_price_helper(stakee_address, 888, scenario);

        test_scenario::end(scenario_val);
    }
    abort
}

#[test, expected_failure(abort_code = sui_system::validator::EGasPriceHigherThanThreshold)]
fun set_gas_price_failure() {
    let validator = validator_builder::new().sui_address(@1);
    let mut runner = test_runner::new().validators(vector[validator]).build();

    // Fails here since the gas price is too high.
    runner.set_sender(@1).set_gas_price(100_001);

        test_scenario::end(scenario_val);
    }
    abort
}

#[test, expected_failure(abort_code = sui_system::validator::ECommissionRateTooHigh)]
fun set_commission_rate_failure() {
    let validator = validator_builder::new().sui_address(@1);
    let mut runner = test_runner::new().validators(vector[validator]).build();

    // Fails here since the gas price is too high.
    runner.set_sender(@1).system_tx!(|system, ctx| {
        system.request_set_commission_rate(2001, ctx);
    });

    abort
}

        test_scenario::next_tx(scenario, @0x2);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
#[test, expected_failure(abort_code = sui_system::sui_system_state_inner::ENotValidator)]
fun report_non_validator_failure() {
    let validator = validator_builder::new().sui_address(@1);
    let mut runner = test_runner::new().validators(vector[validator]).build();

        // Fails here since the commission rate is too high.
        sui_system::request_set_commission_rate(&mut system_state, 2001, test_scenario::ctx(scenario));
        test_scenario::return_shared(system_state);
    // Report a non-validator.
    runner.set_sender(@1).report_validator(@42);

        test_scenario::end(scenario_val);
    }
    abort
}

#[test, expected_failure(abort_code = sui_system::sui_system_state_inner::EReportRecordNotFound)]
// TODO: the error expected here is not correct. Maybe we could improve this.
fun undo_report_non_validator_failure() {
    let validator = validator_builder::new().sui_address(@1);
    let mut runner = test_runner::new().validators(vector[validator]).build();

    // Undo a report on a non-validator.
    runner.set_sender(@1).undo_report_validator(@42);

        set_up_sui_system_state(vector[@0x1, @0x2, @0x3]);
        report_helper(@0x1, @0x42, false, scenario);
        test_scenario::end(scenario_val);
    }
    abort
}

#[test, expected_failure(abort_code = sui_system::sui_system_state_inner::ECannotReportOneself)]
fun report_self_failure() {
    let validator = validator_builder::new().sui_address(@1);
    let mut runner = test_runner::new().validators(vector[validator]).build();

        set_up_sui_system_state(vector[@0x1, @0x2, @0x3]);
        report_helper(@0x1, @0x1, false, scenario);
        test_scenario::end(scenario_val);
    }
    // Report oneself.
    runner.set_sender(@1).report_validator(@1);

    abort
}

#[test, expected_failure(abort_code = sui_system::sui_system_state_inner::EReportRecordNotFound)]
fun undo_report_failure() {
    let mut runner = test_runner::new()
        .validators(vector[
            validator_builder::new().sui_address(@1),
            validator_builder::new().sui_address(@2),
        ])
        .build();

        set_up_sui_system_state(vector[@0x1, @0x2, @0x3]);
        report_helper(@0x2, @0x1, true, scenario);
        test_scenario::end(scenario_val);
    }
    // Undo a report that doesn't exist.
    runner.set_sender(@1).undo_report_validator(@2);

    abort
}

#[test]
fun validator_address_by_pool_id() {
    let validator = validator_builder::new().sui_address(@1);
    let mut runner = test_runner::new().validators(vector[validator]).build();

    runner.system_tx!(|system, _| {
        let pool_id = system.validator_staking_pool_id(@1);
        assert_eq!(system.validator_address_by_pool_id(&pool_id), @1);
    });

    runner.finish();
}

#[test]
fun staking_pool_mappings() {
    let mut runner = test_runner::new()
        .validators(vector[
            validator_builder::new().sui_address(@1),
            validator_builder::new().sui_address(@2),
            validator_builder::new().sui_address(@3),
            validator_builder::new().sui_address(@4),
        ])
        .build();

    // Check that the pool mappings are correct.
    runner.system_tx!(|system, _| {
        let pool_id_1 = system.validator_staking_pool_id(@1);
        let pool_id_2 = system.validator_staking_pool_id(@2);
        let pool_id_3 = system.validator_staking_pool_id(@3);
        let pool_id_4 = system.validator_staking_pool_id(@4);
        let pool_mappings = system.validator_staking_pool_mappings();

        set_up_sui_system_state(vector[@0x1, @0x2, @0x3, @0x4]);
        test_scenario::next_tx(scenario, @0x1);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let pool_id_1 = sui_system::validator_staking_pool_id(&mut system_state, @0x1);
        let pool_id_2 = sui_system::validator_staking_pool_id(&mut system_state, @0x2);
        let pool_id_3 = sui_system::validator_staking_pool_id(&mut system_state, @0x3);
        let pool_id_4 = sui_system::validator_staking_pool_id(&mut system_state, @0x4);
        let mut pool_mappings = sui_system::validator_staking_pool_mappings(&mut system_state);
        assert_eq(table::length(pool_mappings), 4);
        assert_eq(*table::borrow(pool_mappings, pool_id_1), @0x1);
        assert_eq(*table::borrow(pool_mappings, pool_id_2), @0x2);
        assert_eq(*table::borrow(pool_mappings, pool_id_3), @0x3);
        assert_eq(*table::borrow(pool_mappings, pool_id_4), @0x4);
        test_scenario::return_shared(system_state);
        assert_eq!(pool_mappings.length(), 4);
        assert_eq!(pool_mappings[pool_id_1], @1);
        assert_eq!(pool_mappings[pool_id_2], @2);
        assert_eq!(pool_mappings[pool_id_3], @3);
        assert_eq!(pool_mappings[pool_id_4], @4);
    });

        let new_validator_addr = @0xaf76afe6f866d8426d2be85d6ef0b11f871a251d043b2f11e15563bf418f5a5a;
        test_scenario::next_tx(scenario, new_validator_addr);
        // Seed [0; 32]
        let pubkey = x"99f25ef61f8032b914636460982c5cc6f134ef1ddae76657f2cbfec1ebfc8d097374080df6fcf0dcb8bc4b0d8e0af5d80ebbff2b4c599f54f42d6312dfc314276078c1cc347ebbbec5198be258513f386b930d02c2749a803e2330955ebd1a10";
        // Generated with [fn test_proof_of_possession]
        let pop = x"b01cc86f421beca7ab4cfca87c0799c4d038c199dd399fbec1924d4d4367866dba9e84d514710b91feb65316e4ceef43";
    // Add a new validator.
    runner.set_sender(@0);
    let validator = validator_builder::preset().initial_stake(100).build(runner.ctx());
    let new_validator = validator.sui_address();
    runner.add_validator_candidate(validator);
    runner.set_sender(new_validator).add_validator();
    runner.advance_epoch(option::none()).destroy_for_testing();

    // save this for later.
    let pool_id_1;

        test_scenario::next_tx(scenario, @0x1);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let pool_id_5 = sui_system::validator_staking_pool_id(&mut system_state, new_validator_addr);
        pool_mappings = sui_system::validator_staking_pool_mappings(&mut system_state);
        // Check that the previous mappings didn't change as well.
        assert_eq(table::length(pool_mappings), 5);
        assert_eq(*table::borrow(pool_mappings, pool_id_1), @0x1);
        assert_eq(*table::borrow(pool_mappings, pool_id_2), @0x2);
        assert_eq(*table::borrow(pool_mappings, pool_id_3), @0x3);
        assert_eq(*table::borrow(pool_mappings, pool_id_4), @0x4);
        assert_eq(*table::borrow(pool_mappings, pool_id_5), new_validator_addr);
        test_scenario::return_shared(system_state);
    // Check that the pool mappings are correct.
    runner.system_tx!(|system, _| {
        pool_id_1 = system.validator_staking_pool_id(@1);
        let pool_id_2 = system.validator_staking_pool_id(@2);
        let pool_id_3 = system.validator_staking_pool_id(@3);
        let pool_id_4 = system.validator_staking_pool_id(@4);
        let pool_id_5 = system.validator_staking_pool_id(new_validator);
        let pool_mappings = system.validator_staking_pool_mappings();

        assert_eq!(pool_mappings.length(), 5);
        assert_eq!(pool_mappings[pool_id_1], @1);
        assert_eq!(pool_mappings[pool_id_2], @2);
        assert_eq!(pool_mappings[pool_id_3], @3);
        assert_eq!(pool_mappings[pool_id_4], @4);
        assert_eq!(pool_mappings[pool_id_5], new_validator);
    });

        // Remove one of the original validators.
        remove_validator(@0x1, scenario);
        advance_epoch(scenario);

        test_scenario::next_tx(scenario, @0x1);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        pool_mappings = sui_system::validator_staking_pool_mappings(&mut system_state);
        // Check that the previous mappings didn't change as well.
        assert_eq(table::length(pool_mappings), 4);
        assert_eq(table::contains(pool_mappings, pool_id_1), false);
        assert_eq(*table::borrow(pool_mappings, pool_id_2), @0x2);
        assert_eq(*table::borrow(pool_mappings, pool_id_3), @0x3);
        assert_eq(*table::borrow(pool_mappings, pool_id_4), @0x4);
        assert_eq(*table::borrow(pool_mappings, pool_id_5), new_validator_addr);
        test_scenario::return_shared(system_state);

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_stable_staking_pool_mapping() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        set_up_sui_system_state(vector[@0x1, @0x2, @0x3, @0x4]);
        test_scenario::next_tx(scenario, @0x1);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let pool_id_1 = sui_system::validator_stable_staking_pool_id<BUSD>(&mut system_state, @0x1);
        let pool_id_2 = sui_system::validator_stable_staking_pool_id<BJPY>(&mut system_state, @0x2);
        let pool_id_3 = sui_system::validator_stable_staking_pool_id<BBRL>(&mut system_state, @0x3);
        let pool_id_4 = sui_system::validator_stable_staking_pool_id<BARS>(&mut system_state, @0x4);
        let mut pool_mappings = sui_system::validator_stable_staking_pool_mappings(&mut system_state);
        let len = table::length(pool_mappings);
        assert_eq(len, 17 * 4);
        assert_eq(*table::borrow(pool_mappings, pool_id_1), @0x1);
        assert_eq(*table::borrow(pool_mappings, pool_id_2), @0x2);
        assert_eq(*table::borrow(pool_mappings, pool_id_3), @0x3);
        assert_eq(*table::borrow(pool_mappings, pool_id_4), @0x4);
        test_scenario::return_shared(system_state);

        let new_validator_addr = @0xaf76afe6f866d8426d2be85d6ef0b11f871a251d043b2f11e15563bf418f5a5a;
        test_scenario::next_tx(scenario, new_validator_addr);
        // Seed [0; 32]
        let pubkey = x"99f25ef61f8032b914636460982c5cc6f134ef1ddae76657f2cbfec1ebfc8d097374080df6fcf0dcb8bc4b0d8e0af5d80ebbff2b4c599f54f42d6312dfc314276078c1cc347ebbbec5198be258513f386b930d02c2749a803e2330955ebd1a10";
        // Generated with [fn test_proof_of_possession]
        let pop = x"b01cc86f421beca7ab4cfca87c0799c4d038c199dd399fbec1924d4d4367866dba9e84d514710b91feb65316e4ceef43";

        // Add a validator
        add_validator_full_flow(new_validator_addr, b"name2", b"/ip4/127.0.0.1/udp/82", 100, pubkey, pop, scenario);
        advance_epoch(scenario);

        test_scenario::next_tx(scenario, @0x1);

        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let pool_id_5 = sui_system::validator_stable_staking_pool_id<BUSD>(&mut system_state, new_validator_addr);
        pool_mappings = sui_system::validator_stable_staking_pool_mappings(&mut system_state);
        // Check that the previous mappings didn't change as well.
        assert_eq(table::length(pool_mappings), 5 * 17);
        assert_eq(*table::borrow(pool_mappings, pool_id_1), @0x1);
        assert_eq(*table::borrow(pool_mappings, pool_id_2), @0x2);
        assert_eq(*table::borrow(pool_mappings, pool_id_3), @0x3);
        assert_eq(*table::borrow(pool_mappings, pool_id_4), @0x4);
        assert_eq(*table::borrow(pool_mappings, pool_id_5), new_validator_addr);
        test_scenario::return_shared(system_state);

        // Remove one of the original validators.
        remove_validator(@0x1, scenario);
        advance_epoch(scenario);

        test_scenario::next_tx(scenario, @0x1);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        pool_mappings = sui_system::validator_stable_staking_pool_mappings(&mut system_state);
        // Check that the previous mappings didn't change as well.
        assert_eq(table::length(pool_mappings), 4 * 17);
        assert_eq(table::contains(pool_mappings, pool_id_1), false);
        assert_eq(*table::borrow(pool_mappings, pool_id_2), @0x2);
        assert_eq(*table::borrow(pool_mappings, pool_id_3), @0x3);
        assert_eq(*table::borrow(pool_mappings, pool_id_4), @0x4);
        assert_eq(*table::borrow(pool_mappings, pool_id_5), new_validator_addr);
        test_scenario::return_shared(system_state);

        test_scenario::end(scenario_val);
    }

    fun report_helper(sender: address, reported: address, is_undo: bool, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, sender);

        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let cap = test_scenario::take_from_sender<UnverifiedValidatorOperationCap>(scenario);
        if (is_undo) {
            sui_system::undo_report_validator(&mut system_state, &cap, reported);
        } else {
            sui_system::report_validator(&mut system_state, &cap, reported);
        };
        test_scenario::return_to_sender(scenario, cap);
        test_scenario::return_shared(system_state);
    }

    fun set_gas_price_helper(
        sender: address,
        new_gas_price: u64,
        scenario: &mut Scenario,
    ) {
        test_scenario::next_tx(scenario, sender);
        let cap = test_scenario::take_from_sender<UnverifiedValidatorOperationCap>(scenario);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        sui_system::request_set_gas_price(&mut system_state, &cap, new_gas_price);
        test_scenario::return_to_sender(scenario, cap);
        test_scenario::return_shared(system_state);
    }


    fun rotate_operation_cap(sender: address, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, sender);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let ctx = test_scenario::ctx(scenario);
        sui_system::rotate_operation_cap(&mut system_state, ctx);
        test_scenario::return_shared(system_state);
    }

    fun get_reporters_of(addr: address, scenario: &mut Scenario): vector<address> {
        test_scenario::next_tx(scenario, addr);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let res = vec_set::into_keys(sui_system::get_reporters_of(&mut system_state, addr));
        test_scenario::return_shared(system_state);
        res
    }

    fun update_candidate(
        scenario: &mut Scenario,
        system_state: &mut SuiSystemState,
        name: vector<u8>,
        protocol_pub_key: vector<u8>,
        pop: vector<u8>,
        network_address: vector<u8>,
        p2p_address: vector<u8>,
        commission_rate: u64,
        gas_price: u64,
    ) {
        let ctx = test_scenario::ctx(scenario);
        sui_system::update_validator_name(system_state, name, ctx);
        sui_system::update_validator_description(system_state, b"new_desc", ctx);
        sui_system::update_validator_image_url(system_state, b"new_image_url", ctx);
        sui_system::update_validator_project_url(system_state, b"new_project_url", ctx);
        sui_system::update_candidate_validator_network_address(system_state, network_address, ctx);
        sui_system::update_candidate_validator_p2p_address(system_state, p2p_address, ctx);
        sui_system::update_candidate_validator_primary_address(system_state, b"/ip4/127.0.0.1/udp/80", ctx);
        sui_system::update_candidate_validator_worker_address(system_state, b"/ip4/127.0.0.1/udp/80", ctx);
        sui_system::update_candidate_validator_protocol_pubkey(
            system_state,
            protocol_pub_key,
            pop,
            ctx
        );
        sui_system::update_candidate_validator_worker_pubkey(system_state, vector[68, 55, 206, 25, 199, 14, 169, 53, 68, 92, 142, 136, 174, 149, 54, 215, 101, 63, 249, 206, 197, 98, 233, 80, 60, 12, 183, 32, 216, 88, 103, 25], ctx);
        sui_system::update_candidate_validator_network_pubkey(system_state, vector[32, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62], ctx);

        sui_system::set_candidate_validator_commission_rate(system_state, commission_rate, ctx);
        let cap = test_scenario::take_from_sender<UnverifiedValidatorOperationCap>(scenario);
        sui_system::set_candidate_validator_gas_price(system_state, &cap, gas_price);
        test_scenario::return_to_sender(scenario, cap);
    }


    fun verify_candidate(
        validator: &Validator,
        name: vector<u8>,
        protocol_pub_key: vector<u8>,
        pop: vector<u8>,
        network_address: vector<u8>,
        p2p_address: vector<u8>,
        commission_rate: u64,
        gas_price: u64,

    ) {
        verify_current_epoch_metadata(
            validator,
            name,
            protocol_pub_key,
            pop,
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            network_address,
            p2p_address,
            vector[32, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62],
            vector[68, 55, 206, 25, 199, 14, 169, 53, 68, 92, 142, 136, 174, 149, 54, 215, 101, 63, 249, 206, 197, 98, 233, 80, 60, 12, 183, 32, 216, 88, 103, 25],
        );
        assert!(validator::commission_rate(validator) == commission_rate, 0);
        assert!(validator::gas_price(validator) == gas_price, 0);

    }

    // Note: `pop` MUST be a valid signature using sui_address and protocol_pubkey_bytes.
    // To produce a valid PoP, run [fn test_proof_of_possession].
    fun update_metadata(
        scenario: &mut Scenario,
        system_state: &mut SuiSystemState,
        name: vector<u8>,
        protocol_pub_key: vector<u8>,
        pop: vector<u8>,
        network_address: vector<u8>,
        p2p_address: vector<u8>,
        network_pubkey: vector<u8>,
        worker_pubkey: vector<u8>,
    ) {
        let ctx = test_scenario::ctx(scenario);
        sui_system::update_validator_name(system_state, name, ctx);
        sui_system::update_validator_description(system_state, b"new_desc", ctx);
        sui_system::update_validator_image_url(system_state, b"new_image_url", ctx);
        sui_system::update_validator_project_url(system_state, b"new_project_url", ctx);
        sui_system::update_validator_next_epoch_network_address(system_state, network_address, ctx);
        sui_system::update_validator_next_epoch_p2p_address(system_state, p2p_address, ctx);
        sui_system::update_validator_next_epoch_primary_address(system_state, b"/ip4/168.168.168.168/udp/80", ctx);
        sui_system::update_validator_next_epoch_worker_address(system_state, b"/ip4/168.168.168.168/udp/80", ctx);
        sui_system::update_validator_next_epoch_protocol_pubkey(
            system_state,
            protocol_pub_key,
            pop,
            ctx
        );
        sui_system::update_validator_next_epoch_network_pubkey(system_state, network_pubkey, ctx);
        sui_system::update_validator_next_epoch_worker_pubkey(system_state, worker_pubkey, ctx);
    }

    fun verify_metadata(
        validator: &Validator,
        name: vector<u8>,
        protocol_pub_key: vector<u8>,
        pop: vector<u8>,
        network_address: vector<u8>,
        p2p_address: vector<u8>,
        network_pubkey: vector<u8>,
        worker_pubkey: vector<u8>,
        new_protocol_pub_key: vector<u8>,
        new_pop: vector<u8>,
        new_network_address: vector<u8>,
        new_p2p_address: vector<u8>,
        new_network_pubkey: vector<u8>,
        new_worker_pubkey: vector<u8>,
    ) {
        // Current epoch
        verify_current_epoch_metadata(
            validator,
            name,
            protocol_pub_key,
            pop,
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            network_address,
            p2p_address,
            network_pubkey,
            worker_pubkey,
        );

        // Next epoch
        assert!(validator::next_epoch_network_address(validator) == &option::some(string::from_ascii(ascii::string(new_network_address))), 0);
        assert!(validator::next_epoch_p2p_address(validator) == &option::some(string::from_ascii(ascii::string(new_p2p_address))), 0);
        assert!(validator::next_epoch_primary_address(validator) == &option::some(string::from_ascii(ascii::string(b"/ip4/168.168.168.168/udp/80"))), 0);
        assert!(validator::next_epoch_worker_address(validator) == &option::some(string::from_ascii(ascii::string(b"/ip4/168.168.168.168/udp/80"))), 0);
        assert!(
            validator::next_epoch_protocol_pubkey_bytes(validator) == &option::some(new_protocol_pub_key),
            0
        );
        assert!(
            validator::next_epoch_proof_of_possession(validator) == &option::some(new_pop),
            0
        );
        assert!(
            validator::next_epoch_worker_pubkey_bytes(validator) == &option::some(new_worker_pubkey),
            0
        );
        assert!(
            validator::next_epoch_network_pubkey_bytes(validator) == &option::some(new_network_pubkey),
            0
        );
    }

    fun verify_current_epoch_metadata(
        validator: &Validator,
        name: vector<u8>,
        protocol_pub_key: vector<u8>,
        pop: vector<u8>,
        primary_address: vector<u8>,
        worker_address: vector<u8>,
        network_address: vector<u8>,
        p2p_address: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
    ) {
        // Current epoch
        assert!(validator::name(validator) == &string::from_ascii(ascii::string(name)), 0);
        assert!(validator::description(validator) == &string::from_ascii(ascii::string(b"new_desc")), 0);
        assert!(validator::image_url(validator) == &url::new_unsafe_from_bytes(b"new_image_url"), 0);
        assert!(validator::project_url(validator) == &url::new_unsafe_from_bytes(b"new_project_url"), 0);
        assert!(validator::network_address(validator) == &string::from_ascii(ascii::string(network_address)), 0);
        assert!(validator::p2p_address(validator) == &string::from_ascii(ascii::string(p2p_address)), 0);
        assert!(validator::primary_address(validator) == &string::from_ascii(ascii::string(primary_address)), 0);
        assert!(validator::worker_address(validator) == &string::from_ascii(ascii::string(worker_address)), 0);
        assert!(validator::protocol_pubkey_bytes(validator) == &protocol_pub_key, 0);
        assert!(validator::proof_of_possession(validator) == &pop, 0);
        assert!(validator::worker_pubkey_bytes(validator) == &worker_pubkey_bytes, 0);
        assert!(validator::network_pubkey_bytes(validator) == &network_pubkey_bytes, 0);
    }


    fun verify_metadata_after_advancing_epoch(
        validator: &Validator,
        name: vector<u8>,
        protocol_pub_key: vector<u8>,
        pop: vector<u8>,
        network_address: vector<u8>,
        p2p_address: vector<u8>,
        network_pubkey: vector<u8>,
        worker_pubkey: vector<u8>,
    ) {
        // Current epoch
        verify_current_epoch_metadata(
            validator,
            name,
            protocol_pub_key,
            pop,
            b"/ip4/168.168.168.168/udp/80",
            b"/ip4/168.168.168.168/udp/80",
            network_address,
            p2p_address,
            network_pubkey,
            worker_pubkey,
        );

        // Next epoch
        assert!(option::is_none(validator::next_epoch_network_address(validator)), 0);
        assert!(option::is_none(validator::next_epoch_p2p_address(validator)), 0);
        assert!(option::is_none(validator::next_epoch_primary_address(validator)), 0);
        assert!(option::is_none(validator::next_epoch_worker_address(validator)), 0);
        assert!(option::is_none(validator::next_epoch_protocol_pubkey_bytes(validator)), 0);
        assert!(option::is_none(validator::next_epoch_proof_of_possession(validator)), 0);
        assert!(option::is_none(validator::next_epoch_worker_pubkey_bytes(validator)), 0);
        assert!(option::is_none(validator::next_epoch_network_pubkey_bytes(validator)), 0);
    }

    #[test]
    fun test_active_validator_update_metadata() {
        let validator_addr = @0xaf76afe6f866d8426d2be85d6ef0b11f871a251d043b2f11e15563bf418f5a5a;
        // pubkey generated with protocol key on seed [0; 32]
        let pubkey = x"99f25ef61f8032b914636460982c5cc6f134ef1ddae76657f2cbfec1ebfc8d097374080df6fcf0dcb8bc4b0d8e0af5d80ebbff2b4c599f54f42d6312dfc314276078c1cc347ebbbec5198be258513f386b930d02c2749a803e2330955ebd1a10";
        // pop generated using the protocol key and address with [fn test_proof_of_possession]
        let pop = x"b01cc86f421beca7ab4cfca87c0799c4d038c199dd399fbec1924d4d4367866dba9e84d514710b91feb65316e4ceef43";

        // pubkey generated with protocol key on seed [1; 32]
        let pubkey1 = x"96d19c53f1bee2158c3fcfb5bb2f06d3a8237667529d2d8f0fbb22fe5c3b3e64748420b4103674490476d98530d063271222d2a59b0f7932909cc455a30f00c69380e6885375e94243f7468e9563aad29330aca7ab431927540e9508888f0e1c";
        let pop1 = x"a8a0bcaf04e13565914eb22fa9f27a76f297db04446860ee2b923d10224cedb130b30783fb60b12556e7fc50e5b57a86";

        let new_validator_addr = @0x8e3446145b0c7768839d71840df389ffa3b9742d0baaff326a3d453b595f87d7;
        // pubkey generated with protocol key on seed [2; 32]
        let new_pubkey = x"adf2e2350fe9a58f3fa50777499f20331c4550ab70f6a4fb25a58c61b50b5366107b5c06332e71bb47aa99ce2d5c07fe0dab04b8af71589f0f292c50382eba6ad4c90acb010ab9db7412988b2aba1018aaf840b1390a8b2bee3fde35b4ab7fdf";
        let new_pop = x"926fdb08b2b46d802e3642044f215dcb049e6c17a376a272ffd7dba32739bb995370966698ab235ee172fbd974985cfe";

        // pubkey generated with protocol key on seed [3; 32]
        let new_pubkey1 = x"91b8de031e0b60861c655c8168596d98b065d57f26f287f8c810590b06a636eff13c4055983e95b2f60a4d6ba5484fa4176923d1f7807cc0b222ddf6179c1db099dba0433f098aae82542b3fd27b411d64a0a35aad01b2c07ac67f7d0a1d2c11";
        let new_pop1 = x"b61913eb4dc7ea1d92f174e1a3c6cad3f49ae8de40b13b69046ce072d8d778bfe87e734349c7394fd1543fff0cb6e2d0";

        let mut scenario_val = test_scenario::begin(validator_addr);
        let scenario = &mut scenario_val;

        // Set up SuiSystemState with an active validator
        let mut validators = vector::empty();
        let ctx = test_scenario::ctx(scenario);
        let validator = validator::new_for_testing(
            validator_addr,
            pubkey,
            vector[32, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62],
            vector[68, 55, 206, 25, 199, 14, 169, 53, 68, 92, 142, 136, 174, 149, 54, 215, 101, 63, 249, 206, 197, 98, 233, 80, 60, 12, 183, 32, 216, 88, 103, 25],
            pop,
            b"ValidatorName",
            b"description",
            b"image_url",
            b"project_url",
            b"/ip4/127.0.0.1/tcp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            option::some(balance::create_for_testing<BFC>(100_000_000_000)),
            1,
            0,
            true,
            ctx
        );
        vector::push_back(&mut validators, validator);
        create_sui_system_state_for_testing(validators, 1000, 0, ctx);

        test_scenario::next_tx(scenario, validator_addr);

        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);

        // Test active validator metadata changes
        test_scenario::next_tx(scenario, validator_addr);
        {
            update_metadata(
                scenario,
                &mut system_state,
                b"validator_new_name",
                pubkey1,
                pop1,
                b"/ip4/42.42.42.42/tcp/80",
                b"/ip4/43.43.43.43/udp/80",
                vector[148, 117, 212, 171, 44, 104, 167, 11, 177, 100, 4, 55, 17, 235, 117, 45, 117, 84, 159, 49, 14, 159, 239, 246, 237, 21, 83, 166, 112, 53, 62, 199],
                vector[215, 64, 85, 185, 231, 116, 69, 151, 97, 79, 4, 183, 20, 70, 84, 51, 211, 162, 115, 221, 73, 241, 240, 171, 192, 25, 232, 106, 175, 162, 176, 43],
            );
        };

        test_scenario::next_tx(scenario, validator_addr);
        let validator = sui_system::active_validator_by_address(&mut system_state, validator_addr);
        verify_metadata(
            validator,
            b"validator_new_name",
            pubkey,
            pop,
            b"/ip4/127.0.0.1/tcp/80",
            b"/ip4/127.0.0.1/udp/80",
            vector[32, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62],
            vector[68, 55, 206, 25, 199, 14, 169, 53, 68, 92, 142, 136, 174, 149, 54, 215, 101, 63, 249, 206, 197, 98, 233, 80, 60, 12, 183, 32, 216, 88, 103, 25],
            pubkey1,
            pop1,
            b"/ip4/42.42.42.42/tcp/80",
            b"/ip4/43.43.43.43/udp/80",
            vector[148, 117, 212, 171, 44, 104, 167, 11, 177, 100, 4, 55, 17, 235, 117, 45, 117, 84, 159, 49, 14, 159, 239, 246, 237, 21, 83, 166, 112, 53, 62, 199],
            vector[215, 64, 85, 185, 231, 116, 69, 151, 97, 79, 4, 183, 20, 70, 84, 51, 211, 162, 115, 221, 73, 241, 240, 171, 192, 25, 232, 106, 175, 162, 176, 43],
        );

        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);

        // Test pending validator metadata changes
        let mut scenario_val = test_scenario::begin(new_validator_addr);
        let scenario = &mut scenario_val;
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        test_scenario::next_tx(scenario, new_validator_addr);
        {
            let ctx = test_scenario::ctx(scenario);
            sui_system::request_add_validator_candidate(
                &mut system_state,
                new_pubkey,
                vector[33, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62],
                vector[69, 55, 206, 25, 199, 14, 169, 53, 68, 92, 142, 136, 174, 149, 54, 215, 101, 63, 249, 206, 197, 98, 233, 80, 60, 12, 183, 32, 216, 88, 103, 25],
                new_pop,
                b"ValidatorName2",
                b"description2",
                b"image_url2",
                b"project_url2",
                b"/ip4/127.0.0.2/tcp/80",
                b"/ip4/127.0.0.2/udp/80",
                b"/ip4/127.0.0.1/udp/80",
                b"/ip4/127.0.0.1/udp/80",
                1,
                0,
                ctx,
            );
            sui_system::request_add_validator_for_testing(&mut system_state, 0, ctx);
        };

        test_scenario::next_tx(scenario, new_validator_addr);
        {
            update_metadata(
                scenario,
                &mut system_state,
                b"new_validator_new_name",
                new_pubkey1,
                new_pop1,
                b"/ip4/66.66.66.66/tcp/80",
                b"/ip4/77.77.77.77/udp/80",
                vector[215, 65, 85, 185, 231, 116, 69, 151, 97, 79, 4, 183, 20, 70, 84, 51, 211, 162, 115, 221, 73, 241, 240, 171, 192, 25, 232, 106, 175, 162, 176, 43],
                vector[149, 117, 212, 171, 44, 104, 167, 11, 177, 100, 4, 55, 17, 235, 117, 45, 117, 84, 159, 49, 14, 159, 239, 246, 237, 21, 83, 166, 112, 53, 62, 199],
            );
        };

        test_scenario::next_tx(scenario, new_validator_addr);
        let validator = sui_system::pending_validator_by_address(&mut system_state, new_validator_addr);
        verify_metadata(
            validator,
            b"new_validator_new_name",
            new_pubkey,
            new_pop,
            b"/ip4/127.0.0.2/tcp/80",
            b"/ip4/127.0.0.2/udp/80",
            vector[33, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62],
            vector[69, 55, 206, 25, 199, 14, 169, 53, 68, 92, 142, 136, 174, 149, 54, 215, 101, 63, 249, 206, 197, 98, 233, 80, 60, 12, 183, 32, 216, 88, 103, 25],
            new_pubkey1,
            new_pop1,
            b"/ip4/66.66.66.66/tcp/80",
            b"/ip4/77.77.77.77/udp/80",
            vector[215, 65, 85, 185, 231, 116, 69, 151, 97, 79, 4, 183, 20, 70, 84, 51, 211, 162, 115, 221, 73, 241, 240, 171, 192, 25, 232, 106, 175, 162, 176, 43],
            vector[149, 117, 212, 171, 44, 104, 167, 11, 177, 100, 4, 55, 17, 235, 117, 45, 117, 84, 159, 49, 14, 159, 239, 246, 237, 21, 83, 166, 112, 53, 62, 199],
        );

        test_scenario::return_shared(system_state);

        // Advance epoch to effectuate the metadata changes.
        test_scenario::next_tx(scenario, new_validator_addr);
        advance_epoch(scenario);

        // Now both validators are active, verify their metadata.
        test_scenario::next_tx(scenario, new_validator_addr);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let validator = sui_system::active_validator_by_address(&mut system_state, validator_addr);
        verify_metadata_after_advancing_epoch(
            validator,
            b"validator_new_name",
            pubkey1,
            pop1,
            b"/ip4/42.42.42.42/tcp/80",
            b"/ip4/43.43.43.43/udp/80",
            vector[148, 117, 212, 171, 44, 104, 167, 11, 177, 100, 4, 55, 17, 235, 117, 45, 117, 84, 159, 49, 14, 159, 239, 246, 237, 21, 83, 166, 112, 53, 62, 199],
            vector[215, 64, 85, 185, 231, 116, 69, 151, 97, 79, 4, 183, 20, 70, 84, 51, 211, 162, 115, 221, 73, 241, 240, 171, 192, 25, 232, 106, 175, 162, 176, 43],
        );

        let validator = sui_system::active_validator_by_address(&mut system_state, new_validator_addr);
        verify_metadata_after_advancing_epoch(
            validator,
            b"new_validator_new_name",
            new_pubkey1,
            new_pop1,
            b"/ip4/66.66.66.66/tcp/80",
            b"/ip4/77.77.77.77/udp/80",
            vector[215, 65, 85, 185, 231, 116, 69, 151, 97, 79, 4, 183, 20, 70, 84, 51, 211, 162, 115, 221, 73, 241, 240, 171, 192, 25, 232, 106, 175, 162, 176, 43],
            vector[149, 117, 212, 171, 44, 104, 167, 11, 177, 100, 4, 55, 17, 235, 117, 45, 117, 84, 159, 49, 14, 159, 239, 246, 237, 21, 83, 166, 112, 53, 62, 199],
        );

        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }


    #[test]
    fun test_validator_candidate_update() {
        let validator_addr = @0xaf76afe6f866d8426d2be85d6ef0b11f871a251d043b2f11e15563bf418f5a5a;
        // pubkey generated with protocol key on seed [0; 32]
        let pubkey = x"99f25ef61f8032b914636460982c5cc6f134ef1ddae76657f2cbfec1ebfc8d097374080df6fcf0dcb8bc4b0d8e0af5d80ebbff2b4c599f54f42d6312dfc314276078c1cc347ebbbec5198be258513f386b930d02c2749a803e2330955ebd1a10";
        // pop generated using the protocol key and address with [fn test_proof_of_possession]
        let pop = x"b01cc86f421beca7ab4cfca87c0799c4d038c199dd399fbec1924d4d4367866dba9e84d514710b91feb65316e4ceef43";

        // pubkey generated with protocol key on seed [1; 32]
        let pubkey1 = x"96d19c53f1bee2158c3fcfb5bb2f06d3a8237667529d2d8f0fbb22fe5c3b3e64748420b4103674490476d98530d063271222d2a59b0f7932909cc455a30f00c69380e6885375e94243f7468e9563aad29330aca7ab431927540e9508888f0e1c";
        let pop1 = x"a8a0bcaf04e13565914eb22fa9f27a76f297db04446860ee2b923d10224cedb130b30783fb60b12556e7fc50e5b57a86";

        let mut scenario_val = test_scenario::begin(validator_addr);
        let scenario = &mut scenario_val;

        set_up_sui_system_state(vector[@0x1, @0x2, @0x3]);
        test_scenario::next_tx(scenario, validator_addr);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        test_scenario::next_tx(scenario, validator_addr);
        {
            sui_system::request_add_validator_candidate_for_testing(
                &mut system_state,
                pubkey,
                vector[215, 64, 85, 185, 231, 116, 69, 151, 97, 79, 4, 183, 20, 70, 84, 51, 211, 162, 115, 221, 73, 241, 240, 171, 192, 25, 232, 106, 175, 162, 176, 43],
                vector[148, 117, 212, 171, 44, 104, 167, 11, 177, 100, 4, 55, 17, 235, 117, 45, 117, 84, 159, 49, 14, 159, 239, 246, 237, 21, 83, 166, 112, 53, 62, 199],
                pop,
                b"ValidatorName2",
                b"description2",
                b"image_url2",
                b"project_url2",
                b"/ip4/127.0.0.2/tcp/80",
                b"/ip4/127.0.0.2/udp/80",
                b"/ip4/168.168.168.168/udp/80",
                b"/ip4/168.168.168.168/udp/80",
                1,
                0,
                test_scenario::ctx(scenario),
            );
        };

        test_scenario::next_tx(scenario, validator_addr);
        update_candidate(
            scenario,
            &mut system_state,
            b"validator_new_name",
            pubkey1,
            pop1,
            b"/ip4/42.42.42.42/tcp/80",
            b"/ip4/43.43.43.43/udp/80",
            42,
            7,
        );

        test_scenario::next_tx(scenario, validator_addr);

        let validator = sui_system::candidate_validator_by_address(&mut system_state, validator_addr);
        verify_candidate(
            validator,
            b"validator_new_name",
            pubkey1,
            pop1,
            b"/ip4/42.42.42.42/tcp/80",
            b"/ip4/43.43.43.43/udp/80",
            42,
            7,
        );



        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = validator::EMetadataInvalidWorkerPubkey)]
    fun test_add_validator_candidate_failure_invalid_metadata() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        // Generated using [fn test_proof_of_possession]
        let new_validator_addr = @0x8e3446145b0c7768839d71840df389ffa3b9742d0baaff326a3d453b595f87d7;
        let pubkey = x"99f25ef61f8032b914636460982c5cc6f134ef1ddae76657f2cbfec1ebfc8d097374080df6fcf0dcb8bc4b0d8e0af5d80ebbff2b4c599f54f42d6312dfc314276078c1cc347ebbbec5198be258513f386b930d02c2749a803e2330955ebd1a10";
        let pop = x"83809369ce6572be211512d85621a075ee6a8da57fbb2d867d05e6a395e71f10e4e957796944d68a051381eb91720fba";

        set_up_sui_system_state(vector[@0x1, @0x2, @0x3]);
        test_scenario::next_tx(scenario, new_validator_addr);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        sui_system::request_add_validator_candidate(
            &mut system_state,
            pubkey,
            vector[32, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62],
            vector[42], // invalid
            pop,
            b"ValidatorName2",
            b"description2",
            b"image_url2",
            b"project_url2",
            b"/ip4/127.0.0.2/tcp/80",
            b"/ip4/127.0.0.2/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            1,
            0,
            test_scenario::ctx(scenario),
        );
        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }
    // Remove one of the original validators.
    runner.set_sender(@1).remove_validator();
    runner.advance_epoch(option::none()).destroy_for_testing();

    // Check pool mappings one last time. Validator 1 is expected to be removed.
    runner.system_tx!(|system, _| {
        let pool_id_2 = system.validator_staking_pool_id(@2);
        let pool_id_3 = system.validator_staking_pool_id(@3);
        let pool_id_4 = system.validator_staking_pool_id(@4);
        let pool_id_5 = system.validator_staking_pool_id(new_validator);
        let pool_mappings = system.validator_staking_pool_mappings();

        set_up_sui_system_state(vector[@0x1, @0x2, @0x3]);
        test_scenario::next_tx(scenario, new_validator_addr);
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        sui_system::request_add_validator_candidate(
            &mut system_state,
            pubkey,
            vector[32, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62],
            vector[68, 55, 206, 25, 199, 14, 169, 53, 68, 92, 142, 136, 174, 149, 54, 215, 101, 63, 249, 206, 197, 98, 233, 80, 60, 12, 183, 32, 216, 88, 103, 25],
            pop,
            b"ValidatorName2",
            b"description2",
            b"image_url2",
            b"project_url2",
            b"/ip4/127.0.0.2/tcp/80",
            b"/ip4/127.0.0.2/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            1,
            0,
            test_scenario::ctx(scenario),
        );
        assert!(!pool_mappings.contains(pool_id_1));
        assert_eq!(pool_mappings.length(), 4);
        assert_eq!(pool_mappings[pool_id_2], @2);
        assert_eq!(pool_mappings[pool_id_3], @3);
        assert_eq!(pool_mappings[pool_id_4], @4);
        assert_eq!(pool_mappings[pool_id_5], new_validator);
    });

        // Add the same address as candidate again, should fail this time.
        sui_system::request_add_validator_candidate(
            &mut system_state,
            pubkey,
            vector[32, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62],
            vector[68, 55, 206, 25, 199, 14, 169, 53, 68, 92, 142, 136, 174, 149, 54, 215, 101, 63, 249, 206, 197, 98, 233, 80, 60, 12, 183, 32, 216, 88, 103, 25],
            pop,
            b"ValidatorName2",
            b"description2",
            b"image_url2",
            b"project_url2",
            b"/ip4/127.0.0.2/tcp/80",
            b"/ip4/127.0.0.2/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            1,
            0,
            test_scenario::ctx(scenario),
        );
        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }
    runner.finish();
}

#[random_test]
/// Check that the stake subsidy distribution counter is incremented correctly,
/// depending on the configured epoch duration and the timestamp of the epoch start.
///
/// The test is parameterized by the epoch duration, which is chosen randomly.
fun skip_stake_subsidy(epoch_duration: u16) {
    let epoch_duration = epoch_duration as u64;
    let mut runner = test_runner::new()
        .epoch_duration(epoch_duration)
        .validators(vector[
            validator_builder::new().sui_address(@1),
            validator_builder::new().sui_address(@2),
        ])
        .build();

        // Set up SuiSystemState with an active validator
        let ctx = test_scenario::ctx(scenario);
        let validator = validator::new_for_testing(
            validator_addr,
            pubkey,
            vector[32, 219, 38, 23, 242, 109, 116, 235, 225, 192, 219, 45, 40, 124, 162, 25, 33, 68, 52, 41, 123, 9, 98, 11, 184, 150, 214, 62, 60, 210, 121, 62],
            vector[68, 55, 206, 25, 199, 14, 169, 53, 68, 92, 142, 136, 174, 149, 54, 215, 101, 63, 249, 206, 197, 98, 233, 80, 60, 12, 183, 32, 216, 88, 103, 25],
            pop,
            b"ValidatorName",
            b"description",
            b"image_url",
            b"project_url",
            b"/ip4/127.0.0.1/tcp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            option::some(balance::create_for_testing<BFC>(100_000_000_000)),
            1,
            0,
            true,
            ctx
        );
        create_sui_system_state_for_testing(vector[validator], 1000, 0, ctx);
    // Advance epoch with the epoch duration timestamp.
    // Expect the counter to be incremented.
    let time = epoch_duration;
    let opts = runner.advance_epoch_opts().epoch_start_time(time);
    runner.advance_epoch(option::some(opts)).destroy_for_testing();
    runner.system_tx!(|system, _| {
        let counter = system.get_stake_subsidy_distribution_counter();
        assert_eq!(counter, 1);
    });

        test_scenario::next_tx(scenario, new_addr);
    // Advance epoch with the epoch duration slightly less than the timestamp.
    // Expect the counter to not be incremented.
    let time = time + epoch_duration - 1;
    let opts = runner.advance_epoch_opts().epoch_start_time(time);
    runner.advance_epoch(option::some(opts)).destroy_for_testing();
    runner.system_tx!(|system, _| {
        let counter = system.get_stake_subsidy_distribution_counter();
        assert_eq!(counter, 1);
    });

        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
    // Advance epoch with the full epoch duration.
    // Expect the counter to be incremented.
    let time = time + epoch_duration;
    let opts = runner.advance_epoch_opts().epoch_start_time(time);
    runner.advance_epoch(option::some(opts)).destroy_for_testing();
    runner.system_tx!(|system, _| {
        let counter = system.get_stake_subsidy_distribution_counter();
        assert_eq!(counter, 2);
    });

        // Add a candidate with the same name. Fails due to duplicating with an already active validator.
        sui_system::request_add_validator_candidate(
            &mut system_state,
            new_pubkey,
            vector[115, 220, 238, 151, 134, 159, 173, 41, 80, 2, 66, 196, 61, 17, 191, 76, 103, 39, 246, 127, 171, 85, 19, 235, 210, 106, 97, 97, 116, 48, 244, 191],
            vector[149, 128, 161, 13, 11, 183, 96, 45, 89, 20, 188, 205, 26, 127, 147, 254, 184, 229, 184, 102, 64, 170, 104, 29, 191, 171, 91, 99, 58, 178, 41, 156],
            new_pop,
            // same name
            b"ValidatorName",
            b"description2",
            b"image_url2",
            b"project_url2",
            b"/ip4/127.0.0.2/tcp/80",
            b"/ip4/127.0.0.2/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            b"/ip4/127.0.0.1/udp/80",
            1,
            0,
            test_scenario::ctx(scenario),
        );
        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }
    runner.finish();
}

    #[test]
    fun test_skip_stake_subsidy() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        // Epoch duration is set to be 42 here.
        set_up_sui_system_state(vector[@0x1, @0x2]);

        // If the epoch length is less than 42 then the stake subsidy distribution counter should not be incremented. Otherwise it should.
        advance_epoch_and_check_distribution_counter(scenario, 42, true);
        advance_epoch_and_check_distribution_counter(scenario, 32, false);
        advance_epoch_and_check_distribution_counter(scenario, 52, true);
        test_scenario::end(scenario_val);
    }

    fun advance_epoch_and_check_distribution_counter(scenario: &mut Scenario, epoch_length: u64, should_increment_counter: bool) {
        test_scenario::next_tx(scenario, @0x0);
        let new_epoch = tx_context::epoch(test_scenario::ctx(scenario)) + 1;
        let mut system_state = test_scenario::take_shared<SuiSystemState>(scenario);
        let prev_epoch_time = sui_system::epoch_start_timestamp_ms(&mut system_state);
        let prev_counter = sui_system::get_stake_subsidy_distribution_counter(&mut system_state);

        let rebate = sui_system::advance_epoch_for_testing(
            &mut system_state, new_epoch, 1, 0, 0, 0, 0, 0, 0, prev_epoch_time + epoch_length, test_scenario::ctx(scenario),
        );
        destroy(rebate);
        assert_eq(sui_system::get_stake_subsidy_distribution_counter(&mut system_state), prev_counter + (if (should_increment_counter) 1 else 0));
        test_scenario::return_shared(system_state);
        test_scenario::next_epoch(scenario, @0x0);
    }

    #[test]
    fun test_withdraw_inactive_stake() {
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        // Epoch duration is set to be 42 here.
        set_up_sui_system_state(vector[@0x1, @0x2]);

        {
            scenario.next_tx(@0x0);
            let mut system_state = scenario.take_shared<SuiSystemState>();
            let staking_pool = system_state.active_validator_by_address(@0x1).get_staking_pool_ref();

            assert!(staking_pool.pending_stake_amount() == 0, 0);
            assert!(staking_pool.pending_stake_withdraw_amount() == 0, 0);
            assert!(staking_pool.sui_balance() == 100 * 1_000_000_000, 0);

            test_scenario::return_shared(system_state);
        };

        stake_with(@0x0, @0x1, 1, scenario);

        {
            scenario.next_tx(@0x0);
            let mut system_state = scenario.take_shared<SuiSystemState>();
            let staking_pool = system_state.active_validator_by_address(@0x1).get_staking_pool_ref();
#[random_test]
// Stake random amount of SUI and check that the pending and withdraw amounts are correct.
fun withdraw_inactive_stake(stake: u16) {
    let stake_amount = stake as u64;
    let validator = validator_builder::new().sui_address(@1).initial_stake(100);
    let mut runner = test_runner::new().validators(vector[validator]).build();

    // Check initial staking values.
    runner.system_tx!(|system, _| {
        let pool = system.active_validator_by_address(@1).get_staking_pool_ref();
        assert_eq!(pool.pending_stake_amount(), 0);
        assert_eq!(pool.pending_stake_withdraw_amount(), 0);
        assert_eq!(pool.sui_balance(), 100 * MIST_PER_SUI);
    });

    // Stake 1 SUI.
    runner.set_sender(@5).stake_with(@1, stake_amount);

    // Check that pending stake amount is 1 SUI.
    runner.system_tx!(|system, _| {
        let pool = system.active_validator_by_address(@1).get_staking_pool_ref();
        assert_eq!(pool.pending_stake_amount(), stake_amount * MIST_PER_SUI);
        assert_eq!(pool.pending_stake_withdraw_amount(), 0);
        assert_eq!(pool.sui_balance(), 100 * MIST_PER_SUI);
    });

    // Unstake before activation epoch.
    runner.set_sender(@5).unstake(0);

    // Check that pending stake amount is 0.
    runner.system_tx!(|system, _| {
        let pool = system.active_validator_by_address(@1).get_staking_pool_ref();
        assert_eq!(pool.pending_stake_amount(), 0);
        assert_eq!(pool.pending_stake_withdraw_amount(), 0);
        assert_eq!(pool.sui_balance(), 100 * MIST_PER_SUI);
    });

    runner.finish();
}

#[random_test]
// Stake random amount of SUI and check that the pending stake amount is correct.
// Convert to fungible staked SUI and redeem it.
// Check that the stake amount is correct.
fun convert_to_fungible_staked_sui_and_redeem(stake: u16) {
    let stake_amount = stake as u64;
    let validator = validator_builder::new().sui_address(@1).initial_stake(100);
    let mut runner = test_runner::new().validators(vector[validator]).build();

    // Check initial stake values.
    runner.system_tx!(|system, _| {
        let pool = system.active_validator_by_address(@1).get_staking_pool_ref();
        assert_eq!(pool.pending_stake_amount(), 0);
        assert_eq!(pool.pending_stake_withdraw_amount(), 0);
        assert_eq!(pool.sui_balance(), 100 * MIST_PER_SUI);
    });

    let staked_sui = runner.set_sender(@5).stake_with_and_take(@1, stake_amount);

    assert_eq!(staked_sui.amount(), stake_amount * MIST_PER_SUI);

    // Stake is now active. Check that the stake amount is correct.
    runner.advance_epoch(option::none()).destroy_for_testing();
    runner.system_tx!(|system, _| {
        let pool = system.active_validator_by_address(@1).get_staking_pool_ref();
        assert_eq!(pool.pending_stake_amount(), 0);
        assert_eq!(pool.pending_stake_withdraw_amount(), 0);
        assert_eq!(pool.sui_balance(), (100 + stake_amount) * MIST_PER_SUI);
    });

    // Convert to fungible staked SUI.
    let fungible_staked_sui;
    runner.system_tx!(|system, ctx| {
        fungible_staked_sui = system.convert_to_fungible_staked_sui(staked_sui, ctx);
    });

    assert_eq!(fungible_staked_sui.value(), stake_amount * MIST_PER_SUI);

    let sui;
    runner.system_tx!(|system, ctx| {
        sui = system.redeem_fungible_staked_sui(fungible_staked_sui, ctx);
    });

    assert_eq!(sui.destroy_for_testing(), stake_amount * MIST_PER_SUI);

    runner.advance_epoch(option::none()).destroy_for_testing();
    runner.system_tx!(|system, _| {
        let pool = system.active_validator_by_address(@1).get_staking_pool_ref();
        assert_eq!(pool.pending_stake_amount(), 0);
        assert_eq!(pool.pending_stake_withdraw_amount(), 0);
        assert_eq!(pool.sui_balance(), 100 * MIST_PER_SUI);
    });

        let sui = system_state.redeem_fungible_staked_sui(
            fungible_staked_sui,
            scenario.ctx()
        );

        assert!(sui.value() == 100_000_000_000, 0);

        test_scenario::return_shared(system_state);

        advance_epoch(scenario);

        sui::test_utils::destroy(sui);
        scenario_val.end();
    }

    #[test]
    fun test_request_add_stable_stake(){
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        // Epoch duration is set to be 42 here.
        set_up_sui_system_state(vector[@0x1, @0x2]);

         {
            scenario.next_tx(@0x0);
            let mut system_state = scenario.take_shared<SuiSystemState>();
            let staking_pool = system_state.active_validator_by_address(@0x1).get_stable_pool_ref<BUSD>();

            assert!(staking_pool.pending_stake_amount() == 0, 0);
            assert!(staking_pool.pending_stake_withdraw_amount() == 0, 0);
            assert!(staking_pool.stable_balance() == 0, 0);
            test_scenario::return_shared(system_state);
        };

        scenario.next_tx(@0x0);

        let mut system_state = scenario.take_shared<SuiSystemState>();

        let staked = system_state.request_add_stable_stake_non_entry<BUSD>(
            coin::mint_for_testing(100_000_000_000, scenario.ctx()),
            @0x1,
            scenario.ctx()
        );
        transfer::public_transfer(staked, @0x3);
        test_scenario::return_shared(system_state);


        scenario.next_tx(@0x1);
        let mut system_state = scenario.take_shared<SuiSystemState>();
        {
            let staking_pool = system_state.active_validator_by_address(@0x1).get_stable_pool_ref<BUSD>();
            assert!(staking_pool.pending_stake_amount() == 100_000_000_000, 0);
            test_scenario::return_shared(system_state);
            advance_epoch(scenario);
        };

        scenario.next_tx(@0x1);
        let mut system_state = scenario.take_shared<SuiSystemState>();
        {
             let staking_pool = system_state.active_validator_by_address(@0x1).get_stable_pool_ref<BUSD>();
            assert!(staking_pool.stable_balance()== 100_000_000_000, 0);
            assert!(staking_pool.pending_stake_amount()== 0, 0);
        };
        test_scenario::return_shared(system_state);
        scenario_val.end();
    }

    #[test]
    #[expected_failure(abort_code = validator::EInvalidCoinType)]
   fun test_request_add_other_stable_stake(){
        let mut scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        // Epoch duration is set to be 42 here.
        set_up_sui_system_state(vector[@0x1, @0x2]);

         {
            scenario.next_tx(@0x0);
            let mut system_state = scenario.take_shared<SuiSystemState>();
            let staking_pool = system_state.active_validator_by_address(@0x1).get_stable_pool_ref<BUSD>();

            assert!(staking_pool.pending_stake_amount() == 0, 0);
            assert!(staking_pool.pending_stake_withdraw_amount() == 0, 0);
            assert!(staking_pool.stable_balance() == 0, 0);
            test_scenario::return_shared(system_state);
        };

        scenario.next_tx(@0x0);

        let mut system_state = scenario.take_shared<SuiSystemState>();

        let staked = system_state.request_add_stable_stake_non_entry<BJPY>(
            coin::mint_for_testing(100_000_000_000, scenario.ctx()),
            @0x1,
            scenario.ctx()
        );
        transfer::public_transfer(staked, @0x3);
        test_scenario::return_shared(system_state);
        scenario_val.end();
    }
    runner.finish();
}
