#[test_only]
#[allow(unused_mut_ref)]
module bfc_system::vault_test {
    use std::ascii::string;
    use std::debug;
    use std::type_name;
    use std::debug::print;
    //use sui::balance;
    use sui::clock;
    //use sui::coin;
    //use sui::bfc::BFC;
    use sui::test_scenario;

    use bfc_system::treasury;
    use bfc_system::treasury::Treasury;
    use bfc_system::busd::{BUSD};
    use bfc_system::vault;
    use bfc_system::test_utils;

    const IS_DEBUG: bool = false;

    #[test]
    public fun test_init_and_rebalance() {
        let owner = @0x0;
        let mut scenario_val = test_scenario::begin(owner);
        test_utils::setup_with_parameters(
            3600 * 4,
            1 << 64,
            50000_000000000,
            9,
            60,
            10,
            5,
            1008611,
            &mut scenario_val,
            owner
        );
        let mut c = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        clock::increment_for_testing(&mut c, 3600 * 4 * 1000 + 1000);
        let mut t = test_scenario::take_shared<Treasury>(&scenario_val);
        treasury::rebalance(&mut t, 0, &c, test_scenario::ctx(&mut scenario_val));

        let usd_mut_vault = treasury::borrow_mut_vault<BUSD>(&mut t, type_name::into_string(type_name::get<BUSD>()));
        let (balance0, balance1) = vault::balances<BUSD>(usd_mut_vault);
        if (IS_DEBUG) {
            debug::print(&string(b"balance0: "));
            debug::print(&balance0);
            debug::print(&string(b"balance1: "));
            debug::print(&balance1);
        };

        let (amount_a, amount_b) = vault::get_position_amounts(usd_mut_vault, 5, true);
        if (IS_DEBUG) {
            debug::print(&amount_b);
            debug::print(&amount_a);
        };
        print(&amount_b);
        assert!(amount_b == 297500000000000, 100);
        assert!(amount_a == 297500000000000, 101);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(c);
        test_scenario::end(scenario_val);
    }

    // #[test]
    // public fun test_rebalance_after_mint0x01() {
    //     let owner = @0x0;
    //     let mut scenario = test_scenario::begin(owner);
    //     test_utils::setup_with_parameters(
    //         3600 * 4,
    //         1 << 64,
    //         1000_000000000,
    //         9,
    //         60,
    //         10,
    //         0,
    //         1008611,
    //         &mut scenario,
    //         owner
    //     );
    //     let mut c = clock::create_for_testing(test_scenario::ctx(&mut scenario));
    //     clock::increment_for_testing(&mut c, 3600 * 4 * 1000 + 1000);
    //     let mut t = test_scenario::take_shared<Treasury>(&scenario);
    //     treasury::rebalance(&mut t, 0, &c, test_scenario::ctx(&mut scenario));
    //     let usd_mut_vault = treasury::borrow_mut_vault<BUSD>(&mut t, type_name::into_string(type_name::get<BUSD>()));
    //     if (IS_DEBUG) {
    //         let (amount_a, amount_b) = vault::get_position_amounts(usd_mut_vault, 5, true);
    //         let liquidity = vault::get_position_liquidity(usd_mut_vault, 5);
    //         let (tick_lower, tick_upper, price_lower, price_upper) = vault::get_position_tick_range_and_price(
    //             usd_mut_vault,
    //             5
    //         );
    //         debug::print(&amount_a);
    //         debug::print(&amount_b);
    //         debug::print(usd_mut_vault);
    //
    //         debug::print(&liquidity);
    //         debug::print(&tick_lower);
    //         debug::print(&tick_upper);
    //         debug::print(&price_lower);
    //         debug::print(&price_upper);
    //     };
    //     assert!(vault::get_vault_state(usd_mut_vault) == 0, 102);
    //
    //     let balance_alice = balance::create_for_testing<BFC>(2_000_000_000);
    //     let alice_coin_to_mint = coin::from_balance(balance_alice, test_scenario::ctx(&mut scenario));
    //
    //     treasury::mint<BUSD>(
    //         &mut t,
    //         alice_coin_to_mint,
    //         &c,
    //         1_000_000_000,
    //         0,
    //         9999999999,
    //         test_scenario::ctx(&mut scenario)
    //     );
    //     clock::increment_for_testing(&mut c, 3600 * 4 * 1000 + 1000);
    //     treasury::rebalance(&mut t, 0, &c, test_scenario::ctx(&mut scenario));
    //
    //     test_scenario::return_shared(t);
    //     clock::destroy_for_testing(c);
    //     test_scenario::end(scenario);
    // }

