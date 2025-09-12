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

