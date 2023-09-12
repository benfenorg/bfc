#[test_only]
module obc_system::treasury_test {
    use sui::test_scenario::{Self};
    use std::debug;
    use std::vector;
    use std::ascii::string;
    use sui::coin::{Self, Coin};
    use sui::balance;
    use sui::obc::OBC;
    use sui::clock;
    use sui::transfer;
    use obc_system::treasury::{Self, Treasury};
    use obc_system::vault;
    use obc_system::usd::{Self, USD};

    const IS_DEBUG: bool = true;

    #[test]
    public fun test_treasury() {
        let owner = @0x0;
        let scenario_val = test_scenario::begin(owner);

        //create treasury
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = treasury::create_treasury(
                3600 * 4,
                test_scenario::ctx(&mut scenario_val),
            );
            transfer::public_share_object(t);
        };

        // check info
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            assert!(treasury::index(&t) == 0, 0);
            assert!(treasury::get_balance(&t) == 0, 1);
            test_scenario::return_shared(t);
        };

        let position_number = 3;
        let tick_spacing = 60;
        let spacing_times = 10;
        let initialize_price = 18446744073709551616; //2 ** 64
        let base_point = 1_000_000_000_000; // 1000 obc
        let max_counter_times = 5;

        // create vault
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::increment_for_testing(&mut clock, 360000);
            let usd_supply = usd::new(test_scenario::ctx(&mut scenario_val));
            treasury::create_vault<USD>(
                &mut t,
                usd_supply,
                position_number,
                tick_spacing,
                spacing_times,
                initialize_price,
                base_point,
                max_counter_times,
                clock::timestamp_ms(&clock),
                test_scenario::ctx(&mut scenario_val),
            );
            clock::destroy_for_testing(clock);
            test_scenario::return_shared(t);
        };

        // init positions
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let usd_vault_key = treasury::get_vault_key<USD>();
            let mut_vault = treasury::borrow_mut_vault<USD>(&mut t, usd_vault_key);
            let ticks = vault::init_positions<USD>(
                mut_vault,
                spacing_times,
                test_scenario::ctx(&mut scenario_val),
            );
            if (IS_DEBUG) {
                debug::print(&string(b"get ticks"));
                debug::print(&ticks);
            };
            assert!(vector::length(&ticks) == (position_number as u64), 100);
            test_scenario::return_shared(t);
        };

        // vault info
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let usd_vault_key = treasury::get_vault_key<USD>();
            let usd_vault = treasury::borrow_vault<USD>(&t, usd_vault_key);
            let (amount_a, amount_b) = vault::get_position_amounts<USD>(
                usd_vault,
                1,
                true,
            );
            assert!(amount_a == 0, 101);
            assert!(amount_b == 0, 102);

            let (balance_a, balance_b) = vault::balances<USD>(
                usd_vault,
            );
            assert!(balance_a == 0, 103);
            assert!(balance_b == 0, 104);

            test_scenario::return_shared(t);
        };

        // add liquidity
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let position_index = 2;
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let usd_vault_key = treasury::get_vault_key<USD>();
            let usd_mut_vault = treasury::borrow_mut_vault<USD>(&mut t, usd_vault_key);
            // obc-amount = base-point ;  price = 1
            let liquidity = 67171249877264u128;
            let balance_a = balance::create_for_testing<USD>(base_point);
            let balance_b = balance::create_for_testing<OBC>(base_point);

            let receipt = vault::add_liquidity<USD>(
                usd_mut_vault,
                position_index,
                liquidity,
            );
            // repay
            vault::repay_add_liquidity<USD>(
                usd_mut_vault,
                balance_a,
                balance_b,
                receipt,
            );

            let usd_vault = treasury::borrow_vault<USD>(&t, usd_vault_key);
            let (amount_a, amount_b) = vault::balances<USD>(usd_vault);
            if (IS_DEBUG) {
                debug::print(&string(b"get balance after add-l"));
                debug::print(&amount_a);
                debug::print(&amount_b);
            };
            assert!(amount_a == base_point, 103);
            assert!(amount_b == base_point, 104);

            let (amount_a, amount_b) = vault::get_position_amounts<USD>(
                usd_vault,
                position_index,
                true,
            );
            assert!(amount_a == base_point, 105);
            assert!(amount_b == base_point, 106);

            test_scenario::return_shared(t);
        };

        // check price before swap
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let position_index = 2;
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let usd_vault_key = treasury::get_vault_key<USD>();

            let usd_vault = treasury::borrow_vault<USD>(&t, usd_vault_key);
            let current_sqrt_price = vault::vault_current_sqrt_price(usd_vault);
            let l = vault::get_position_liquidity(usd_vault, position_index);
            if (IS_DEBUG) {
                debug::print(&string(b"current_sqrt_price before..."));
                debug::print(&current_sqrt_price);
                debug::print(&l);
            };

            test_scenario::return_shared(t);
        };

        // alice swap obc-usd
        let alice = @0xA1;
        let amount_obc = 1_000_000_000u64;
        let total_amount_obc = amount_obc * 2;
        test_scenario::next_tx(&mut scenario_val, alice);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let input_obc = balance::create_for_testing<OBC>(total_amount_obc);
            let coin_obc = coin::from_balance(
                input_obc,
                test_scenario::ctx(&mut scenario_val),
            );
            if (IS_DEBUG) {
                debug::print(&string(b"Alice balances before mint ..."));
                debug::print(&coin_obc);
            };
            treasury::mint<USD>(
                &mut t,
                coin_obc,
                amount_obc,
                test_scenario::ctx(&mut scenario_val),
            );
            test_scenario::return_shared(t);
        };

        // alice check balance
        test_scenario::next_tx(&mut scenario_val, alice);
        {
            let coin_usd = test_scenario::take_from_sender<Coin<USD>>(&scenario_val);
            let coin_obc = test_scenario::take_from_sender<Coin<OBC>>(&scenario_val);
            if (IS_DEBUG) {
                debug::print(&string(b"Alice balances after mint ..."));
                debug::print(&coin_usd);
                debug::print(&coin_obc);
            };
            assert!(coin::value(&coin_usd) > 0, 301);
            test_scenario::return_to_sender(&scenario_val, coin_usd);
            test_scenario::return_to_sender(&scenario_val, coin_obc);
        };

        // check price after swap
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let position_index = 2;
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let usd_vault_key = treasury::get_vault_key<USD>();

            let usd_vault = treasury::borrow_vault<USD>(&t, usd_vault_key);
            let current_sqrt_price = vault::vault_current_sqrt_price(usd_vault);
            let l = vault::get_position_liquidity(usd_vault, position_index);
            if (IS_DEBUG) {
                debug::print(&string(b"current_sqrt_price after..."));
                debug::print(&current_sqrt_price);
                debug::print(&l);
            };

            test_scenario::return_shared(t);
        };

        // alice swap osd-obc
        test_scenario::next_tx(&mut scenario_val, alice);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let coin_usd = test_scenario::take_from_sender<Coin<USD>>(&scenario_val);
            let amount = coin::value(&coin_usd) / 2;
            if (IS_DEBUG) {
                debug::print(&string(b"Alice balances redeem obc ..."));
                debug::print(&amount);
            };
            treasury::redeem<USD>(
                &mut t,
                coin_usd,
                amount,
                test_scenario::ctx(&mut scenario_val),
            );
            test_scenario::return_shared(t);
        };

        // alice check balance
        test_scenario::next_tx(&mut scenario_val, alice);
        {
            let coin_usd = test_scenario::take_from_sender<Coin<USD>>(&scenario_val);
            let coin_obc = test_scenario::take_from_sender<Coin<OBC>>(&scenario_val);
            let coin_obc_1 = test_scenario::take_from_sender<Coin<OBC>>(&scenario_val);
            if (IS_DEBUG) {
                debug::print(&string(b"Alice balances after redeem ..."));
                debug::print(&coin_usd);
                debug::print(&coin_obc);
                debug::print(&coin_obc_1);
            };
            test_scenario::return_to_sender(&scenario_val, coin_usd);
            test_scenario::return_to_sender(&scenario_val, coin_obc);
            test_scenario::return_to_sender(&scenario_val, coin_obc_1);
        };

        test_scenario::end(scenario_val);
    }
}


