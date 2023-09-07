#[test_only]
module obc_system::obc_system_tests {

    use obc_system::obc_system::ObcSystemState;
    use sui::object;
    use sui::test_scenario;
    use obc_system::obc_system;
    use sui::tx_context::TxContext;

    #[test]
    fun test_round() {
        let obc_addr = @0x0;
        let scenario_val = test_scenario::begin(obc_addr);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        create_sui_system_state_for_testing(ctx);
        test_scenario::next_tx(scenario, obc_addr);
        let system_state = test_scenario::take_shared<ObcSystemState>(scenario);
        obc_system::obc_round(&mut system_state, 0, test_scenario::ctx(scenario));

        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }

    public fun create_sui_system_state_for_testing(ctx: &mut TxContext) {
        let usd_supply = obc_system::usd::new(ctx);
        let treasury_parameters = obc_system::obc_system_stat_parameter(
            9,
            60,
            18446744073709551616, // 2 ** 64
            2000,
        );
        obc_system::create(
            object::new(ctx),
            usd_supply,
            treasury_parameters,
            ctx,
        );
    }

}