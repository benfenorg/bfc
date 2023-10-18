#[test_only]

module obc_system::exchange_inner_tests {
    use obc_system::busd::BUSD;
    use sui::test_scenario;
    use obc_system::exchange_inner;
    use sui::bfc::BFC;
    use sui::balance;
    use sui::coin;
    use sui::test_utils;
    use sui::test_utils::assert_eq;

    #[test]
    fun test_exchange_inner_flow() {
        let scenario_val = test_scenario::begin(@0x3);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        // new exchange pool
        let exchange_pool = exchange_inner::new_exchange_pool(ctx, 0);
        // init obc balance
        let obc = balance::create_for_testing<BFC>(10);
        exchange_inner::add_obc_to_pool(&mut exchange_pool, coin::from_balance(obc, ctx));
        assert!(exchange_inner::get_obc_amount(&exchange_pool) == 10, 100);
        // exchange where rate = 10000
        let stable = balance::create_for_testing<BUSD>(50000);
        let stable_coin = coin::from_balance(stable, ctx);
        let exchanged = exchange_inner::request_exchange_stable(10000, &mut exchange_pool, stable_coin, ctx);
        assert!(balance::value(&exchanged) == 5, 102);
        //check stable amout and obc amount
        assert!(exchange_inner::get_stable_amount(&exchange_pool) == 50000, 103);
        assert_eq(exchange_inner::get_obc_amount(&exchange_pool), 5);

        test_utils::destroy(exchanged);
        test_utils::destroy(exchange_pool);
        test_scenario::end(scenario_val);
    }
}