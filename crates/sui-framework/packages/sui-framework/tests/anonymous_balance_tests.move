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
        let object_id : address = @0xd4c2360f11b1608f3be0b8d89bc97ff3047378dbc34d0b3976f40e6392496fd5;
        let signature = x"0f31177f8ece16b2cfb8c1ba0b71f73252acaa6cfbbe13d36c3320617f05bc7f9a860f16c8b10c787455a01ca7bcca3469858aae4e369bc994ab64967f1fd20f";
        let publickey = x"8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
        balance.destroy_zero(signature, object_id, publickey);

        let mut coin = anonymous_coin::mint_for_testing<ABFC>(100, scenario.ctx());
        let balance_mut = anonymous_coin::balance_mut(&mut coin);
        let sub_balance = balance_mut.split(50);

        debug::print(&sub_balance.get_encode_data());
        //debug::print(&coin.into_balance().get_encode_data());

        let mut balance = coin.into_balance();
        balance.join(sub_balance);

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

        let object_id : address = @0xd4c2360f11b1608f3be0b8d89bc97ff3047378dbc34d0b3976f40e6392496fd5;
        let signature = x"0f31177f8ece16b2cfb8c1ba0b71f73252acaa6cfbbe13d36c3320617f05bc7f9a860f16c8b10c787455a01ca7bcca3469858aae4e369bc994ab64967f1fd20f";
        let publickey = x"8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
        balance.destroy_zero(signature, object_id, publickey);

        test_utils::destroy(balance1);
        test_utils::destroy(balance2);
       test_utils::destroy(balance3);
    }
}
