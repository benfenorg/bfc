module obc_system::obc_dao_voting_pool_test{
    #[test_only]
    use std::debug;
    #[test_only]
    use obc_system::obc_dao_manager::OBCDaoManageKey;
    #[test_only]
    use obc_system::obc_dao::Dao;
    #[test_only]
    use sui::obc::OBC;
    #[test_only]
    use sui::coin;
    #[test_only]
    use obc_system::voting_pool::VotingObc;
    #[test_only]
    use obc_system::voting_pool;
    #[test_only]
    use sui::balance;
    #[test_only]
    use sui::test_utils;



    #[test]
    public fun test_voting_pool_init() {
        use sui::test_scenario;
        use obc_system::obc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        //let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner];
            obc_dao::create_dao_and_share(admins, test_scenario::ctx(&mut scenario_val));
            //transfer::share_object(dao);

        };


        //create voting obc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );
            let coin =  coin::mint_for_testing<OBC>(10000000000, test_scenario::ctx(&mut scenario_val));
            obc_dao::create_voting_obc(&mut dao,  coin, test_scenario::ctx(&mut scenario_val));



            test_scenario::return_to_sender(&mut scenario_val, key);
            test_scenario::return_shared(dao);

        };



        //split obc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vObc = test_scenario::take_from_sender<VotingObc>(&mut scenario_val );
            let splitObc =  voting_pool::split(&mut vObc, 3000000000, test_scenario::ctx(&mut scenario_val));
            let balance =  voting_pool::unwrap_voting_obc(splitObc);
            debug::print(&balance::value(&balance));
            assert!(balance::value(&balance) == 3000000000, 0);

            //let _ = balance;
            test_utils::destroy(balance);
            test_scenario::return_to_sender(&mut scenario_val, vObc);
        };

        //unwrap voting obc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vObc = test_scenario::take_from_sender<VotingObc>(&mut scenario_val );
            let balance =  voting_pool::unwrap_voting_obc(vObc);
            //assert!(balance::value(&balance) == 10, 0);
            debug::print(&balance::value(&balance));

            //let _ = balance;
            test_utils::destroy(balance);

        };


        let data: vector<u8> = b"hello world";
        debug::print(&data);


        test_scenario::end(scenario_val);
    }
}