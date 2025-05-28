// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module sui::anonymous_coin_balance_tests {
    use std::debug;
    use sui::test_scenario;
    use sui::anonymous_pay;
    use sui::anonymous_coin;
    use sui::anonymous_balance;
    use sui::test_utils;

    use sui::abfc::ABFC;

    #[test]
    fun type_morphing() {
        let mut scenario = test_scenario::begin(@0x1);

        let balance = anonymous_balance::zero<ABFC>();
        let coin = balance.into_coin(scenario.ctx());
        let balance = coin.into_balance();

        balance.destroy_zero();

        let mut coin = anonymous_coin::mint_for_testing<ABFC>(100, scenario.ctx());
        let balance_mut = anonymous_coin::balance_mut(&mut coin);
        let sub_balance = balance_mut.split(50);

        assert!(sub_balance.value() == 50);
        assert!(coin.value() == 50);

        debug::print(&sub_balance.get_encode_data());
        //debug::print(&coin.into_balance().get_encode_data());

        let mut balance = coin.into_balance();
        balance.join(sub_balance);

        assert!(balance.value() == 100);

        let coin = balance.into_coin(scenario.ctx());
        anonymous_pay::keep(coin, scenario.ctx());
        scenario.end();
    }

    #[test]
    fun test_balance() {
        let mut balance = anonymous_balance::zero<ABFC>();
        debug::print(&b"balance before join()".to_string());
        debug::print(&balance.get_encode_data());
        let another = anonymous_balance::create_for_testing(1000);



        balance.join(another);

        assert!(balance.value() == 1000);
        debug::print(&b"balance after join()".to_string());
        debug::print(&balance.get_encode_data());
        debug::print(&balance.value1());
        debug::print(&balance.value2());


        let balance1 = balance.split(333);
        let balance2 = balance.split(333);
        let balance3 = balance.split(334);
        debug::print(&b"balance after split()".to_string());
        debug::print(&balance1.get_encode_data());
        debug::print(&balance3.get_encode_data());


        balance.destroy_zero();

        assert!(balance1.value() == 333);
        assert!(balance2.value() == 333);
        assert!(balance3.value() == 334);

        test_utils::destroy(balance1);
        test_utils::destroy(balance2);
        test_utils::destroy(balance3);
    }
}
