#[test_only]
#[allow(duplicate_alias,unused_mut_ref)]
module bfc_system::bfc_system_tests {

    use std::ascii;
    use std::debug;
    use std::vector;
    use bfc_system::usdc::USDC;
    use bfc_system::busd::BUSD;
    use bfc_system::bjpy::BJPY;
    use bfc_system::treasury;
    use bfc_system::treasury::Treasury;
    use sui::object;
    use sui::test_scenario;
    use sui::tx_context::{TxContext};
    use sui::clock::{Self};
    use sui::balance::{Self};
    use sui::coin::{Self};
    use sui::bfc::BFC;
    use sui::test_scenario::Scenario;
    use sui::tx_context;
    use sui::vec_map::{Self};
    use sui::vec_set;
    use bfc_system::treasury_pool;
    use bfc_system::treasury::{ERR_INSUFFICIENT, TreasuryPauseCap};
    use bfc_system::bfc_system_state_inner::{ERR_MINT_UNAUTHORIZED,ERR_DAILY_LIMIT, ERR_SWAP_STABLE_NOT_ENOUGH, ERR_MINT_BUSD, ERR_REBALANCE_NOT_BUSD,
        BfcSystemModifyCap, ERR_ADMIN_ALREADY_INITED, ERR_ADD_ADMIN_COUNT_ZERO, ERR_ADMIN_COUNT_ZERO,
        BfcSystemAdminCap, ERR_MINT_AMOUNT_ZERO, ERR_SET_CONFIG_UNAUTHORIZED
    };

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
    const MINT_USDC_USDT_RIGHT_KEY: vector<u8> = b"MINT-USDT-USDC-right_key";
    const MINT_USDC_USDT_WRONG_KEY: vector<u8> = b"MINT-USDT-USDC-wrong_key";
    const MINT_OTHER_STABLECOIN_RIGHT_KEY: vector<u8> = b"MINT-OTHER-STABLECOIN-right_key";
    const BFC_ADDR: address = @0x0 ;

