module anonymous_usd::anonymous_usd_tests{
// uncomment this line to import the module
use anonymous_usd::{anonymous_usd};
use sui::test_scenario::{Self};
use sui::anonymous_coin;

    #[test]
    fun test_anonymous_usd_mint() {
        let mut scenario = test_scenario::begin(@0);
        let mut treasury = anonymous_usd::new_for_testing(scenario.ctx());
        anonymous_usd::mint(&mut treasury, 20000, scenario.ctx());
        scenario.next_epoch(@0); // needed or else we won't have a value for `most_recent_id_for_address` coming up next.


        let coin = scenario.take_from_address<anonymous_coin::Anonymous_Coin<anonymous_usd::ANONYMOUS_USD>>(@0);

        let object_id : address = @0xd4c2360f11b1608f3be0b8d89bc97ff3047378dbc34d0b3976f40e6392496fd5;
        let signature = x"0f31177f8ece16b2cfb8c1ba0b71f73252acaa6cfbbe13d36c3320617f05bc7f9a860f16c8b10c787455a01ca7bcca3469858aae4e369bc994ab64967f1fd20f";
        let publickey = x"8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
        let value = coin.value(signature, object_id, publickey);
        assert!(value == 20000);

        transfer::public_transfer(treasury, tx_context::sender(scenario.ctx()));
        transfer::public_transfer(coin, tx_context::sender(scenario.ctx()));

        test_scenario::next_tx(&mut scenario, @0x0);
        scenario.end();
    }

    #[test]
     fun test_anonymous_usd_transfer() {
            let mut scenario = test_scenario::begin(@0);
            let mut treasury = anonymous_usd::new_for_testing(scenario.ctx());
            anonymous_usd::mint(&mut treasury, 20000, scenario.ctx());
            scenario.next_epoch(@0); // needed or else we won't have a value for `most_recent_id_for_address` coming up next.


            let coin = scenario.take_from_address<anonymous_coin::Anonymous_Coin<anonymous_usd::ANONYMOUS_USD>>(@0);

            let object_id : address = @0xd4c2360f11b1608f3be0b8d89bc97ff3047378dbc34d0b3976f40e6392496fd5;
            let signature = x"0f31177f8ece16b2cfb8c1ba0b71f73252acaa6cfbbe13d36c3320617f05bc7f9a860f16c8b10c787455a01ca7bcca3469858aae4e369bc994ab64967f1fd20f";
            let publickey = x"8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
            let value = coin.value(signature, object_id, publickey);
            assert!(value == 20000);
            anonymous_usd::transfer(coin, @0x2);
            transfer::public_transfer(treasury, tx_context::sender(scenario.ctx()));

            scenario.next_epoch(@0x2); // needed or else we won't have a value for `most_recent_id_for_address` coming up next.

            let coin2 = scenario.take_from_address<anonymous_coin::Anonymous_Coin<anonymous_usd::ANONYMOUS_USD>>(@0x2);
            let value = coin2.value(signature, object_id, publickey);
            assert!(value == 20000);

            transfer::public_transfer(coin2, tx_context::sender(scenario.ctx()));

            test_scenario::next_tx(&mut scenario, @0x0);
            scenario.end();
     }
}

