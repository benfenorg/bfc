#[allow(unused_const,unused_mut_ref)]
module bfc_system::bfc_dao_test {
    #[test_only]
    use sui::clock;
    #[test_only]
    use bfc_system::bfc_dao::{Dao, Proposal, Vote, modify_proposal_obj, get_bfcdao_actionid};
    #[test_only]
    use std::debug;
    #[test_only]
    use bfc_system::bfc_dao_manager::BFCDaoManageKey;
    #[test_only]
    use sui::coin;
    #[test_only]
    use sui::bfc::BFC;
    #[test_only]
    use bfc_system::voting_pool::VotingBfc;


    /// Proposal state
    const PENDING: u8 = 1;
    const ACTIVE: u8 = 2;
    const DEFEATED: u8 = 3;
    const AGREED: u8 = 4;
    const QUEUED: u8 = 5;
    const EXECUTABLE: u8 = 6;
    const EXTRACTED: u8 = 7;


    #[test]
    public fun test_dao_init(){
        use sui::test_scenario;
        use bfc_system::bfc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        //let user2 = @0xB1;

        let mut scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner];
            bfc_dao::create_dao_and_share(admins, test_scenario::ctx( &mut scenario_val));
            //transfer::share_object(dao);
        };


        //set new delay, votingPeriod, quorum, min_action_delay
        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );

            let old_delay = bfc_dao::voting_delay(&mut dao);
            debug::print(&old_delay);
            bfc_dao::set_voting_delay(&mut dao, &key, 99999);
            let new_deley = bfc_dao::voting_delay(&mut dao);
            debug::print(&new_deley);
            assert!(new_deley == 99999, 0);

            let old_votingPeriod = bfc_dao::voting_period(&mut dao);
            debug::print(&old_votingPeriod);
            bfc_dao::set_voting_period(&mut dao, &key, 99999);
            let new_votingPeriod = bfc_dao::voting_period(&mut dao);
            debug::print(&new_votingPeriod);
            assert!(new_votingPeriod == 99999, 0);


            let old_quorum = bfc_dao::voting_quorum_rate(&mut dao);
            debug::print(&old_quorum);
            bfc_dao::set_voting_quorum_rate(&mut dao, &key, 9);
            let new_quorum = bfc_dao::voting_quorum_rate(&mut dao);
            debug::print(&new_quorum);
            assert!(new_quorum == 9, 0);


            let old_action_delay = bfc_dao::min_action_delay(&mut dao);
            debug::print(&old_action_delay);
            bfc_dao::set_min_action_delay(&mut dao, &key, 99999);
            let new_action_delay = bfc_dao::min_action_delay(&mut dao);
            debug::print(&new_action_delay);
            assert!(new_action_delay == 99999, 0);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);
            //test_scenario::return_to_sender(&mut scenario_val, dao);

        };


        let data: vector<u8> = b"hello world";
        debug::print(&data);


        test_scenario::end(scenario_val);
    }


    #[test]
    public fun test_dao_proposal(){
        use sui::test_scenario;
        use bfc_system::bfc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        //let user2 = @0xB1;

        let mut scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner];
            bfc_dao::create_dao_and_share(admins, test_scenario::ctx(&mut scenario_val));
            //transfer::share_object(dao);
        };
        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            //let _ = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let mint_new_action_cost: u64 = 10 * 1000000000; // 10 BFC
            let mut action_coin =  coin::mint_for_testing<BFC>(mint_new_action_cost, test_scenario::ctx(&mut scenario_val));
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            let action = bfc_dao::create_bfcdao_action(&mut dao,&mut action_coin, b"hello world",  &clock, test_scenario::ctx(&mut scenario_val));
            
            coin::destroy_zero(action_coin);
            
            let actionId = get_bfcdao_actionid(action);

            let min_new_propose_cost: u64 = 200 * 1000000000; // 200 BFC
            let mut coin =  coin::mint_for_testing<BFC>(min_new_propose_cost, test_scenario::ctx(&mut scenario_val));

            bfc_dao::propose(&mut dao, 19,  &mut coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));
            coin::destroy_zero(coin);

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            //test_scenario::return_to_sender(&mut scenario_val, key);


        };

        //get propose info, get propose status
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            bfc_dao::proposal_info(&mut p);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            bfc_dao::proposal_state(&mut p, &clock);
            test_scenario::return_shared(p);
            clock::destroy_for_testing(clock);


        };



        test_scenario::end(scenario_val);
    }

    //test_scenario
    #[test]
    //#[expected_failure(abort_code = bfc_system::bfc_dao::ERR_NOT_AUTHORIZED)]
    public fun test_propose_voting(){
        use sui::test_scenario;
        use bfc_system::bfc_dao;
        let owner = @0xC0FFEE;
        let user1 = @0xA1;
        let user2 = @0xB1;

        let mut scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            bfc_dao::create_dao_and_share(admins,test_scenario::ctx(&mut scenario_val));
        };
        //create voting bfc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let coin =  coin::mint_for_testing<BFC>(10000000000, test_scenario::ctx(&mut scenario_val));
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            bfc_dao::create_voting_bfc(&mut dao,  coin, &clock,test_scenario::ctx(&mut scenario_val));


            clock::destroy_for_testing(clock);
            test_scenario::return_to_sender(&mut scenario_val, key);
            test_scenario::return_shared(dao);

        };

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            let mint_new_action_cost: u64 = 10 * 1000000000; // 10 BFC

            let mut action_coin = coin::mint_for_testing<BFC>(mint_new_action_cost, test_scenario::ctx(&mut scenario_val));

            let action = bfc_dao::create_bfcdao_action(&mut dao,&mut action_coin, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));

            coin::destroy_zero(action_coin);

            let actionId = get_bfcdao_actionid(action);


            let min_new_propose_cost: u64 = 200 * 1000000000; // 200 BFC
            let mut coin =  coin::mint_for_testing<BFC>(min_new_propose_cost, test_scenario::ctx(&mut scenario_val));

            bfc_dao::propose(&mut dao, 19,  &mut coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));
            coin::destroy_zero(coin);

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //voting for
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let vBfc = test_scenario::take_from_sender<VotingBfc>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal_obj(&mut dao, &mut p, ACTIVE, &clock);


            //let amount = voting_pool::voting_bfc_amount(&mut vBfc);
            bfc_dao::cast_vote(&mut dao, &mut p, vBfc, 1, &clock, test_scenario::ctx(&mut scenario_val));
            //debug::print(&amount);

            let (forvote, agaistvote) = bfc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);
            test_scenario::return_shared(dao);

        };

        //create VotingBfc agaist for user1
        test_scenario::next_tx(&mut scenario_val, user1);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let coin =  coin::mint_for_testing<BFC>(100000000000, test_scenario::ctx(&mut scenario_val));
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            bfc_dao::create_voting_bfc(&mut dao,  coin, &clock,test_scenario::ctx(&mut scenario_val));
            clock::destroy_for_testing(clock);
            test_scenario::return_shared(dao);
        };
        //create VotingBfc agaist for user2
        test_scenario::next_tx(&mut scenario_val, user2);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let coin =  coin::mint_for_testing<BFC>(110000000000, test_scenario::ctx(&mut scenario_val));

            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            bfc_dao::create_voting_bfc(&mut dao,  coin, &clock, test_scenario::ctx(&mut scenario_val));
            clock::destroy_for_testing(clock);
            test_scenario::return_shared(dao);
        };



        //voting against : user1
        test_scenario::next_tx(&mut scenario_val, user1);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let vBfc = test_scenario::take_from_sender<VotingBfc>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal_obj(&mut dao, &mut p, ACTIVE, &clock);


            bfc_dao::cast_vote(&mut dao, &mut p, vBfc, 0, &clock, test_scenario::ctx(&mut scenario_val));

            let (forvote, agaistvote) = bfc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);
            test_scenario::return_shared(dao);

        };

        //has vote: true for user1
        test_scenario::next_tx(&mut scenario_val, user1);
        {
            let vote = test_scenario::take_from_sender<Vote>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let has = bfc_dao::has_vote(&vote, &mut p);
            debug::print(&has);


            test_scenario::return_shared(p);
            test_scenario::return_to_sender(&mut scenario_val, vote);
        };


        //create action, create propose, user2
        test_scenario::next_tx(&mut scenario_val, user2);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            let mint_new_action_cost: u64 = 10 * 1000000000; // 10 BFC
            let mut action_coin = coin::mint_for_testing<BFC>(mint_new_action_cost, test_scenario::ctx(&mut scenario_val));

            let action = bfc_dao::create_bfcdao_action(&mut dao,&mut action_coin, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));

            coin::destroy_zero(action_coin);
            let actionId = get_bfcdao_actionid(action);

            let min_new_propose_cost: u64 = 200 * 1000000000; // 200 BFC
            let mut coin =  coin::mint_for_testing<BFC>(min_new_propose_cost, test_scenario::ctx(&mut scenario_val));

            bfc_dao::propose(&mut dao, 19,  &mut coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));
            coin::destroy_zero(coin);

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        // //has vote: false for user2's propose
        test_scenario::next_tx(&mut scenario_val, user2);
        {
            let vote = test_scenario::take_from_address<Vote>(&mut scenario_val, user1);
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let has = bfc_dao::has_vote(&vote, &mut p);
            debug::print(&has);


            test_scenario::return_shared(p);
            test_scenario::return_to_address(user1, vote);
        };


        test_scenario::end(scenario_val);
    }


    #[test]
    public fun test_change_voting(){
        use sui::test_scenario;
        use bfc_system::bfc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        let user2 = @0xB1;

        let mut scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            bfc_dao::create_dao_and_share(admins,test_scenario::ctx(&mut scenario_val));
            //transfer::share_object(dao);
        };
        //create voting bfc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let coin =  coin::mint_for_testing<BFC>(10000000000, test_scenario::ctx(&mut scenario_val));
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            bfc_dao::create_voting_bfc(&mut dao,  coin, &clock, test_scenario::ctx(&mut scenario_val));


            clock::destroy_for_testing(clock);

            test_scenario::return_to_sender(&mut scenario_val, key);
            test_scenario::return_shared(dao);

        };

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );

            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            let mint_new_action_cost: u64 = 10 * 1000000000; // 10 BFC
            let mut action_coin = coin::mint_for_testing<BFC>(mint_new_action_cost, test_scenario::ctx(&mut scenario_val));

            let action = bfc_dao::create_bfcdao_action(&mut dao,&mut action_coin, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));

            coin::destroy_zero(action_coin);
            let actionId = get_bfcdao_actionid(action);


            let min_new_propose_cost: u64 = 200 * 1000000000; // 200 BFC
            let mut coin =  coin::mint_for_testing<BFC>(min_new_propose_cost, test_scenario::ctx(&mut scenario_val));

            bfc_dao::propose(&mut dao, 19,  &mut coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));
            coin::destroy_zero(coin);

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //voting for
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let vBfc = test_scenario::take_from_sender<VotingBfc>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );

            //change status
            modify_proposal_obj(&mut dao, &mut p, ACTIVE, &clock);
            //change status for dao inside proposal info.

            //let amount = voting_pool::voting_bfc_amount(&mut vBfc);
            bfc_dao::cast_vote(&mut dao, &mut p, vBfc, 1, &clock, test_scenario::ctx(&mut scenario_val));
            //debug::print(&amount);

            let (forvote, agaistvote) = bfc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);
            test_scenario::return_shared(dao);

        };

        //change vote
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let mut vote = test_scenario::take_from_sender<Vote>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status for proposal obj
            modify_proposal_obj(&mut dao, &mut p, ACTIVE, &clock);
            //change status for dao inside proposal info.

            let (forvote, agaistvote) = bfc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);
            assert!(forvote == 10000000000, 0);
            assert!(agaistvote == 0, 0);

            bfc_dao::change_vote(&mut dao, &mut vote, &mut p, false, &clock, test_scenario::ctx(&mut scenario_val));


            let (forvote, agaistvote) = bfc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);
            assert!(forvote == 0, 0);
            assert!(agaistvote == 10000000000, 0);



            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);
            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, vote);
        };

        test_scenario::end(scenario_val);
    }


    #[test]
    public fun test_revoke_voting(){
        use sui::test_scenario;
        use bfc_system::bfc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        let user2 = @0xB1;

        let mut scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            bfc_dao::create_dao_and_share(admins,test_scenario::ctx(&mut scenario_val));
            //transfer::share_object(dao);
        };
        //create voting bfc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let coin =  coin::mint_for_testing<BFC>(10000000000, test_scenario::ctx(&mut scenario_val));
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            bfc_dao::create_voting_bfc(&mut dao,  coin, &clock, test_scenario::ctx(&mut scenario_val));


            clock::destroy_for_testing(clock);
            test_scenario::return_to_sender(&mut scenario_val, key);
            test_scenario::return_shared(dao);

        };

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );

            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            let mint_new_action_cost: u64 = 10 * 1000000000; // 10 BFC
            let mut action_coin = coin::mint_for_testing<BFC>(mint_new_action_cost, test_scenario::ctx(&mut scenario_val));

            let action = bfc_dao::create_bfcdao_action(&mut dao,&mut action_coin, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));
            coin::destroy_zero(action_coin);

            let actionId = get_bfcdao_actionid(action);


            let min_new_propose_cost: u64 = 200 * 1000000000; // 200 BFC
            let mut coin =  coin::mint_for_testing<BFC>(min_new_propose_cost, test_scenario::ctx(&mut scenario_val));

            bfc_dao::propose(&mut dao, 19,  &mut coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));
            coin::destroy_zero(coin);

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //voting for
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let vBfc = test_scenario::take_from_sender<VotingBfc>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal_obj(&mut dao,&mut p, ACTIVE, &clock);

            bfc_dao::cast_vote(&mut dao, &mut p, vBfc, 1, &clock, test_scenario::ctx(&mut scenario_val));

            let (forvote, agaistvote) = bfc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);


            clock::destroy_for_testing(clock);
            test_scenario::return_shared(dao);
            test_scenario::return_shared(p);


        };
        //revoke voting
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let vote = test_scenario::take_from_sender<Vote>(&mut scenario_val );

            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal_obj(&mut dao, &mut p, ACTIVE, &clock);


            bfc_dao::vote_of(&vote, &mut p,test_scenario::ctx(&mut scenario_val));

            bfc_dao::revoke_vote(&mut dao, &mut p, vote, 1000000000, &clock, test_scenario::ctx(&mut scenario_val));


            let (forvote, agaistvote) = bfc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(dao);
            test_scenario::return_shared(p);

        };

        test_scenario::end(scenario_val);
    }

    #[test]
    public fun test_unvote_voting(){
        use sui::test_scenario;
        use bfc_system::bfc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        let user2 = @0xB1;

        let mut scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            bfc_dao::create_dao_and_share(admins,test_scenario::ctx(&mut scenario_val));
            //transfer::share_object(dao);
        };
        //create voting bfc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let coin =  coin::mint_for_testing<BFC>(10000000000, test_scenario::ctx(&mut scenario_val));
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            bfc_dao::create_voting_bfc(&mut dao,  coin, &clock, test_scenario::ctx(&mut scenario_val));


            clock::destroy_for_testing(clock);
            test_scenario::return_to_sender(&mut scenario_val, key);
            test_scenario::return_shared(dao);

        };

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );

           let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            let mint_new_action_cost: u64 = 10 * 1000000000; // 10 BFC
            let mut action_coin = coin::mint_for_testing<BFC>(mint_new_action_cost, test_scenario::ctx(&mut scenario_val));

            let action = bfc_dao::create_bfcdao_action(&mut dao,&mut action_coin, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));

            coin::destroy_zero(action_coin);
            let actionId = get_bfcdao_actionid(action);

            let min_new_propose_cost: u64 = 200 * 1000000000; // 200 BFC
            let mut coin =  coin::mint_for_testing<BFC>(min_new_propose_cost, test_scenario::ctx(&mut scenario_val));

            bfc_dao::propose(&mut dao, 19,  &mut coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));
            coin::destroy_zero(coin);

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //voting for
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let vBfc = test_scenario::take_from_sender<VotingBfc>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal_obj(&mut dao, &mut p, ACTIVE, &clock);

            bfc_dao::cast_vote(&mut dao, &mut p, vBfc, 1, &clock, test_scenario::ctx(&mut scenario_val));

            let (forvote, agaistvote) = bfc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);
            test_scenario::return_shared(dao);


        };
        //unstake voting
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let vote = test_scenario::take_from_sender<Vote>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );

            //change status
            modify_proposal_obj(&mut dao, &mut p, AGREED, &clock);

            bfc_dao::unvote_votes(&mut p, vote, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);
            test_scenario::return_shared(dao);
        };
        test_scenario::end(scenario_val);

    }

    #[test]
    public fun test_other_proposal_action(){
        use sui::test_scenario;
        use bfc_system::bfc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        let user2 = @0xB1;

        let mut scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            bfc_dao::create_dao_and_share(admins,test_scenario::ctx(&mut scenario_val));
            //transfer::share_object(dao);
        };
        //create voting bfc
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let coin =  coin::mint_for_testing<BFC>(10000000000, test_scenario::ctx(&mut scenario_val));
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            bfc_dao::create_voting_bfc(&mut dao,  coin, &clock, test_scenario::ctx(&mut scenario_val));


            clock::destroy_for_testing(clock);
            test_scenario::return_to_sender(&mut scenario_val, key);
            test_scenario::return_shared(dao);

        };

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );

           let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            let mint_new_action_cost: u64 = 10 * 1000000000; // 10 BFC
            let mut action_coin = coin::mint_for_testing<BFC>(mint_new_action_cost, test_scenario::ctx(&mut scenario_val));

            let action = bfc_dao::create_bfcdao_action(&mut dao,&mut action_coin, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));

            coin::destroy_zero(action_coin);
            let actionId = get_bfcdao_actionid(action);


            let min_new_propose_cost: u64 = 200 * 1000000000; // 200 BFC
            let mut coin =  coin::mint_for_testing<BFC>(min_new_propose_cost, test_scenario::ctx(&mut scenario_val));

            bfc_dao::propose(&mut dao, 19,  &mut coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, b"hello world", &clock, test_scenario::ctx(&mut scenario_val));
            coin::destroy_zero(coin);

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //other proposal action::queue_proposal_action
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );

            modify_proposal_obj(&mut dao, &mut p, AGREED, &clock);
            bfc_dao::queue_proposal_action(&mut dao, &key, &mut p, &clock);
            test_scenario::return_shared(dao);
            test_scenario::return_shared(p);
            test_scenario::return_to_sender(&mut scenario_val, key);
            clock::destroy_for_testing(clock);
        };

        //other proposal action::terminal
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let key = test_scenario::take_from_sender<BFCDaoManageKey>(&mut scenario_val );
            let mut p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let mut dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );

            modify_proposal_obj(&mut dao, &mut p, EXTRACTED, &clock);

            //bfc_dao::queue_proposal_action(&key, &mut p, &clock);
            bfc_dao::destroy_terminated_proposal(&mut dao, &key, &mut p, &clock);


            test_scenario::return_shared(dao);
            test_scenario::return_shared(p);
            test_scenario::return_to_sender(&mut scenario_val, key);
            clock::destroy_for_testing(clock);
        };


        test_scenario::end(scenario_val);
    }
}

//bfc move test test_dao_init
//