    #[test]
    fun print_stable_rate() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);

        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);

        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        let length = stable_rate.size();

        let mut i = 0;
        while (i < length) {
            let (key, value) = stable_rate.get_entry_by_idx(i);
            debug::print(key);
            // let busd_vault_key = treasury::get_vault_key<BUSD>();
            // debug::print(&busd_vault_key);

            debug::print(value);
            // assert!(key == busd_vault_key, 1);

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
        let bfc_addr = BFC_ADDR;
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

    #[test]
    #[expected_failure]
    fun test_round_v2_invalid_param() {
        let bfc_addr = BFC_ADDR;
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

        let mut stable_type_name_vector : vector<ascii::String> = vector::empty();
        let mut stable_rate_vector : vector<u64> = vector::empty();
        vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::unkonw::unkonw"));
        vector::push_back(&mut stable_rate_vector, 1000000);

        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));

        test_scenario::return_shared(system_state);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(clock);
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure]
    fun test_round_v2_invalid_vector_length() {
        let bfc_addr = BFC_ADDR;
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

        let mut stable_type_name_vector : vector<ascii::String> = vector::empty();
        let mut stable_rate_vector : vector<u64> = vector::empty();
        vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::unkonw::unkonw"));
        vector::push_back(&mut stable_rate_vector, 1000000);
        vector::push_back(&mut stable_rate_vector, 1000000);

        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));

        test_scenario::return_shared(system_state);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(clock);
        test_scenario::end(scenario_val);
    }


    #[test]
    fun test_round_v2() {
        let bfc_addr = BFC_ADDR;
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

        // before bfc_round_v2
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        debug::print(&stable_rate);
        let busd_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD"));

        let mut stable_type_name_vector : vector<ascii::String> = vector::empty();
        let mut stable_rate_vector : vector<u64> = vector::empty();
        vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
        vector::push_back(&mut stable_rate_vector, 1000000000);
        // add beur，rate is 1000000000
        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        debug::print(&stable_rate);
        // check beur
        let beur_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
        assert!(beur_rate == busd_rate, 2);

        // clear stable_type_name_vector, stable_rate_vector
        stable_type_name_vector = vector::empty();
        stable_rate_vector = vector::empty();
        // add bjpy
        vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY"));
        vector::push_back(&mut stable_rate_vector, 10000);
        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        debug::print(&stable_rate);
        // check bjpy
        let bjpy_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY"));
        assert!(bjpy_rate != busd_rate, 4);
        // check beur, no update
        let beur_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
        assert!(beur_rate == busd_rate, 5);

        // clear stable_type_name_vector, stable_rate_vector
        stable_type_name_vector = vector::empty();
        stable_rate_vector = vector::empty();
        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));
        // should not update
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        debug::print(&stable_rate);
        // check bjpy
        let new_bjpy_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY"));
        assert!(bjpy_rate == new_bjpy_rate, 7);
        // check beur, no update
        let new_beur_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
        assert!(beur_rate == new_beur_rate, 8);


        let old_bjpy_rate = new_bjpy_rate;
        // clear stable_type_name_vector, stable_rate_vector
        stable_type_name_vector = vector::empty();
        stable_rate_vector = vector::empty();
        // update bjpy, rate is 9999
        vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY"));
        vector::push_back(&mut stable_rate_vector, 9999);
        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        debug::print(&stable_rate);
        // check bjpy
        let new_bjpy_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY"));
        assert!(new_bjpy_rate != old_bjpy_rate, 10);

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


    fun setup(bfc_amount: u64, key: vector<u8>): Scenario {
        let bfc_addr = BFC_ADDR;
        let mut scenario_val = test_scenario::begin(bfc_addr);

        test_scenario::next_tx(&mut scenario_val, bfc_addr);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);

            create_sui_system_state_for_testing(ctx, bfc_amount);
        };

        test_scenario::next_tx(&mut scenario_val, bfc_addr);
        {
            let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
            let ctx1 = test_scenario::ctx(&mut scenario_val);
            let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx1);

            bfc_system_state_inner::add_bfc_system_admin_cap(system_state_v2, _ctx, vector[bfc_addr, @0x1]);

            let mut operate_addresses = vec_set::empty<address>();
            operate_addresses.insert(bfc_addr);
            bfc_system_state_inner::set_operation_capability(system_state_v2, std::ascii::string(key), operate_addresses, _ctx);
            test_scenario::return_shared(system_state);
        };
        test_scenario::next_tx(&mut scenario_val, bfc_addr);

        scenario_val
    }

    fun tearDown(s: Scenario) {
        test_scenario::end(s);
    }

    #[test]
    fun test_next_epoch_bfc_required() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);

        let system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let amount = bfc_system::next_epoch_bfc_required(&system_state);
        std::debug::print(&amount);
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
        let mut scenario_val = setup(0, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let (_system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);

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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
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
        let scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let system_state = test_scenario::take_shared<BfcSystemState>(&scenario_val);
        let positons = bfc_system::vault_positions<BUSD>(&system_state);
        assert!(vector::length(&positons) == 9, 300);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_vault_set_pause() {
        let scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let cap = test_scenario::take_from_sender<TreasuryPauseCap>(&scenario_val);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&scenario_val);
        bfc_system::vault_set_pause<BUSD>(&cap, &mut system_state, true);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, cap);
        tearDown(scenario_val);
    }
    #[test]
    fun test_rebalance_stablecoin() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        clock::increment_for_testing(&mut clock, 3600 * 4 * 1000 + 1000);

        bfc_system::rebalance_with_one_stablecoin<BUSD>(&mut system_state, &clock, test_scenario::ctx(&mut scenario_val));

        test_scenario::return_shared(system_state);
        clock::destroy_for_testing(clock);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_REBALANCE_NOT_BUSD)]
    fun test_rebalance_stablecoin_not_busd() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        // Add 4 hours and 1 second
        clock::increment_for_testing(&mut clock, 3600 * 4 * 1000 + 1000);

        bfc_system::rebalance_with_one_stablecoin<BJPY>(&mut system_state, &clock, test_scenario::ctx(&mut scenario_val));

        test_scenario::return_shared(system_state);
        clock::destroy_for_testing(clock);
        tearDown(scenario_val);
    }

    #[test]
    fun test_mint_stable_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&mut scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let coin = bfc_system::mint_stable<USDC>(&mut system_state, 100, &modify_cap, ctx);
        assert!(coin.value() == 100, 1);

        coin::burn_for_testing(coin);
        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_mint_bjpy_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_OTHER_STABLECOIN_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);

        let (treasury, _) = bfc_system_state_inner::get_treasury_and_treasury_pool(system_state_v2);
        let coin_a_amount_before = treasury::get_coin_a_amount<BJPY>(treasury);
        debug::print(&std::ascii::string(b"coin_a_amount_before"));
        debug::print(&coin_a_amount_before);
        assert!(coin_a_amount_before == 37707208591093079, 1);


        let coin = bfc_system::mint_stable<BJPY>(&mut system_state, 200, &modify_cap, ctx);
        assert!(coin.value() == 200, 1);
        coin::burn_for_testing(coin);

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_MINT_AMOUNT_ZERO)]
    fun test_mint_stable_with_zero() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&mut scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let coin = bfc_system::mint_stable<USDC>(&mut system_state, 0, &modify_cap, ctx);

        coin::burn_for_testing(coin);
        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_MINT_UNAUTHORIZED)]
    fun test_mint_stable_fail_unauthorized() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_WRONG_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::remove_operation_capability(&mut system_state, &admin_cap, MINT_USDC_USDT_WRONG_KEY, tx_context::sender(ctx), ctx);
        let coin = bfc_system::mint_stable<USDC>(&mut system_state, 100, &modify_cap, ctx);

        coin::burn_for_testing(coin);

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_MINT_BUSD)]
    fun test_mint_stable_fail_busd() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);

        let coin = bfc_system::mint_stable<BUSD>(&mut system_state, 100, &modify_cap, ctx);

        coin::burn_for_testing(coin);

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_exchange_stable_to_busd_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);

        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state, 100, tx_context::sender(ctx), &modify_cap, ctx);

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_exchange_busd_to_stable_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state,10000_000_000_000u64, tx_context::sender(ctx), &modify_cap, ctx);

        let busd = balance::create_for_testing<BUSD>(100);
        let busd_coin = coin::from_balance(busd, test_scenario::ctx(&mut scenario_val));
        let receiver_address = BFC_ADDR;
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin, test_scenario::ctx(&mut scenario_val));
        test_scenario::next_tx(&mut scenario_val, receiver_address);

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_exchange_busd_to_stable_success_with_zero() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state,10000_000_000_000u64, tx_context::sender(ctx), &modify_cap, ctx);

        let busd = balance::create_for_testing<BUSD>(0);
        let busd_coin = coin::from_balance(busd, test_scenario::ctx(&mut scenario_val));
        let receiver_address = BFC_ADDR;
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin, test_scenario::ctx(&mut scenario_val));
        test_scenario::next_tx(&mut scenario_val, receiver_address);

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_DAILY_LIMIT)]
    fun test_exchange_busd_to_stable_exceed_daily_limit() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state, 50000_000_000_000u64, tx_context::sender(ctx), &modify_cap, ctx);

        let busd = balance::create_for_testing<BUSD>(50000_000_000_000u64);
        let busd_coin = coin::from_balance(busd, test_scenario::ctx(&mut scenario_val));
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin, test_scenario::ctx(&mut scenario_val));

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_DAILY_LIMIT)]
    fun test_exchange_busd_to_stable_exceed_daily_limit_v2() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state, 50000_000_000_000u64, tx_context::sender(ctx), &modify_cap, ctx);

        let busd = balance::create_for_testing<BUSD>(39000_000_000_000u64);
        let busd_coin = coin::from_balance(busd, test_scenario::ctx(&mut scenario_val));
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin, test_scenario::ctx(&mut scenario_val));

        let busd_v1 = balance::create_for_testing<BUSD>(10000_000_000_000u64);
        let busd_coin_v1 = coin::from_balance(busd_v1, test_scenario::ctx(&mut scenario_val));
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin_v1, test_scenario::ctx(&mut scenario_val));

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_exchange_busd_to_stable_success_daily_limit_v3() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state, 50000_000_000_000u64, tx_context::sender(ctx), &modify_cap, ctx);

        let busd = balance::create_for_testing<BUSD>(39000_000_000_000u64);
        let busd_coin = coin::from_balance(busd, test_scenario::ctx(&mut scenario_val));
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin, test_scenario::ctx(&mut scenario_val));

        bfc_system::set_daily_epoch(&mut system_state, 100, test_scenario::ctx(&mut scenario_val));

        let busd_v1 = balance::create_for_testing<BUSD>(10000_000_000_000u64);
        let busd_coin_v1 = coin::from_balance(busd_v1, test_scenario::ctx(&mut scenario_val));
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin_v1, test_scenario::ctx(&mut scenario_val));

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_SWAP_STABLE_NOT_ENOUGH)]
    fun test_exchange_busd_to_stable_swap_stable_not_enough() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::exchange_stable_to_busd<USDC>(&mut system_state, 10000_000_000_000u64, tx_context::sender(ctx), &modify_cap, ctx);

        let busd = balance::create_for_testing<BUSD>(20000_000_000_000u64);
        let busd_coin = coin::from_balance(busd, test_scenario::ctx(&mut scenario_val));
        bfc_system::exchange_busd_to_stable<USDC>(&mut system_state, busd_coin, test_scenario::ctx(&mut scenario_val));

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_request_set_daily_out_limit_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let mut id = object::bfc_system_state_for_test();
        let limit = 1000u64;
        let before = bfc_system::get_daily_out_limit(&mut id);
        assert!(before == 40000_000_000_000u64, 1);
        bfc_system::set_daily_out_limit(&mut system_state, &admin_cap, limit, ctx);
        let after = bfc_system::get_daily_out_limit(&mut id);
        assert!(after == limit, 1);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        object::delete(id);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_SET_CONFIG_UNAUTHORIZED)]
    fun test_request_set_daily_out_limit_unauthorized() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        bfc_system::remove_admin_capability(&mut system_state, BFC_ADDR, &admin_cap, ctx);

        let mut id = object::bfc_system_state_for_test();
        let limit = 1000u64;
        let before = bfc_system::get_daily_out_limit(&mut id);
        assert!(before == 40000_000_000_000u64, 1);
        bfc_system::set_daily_out_limit(&mut system_state, &admin_cap, limit, ctx);
        let after = bfc_system::get_daily_out_limit(&mut id);
        assert!(after == limit, 1);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        object::delete(id);
        tearDown(scenario_val);
    }

    #[test]
    fun test_add_operation_capability_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let key = b"key";
        let before = bfc_system::get_operation_capability_by_key(&mut system_state, key, ctx);
        assert!(before.size() == 0, 1);
        bfc_system::add_operation_capability(&mut system_state, &admin_cap, key, @0x0, ctx);
        let after = bfc_system::get_operation_capability_by_key(&mut system_state, key, ctx);
        assert!(after.size() == 1, 1);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_SET_CONFIG_UNAUTHORIZED)]
    fun test_add_operation_capability_unauthorized() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let key = b"key";
        bfc_system::remove_admin_capability(&mut system_state, BFC_ADDR, &admin_cap, ctx);
        bfc_system::add_operation_capability(&mut system_state, &admin_cap, key, @0x0, ctx);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        tearDown(scenario_val);
    }

    #[test]
    fun test_remove_operation_capability_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let key = b"key";
        bfc_system::add_operation_capability(&mut system_state, &cap, key, @0x0, ctx);
        bfc_system::remove_operation_capability(&mut system_state, &cap, key, @0x0, ctx);
        let after = bfc_system::get_operation_capability_by_key(&mut system_state, key, ctx);
        assert!(after.size() == 0, 1);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, cap);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_SET_CONFIG_UNAUTHORIZED)]
    fun test_remove_operation_capability_unauthorized() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let key = b"key";
        bfc_system::add_operation_capability(&mut system_state, &cap, key, @0x0, ctx);
        bfc_system::remove_admin_capability(&mut system_state, BFC_ADDR, &cap, ctx);
        bfc_system::remove_operation_capability(&mut system_state, &cap, key, @0x0, ctx);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, cap);
        tearDown(scenario_val);
    }

    #[test]
    fun test_set_operation_capability_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let key = b"key";
        let before = bfc_system::get_operation_capability_by_key(&mut system_state, key, ctx);
        assert!(before.size() == 0, 1);
        let v = vec_set::singleton(@0x0);
        bfc_system::set_operation_capability(&mut system_state, &cap, key, v, ctx);
        let after = bfc_system::get_operation_capability_by_key(&mut system_state, key, ctx);
        assert!(after.size() == 1, 1);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, cap);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_SET_CONFIG_UNAUTHORIZED)]
    fun test_set_operation_capability_unauthorized() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        let key = b"key";
        bfc_system::remove_admin_capability(&mut system_state, BFC_ADDR, &cap, ctx);
        let v = vec_set::singleton(@0x0);
        bfc_system::set_operation_capability(&mut system_state, &cap, key, v, ctx);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, cap);
        tearDown(scenario_val);
    }

    #[test]
    fun test_inner_busd_to_bfc_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
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


    #[test]
    fun test_init_admin_capability_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        // test init
        let test_address = @0x639a680b36b6a02ff29061383efec63c89c8d70d642357fa6b01d2fc7f293457;
        let test_addresses = vector[test_address];
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::init_admin_capability(&mut system_state, test_addresses, ctx);

        let (system_state_v2, _) = bfc_system::load_system_state_mut_test(&mut system_state, ctx);
        assert!(vec_set::contains<address>(&system_state_v2.get_admin_capability(), &test_address), 1);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_ADMIN_ALREADY_INITED)]
    fun test_init_admin_capability_fail() {
        let test_address = @0x639a680b36b6a02ff29061383efec63c89c8d70d642357fa6b01d2fc7f293457;
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let test_addresses = vector[test_address];
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::init_admin_capability(&mut system_state, test_addresses, ctx);

        let test_addresses1 = vector[test_address];
        bfc_system::init_admin_capability(&mut system_state, test_addresses1, ctx);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_add_admin_capability_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);

        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let test_address = @0x639a680b36b6a02ff29061383efec63c89c8d70d642357fa6b01d2fc7f293457;
        let test_addresses = vector[test_address];
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::add_admin_capability(&mut system_state, test_addresses, &admin_cap, ctx);

        let (system_state_v2, _) = bfc_system::load_system_state_mut_test(&mut system_state, ctx);
        assert!(vec_set::contains<address>(&system_state_v2.get_admin_capability(), &test_address), 1);

        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_ADD_ADMIN_COUNT_ZERO)]
    fun test_add_admin_capability_fail() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let test_addresses = vector[];
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::add_admin_capability(&mut system_state, test_addresses, &admin_cap, ctx);

        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = sui::vec_set::EKeyAlreadyExists)]
    fun test_add_admin_capability_fail_already_exist() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let test_address = @0x639a680b36b6a02ff29061383efec63c89c8d70d642357fa6b01d2fc7f293457;
        let test_addresses = vector[test_address];
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        // first
        bfc_system::add_admin_capability(&mut system_state, test_addresses, &admin_cap, ctx);
        // second
        bfc_system::add_admin_capability(&mut system_state, test_addresses, &admin_cap, ctx);

        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }


    #[test]
    fun test_remove_admin_capability_success() {
        let test_address = @0x639a680b36b6a02ff29061383efec63c89c8d70d642357fa6b01d2fc7f293457;
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);

        // add
        {
            let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

            let test_addresses = vector[test_address];
            let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

            let ctx = test_scenario::ctx(&mut scenario_val);

            bfc_system::add_admin_capability(&mut system_state, test_addresses, &admin_cap, ctx);

            test_scenario::return_to_sender(&scenario_val, admin_cap);
            test_scenario::return_shared(system_state);
        };
        test_scenario::next_tx(&mut scenario_val, BFC_ADDR);

        // remove
        {
            let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
            let new_admin_cap = test_scenario::take_from_address<BfcSystemAdminCap>(&scenario_val, test_address);

            let ctx = test_scenario::ctx(&mut scenario_val);
            bfc_system::remove_admin_capability(&mut system_state, BFC_ADDR, &new_admin_cap, ctx);

            let (system_state_v2, _) = bfc_system::load_system_state_mut_test(&mut system_state, ctx);
            assert!(vec_set::size(&system_state_v2.get_admin_capability()) == 1, 1);

            test_scenario::return_to_address(test_address, new_admin_cap);
            test_scenario::return_shared(system_state);
        };
        test_scenario::next_tx(&mut scenario_val, BFC_ADDR);

        tearDown(scenario_val);
    }

    #[test]

    #[expected_failure(abort_code = ERR_ADMIN_COUNT_ZERO)]
    fun test_remove_admin_capability_fail() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_USDC_USDT_RIGHT_KEY);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        // remove
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);

        bfc_system::remove_admin_capability(&mut system_state, BFC_ADDR, &admin_cap, ctx);

        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);

        tearDown(scenario_val);
    }


}