    #[test]
    public fun test_calculate_swap_result() {
        let owner = @0x0;
        let mut scenario_val = test_scenario::begin(owner);
        test_utils::setup_with_parameters(
            3600 * 4,
            1 << 64,
            5_000_000_000,
            9,
            1,
            2,
            5,
            1008611,
            &mut scenario_val,
            owner
        );
        test_utils::test_rebalance(&mut scenario_val);

        test_scenario::next_tx(&mut scenario_val, owner);

        let t = test_scenario::take_shared<Treasury>(&mut scenario_val);

        test_scenario::next_tx(&mut scenario_val, owner);
        let calcualte_swap_result1 = treasury::calculate_swap_result<BUSD>(
            &t,
            false,
            100_000_000_000,
        );
        let calculate_usd_amount = vault::calculated_swap_result_amount_out(&calcualte_swap_result1);
        if (IS_DEBUG) {
            debug::print(&string(b"calculate_usd_amount.. input bfc"));
            debug::print(&100_000_000_000);
            debug::print(&string(b"calculate_usd_amount.. out usd"));
            debug::print(&calculate_usd_amount);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        let calculate_swap_result2 = treasury::calculate_swap_result<BUSD>(
            &t,
            true,
            100_000_000_000,
        );
        let calculate_bfc_amount = vault::calculated_swap_result_amount_out(&calculate_swap_result2);
        if (IS_DEBUG) {
            debug::print(&string(b"calculate_usd_amount.. input usd"));
            debug::print(&100_000_000_000);
            debug::print(&string(b"calculate_usd_amount.. out bfc"));
            debug::print(&calculate_bfc_amount);
        };

        test_scenario::return_shared(t);
        test_scenario::end(scenario_val);
    }

    #[test]
    public fun test_insufficient_liquidity() {
        let owner = @0x0;
        let mut scenario_val = test_scenario::begin(owner);

        test_utils::setup_with_parameters(
            3600 * 4,
            1 << 64,
            5_000_000_000,
            9,
            1,
            2,
            5,
            1008611,
            &mut scenario_val,
            owner
        );

        // rebalance
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            test_utils::test_rebalance(&mut scenario_val);
        };

        // check info
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let info = treasury::vault_info<BUSD>(&t);
            if (IS_DEBUG) {
                print(&info);
            };
            test_scenario::return_shared(t);
        };

        // check is_exceed
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let res = treasury::calculate_swap_result<BUSD>(
                &t,
                false,
                271_000_000_000,
            );
            let is_exceed = vault::calculated_swap_result_is_exceed(&res);
            assert!(is_exceed, 10001);
            test_scenario::return_shared(t);
        };

        // rebalance
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut c = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::increment_for_testing(&mut c, 3600 * 4 * 3 * 1000 + 1000);

            let mut t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            treasury::rebalance(&mut t, 0, &c, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(c);
            test_scenario::return_shared(t);
        };

        // check info
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let info = treasury::vault_info<BUSD>(&t);
            if (IS_DEBUG) {
                print(&info);
            };
            test_scenario::return_shared(t);
        };

        // check is_exceed
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let res = treasury::calculate_swap_result<BUSD>(
                &t,
                false,
                271_000_000_000,
            );
            let is_exceed = vault::calculated_swap_result_is_exceed(&res);
            assert!(is_exceed, 10002);
            test_scenario::return_shared(t);
        };

        test_scenario::end(scenario_val);
    }

    #[test]
    public fun test_multi_rebalance() {
        let owner = @0x0;
        let mut scenario_val = test_scenario::begin(owner);
        test_utils::setup_with_parameters(
            14400,
            1 << 64,
            50000_000_000_000,
            9,
            1,
            2,
            5,
            1008611,
            &mut scenario_val,
            owner
        );
        test_utils::test_rebalance_first_init(&mut scenario_val);

        test_scenario::next_tx(&mut scenario_val, owner);
        test_utils::test_rebalance(&mut scenario_val);

        test_scenario::next_tx(&mut scenario_val, owner);
        test_utils::test_rebalance(&mut scenario_val);

        test_scenario::next_tx(&mut scenario_val, owner);
        test_utils::test_rebalance(&mut scenario_val);

        test_scenario::next_tx(&mut scenario_val, owner);
        test_utils::test_rebalance(&mut scenario_val);

        test_scenario::next_tx(&mut scenario_val, owner);
        test_utils::test_rebalance(&mut scenario_val);

        test_scenario::next_tx(&mut scenario_val, owner);
        test_utils::test_rebalance(&mut scenario_val);

        test_scenario::end(scenario_val);
    }
}
