// uncomment this line to import the module
// use anonymous_vault::anonymous_vault;


#[test_only]
module anonymous_vault::anonymous_vault_tests{
    const ENotImplemented: u64 = 0;
    use  anonymous_vault::anonymous_vault::{get_latest_action_num,
        new_for_testing, init_admin,create_testing_action,admin_vote_for_action, deposit_contract_upgrade_cap_to_vault
        ,deposit_treasury_cap_to_vault, init_token_pool};

    use sui::test_scenario::{Self};
    use std::debug;
    use std::string::{Self};
    use sui::clock;
    use anonymous_vault::testausd;
    use anonymous_vault::testausc;

    use sui::anonymous_coin;
    use sui::package::{Self, UpgradeCap, UpgradeTicket};
    use anonymous_vault::testausd::{TESTAUSD};
    use anonymous_vault::testausc::{TESTAUSC};


#[test]
fun test_anonymous_vault() {
    // pass
    let mut scenario = test_scenario::begin(@0x0);

    let _vault = new_for_testing(scenario.ctx());
    let num = get_latest_action_num(&_vault);
    debug::print(&num);
    scenario.next_epoch(@0);
    transfer::public_transfer(_vault, tx_context::sender(scenario.ctx()));

    scenario.end();

}


#[test]
fun test_anonymous_vault_admin_init() {
    // pass
    let mut scenario = test_scenario::begin(@0x0);

    let mut _vault = new_for_testing(scenario.ctx());
    let num = get_latest_action_num(&_vault);
    debug::print(&num);
    scenario.next_epoch(@0);


    init_admin(@0x1, & mut _vault);
    init_admin(@0x2, & mut _vault);
    init_admin(@0x3, & mut _vault);
    init_admin(@0x4, & mut _vault);
    init_admin(@0x5, & mut _vault);

    transfer::public_transfer(_vault, tx_context::sender(scenario.ctx()));


    scenario.end();

}

#[test]
fun test_anonymous_vault_admin_after_init_manager_test() {
    // pass
    let mut scenario = test_scenario::begin(@0x0);

    let mut vault = new_for_testing(scenario.ctx());
    let num = get_latest_action_num(&vault);
    debug::print(&num);
    scenario.next_epoch(@0);


    init_admin(@0x1, & mut vault);
    init_admin(@0x2, & mut vault);
    init_admin(@0x3, & mut vault);
    init_admin(@0x4, & mut vault);
    init_admin(@0x5, & mut vault);

    let user1 = @0x1;
    let user2 = @0x2;
    let user3 = @0x3;



    let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));

    //remove admin 0x1
    let mut action  = create_testing_action(1, @0x1, string::utf8(b"object_key"), & mut vault, &clock, scenario.ctx());
    //vote for action
    //create   for user1
    test_scenario::next_tx(&mut scenario, user1);
    {
        admin_vote_for_action<TESTAUSD, TESTAUSD, TESTAUSD>( &mut action,&mut vault, &clock, test_scenario::ctx(&mut scenario));
    };
    test_scenario::next_tx(&mut scenario, user2);
    {
        admin_vote_for_action<TESTAUSD, TESTAUSD, TESTAUSD>( &mut action,&mut vault, &clock, test_scenario::ctx(&mut scenario));
    };
    test_scenario::next_tx(&mut scenario, user3);
    {
        admin_vote_for_action<TESTAUSD, TESTAUSD, TESTAUSD>( &mut action,&mut vault, &clock, test_scenario::ctx(&mut scenario));
    };

//    test_scenario::next_tx(&mut scenario, user1);
//    {
//        admin_vote_for_action<TESTAUSD, TESTAUSD, TESTAUSD>( &mut action,&mut vault, &clock, test_scenario::ctx(&mut scenario));
//    };
    transfer::public_transfer(vault, tx_context::sender(scenario.ctx()));

    transfer::public_transfer(action, tx_context::sender(scenario.ctx()));
    clock::destroy_for_testing(clock);
    scenario.end();
}

