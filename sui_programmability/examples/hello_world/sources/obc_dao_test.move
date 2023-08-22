module hello_world::obc_dao_test {
    //use sui::transfer;
    //use sui::object::{Self, UID};
    //use sui::tx_context::{Self, TxContext};
    //use std::vector;
    //use std::string;
    //use sui::event;
    #[test_only]
    use sui::clock;
    #[test_only]
    use hello_world::obc_dao::{Dao, getOBCDaoActionId};
    #[test_only]
    use std::debug;


    #[test]
    public fun test_dao_init(){
        use sui::test_scenario;
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        //let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            obc_dao::create_dao(test_scenario::ctx(&mut scenario_val));
        };


        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let old_delay = obc_dao::voting_delay(&mut dao);
            debug::print(&old_delay);

            obc_dao::set_voting_delay(&mut dao, 99999, test_scenario::ctx(&mut scenario_val));
            let new_deley = obc_dao::voting_delay(&mut dao);

            debug::print(&new_deley);
            test_scenario::return_to_sender(&mut scenario_val, dao);

        };


        let data: vector<u8> = b"hello world";
        debug::print(&data);


        test_scenario::end(scenario_val);
    }


    #[test]
    public fun test_dao_proposal(){
        use sui::test_scenario;
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        //let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            obc_dao::create_dao(test_scenario::ctx(&mut scenario_val));
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let action = obc_dao::create_obcdao_action(&mut dao, b"hello world", test_scenario::ctx(&mut scenario_val));
            let actionId = getOBCDaoActionId(action);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            obc_dao::propose(&mut dao, actionId, 1000 * 60 * 60 * 24 * 7 + 1000, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);
            test_scenario::return_to_sender(&mut scenario_val, dao);

        };


            test_scenario::end(scenario_val);
    }

    //test_scenario
    #[test]
    #[expected_failure(abort_code = hello_world::obc_dao::ERR_NOT_AUTHORIZED)]
    public fun test_set_dao_init_fail(){
        use sui::test_scenario;
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        let user1 = @0xA1;
        //let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            obc_dao::create_dao(test_scenario::ctx(&mut scenario_val));
        };



        test_scenario::next_tx(&mut scenario_val, user1);
        {

            let dao = test_scenario::take_from_address<Dao>(&mut scenario_val, owner);
            let old_delay = obc_dao::voting_delay(&mut dao);
            debug::print(&old_delay);

            obc_dao::set_voting_delay(&mut dao, 33333, test_scenario::ctx(&mut scenario_val));
            let new_deley = obc_dao::voting_delay(&mut dao);

            debug::print(&new_deley);
            test_scenario::return_to_address(owner, dao);

        };

        let data: vector<u8> = b"hello world";
        debug::print(&data);


        test_scenario::end(scenario_val);
    }
}

//sui move test test_dao_init
//


