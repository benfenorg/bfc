#[test_only]
#[allow(duplicate_alias,unused_mut_ref)]
module bfc_system::bfc_system_tests {

    use std::ascii;
    use std::debug;
    use std::vector;
    use bfc_system::treasury_pool;
    use bfc_system::usdc::USDC;
    use bfc_system::busd::BUSD;
    use bfc_system::bjpy::BJPY;
    use bfc_system::treasury;
    use bfc_system::treasury::Treasury;
    use sui::object;
    use sui::test_scenario;
    use sui::tx_context::TxContext;
    use sui::clock::{Self};
    use sui::balance::{Self};
    use sui::coin::{Self};
    use sui::bfc::BFC;
    use sui::test_scenario::Scenario;
    use sui::tx_context;
    use sui::vec_map::{Self};
    use sui::vec_set;
    use bfc_system::treasury::{ERR_INSUFFICIENT, TreasuryPauseCap};
    use bfc_system::bfc_system_state_inner::{ERR_MINT_UNAUTHORIZED,ERR_DAILY_LIMIT, ERR_SWAP_STABLE_NOT_ENOUGH, ERR_MINT_BUSD};

    use bfc_system::busd;
    use bfc_system::bjpy;
    use bfc_system::mgg;
    use bfc_system::bmxn;
    use bfc_system::bzar;
    use bfc_system::btry;
    use bfc_system::bsar;
    use bfc_system::brub;
    use bfc_system::binr;
    use bfc_system::bidr;
    use bfc_system::bgbp;
    use bfc_system::beur;
    use bfc_system::bcad;
    use bfc_system::bbrl;
    use bfc_system::bars;
    use bfc_system::baud;
    use bfc_system::bkrw;

    use bfc_system::bfc_system;
    use bfc_system::bfc_system::BfcSystemState;
    use bfc_system::test_utils;
    use bfc_system::bfc_system_state_inner;

    const BFC_AMOUNT: u64 = 1_000_000_000_000_000_000;

