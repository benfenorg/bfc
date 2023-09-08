#[test_only]
module obc_system::obc_system_tests {

    use sui::object;
    use sui::test_scenario;
    use sui::tx_context::TxContext;
    use sui::clock::{Self, Clock};

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
        obc_system::obc_round(&mut system_state, 0, &clock, test_scenario::ctx(scenario));

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

    #[test]
    fun test_next_epoch_obc_required() {
        let obc_addr = @0x0;
        let scenario_val = test_scenario::begin(obc_addr);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        create_sui_system_state_for_testing(ctx);
        test_scenario::next_tx(scenario, obc_addr);
        let system_state = test_scenario::take_shared<ObcSystemState>(scenario);

        let amount = obc_system::next_epoch_obc_required(&system_state);
        // basepoint = 1000 /  position = 9 / timeinterval=4h
        assert!(amount == 1000*5*6, 100);

        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }
}