#[test]
fun test_anonymous_vault_action() {
    // pass
    let mut scenario = test_scenario::begin(@0x0);

    let mut _vault = new_for_testing(scenario.ctx());
    let num = get_latest_action_num(&_vault);
    scenario.next_epoch(@0);
    let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));

    let action  = create_testing_action(1, @0x1, string::utf8(b"object_key"), & mut _vault, &clock, scenario.ctx());


    transfer::public_transfer(_vault, tx_context::sender(scenario.ctx()));
    transfer::public_transfer(action, tx_context::sender(scenario.ctx()));

    clock::destroy_for_testing(clock);
    scenario.end();
}

#[test]
fun test_anonymous_vault_object_control() {
    //pass
    let mut scenario = test_scenario::begin(@0x1);
    let mut vault = new_for_testing(scenario.ctx());

    let mut upgradeCap = package::test_publish(@0x42.to_id(), scenario.ctx());
    let mut treasury = testausd::new_for_testing(scenario.ctx());
    testausd::mint(&mut treasury, 20000, scenario.ctx());
init_admin(@0x1, & mut vault);
init_admin(@0x2, & mut vault);
init_admin(@0x3, & mut vault);
init_admin(@0x4, & mut vault);
init_admin(@0x5, & mut vault);
    scenario.next_epoch(@0x1); // needed or else we won't have a value for `most_recent_id_for_address` coming up next.


    deposit_contract_upgrade_cap_to_vault(upgradeCap,string::utf8(b"object_key_upgrade"), &mut vault, scenario.ctx());
    deposit_treasury_cap_to_vault<TESTAUSD>(treasury,string::utf8(b"object_key_treasury"), &mut vault, scenario.ctx());


    let coin = scenario.take_from_address<anonymous_coin::Anonymous_Coin<testausd::TESTAUSD>>(@0x1);
    //transfer::public_transfer(treasury, tx_context::sender(scenario.ctx()));
    transfer::public_transfer(coin, tx_context::sender(scenario.ctx()));
    transfer::public_transfer(vault, tx_context::sender(scenario.ctx()));


    test_scenario::next_tx(&mut scenario, @0x1);
    scenario.end();
}
#[test]
fun test_anonymous_vault_token_control(){
    // pass
    let mut scenario = test_scenario::begin(@0x1);
    let mut vault = new_for_testing(scenario.ctx());

    let mut upgradeCap = package::test_publish(@0x42.to_id(), scenario.ctx());
    let mut treasury_1 = testausd::new_for_testing(scenario.ctx());
    testausd::mint(&mut treasury_1, 20000, scenario.ctx());
    init_admin(@0x1, & mut vault);


    scenario.next_epoch(@0x1); // needed or else we won't have a value for `most_recent_id_for_address` coming up next.
    let coin_1 = scenario.take_from_address<anonymous_coin::Anonymous_Coin<testausd::TESTAUSD>>(@0x1);

    let mut treasury_2 = testausc::new_for_testing(scenario.ctx());
    testausc::mint(&mut treasury_2, 20000, scenario.ctx());
    scenario.next_epoch(@0x1); // needed or else we won't have a value for `most_recent_id_for_address` coming up next.
    let coin_2 = scenario.take_from_address<anonymous_coin::Anonymous_Coin<testausc::TESTAUSC>>(@0x1);

    init_token_pool<TESTAUSD, TESTAUSC>(coin_1, coin_2, &mut vault, scenario.ctx());


    //transfer::public_transfer(coin_2, tx_context::sender(scenario.ctx()));
    transfer::public_transfer(treasury_1, tx_context::sender(scenario.ctx()));

    transfer::public_transfer(treasury_2, tx_context::sender(scenario.ctx()));


    //transfer::public_transfer(coin_1, tx_context::sender(scenario.ctx()));
    transfer::public_transfer(vault, tx_context::sender(scenario.ctx()));
    transfer::public_transfer(upgradeCap, tx_context::sender(scenario.ctx()));
    test_scenario::next_tx(&mut scenario, @0x1);
    scenario.end();

}



}






