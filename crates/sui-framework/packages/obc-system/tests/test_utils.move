#[test_only]
module obc_system::test_utils {
    use obc_system::treasury::Treasury;
    use sui::balance;
    use sui::coin;
    use sui::obc::OBC;
    use obc_system::usd;
    use obc_system::treasury;
    use sui::test_scenario;
    use sui::test_scenario::Scenario;
    use sui::transfer;
    use sui::tx_context::TxContext;

    friend obc_system::vault_test;

    #[test_only]
    friend obc_system::obc_system_tests;

    fun create_treasury_and_init_vault_with_positions(
        time_interval: u32,
        initialize_price: u128,
        base_point: u64,
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        max_counter_times: u32,
        ts: u64,
        ctx: &mut TxContext
    ) {
        let usd_supply = usd::new(ctx);
        let t = treasury::create_treasury(time_interval, ctx);
        treasury::init_vault_with_positions(
            &mut t,
            usd_supply,
            initialize_price,
            base_point,
            position_number,
            tick_spacing,
            spacing_times,
            max_counter_times,
            ts,
            ctx
        );
        transfer::public_share_object(t);
    }

    public(friend) fun setup_without_parameters(
        scenario_val: &mut Scenario,
        owner: address
    )
    {
        setup_with_parameters(
            3600 * 4,
            1 << 64,
            1000_000000000,
            9,
            60,
            10,
            5,
            1008611,
            scenario_val,
            owner
        );
    }

    public(friend) fun setup_with_parameters(
        time_interval: u32,
        initialize_price: u128,
        base_point: u64,
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        max_counter_times: u32,
        ts: u64,
        scenario_val: &mut Scenario,
        owner: address
    )
    {
        create_treasury_and_init_vault_with_positions(
            time_interval,
            initialize_price,
            base_point,
            position_number,
            tick_spacing,
            spacing_times,
            max_counter_times,
            ts,
            test_scenario::ctx(scenario_val),
        );
        test_scenario::next_tx(scenario_val, owner);

        {
            let obc = balance::create_for_testing<OBC>(300000000000000000);
            let t = test_scenario::take_shared<Treasury>(scenario_val);
            treasury::deposit(&mut t, coin::from_balance(obc, test_scenario::ctx(scenario_val)));
            test_scenario::return_shared(t);
        };

        test_scenario::next_tx(scenario_val, owner);
    }
}