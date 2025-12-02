// uncomment this line to import the module
// use anonymous_vault::anonymous_vault;


#[test_only]
module anonymous_vault::anonymous_vault_tests{
    const ENotImplemented: u64 = 0;
    use  anonymous_vault::anonymous_vault::{get_latest_action_num, new_for_testing, init_admin,create_testing_action};
    use sui::test_scenario::{Self};
    use std::debug;
    use std::string::{Self};
    use sui::clock;


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
// pass
}
#[test]
fun test_anonymous_vault_token_control(){
    // pass
//    let mut treasury = ausd::new_for_testing(scenario.ctx());
//    ausd::mint(&mut treasury, 20000, scenario.ctx());
//    scenario.next_epoch(@0); // needed or else we won't have a value for `most_recent_id_for_address` coming up next.
//
//    transfer::public_transfer(treasury, tx_context::sender(scenario.ctx()));
//    scenario.end();
}



}






