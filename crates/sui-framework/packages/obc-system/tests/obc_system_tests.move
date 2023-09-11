#[test_only]
module obc_system::obc_system_tests {

    use sui::object;
    use sui::test_scenario;
    use sui::tx_context::TxContext;
    use sui::clock::{Self, Clock};
    use sui::balance::{Self};
    use sui::coin::{Self};
    use sui::obc::OBC;
    use sui::test_scenario::Scenario;

    use obc_system::obc_system;
    use obc_system::obc_system::ObcSystemState;

    #[test]
    fun test_round() {
        let obc_addr = @0x0;
        let scenario_val = test_scenario::begin(obc_addr);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        create_sui_system_state_for_testing(ctx);
        test_scenario::next_tx(scenario, obc_addr);
        let system_state = test_scenario::take_shared<ObcSystemState>(scenario);

        let clock = clock::create_for_testing(test_scenario::ctx(scenario));
        obc_system::obc_round(&mut system_state, &clock, 0,test_scenario::ctx(scenario));

        test_scenario::return_shared(system_state);
        clock::destroy_for_testing(clock);
        test_scenario::end(scenario_val);
    }


    public fun create_sui_system_state_for_testing(ctx: &mut TxContext) {
        let usd_supply = obc_system::usd::new(ctx);
        let treasury_parameters = obc_system::obc_system_stat_parameter(
            9,
            60,
            10,
            18446744073709551616, // 2 ** 64
            3600 * 4,
            1000,
            2000,
        );
        obc_system::create(
            object::new(ctx),
            usd_supply,
            treasury_parameters,
            ctx,
        );
    }

    fun setup(): Scenario {
        let obc_addr = @0x0;
        let scenario_val = test_scenario::begin(obc_addr);

        create_sui_system_state_for_testing(test_scenario::ctx(&mut scenario_val));
        test_scenario::next_tx(&mut scenario_val, obc_addr);
        scenario_val
    }

    fun tearDown(s: Scenario) {
        test_scenario::end(s);
    }

    #[test]
    fun test_next_epoch_obc_required() {
        let scenario_val = setup();

        let system_state = test_scenario::take_shared<ObcSystemState>(&mut scenario_val);

        let amount = obc_system::next_epoch_obc_required(&system_state);
        // basepoint = 1000 /  position = 9 / timeinterval=4h
        assert!(amount == 1000 * 5 * 6, 100);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = obc_system::treasury::ERR_INSUFFICIENT)]
    fun test_deposit_with_error() {
        let scenario_val = setup();
        let system_state = test_scenario::take_shared<ObcSystemState>(&mut scenario_val);
        let obc = balance::create_for_testing<OBC>(100);
        let current_balance = obc_system::treasury_balance(&system_state);
        assert!(current_balance == 0, 1);
        obc_system::deposit_to_treasury(
            &mut system_state,
            coin::from_balance(obc, test_scenario::ctx(&mut scenario_val)),
        );

        let new_balance = obc_system::treasury_balance(&system_state);
        assert!(new_balance == 0, 1);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_success() {
        let scenario_val = setup();
        let system_state = test_scenario::take_shared<ObcSystemState>(&mut scenario_val);
        let amount = obc_system::next_epoch_obc_required(&system_state);
        let obc = balance::create_for_testing<OBC>(amount);
        let current_balance = obc_system::treasury_balance(&system_state);
        assert!(current_balance == 0, 2);

        obc_system::deposit_to_treasury(
            &mut system_state,
            coin::from_balance(obc, test_scenario::ctx(&mut scenario_val)),
        );

        let new_balance = obc_system::treasury_balance(&system_state);
        assert!(new_balance == amount, 3);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }
}