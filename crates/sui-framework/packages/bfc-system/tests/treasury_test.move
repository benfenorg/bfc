#[test_only]
module bfc_system::treasury_test {
    use sui::test_scenario::{Self};
    use std::debug;
    use std::vector;
    use std::ascii::string;
    use bfc_system::clmm_math;
    use bfc_system::i32;
    use sui::coin::{Self, Coin};
    use sui::balance;
    use sui::bfc::BFC;
    use sui::clock;
    use sui::transfer;
    use bfc_system::treasury::{Self, Treasury};
    use bfc_system::vault;
    use bfc_system::busd::{Self, BUSD};

    const IS_DEBUG: bool = false;

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
        let tick_spacing = 1;
        let spacing_times = 2;
        let initialize_price = 18446744073709551616; //2 ** 64
        let base_point = 1_000_000_000_000; // 1000 bfc
        let max_counter_times = 5;

        // create vault
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::increment_for_testing(&mut clock, 360000);
            let usd_supply = busd::new(test_scenario::ctx(&mut scenario_val));
            treasury::create_vault<BUSD>(
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
            let usd_vault_key = treasury::get_vault_key<BUSD>();
            let mut_vault = treasury::borrow_mut_vault<BUSD>(&mut t, usd_vault_key);
            let ticks = vault::init_positions<BUSD>(
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
            let usd_vault_key = treasury::get_vault_key<BUSD>();
            let usd_vault = treasury::borrow_vault<BUSD>(&t, usd_vault_key);
            let (amount_a, amount_b) = vault::get_position_amounts<BUSD>(
                usd_vault,
                1,
                true,
            );
            assert!(amount_a == 0, 101);
            assert!(amount_b == 0, 102);

            let (balance_a, balance_b) = vault::balances<BUSD>(
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
            let usd_vault_key = treasury::get_vault_key<BUSD>();
            let usd_mut_vault = treasury::borrow_mut_vault<BUSD>(&mut t, usd_vault_key);
            let upper =  i32::from(1);
            let lower = i32::sub(upper, i32::from(2));
            let (liquidity, amount_a, amount_b) = clmm_math::get_liquidity_by_amount(
                lower,
                upper,
                vault::vault_current_tick_index(usd_mut_vault),
                vault::vault_current_sqrt_price(usd_mut_vault),
                base_point,
                false,
            );
            let balance_a = balance::create_for_testing<BUSD>(amount_a);
            let balance_b = balance::create_for_testing<BFC>(amount_b);

            let receipt = vault::add_liquidity<BUSD>(
                usd_mut_vault,
                position_index,
                liquidity,
            );
            // repay
            vault::repay_add_liquidity<BUSD>(
                usd_mut_vault,
                balance_a,
                balance_b,
                receipt,
            );

            let usd_vault = treasury::borrow_vault<BUSD>(&t, usd_vault_key);
            let (amount_a, amount_b) = vault::balances<BUSD>(usd_vault);
            if (IS_DEBUG) {
                debug::print(&string(b"get balance after add-l"));
                debug::print(&amount_a);
                debug::print(&amount_b);
            };
            assert!(amount_a == base_point, 103);
            assert!(amount_b == base_point, 104);

            let (amount_a, amount_b) = vault::get_position_amounts<BUSD>(
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
            let usd_vault_key = treasury::get_vault_key<BUSD>();

            let usd_vault = treasury::borrow_vault<BUSD>(&t, usd_vault_key);
            let current_sqrt_price = vault::vault_current_sqrt_price(usd_vault);
            let l = vault::get_position_liquidity(usd_vault, position_index);
            if (IS_DEBUG) {
                debug::print(&string(b"current_sqrt_price before..."));
                debug::print(&current_sqrt_price);
                debug::print(&l);
            };

            test_scenario::return_shared(t);
        };

        // alice swap bfc-usd
        let alice = @0xA1;
        let amount_bfc = 1_000_000_000u64;
        let total_amount_bfc = amount_bfc * 2;
        test_scenario::next_tx(&mut scenario_val, alice);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let input_bfc = balance::create_for_testing<BFC>(total_amount_bfc);
            let coin_bfc = coin::from_balance(
                input_bfc,
                test_scenario::ctx(&mut scenario_val),
            );
            if (IS_DEBUG) {
                debug::print(&string(b"Alice balances before mint ..."));
                debug::print(&coin_bfc);
            };
            treasury::mint<BUSD>(
                &mut t,
                coin_bfc,
                amount_bfc,
                test_scenario::ctx(&mut scenario_val),
            );
            test_scenario::return_shared(t);
        };

        // alice check balance
        test_scenario::next_tx(&mut scenario_val, alice);
        {
            let coin_usd = test_scenario::take_from_sender<Coin<BUSD>>(&scenario_val);
            let coin_bfc = test_scenario::take_from_sender<Coin<BFC>>(&scenario_val);
            if (IS_DEBUG) {
                debug::print(&string(b"Alice balances after mint ..."));
                debug::print(&coin_usd);
                debug::print(&coin_bfc);
            };
            assert!(coin::value(&coin_usd) > 0, 301);
            test_scenario::return_to_sender(&scenario_val, coin_usd);
            test_scenario::return_to_sender(&scenario_val, coin_bfc);
        };

        // check price after swap
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let position_index = 2;
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let usd_vault_key = treasury::get_vault_key<BUSD>();

            let usd_vault = treasury::borrow_vault<BUSD>(&t, usd_vault_key);
            let current_sqrt_price = vault::vault_current_sqrt_price(usd_vault);
            let l = vault::get_position_liquidity(usd_vault, position_index);
            if (IS_DEBUG) {
                debug::print(&string(b"current_sqrt_price after..."));
                debug::print(&current_sqrt_price);
                debug::print(&l);
            };

            test_scenario::return_shared(t);
        };

        // alice swap osd-bfc
        test_scenario::next_tx(&mut scenario_val, alice);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            let coin_usd = test_scenario::take_from_sender<Coin<BUSD>>(&scenario_val);
            let amount = coin::value(&coin_usd) / 2;
            if (IS_DEBUG) {
                debug::print(&string(b"Alice balances redeem bfc ..."));
                debug::print(&amount);
            };
            treasury::redeem<BUSD>(
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
            let coin_usd = test_scenario::take_from_sender<Coin<BUSD>>(&scenario_val);
            let coin_bfc = test_scenario::take_from_sender<Coin<BFC>>(&scenario_val);
            let coin_bfc_1 = test_scenario::take_from_sender<Coin<BFC>>(&scenario_val);
            if (IS_DEBUG) {
                debug::print(&string(b"Alice balances after redeem ..."));
                debug::print(&coin_usd);
                debug::print(&coin_bfc);
                debug::print(&coin_bfc_1);
            };
            test_scenario::return_to_sender(&scenario_val, coin_usd);
            test_scenario::return_to_sender(&scenario_val, coin_bfc);
            test_scenario::return_to_sender(&scenario_val, coin_bfc_1);
        };

        test_scenario::end(scenario_val);
    }
}


