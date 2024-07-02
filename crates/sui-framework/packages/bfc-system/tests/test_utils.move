#[test_only]
module bfc_system::test_utils {
    use std::debug;
    use std::ascii::string;
    use sui::balance;
    use sui::coin;
    use sui::bfc::BFC;
    use sui::test_scenario::{Self, Scenario};
    use sui::clock;

    use bfc_system::treasury::{Self, Treasury};
    use bfc_system::busd;


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
        let usd_supply = busd::new(ctx);
        let mut t = treasury::create_treasury(time_interval, 500000000_000_000_000,ctx);
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

    public(package) fun setup_without_parameters(
        scenario_val: &mut Scenario,
        owner: address
    )
    {
        setup_with_parameters(
            3600 * 4,
            58333726687135162368,
            50000_000_000_000,
            9,
            60,
            10,
            5,
            1008611,
            scenario_val,
            owner
        );
    }

    public(package) fun setup_with_parameters(
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
            // let bfc = balance::create_for_testing<BFC>(300000000_000_000_000);
            let mut t = test_scenario::take_shared<Treasury>(scenario_val);
            let required_bfc = treasury::bfc_required(&t);
            debug::print(&string(b"require bfc"));
            debug::print(&required_bfc);
            let bfc = balance::create_for_testing<BFC>(required_bfc * 2);
            treasury::deposit(&mut t, coin::from_balance(bfc, test_scenario::ctx(scenario_val)));
            test_scenario::return_shared(t);
        };

        test_scenario::next_tx(scenario_val, owner);
    }

    public(package) fun test_rebalance_first_init(
        scenario_val: &mut Scenario,
    ) {
        let mut t = test_scenario::take_shared<Treasury>(scenario_val);
        treasury::rebalance_internal(&mut t, false, test_scenario::ctx(scenario_val));
        test_scenario::return_shared(t);
    }

    public(package) fun test_rebalance(
        scenario_val: &mut Scenario,
    ) {
        let mut c = clock::create_for_testing(test_scenario::ctx(scenario_val));
        clock::increment_for_testing(&mut c, 3600 * 4 * 1000 + 1000);

        let mut t = test_scenario::take_shared<Treasury>(scenario_val);
        treasury::rebalance(&mut t, 0, false, &c, test_scenario::ctx(scenario_val));

        clock::destroy_for_testing(c);
        test_scenario::return_shared(t);
    }
}