    #[test]
    fun print_stable_rate() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);

        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);

        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        let length = stable_rate.size();

        let mut i = 0;
        while (i < length) {
            let (key, _value) = stable_rate.get_entry_by_idx(i);
            debug::print(key);
            i = i + 1;
        };

        let (_treasury, treasury_pool) = bfc_system_state_inner::get_treasury_and_treasury_pool(system_state_v2);

        debug::print(&std::ascii::string(b"system_state_v2, treasury_pool.balance"));
        debug::print(&treasury_pool::get_balance(treasury_pool));

        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_round() {
        let bfc_addr = @0x0;
        let mut scenario_val = test_scenario::begin(bfc_addr);
        test_utils::setup_without_parameters(&mut scenario_val, bfc_addr);
        let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        clock::increment_for_testing(&mut clock, 3600 * 4 * 1000 + 1000);
        let mut t = test_scenario::take_shared<Treasury>(&scenario_val);
        treasury::rebalance(&mut t, 0, true, &clock, test_scenario::ctx(&mut scenario_val));

        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        create_sui_system_state_for_testing(ctx, BFC_AMOUNT);
        test_scenario::next_tx(scenario, bfc_addr);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(scenario);

        bfc_system::bfc_round_test(&mut system_state, &clock, 0, 1000000,test_scenario::ctx(scenario));

        test_scenario::return_shared(system_state);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(clock);
        test_scenario::end(scenario_val);
    }

    public fun create_sui_system_state_for_testing(ctx: &mut TxContext, bfc_amount: u64) {
        let mut treasury_parameters = vec_map::empty<ascii::String, bfc_system_state_inner::TreasuryParameters>();
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BUSD"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 58333726687135162368, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"MGG"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 14986205729530720256, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BJPY"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 4915287178933356544, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BKRW"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 1618695223101379840, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BAUD"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 48103223333394006016, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BARS"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 2020739568339092224, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BBRL"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 26731871811266244608, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BCAD"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 50854163925868765184, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BEUR"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 61180928696206655488, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BGBP"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 65738771359798919168, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BIDR"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 470301539970485312, 10000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BINR"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 6390139593977006080, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BRUB"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 6118092869620665344, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BSAR"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 30311093525086388224, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BTRY"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 10756207731032303616, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BZAR"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 13555533118889377792, 50000_000_000_000, 4
            )
        );
        vec_map::insert(
            &mut treasury_parameters,
            ascii::string(b"BMXN"),
            bfc_system_state_inner::bfc_system_treasury_parameters(
                9, 1, 2, 14169212980379457536, 50000_000_000_000, 4
            )
        );

        bfc_system::create(
            object::bfc_system_state_for_test(),
            balance::create_for_testing<BFC>(bfc_amount),
            busd::new_for_test(ctx),
            bjpy::new_for_test(ctx),
            bkrw::new_for_test(ctx),
            baud::new_for_test(ctx),
            bars::new_for_test(ctx),
            bbrl::new_for_test(ctx),
            bcad::new_for_test(ctx),
            beur::new_for_test(ctx),
            bgbp::new_for_test(ctx),
            bidr::new_for_test(ctx),
            binr::new_for_test(ctx),
            brub::new_for_test(ctx),
            bsar::new_for_test(ctx),
            btry::new_for_test(ctx),
            bzar::new_for_test(ctx),
            bmxn::new_for_test(ctx),
            mgg::new_for_test(ctx),
            bfc_system_state_inner::bfc_system_parameters(
                3600 * 4,
                2000,
                treasury_parameters,
            ),
            ctx,
        );
    }

    fun setup(bfc_amount: u64): Scenario {
        let bfc_addr = @0x0;
        let mut scenario_val = test_scenario::begin(bfc_addr);

        create_sui_system_state_for_testing(test_scenario::ctx(&mut scenario_val), bfc_amount);
        test_scenario::next_tx(&mut scenario_val, bfc_addr);
        scenario_val
    }

    fun tearDown(s: Scenario) {
        test_scenario::end(s);
    }

    #[test]
    fun test_next_epoch_bfc_required() {
        let mut scenario_val = setup(BFC_AMOUNT);

        let system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let amount = bfc_system::next_epoch_bfc_required(&system_state);
        assert!(amount == 0, 1);
        // basepoint = 1000 /  position = 9 / timeinterval=4h
        let _total = (
                50000_000_000_000 * 5 * 6 * 6 + // usd
                50000_000_000_000 * 5 * 6 * 6 + // mgg
                50000_000_000_000 * 5 * 6 * 6 + // jpy
                50000_000_000_000 * 5 * 6 * 6 + // krw
                50000_000_000_000 * 5 * 6 * 6 + // aud
                50000_000_000_000 * 5 * 6 * 6 + // ars
                50000_000_000_000 * 5 * 6 * 6 + // cad
                50000_000_000_000 * 5 * 6 * 6 + // eur
                50000_000_000_000 * 5 * 6 * 6 + // gbp
                10000_000_000_000 * 5 * 6 * 6 + // idr
                50000_000_000_000 * 5 * 6 * 6 + // inr
                50000_000_000_000 * 5 * 6 * 6 + // rub
                50000_000_000_000 * 5 * 6 * 6 + // sar
                50000_000_000_000 * 5 * 6 * 6 + // try
                50000_000_000_000 * 5 * 6 * 6 + // zar
                50000_000_000_000 * 5 * 6 * 6 + // mxn
                50000_000_000_000 * 5 * 6 * 6 // brl
        );

        //debug::print(&total);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_INSUFFICIENT)]
    fun test_deposit_with_error() {
        let mut scenario_val = setup(0);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let bfc = balance::create_for_testing<BFC>(100);
        let current_balance = bfc_system::treasury_balance(&system_state);
        assert!(current_balance == 0, 1);
        bfc_system::deposit_to_treasury(
            &mut system_state,
            coin::from_balance(bfc, test_scenario::ctx(&mut scenario_val)),
        );

        let new_balance = bfc_system::treasury_balance(&system_state);
        assert!(new_balance == 8673437554478, 1);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let amount = bfc_system::next_epoch_bfc_required(&system_state);
        let bfc = balance::create_for_testing<BFC>(amount);
        let current_balance = bfc_system::treasury_balance(&system_state);
        assert!(current_balance == 8673437554378, 2);

        bfc_system::deposit_to_treasury(
            &mut system_state,
            coin::from_balance(bfc, test_scenario::ctx(&mut scenario_val)),
        );

        let new_balance = bfc_system::treasury_balance(&system_state);
        assert!(new_balance == amount + 8673437554378, 3);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_fetch_positions() {
        let scenario_val = setup(BFC_AMOUNT);
        let system_state = test_scenario::take_shared<BfcSystemState>(&scenario_val);
        let positons = bfc_system::vault_positions<BUSD>(&system_state);
        assert!(vector::length(&positons) == 9, 300);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_vault_set_pause() {
        let scenario_val = setup(BFC_AMOUNT);
        let cap = test_scenario::take_from_sender<TreasuryPauseCap>(&scenario_val);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&scenario_val);

        bfc_system::vault_set_pause<BUSD>(&cap, &mut system_state, true);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, cap);
        tearDown(scenario_val);
    }
    #[test]
    fun test_rebalance_stablecoin() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        clock::increment_for_testing(&mut clock, 3600 * 4 * 1000 + 1000);

        bfc_system::rebalance_with_one_stablecoin<BUSD>(&mut system_state, &clock, test_scenario::ctx(&mut scenario_val));

        test_scenario::return_shared(system_state);
        clock::destroy_for_testing(clock);
        tearDown(scenario_val);
    }

    #[test]
    fun test_mint_stable_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let test_key = b"right_key";
        bfc_system::add_operation_capability_test(&mut system_state, std::ascii::string(test_key), tx_context::sender(ctx), ctx);

        let coin = bfc_system::mint_stable<USDC>(&mut system_state, 100, test_key, ctx);
        assert!(coin.value() == 100, 1);

        coin::burn_for_testing(coin);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_mint_bjpy_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let test_key = b"right_key";
        bfc_system::add_operation_capability_test(&mut system_state, std::ascii::string(test_key), tx_context::sender(ctx), ctx);
        //todo:transfer bjpy from vault
        let bjpy = balance::create_for_testing<BJPY>(100);
        let (system_state_v2, ctx2) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        bfc_system_state_inner::add_balance_to_vault(system_state_v2, bjpy, ctx2);

        let (treasury, _) = bfc_system_state_inner::get_treasury_and_treasury_pool(system_state_v2);
        let coin_a_amount = treasury::get_coin_a_amount<BJPY>(treasury); 
        assert!(coin_a_amount == 100, 1);
        
        let coin = bfc_system::mint_stable<BJPY>(&mut system_state, 200, test_key, ctx);
        assert!(coin.value() == 200, 1);
        coin::burn_for_testing(coin);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_MINT_UNAUTHORIZED)]
    fun test_mint_stable_fail_unauthorized() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let test_key = b"wrong_key";
        let coin = bfc_system::mint_stable<USDC>(&mut system_state, 100, test_key, ctx);

        coin::burn_for_testing(coin);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_MINT_BUSD)]
    fun test_mint_stable_fail_busd() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let test_key = b"right_key";
        bfc_system::add_operation_capability_test(&mut system_state, std::ascii::string(test_key), tx_context::sender(ctx), ctx);

        let coin = bfc_system::mint_stable<BUSD>(&mut system_state, 100, test_key, ctx);

        coin::burn_for_testing(coin);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_exchange_stable_to_busd_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let test_key = b"right_key";
        bfc_system::add_operation_capability_test(&mut system_state, std::ascii::string(test_key), tx_context::sender(ctx), ctx);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state, 100, test_key, tx_context::sender(ctx), ctx);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_exchange_busd_to_stable_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let test_key = b"right_key";
        bfc_system::add_operation_capability_test(&mut system_state, std::ascii::string(test_key), tx_context::sender(ctx), ctx);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state,10000_000_000_000u64, test_key, tx_context::sender(ctx), ctx);

        let busd = balance::create_for_testing<BUSD>(100);
        let busd_coin = coin::from_balance(busd, test_scenario::ctx(&mut scenario_val));
        let receiver_address = @0x0;
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin, test_scenario::ctx(&mut scenario_val));
        test_scenario::next_tx(&mut scenario_val, receiver_address);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_DAILY_LIMIT)]
    fun test_exchange_busd_to_stable_exceed_daily_limit() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let test_key = b"right_key";
        bfc_system::add_operation_capability_test(&mut system_state, std::ascii::string(test_key), tx_context::sender(ctx), ctx);
        // let coin = bfc_system::mint_stable<USDC>(&mut system_state, 50000_000_000_000u64, &test_key, ctx);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state, 50000_000_000_000u64, test_key, tx_context::sender(ctx), ctx);

        let busd = balance::create_for_testing<BUSD>(50000_000_000_000u64);
        let busd_coin = coin::from_balance(busd, test_scenario::ctx(&mut scenario_val));
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin, test_scenario::ctx(&mut scenario_val));

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_SWAP_STABLE_NOT_ENOUGH)]
    fun test_exchange_busd_to_stable_swap_stable_not_enough() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let test_key = b"right_key";
        bfc_system::add_operation_capability_test(&mut system_state, std::ascii::string(test_key), tx_context::sender(ctx), ctx);
        // let coin = bfc_system::mint_stable<USDC>(&mut system_state, 10000_000_000_000u64, &test_key, ctx);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state, 10000_000_000_000u64, test_key, tx_context::sender(ctx), ctx);

        let busd = balance::create_for_testing<BUSD>(20000_000_000_000u64);
        let busd_coin = coin::from_balance(busd, test_scenario::ctx(&mut scenario_val));
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin, test_scenario::ctx(&mut scenario_val));

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_request_set_daily_out_limit_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);

        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let mut id = object::bfc_system_state_for_test();
        let limit = 1000u64;
        let before = bfc_system::get_daily_out_limit(&mut id);
        assert!(before == 40000_000_000_000u64, 1);

        bfc_system::set_daily_out_limit(&mut id, limit);
        let after = bfc_system::get_daily_out_limit(&mut id);
        assert!(after == limit, 1);

        test_scenario::return_shared(system_state);
        object::delete(id);
        tearDown(scenario_val);
    }

    #[test]
    fun test_add_operation_capability_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);

        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let mut id = object::bfc_system_state_for_test();
        let key = ascii::string(b"key");
        let before = bfc_system::get_operation_capability_by_key(&mut id, &key);
        assert!(before.size() == 0, 1);
        bfc_system::add_operation_capability(&mut id, key, @0x0);
        let after = bfc_system::get_operation_capability_by_key(&mut id, &key);
        assert!(after.size() == 1, 1);

        test_scenario::return_shared(system_state);
        object::delete(id);
        tearDown(scenario_val);
    }

    #[test]
    fun test_remove_operation_capability_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);

        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let mut id = object::bfc_system_state_for_test();
        let key = ascii::string(b"key");
        bfc_system::add_operation_capability(&mut id, key, @0x0);
        bfc_system::remove_operation_capability(&mut id, &key, @0x0);
        let after = bfc_system::get_operation_capability_by_key(&mut id, &key);
        assert!(after.size() == 0, 1);

        test_scenario::return_shared(system_state);
        object::delete(id);
        tearDown(scenario_val);
    }

    #[test]
    fun test_set_operation_capability_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);

        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let mut id = object::bfc_system_state_for_test();
        let key = ascii::string(b"key");
        let before = bfc_system::get_operation_capability_by_key(&mut id, &key);
        assert!(before.size() == 0, 1);
        let v = vec_set::singleton(@0x0);
        bfc_system::set_operation_capability(&mut id, key, v);
        let after = bfc_system::get_operation_capability_by_key(&mut id, &key);
        assert!(after.size() == 1, 1);

        test_scenario::return_shared(system_state);
        object::delete(id);
        tearDown(scenario_val);
    }

    #[test]
    fun test_inner_busd_to_bfc_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);

        bfc_system::load_system_state_mut_test(&mut system_state, ctx);
        let balance = balance::create_for_testing<BUSD>(100);
        let result = bfc_system::inner_stablecoin_to_bfc_test(&mut system_state, balance, 100, ctx);
        assert!(result.value() == 100, 1);

        test_scenario::return_shared(system_state);
        coin::burn_for_testing(coin::from_balance(result, ctx));
        tearDown(scenario_val);
    }

    #[test]
    fun test_inner_bjpy_to_bfc_success() {
        let mut scenario_val = setup(BFC_AMOUNT);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);

        bfc_system::load_system_state_mut_test(&mut system_state, ctx);
        let balance = balance::create_for_testing<BJPY>(100);
        let result = bfc_system::inner_stablecoin_to_bfc_test(&mut system_state, balance, 100, ctx);
        assert!(result.value() == 100, 1);

        test_scenario::return_shared(system_state);
        coin::burn_for_testing(coin::from_balance(result, ctx));
        tearDown(scenario_val);
    }
}
