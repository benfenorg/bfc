module bfc_system::bfc_dao_voting_pool_test {
    #[test_only]
    use std::debug;
    #[test_only]
    use bfc_system::bfc_dao_manager::BFCDaoManageKey;
    #[test_only]
    use bfc_system::bfc_dao::Dao;
    #[test_only]
    use sui::bfc::BFC;
    #[test_only]
    use sui::coin;
    #[test_only]
    use bfc_system::voting_pool::{VotingBfc};
    #[test_only]
    use bfc_system::voting_pool;
    #[test_only]
    use sui::balance;
    #[test_only]
    use sui::clock;
    #[test_only]
    use sui::test_utils;



    #[test]
    public fun test_voting_pool_init() {
        use sui::test_scenario;
        use bfc_system::bfc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        //let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner];
            bfc_dao::create_dao_and_share(admins, test_scenario::ctx(&mut scenario_val));
            //transfer::share_object(dao);

        };


        //create voting bfc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let coin =  coin::mint_for_testing<BFC>(10000000000, test_scenario::ctx(&mut scenario_val));
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            bfc_dao::create_voting_bfc(&mut dao,  coin, &clock,  test_scenario::ctx(&mut scenario_val));


            clock::destroy_for_testing(clock);
            test_scenario::return_to_sender(&mut scenario_val, key);
            test_scenario::return_shared(dao);

        };



        //split bfc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vBfc = test_scenario::take_from_sender<VotingBfc>(&mut scenario_val );
            let splitBfc =  voting_pool::split(&mut vBfc, 3000000000, test_scenario::ctx(&mut scenario_val));
            let balance =  voting_pool::unwrap_voting_bfc(splitBfc);
            debug::print(&balance::value(&balance));
            assert!(balance::value(&balance) == 3000000000, 0);

            //let _ = balance;
            test_utils::destroy(balance);
            test_scenario::return_to_sender(&mut scenario_val, vBfc);
        };

        //unwrap voting bfc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vBfc = test_scenario::take_from_sender<VotingBfc>(&mut scenario_val );
            let balance =  voting_pool::unwrap_voting_bfc(vBfc);
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