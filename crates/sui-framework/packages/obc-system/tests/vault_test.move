#[test_only]
module obc_system::vault_test {
    use std::ascii::string;
    use std::debug;
    use std::type_name;

    use sui::balance;
    use sui::clock;
    use sui::coin;
    use sui::obc::OBC;
    use sui::test_scenario;
    use sui::test_scenario::Scenario;
    use sui::transfer;
    use sui::tx_context::TxContext;

    use obc_system::treasury;
    use obc_system::treasury::Treasury;
    use obc_system::usd::{Self, USD};
    use obc_system::vault;

    fun setup(
        time_interval: u32,
        initialize_price: u128,
        base_point: u64,
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        max_counter_times: u32,
        ts: u64,
    ): Scenario {
        let owner = @0x0;
        let scenario_val = test_scenario::begin(owner);
        create_treasury_and_init_vault_with_positions(
            time_interval,
            initialize_price,
            base_point,
            position_number,
            tick_spacing,
            spacing_times,
            max_counter_times,
            ts,
            test_scenario::ctx(&mut scenario_val),
        );
        test_scenario::next_tx(&mut scenario_val, owner);

        {
            let obc = balance::create_for_testing<OBC>(30000000000001);
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            treasury::deposit(&mut t, coin::from_balance(obc, test_scenario::ctx(&mut scenario_val)));
            test_scenario::return_shared(t);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        scenario_val
    }

    fun tearDown(s: Scenario) {
        test_scenario::end(s);
    }

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

    #[test]
    public fun test_init_and_rebalance() {
        let scenario = setup(
            3600 * 4,
            1 << 64,
            1000_000000000,
            9,
            60,
            10,
            5,
            1008611
        );
        let c = clock::create_for_testing(test_scenario::ctx(&mut scenario));
        clock::increment_for_testing(&mut c, 3600 * 4 * 1000 + 1000);
        let t = test_scenario::take_shared<Treasury>(&scenario);
        treasury::rebalance(&mut t, &c, test_scenario::ctx(&mut scenario));

        let usd_mut_vault = treasury::borrow_mut_vault<USD>(&mut t, type_name::into_string(type_name::get<USD>()));
        let (balance0, balance1) = vault::balances<USD>(usd_mut_vault);
        debug::print(&string(b"balance0: "));
        debug::print(&balance0);
        debug::print(&string(b"balance1: "));
        debug::print(&balance1);

        let (amount_a, amount_b) = vault::get_position_amounts(usd_mut_vault, 5, true);
        debug::print(&amount_b);
        debug::print(&amount_a);
        assert!(amount_b == 1000_000000000, 100);
        assert!(amount_a == 1000_000000000, 101);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(c);
        tearDown(scenario);
    }
}