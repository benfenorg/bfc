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
        let signature = x"cd5f94646b13eaa370a55fe9c084d6b266e1c3856c16e43fbc8b2e9a28076ffe7b73fec594974a0ff7a7ebac2cf9ad2196ff89fdd97c14c0883159ec0181730c";
        let publickey = x"8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
        let object_id : address = @0x33a2598b7c5e22d03967b42671926c5c18e82f0be9973c077041e9912696f910;

        let mut scenario = test_scenario::begin(@0x1);

        let balance = anonymous_balance::zero<ABFC>();
        let coin = balance.into_coin(scenario.ctx());
        let balance = coin.into_balance();

        balance.destroy_zero(signature, object_id, publickey);

        let mut coin = anonymous_coin::mint_for_testing<ABFC>(100, scenario.ctx());
        let balance_mut = anonymous_coin::balance_mut(&mut coin);
        let sub_balance = balance_mut.split(50);

        assert!(sub_balance.value(signature, object_id, publickey) == 50);
        assert!(coin.value(signature, object_id, publickey) == 50);

        debug::print(&sub_balance.get_encode_data());
        //debug::print(&coin.into_balance().get_encode_data());

        let mut balance = coin.into_balance();
        balance.join(sub_balance);

        assert!(balance.value(signature, object_id, publickey) == 100);

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

        let signature = x"cd5f94646b13eaa370a55fe9c084d6b266e1c3856c16e43fbc8b2e9a28076ffe7b73fec594974a0ff7a7ebac2cf9ad2196ff89fdd97c14c0883159ec0181730c";
        let publickey = x"8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
        let object_id : address = @0x33a2598b7c5e22d03967b42671926c5c18e82f0be9973c077041e9912696f910;

        balance.join(another);

        assert!(balance.value(signature, object_id, publickey) == 1000);
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


        balance.destroy_zero(signature, object_id, publickey);

        assert!(balance1.value(signature, object_id, publickey) == 333);
        assert!(balance2.value(signature, object_id, publickey) == 333);
        assert!(balance3.value(signature, object_id, publickey) == 334);

        test_utils::destroy(balance1);
        test_utils::destroy(balance2);
       test_utils::destroy(balance3);
    }
}
