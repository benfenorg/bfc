#[test_only]
module bfc_system::bfc_system_tests {

    use std::ascii;
    use std::debug;
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
    use sui::vec_map::{Self};

    use bfc_system::bfc_system;
    use bfc_system::bfc_system::BfcSystemState;
    use bfc_system::test_utils;
    use bfc_system::bfc_system_state_inner;


    #[test]
    fun test_round() {
        let bfc_addr = @0x0;
        let scenario_val = test_scenario::begin(bfc_addr);
        test_utils::setup_without_parameters(&mut scenario_val, bfc_addr);
        let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
        clock::increment_for_testing(&mut clock, 3600 * 4 * 1000 + 1000);
        let t = test_scenario::take_shared<Treasury>(&scenario_val);
        treasury::rebalance(&mut t, 0, &clock, test_scenario::ctx(&mut scenario_val));

        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        create_sui_system_state_for_testing(ctx);
        test_scenario::next_tx(scenario, bfc_addr);
        let system_state = test_scenario::take_shared<BfcSystemState>(scenario);

        bfc_system::bfc_round(&mut system_state, &clock, 0,test_scenario::ctx(scenario));

        test_scenario::return_shared(system_state);
        test_scenario::return_shared(t);
        clock::destroy_for_testing(clock);
        test_scenario::end(scenario_val);
    }

    public fun create_sui_system_state_for_testing(ctx: &mut TxContext) {
        let treasury_parameters = vec_map::empty<ascii::String, bfc_system_state_inner::TreasuryParameters>();
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
            balance::create_for_testing<BFC>(0),
            bfc_system::busd::new_for_test(ctx),
            bfc_system::bjpy::new_for_test(ctx),
            bfc_system::bkrw::new_for_test(ctx),
            bfc_system::baud::new_for_test(ctx),
            bfc_system::bars::new_for_test(ctx),
            bfc_system::bbrl::new_for_test(ctx),
            bfc_system::bcad::new_for_test(ctx),
            bfc_system::beur::new_for_test(ctx),
            bfc_system::bgbp::new_for_test(ctx),
            bfc_system::bidr::new_for_test(ctx),
            bfc_system::binr::new_for_test(ctx),
            bfc_system::brub::new_for_test(ctx),
            bfc_system::bsar::new_for_test(ctx),
            bfc_system::btry::new_for_test(ctx),
            bfc_system::bzar::new_for_test(ctx),
            bfc_system::bmxn::new_for_test(ctx),
            bfc_system::mgg::new_for_test(ctx),
            bfc_system_state_inner::bfc_system_parameters(
                3600 * 4,
                2000,
                treasury_parameters,
            ),
            ctx,
        );
    }

    fun setup(): Scenario {
        let bfc_addr = @0x0;
        let scenario_val = test_scenario::begin(bfc_addr);

        create_sui_system_state_for_testing(test_scenario::ctx(&mut scenario_val));
        test_scenario::next_tx(&mut scenario_val, bfc_addr);
        scenario_val
    }

    fun tearDown(s: Scenario) {
        test_scenario::end(s);
    }

    #[test]
    fun test_next_epoch_bfc_required() {
        let scenario_val = setup();

        let system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);

        let amount = bfc_system::next_epoch_bfc_required(&system_state);
        // basepoint = 1000 /  position = 9 / timeinterval=4h
        let total = (
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

        debug::print(&total);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = bfc_system::treasury::ERR_INSUFFICIENT)]
    fun test_deposit_with_error() {
        let scenario_val = setup();
        let system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let bfc = balance::create_for_testing<BFC>(100);
        let current_balance = bfc_system::treasury_balance(&system_state);
        assert!(current_balance == 0, 1);
        bfc_system::deposit_to_treasury(
            &mut system_state,
            coin::from_balance(bfc, test_scenario::ctx(&mut scenario_val)),
        );

        let new_balance = bfc_system::treasury_balance(&system_state);
        assert!(new_balance == 0, 1);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }

    #[test]
    fun test_deposit_success() {
        let scenario_val = setup();
        let system_state = test_scenario::take_shared<BfcSystemState>(&mut scenario_val);
        let amount = bfc_system::next_epoch_bfc_required(&system_state);
        let bfc = balance::create_for_testing<BFC>(amount);
        let current_balance = bfc_system::treasury_balance(&system_state);
        assert!(current_balance == 0, 2);

        bfc_system::deposit_to_treasury(
            &mut system_state,
            coin::from_balance(bfc, test_scenario::ctx(&mut scenario_val)),
        );

        let new_balance = bfc_system::treasury_balance(&system_state);
        assert!(new_balance == amount, 3);

        test_scenario::return_shared(system_state);
        tearDown(scenario_val);
    }
}