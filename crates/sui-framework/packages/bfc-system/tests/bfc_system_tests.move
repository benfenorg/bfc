#[test_only]
module bfc_system::bfc_system_tests {

    use bfc_system::treasury;
    use bfc_system::treasury::Treasury;
    use sui::object;
    use sui::test_scenario;
    use sui::tx_context::TxContext;
    use sui::clock::{Self};
    use sui::balance::{Self};
    use sui::coin::{Self};
    use sui::bfc::BFC;
    use sui::test_scenario::Scenario;

    use bfc_system::bfc_system;
    use bfc_system::bfc_system::BfcSystemState;
    use bfc_system::test_utils;


    #[test]
    fun test_round() {
        let bfc_addr = @0x0;
        let scenario_val = test_scenario::begin(bfc_addr);
        test_utils::setup_without_parameters(&mut scenario_val, bfc_addr);
        let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        clock::increment_for_testing(&mut clock, 3600 * 4 * 1000 + 1000);
        let t = test_scenario::take_shared<Treasury>(&scenario_val);
        treasury::rebalance(&mut t, &clock, test_scenario::ctx(&mut scenario_val));

        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        create_sui_system_state_for_testing(ctx);
        test_scenario::next_tx(scenario, bfc_addr);
        let system_state = test_scenario::take_shared<BfcSystemState>(scenario);

        // let clock = clock::create_for_testing(test_scenario::ctx(scenario));
        bfc_system::bfc_round(&mut system_state, &clock, 0,test_scenario::ctx(scenario));

        test_scenario::return_shared(system_state);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(clock);
        test_scenario::end(scenario_val);
    }

    public fun create_sui_system_state_for_testing(ctx: &mut TxContext) {
        let usd_supply = bfc_system::busd::new_for_test(ctx);
        let treasury_parameters = bfc_system::bfc_system_stat_parameter(
            9,
            60,
            10,
            18446744073709551616, // 2 ** 64
            3600 * 4,
            1000,
            4,
            2000,
        );
        bfc_system::create(
            object::bfc_system_state_for_test(),
            usd_supply,
            balance::zero<BFC>(),
            treasury_parameters,
            ctx,
        );
    }

    fun setup(): Scenario {
        let bfc_addr = @0x0;
        let scenario_val = test_scenario::begin(bfc_addr);

        create_sui_system_state_for_testing(test_scenario::ctx(&mut scenario_val));
        test_scenario::next_tx(&mut scenario_val, bfc_addr);
        scenario_val
    }

    fun tearDown(s: Scenario) {
        test_scenario::end(s);
    }

    #[test]
    fun test_next_epoch_bfc_required() {
        let scenario_val = setup();

        let system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let amount = bfc_system::next_epoch_bfc_required(&system_state);
        // basepoint = 1000 /  position = 9 / timeinterval=4h
        assert!(amount == 1000 * 5 * 6, 100);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = bfc_system::treasury::ERR_INSUFFICIENT)]
    fun test_deposit_with_error() {
        let scenario_val = setup();
        let system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let bfc = balance::create_for_testing<BFC>(100);
        let current_balance = bfc_system::treasury_balance(&system_state);
        assert!(current_balance == 0, 1);
        bfc_system::deposit_to_treasury(
            &mut system_state,
            coin::from_balance(bfc, test_scenario::ctx(&mut scenario_val)),
        );

        let new_balance = bfc_system::treasury_balance(&system_state);
        assert!(new_balance == 0, 1);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_success() {
        let scenario_val = setup();
        let system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let amount = bfc_system::next_epoch_bfc_required(&system_state);
        let bfc = balance::create_for_testing<BFC>(amount);
        let current_balance = bfc_system::treasury_balance(&system_state);
        assert!(current_balance == 0, 2);

        bfc_system::deposit_to_treasury(
            &mut system_state,
            coin::from_balance(bfc, test_scenario::ctx(&mut scenario_val)),
        );

        let new_balance = bfc_system::treasury_balance(&system_state);
        assert!(new_balance == amount, 3);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }
}