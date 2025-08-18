#[test_only]
#[allow(duplicate_alias,unused_mut_ref)]
module bfc_system::bfc_system_tests {

    use std::ascii;
    use std::debug;
    use std::vector;
    use bfc_system::busd::BUSD;
    use bfc_system::bjpy::BJPY;
    use bfc_system::treasury;
    use bfc_system::math_u64;
    use bfc_system::treasury::Treasury;
    use sui::object;
    use sui::test_scenario;
    use sui::tx_context::{TxContext};
    use sui::clock::{Self};
    use sui::balance::{Self};
    use sui::coin::{Self};
    use sui::bfc::BFC;
    use sui::test_scenario::Scenario;
    use sui::vec_map::{Self};
    use sui::vec_set;
    use bfc_system::treasury_pool;
    use bfc_system::treasury::{ERR_INSUFFICIENT, TreasuryPauseCap};
    use bfc_system::bfc_system_state_inner::{ERR_REBALANCE_NOT_BUSD,
        BfcSystemModifyCap, ERR_ADMIN_ALREADY_INITED, ERR_ADD_ADMIN_COUNT_ZERO, ERR_ADMIN_COUNT_ZERO,ERROR_MINT_COIN_TYPE,
        BfcSystemAdminCap, ERR_SET_CONFIG_UNAUTHORIZED, ERR_MINT_AMOUNT_ZERO, ERR_MINT_UNAUTHORIZED, ERR_MINT_OPERATION_UNAUTHORIZED
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
    use bfc_system::bkrw::BKRW;

    use bfc_system::bfc_system;
    use bfc_system::bfc_system::BfcSystemState;
    use bfc_system::test_utils;
    use bfc_system::bfc_system_state_inner;

    const BFC_AMOUNT: u64 = 1_000_000_000_000_000_000;
    const MINT_BUSD_RIGHT_KEY: vector<u8> = b"MINT-BUSD-right_key";
    const MINT_BUSD_WRONG_SHORT_KEY: vector<u8> = b"MINT-B";
    const MINT_BUSD_WRONG_KEY: vector<u8> = b"MINT-BUSD-wrong_key";
    const MINT_OTHER_STABLECOIN_RIGHT_KEY: vector<u8> = b"MINT-OTHER-STABLECOIN-right_key";
    const MINT_OTHER_STABLECOIN_WRONG_SHORT_KEY: vector<u8> = b"MINT-OTHER-S";
    const BFC_ADDR: address = @0x0 ;
    const OVERFLOW_TEST_INT: u64 = 2667027000000000;

    #[test]
    fun print_stable_rate() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
    fun test_external_filed() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        {
            let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(&mut scenario_val));
            assert!(!bfc_system_state_inner::in_external_stable_gas_coin_list(system_state_v2, ascii::string(b"ddd")));

            let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
            let length = extra_fields.length();
            debug::print(&length);
            assert!(length == 0);
        };

        {
            let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(&mut scenario_val));

            let mut list = vector::empty<ascii::String>();
            list.insert(ascii::string(b"ddd"), 0);
            bfc_system_state_inner::add_external_stable_gas_coin(system_state_v2, list, _ctx);
            let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
            let length = extra_fields.length();
            debug::print(&length);
            assert!(length == 1);
            let list = bfc_system_state_inner::get_external_stable_gas_coin_list(system_state_v2);
            debug::print(list);
            assert!(list.length() == 1);

