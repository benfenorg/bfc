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

    use obc_system::treasury;
    use obc_system::treasury::Treasury;
    use obc_system::usd::{USD};
    use obc_system::vault;
    use obc_system::test_utils;

    const IS_DEBUG: bool = true;

    #[test]
    public fun test_init_and_rebalance() {
        let owner = @0x0;
        let scenario_val = test_scenario::begin(owner);
        test_utils::setup_with_parameters(
            3600 * 4,
            1 << 64,
            1000_000000000,
            9,
            60,
            10,
            5,
            1008611,
            &mut scenario_val,
            owner
        );
        let c = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        clock::increment_for_testing(&mut c, 3600 * 4 * 1000 + 1000);
        let t = test_scenario::take_shared<Treasury>(&scenario_val);
        treasury::rebalance(&mut t, &c, test_scenario::ctx(&mut scenario_val));

        let usd_mut_vault = treasury::borrow_mut_vault<USD>(&mut t, type_name::into_string(type_name::get<USD>()));
        let (balance0, balance1) = vault::balances<USD>(usd_mut_vault);
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
        assert!(amount_b == 1000_000000000, 100);
        assert!(amount_a == 1000_000000000, 101);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(c);
        test_scenario::end(scenario_val);
    }

    #[test]
    public fun test_rebalance_after_mint0x01() {
        let owner = @0x0;
        let scenario = test_scenario::begin(owner);
        test_utils::setup_with_parameters(
            3600 * 4,
            1 << 64,
            1000_000000000,
            9,
            60,
            10,
            0,
            1008611,
            &mut scenario,
            owner
        );
        let c = clock::create_for_testing(test_scenario::ctx(&mut scenario));
        clock::increment_for_testing(&mut c, 3600 * 4 * 1000 + 1000);
        let t = test_scenario::take_shared<Treasury>(&scenario);
        treasury::rebalance(&mut t, &c, test_scenario::ctx(&mut scenario));
        let usd_mut_vault = treasury::borrow_mut_vault<USD>(&mut t, type_name::into_string(type_name::get<USD>()));
        if (IS_DEBUG) {
            let (amount_a, amount_b) = vault::get_position_amounts(usd_mut_vault, 5, true);
            let liquidity = vault::get_position_liquidity(usd_mut_vault, 5);
            let (tick_lower, tick_upper, price_lower, price_upper) = vault::get_position_tick_range_and_price(
                usd_mut_vault,
                5
            );
            debug::print(&amount_a);
            debug::print(&amount_b);
            debug::print(usd_mut_vault);

            debug::print(&liquidity);
            debug::print(&tick_lower);
            debug::print(&tick_upper);
            debug::print(&price_lower);
            debug::print(&price_upper);
        };
        assert!(vault::get_vault_state(usd_mut_vault) == 0, 102);

        let balance_alice = balance::create_for_testing<OBC>(2_000_000_000);
        let alice_coin_to_mint = coin::from_balance(balance_alice, test_scenario::ctx(&mut scenario));

        treasury::mint<USD>(&mut t, alice_coin_to_mint, 1_000_000_000, test_scenario::ctx(&mut scenario));
        clock::increment_for_testing(&mut c, 3600 * 4 * 1000 + 1000);
        treasury::rebalance(&mut t, &c, test_scenario::ctx(&mut scenario));

        test_scenario::return_shared(t);
        clock::destroy_for_testing(c);
        test_scenario::end(scenario);
    }

    #[test]
    public fun test_calculate_swap_result() {
        let owner = @0x0;
        let scenario_val = test_scenario::begin(owner);
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

        let t = test_scenario::take_shared<Treasury>(&mut scenario_val);

        test_scenario::next_tx(&mut scenario_val, owner);
        let calculate_usd_amount = treasury::calculate_swap_result<USD>(
            &t,
            false,
            100_000_000_000,
        );
        if (IS_DEBUG) {
            debug::print(&string(b"calculate_usd_amount.. input obc"));
            debug::print(&100_000_000_000);
            debug::print(&string(b"calculate_usd_amount.. out usd"));
            debug::print(&calculate_usd_amount);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        let calculate_obc_amount = treasury::calculate_swap_result<USD>(
            &t,
            true,
            100_000_000_000,
        );
        if (IS_DEBUG) {
            debug::print(&string(b"calculate_usd_amount.. input usd"));
            debug::print(&100_000_000_000);
            debug::print(&string(b"calculate_usd_amount.. out obc"));
            debug::print(&calculate_obc_amount);
        };

        test_scenario::return_shared(t);
        test_scenario::end(scenario_val);
    }
}