            assert!(!bfc_system_state_inner::in_external_stable_gas_coin_list(system_state_v2, ascii::string(b"xx")));
            assert!(bfc_system_state_inner::in_external_stable_gas_coin_list(system_state_v2, ascii::string(b"ddd")));
        };

        {
            let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(&mut scenario_val));

            bfc_system_state_inner::delete_external_stable_gas_coin(system_state_v2, ascii::string(b"ddd"), _ctx);
            bfc_system_state_inner::delete_external_stable_gas_coin(system_state_v2, ascii::string(b"ddd"), _ctx);
            let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
            let length = extra_fields.length();
            debug::print(extra_fields);
            debug::print(&length);
            assert!(length == 2);

            let list = bfc_system_state_inner::get_external_stable_gas_coin_list(system_state_v2);
            debug::print(list);
            assert!(list.length() == 1);
            let list = bfc_system_state_inner::get_to_delete_stable_gas_coin_list(system_state_v2);
            debug::print(list);
            assert!(list.length() == 1);
        };

        let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        {
            let scenario = &mut scenario_val;
            bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, vector::empty(), vector::empty(), test_scenario::ctx(scenario));
        };

        {
            let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(&mut scenario_val));

            let list = bfc_system_state_inner::get_external_stable_gas_coin_list(system_state_v2);
            debug::print(list);
            assert!(list.length() == 0);
        };

        clock::destroy_for_testing(clock);
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
    fun test_round_v2_invalid_zero_price() {
        let bfc_addr = BFC_ADDR;
        let mut scenario_val = test_scenario::begin(bfc_addr);
        test_utils::setup_without_parameters(&mut scenario_val, bfc_addr);
        let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        clock::increment_for_testing(&mut clock, 3600 * 4 * 1000 + 1000);
        let mut t = test_scenario::take_shared<Treasury>(&scenario_val);
        treasury::rebalance(&mut t, 0, true, &clock, test_scenario::ctx(&mut scenario_val));

        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        create_sui_system_state_no_skip_init_vault(ctx, BFC_AMOUNT);
        test_scenario::next_tx(scenario, bfc_addr);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(scenario);

        // before bfc_round_v2
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        // debug::print(&stable_rate);
        let busd_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD"));

        let mut stable_type_name_vector : vector<ascii::String> = vector::empty();
        let mut stable_rate_vector : vector<u64> = vector::empty();
        vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
        vector::push_back(&mut stable_rate_vector, 1000000000);
        // add beur，rate is 1000000000
        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        // debug::print(&stable_rate);
        // check beur
        let beur_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
        assert!(beur_rate == busd_rate, 2);

        {
            // set zero rate
            let mut stable_type_name_vector : vector<ascii::String> = vector::empty();
            let mut stable_rate_vector : vector<u64> = vector::empty();
            vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
            vector::push_back(&mut stable_rate_vector, 0);
            bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));
            let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
            let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
            // debug::print(&stable_rate);
            // check beur
            let new_beur_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
            assert!(new_beur_rate != 0, 2);
            assert!(new_beur_rate == beur_rate, 2);
        };

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

        // add unknow coion
        let mut stable_type_name_vector : vector<ascii::String> = vector::empty();
        let mut stable_rate_vector : vector<u64> = vector::empty();
        vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::unkonw::unkonw"));
        vector::push_back(&mut stable_rate_vector, 1000000);

        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));
        // check beur, no update
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        let beur_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
        assert!(beur_rate == busd_rate, 5);

        // should abort
        let _ = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::unkonw::unkonw"));

        test_scenario::return_shared(system_state);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(clock);
        test_scenario::end(scenario_val);
    }

    #[test]
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
    fun test_math_u64_overflowing() {
        let (_, overflowing) = math_u64::overflowing_mul(1000000000000, 1000000000000);
        assert!(overflowing, 1);
        let c = math_u64::wrapping_mul(111, 3);
        assert!(c == 333, 2);

        let (_, overflowing) = math_u64::overflowing_mul(1, 1);
        assert!(!overflowing, 3);
    }

    #[test]
    fun test_round_v2_overflowing() {
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
        debug::print(busd_rate);

        let mut stable_type_name_vector : vector<ascii::String> = vector::empty();
        let mut stable_rate_vector : vector<u64> = vector::empty();
        vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR"));
        vector::push_back(&mut stable_rate_vector, OVERFLOW_TEST_INT);
        // add beur，rate is 1000000000
        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        debug::print(&stable_rate);

        test_scenario::return_shared(system_state);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(clock);
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure]
    fun test_round_v2_overflowing2() {
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

        // shoud overflow
        let _  = *busd_rate * OVERFLOW_TEST_INT;


        test_scenario::return_shared(system_state);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(clock);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_round_v2_invalid_busd() {
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
        vector::push_back(&mut stable_type_name_vector, ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD"));
        vector::push_back(&mut stable_rate_vector, 1_000);

        bfc_system::bfc_round_v2_test(&mut system_state, &clock, 0, 1000000, stable_type_name_vector, stable_rate_vector, test_scenario::ctx(scenario));
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, test_scenario::ctx(scenario));
        let stable_rate = bfc_system_state_inner::get_rate_map(system_state_v2);
        let busd_rate = vec_map::get(&stable_rate, &ascii::string(b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD"));
        assert!(*busd_rate > 1_000_000_000, 5);

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
        create_sui_system_state_no_skip_init_vault(ctx, BFC_AMOUNT);
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

    public fun create_sui_system_state_for_testing_v2(ctx: &mut TxContext, bfc_amount: u64) : address {
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

        bfc_system::create_for_test(
            // object::new(ctx),
            // object::bfc_system_state(ctx),
            // object::bfc_system_state_for_test(),
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
            1,
            ctx,
        )
    }

    public fun create_sui_system_state_no_skip_init_vault(ctx: &mut TxContext, bfc_amount: u64) {
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
        0,
        ctx,
        );
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
            1,
            ctx,
        );
    }


    fun setup_no_skip_init_vault(bfc_amount: u64, key: vector<u8>): Scenario {
        let bfc_addr = BFC_ADDR;
        let mut scenario_val = test_scenario::begin(bfc_addr);

        test_scenario::next_tx(&mut scenario_val, bfc_addr);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            create_sui_system_state_no_skip_init_vault(ctx, bfc_amount);
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

    public fun public_setup(bfc_amount: u64, key: vector<u8>): Scenario {
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);

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
        let mut scenario_val = setup(0, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup_no_skip_init_vault(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let system_state = test_scenario::take_shared<BfcSystemState>(&scenario_val);
        let positons = bfc_system::vault_positions<BUSD>(&system_state);
        assert!(vector::length(&positons) == 9, 300);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_vault_set_pause() {
        let scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let cap = test_scenario::take_from_sender<TreasuryPauseCap>(&scenario_val);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&scenario_val);
        bfc_system::vault_set_pause<BUSD>(&cap, &mut system_state, true);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, cap);
        tearDown(scenario_val);
    }
    #[test]
    fun test_rebalance_stablecoin() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
    #[expected_failure(abort_code = ERROR_MINT_COIN_TYPE)]
    fun test_mint_not_busd() {
        let mut scenario_val = setup_no_skip_init_vault(BFC_AMOUNT, MINT_OTHER_STABLECOIN_RIGHT_KEY);
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
    fun test_burn_token(){
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_WRONG_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        let coin = bfc_system::mint_stable<BUSD>(&mut system_state, 200, &modify_cap, ctx);
        assert!(coin.value() == 200, 1);
        bfc_system::burn_stable<BUSD>(&mut system_state,coin,ctx);
        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_MINT_AMOUNT_ZERO)]
    fun test_mint_stable_with_zero() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&mut scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        let coin = bfc_system::mint_stable<BUSD>(&mut system_state, 0, &modify_cap, ctx);

        coin::burn_for_testing(coin);
        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_MINT_UNAUTHORIZED)]
    fun test_mint_stable_fail_unauthorized() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_WRONG_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let modify_cap = test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::remove_operation_capability(&mut system_state, &admin_cap,
            MINT_BUSD_WRONG_KEY, tx_context::sender(ctx), ctx);
        let coin = bfc_system::mint_stable<BUSD>(&mut system_state, 100, &modify_cap, ctx);
        coin::burn_for_testing(coin);

        test_scenario::return_to_sender(&scenario_val, modify_cap);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_MINT_OPERATION_UNAUTHORIZED)]
    fun test_mint_stable_fail_operation_unauthorized() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_OTHER_STABLECOIN_RIGHT_KEY);
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
    #[expected_failure(abort_code = ERR_MINT_OPERATION_UNAUTHORIZED)]
    fun test_mint_stable_fail_operation_unauthorized_v2() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_WRONG_SHORT_KEY);
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
    #[expected_failure(abort_code = ERR_MINT_OPERATION_UNAUTHORIZED)]
    fun test_mint_stable_fail_operation_unauthorized_v3() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_OTHER_STABLECOIN_WRONG_SHORT_KEY);
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
    fun test_set_get_oracle_address_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);

        bfc_system::set_oracle_address(&mut system_state, @0x99, ctx);
        let addr = bfc_system::get_oracle_address(&mut system_state, ctx);
        assert!(addr.borrow() == @0x99, 1);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_SET_CONFIG_UNAUTHORIZED)]
    fun test_set_get_oracle_address_unauthorized() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let admin_cap = test_scenario::take_from_sender<BfcSystemAdminCap>(&scenario_val);
        let ctx = test_scenario::ctx(&mut scenario_val);
        bfc_system::load_system_state_mut_test(&mut system_state, ctx);
        bfc_system::remove_admin_capability(&mut system_state, BFC_ADDR, &admin_cap, ctx);

        bfc_system::set_oracle_address(&mut system_state, @0x99, ctx);
        let addr = bfc_system::get_oracle_address(&mut system_state, ctx);
        assert!(addr.borrow() == @0x99, 1);

        test_scenario::return_shared(system_state);
        test_scenario::return_to_sender(&scenario_val, admin_cap);
        tearDown(scenario_val);
    }

    #[test]
    fun test_add_operation_capability_success() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup_no_skip_init_vault(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);

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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
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
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);

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
            assert!(vec_set::size(&system_state_v2.get_admin_capability()) == 2, 1);

            test_scenario::return_to_address(test_address, new_admin_cap);
            test_scenario::return_shared(system_state);
        };
        test_scenario::next_tx(&mut scenario_val, BFC_ADDR);

        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = ERR_ADMIN_COUNT_ZERO)]
    fun test_remove_admin_capability_fail() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let admin_cap = test_scenario::take_from_address<BfcSystemAdminCap>(&scenario_val, BFC_ADDR);
        let admin_cap_1 = test_scenario::take_from_address<BfcSystemAdminCap>(&scenario_val, @0x1);

        // remove
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);

        bfc_system::remove_admin_capability(&mut system_state, @0x1, &admin_cap_1, ctx);
        bfc_system::remove_admin_capability(&mut system_state, BFC_ADDR, &admin_cap, ctx);

        test_scenario::return_to_address(BFC_ADDR, admin_cap);
        test_scenario::return_to_address(@0x1, admin_cap_1);
        test_scenario::return_shared(system_state);

        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_to_treasury() {
        let mut scenario_val = setup_no_skip_init_vault(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let ctx = test_scenario::ctx(&mut scenario_val);

        let coin =  coin::mint_for_testing<BFC>(100_000000000, ctx);
        bfc_system::deposit_to_treasury(&mut system_state, coin);

        let (parameter_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        let (t, _tp) = bfc_system_state_inner::get_treasury_and_treasury_pool(parameter_v2);
        assert!(treasury::get_balance(t) == 8773437554378, 1);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_first_deposit() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        let extra_fields_before = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields_before.length() == 0, 1);
        
        // Verify initial balance is 0
        let initial_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(initial_balance == 0, 2);
        
        let busd_balance = balance::create_for_testing<BUSD>(1000);
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, busd_balance, _ctx);
        
        let extra_fields_after = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields_after.length() == 1, 3);
        
        // Verify deposited balance is correct
        let final_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(final_balance == 1000, 4);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_multiple_types() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        let busd_balance = balance::create_for_testing<BUSD>(1000);
        let bjpy_balance = balance::create_for_testing<BJPY>(2000);
        
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, busd_balance, _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, bjpy_balance, _ctx);
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 2, 1);
        
        // Verify both coin types have correct balances
        let busd_final_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        let bjpy_final_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BJPY>(system_state_v2);
        assert!(busd_final_balance == 1000, 2);
        assert!(bjpy_final_balance == 2000, 3);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_same_type_accumulation() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        let busd_balance1 = balance::create_for_testing<BUSD>(1000);
        let busd_balance2 = balance::create_for_testing<BUSD>(500);
        
        // First deposit
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, busd_balance1, _ctx);
        let balance_after_first = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(balance_after_first == 1000, 1);
        
        // Second deposit - should accumulate
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, busd_balance2, _ctx);
        let balance_after_second = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(balance_after_second == 1500, 2); // 1000 + 500
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 1, 3);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_zero_balance() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        let zero_balance = balance::zero<BUSD>();
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, zero_balance, _ctx);
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 1, 1);
        
        // Verify zero deposit results in zero balance
        let final_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(final_balance == 0, 2);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_busd_bjpy_only() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(100), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(200), _ctx);
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 2, 1);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_large_amounts() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        let large_amount = 18446744073709551615u64;
        let large_balance = balance::create_for_testing<BUSD>(large_amount);
        
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, large_balance, _ctx);
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 1, 1);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_mixed_operations() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(1000), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(2000), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(500), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BKRW>(system_state_v2, balance::create_for_testing<BKRW>(3000), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(1000), _ctx);
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 3, 1);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_balance_accumulation_busd() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        // First deposit: 1000 BUSD
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(1000), _ctx);
        let balance_after_first = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(balance_after_first == 1000, 1);
        
        // Second deposit: 500 BUSD, total should be 1500
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(500), _ctx);
        let balance_after_second = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(balance_after_second == 1500, 2); // 1000 + 500
        
        // Third deposit: 2000 BUSD, total should be 3500
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(2000), _ctx);
        let balance_after_third = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(balance_after_third == 3500, 3); // 1500 + 2000
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 1, 4);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_balance_accumulation_multiple_coins() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        // BUSD deposits: 100 + 200 + 300 = 600 total
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(100), _ctx);
        let busd_balance_1 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(busd_balance_1 == 100, 1);
        
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(200), _ctx);
        let busd_balance_2 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(busd_balance_2 == 300, 2); // 100 + 200
        
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(300), _ctx);
        let busd_final_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(busd_final_balance == 600, 3); // 300 + 300
        
        // BJPY deposits: 1000 + 500 + 750 = 2250 total
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(1000), _ctx);
        let bjpy_balance_1 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BJPY>(system_state_v2);
        assert!(bjpy_balance_1 == 1000, 4);
        
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(500), _ctx);
        let bjpy_balance_2 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BJPY>(system_state_v2);
        assert!(bjpy_balance_2 == 1500, 5); // 1000 + 500
        
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(750), _ctx);
        let bjpy_final_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BJPY>(system_state_v2);
        assert!(bjpy_final_balance == 2250, 6); // 1500 + 750
        
        // Verify final balances are correct and don't interfere with each other
        let final_busd_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        let final_bjpy_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BJPY>(system_state_v2);
        assert!(final_busd_balance == 600, 7);
        assert!(final_bjpy_balance == 2250, 8);
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 2, 9);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_sequential_deposits_same_coin() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        // Perform 5 sequential deposits of the same coin type
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(1000), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(2000), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(3000), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(4000), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(5000), _ctx);
        // Total should be: 1000 + 2000 + 3000 + 4000 + 5000 = 15000
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 1, 1);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_with_zero_amounts() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        // Deposit non-zero amount first
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(1000), _ctx);
        let balance_after_first = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(balance_after_first == 1000, 1);
        
        // Then deposit zero amount - should not change the total
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::zero<BUSD>(), _ctx);
        let balance_after_zero = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(balance_after_zero == 1000, 2); // Should remain 1000
        
        // Then deposit another non-zero amount
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(500), _ctx);
        let final_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(final_balance == 1500, 3); // 1000 + 0 + 500 = 1500
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 1, 4);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_interleaved_deposits() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        // Interleaved deposits of different coin types
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(100), _ctx);
        let busd_balance_1 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        let bjpy_balance_1 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BJPY>(system_state_v2);
        assert!(busd_balance_1 == 100, 1);
        assert!(bjpy_balance_1 == 0, 2);
        
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(200), _ctx);
        let busd_balance_2 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        let bjpy_balance_2 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BJPY>(system_state_v2);
        assert!(busd_balance_2 == 100, 3); // BUSD unchanged
        assert!(bjpy_balance_2 == 200, 4);
        
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(300), _ctx);
        let busd_balance_3 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        let bjpy_balance_3 = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BJPY>(system_state_v2);
        assert!(busd_balance_3 == 400, 5); // 100 + 300
        assert!(bjpy_balance_3 == 200, 6); // BJPY unchanged
        
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(400), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(500), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BJPY>(system_state_v2, balance::create_for_testing<BJPY>(600), _ctx);
        
        // Final verification: BUSD total: 100 + 300 + 500 = 900, BJPY total: 200 + 400 + 600 = 1200
        let final_busd_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        let final_bjpy_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BJPY>(system_state_v2);
        assert!(final_busd_balance == 900, 7);
        assert!(final_bjpy_balance == 1200, 8);
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 2, 9);
        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_stable_gas_coin_maximum_value_accumulation() {
        let mut scenario_val = setup(BFC_AMOUNT, MINT_BUSD_RIGHT_KEY);
        let mut system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        
        let ctx = test_scenario::ctx(&mut scenario_val);
        let (system_state_v2, _ctx) = bfc_system::load_system_state_mut_for_test(&mut system_state, ctx);
        
        // Test with large values that might approach u64 limits
        let large_amount1 = 9223372036854775807u64; // Close to u64::MAX / 2
        let large_amount2 = 1000000000000000000u64; // 1 * 10^18
        
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(large_amount1), _ctx);
        bfc_system_state_inner::deposit_stable_gas_coin<BUSD>(system_state_v2, balance::create_for_testing<BUSD>(large_amount2), _ctx);
        
        let extra_fields = bfc_system_state_inner::get_extra_fields(system_state_v2);
        assert!(extra_fields.length() == 1, 1);

        let final_balance = bfc_system_state_inner::get_deposited_stable_gas_coin_balance<BUSD>(system_state_v2);
        assert!(final_balance == large_amount1 + large_amount2, 3); // Should equal the sum of both deposits

        
        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }
}
