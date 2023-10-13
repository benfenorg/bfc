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
    use hello_world::obc_dao::{Dao, getOBCDaoActionId, Proposal, modify_proposal, Vote};
    #[test_only]
    use std::debug;
    #[test_only]
    use hello_world::obc_dao_manager::OBCDaoManageKey;
    #[test_only]
    use sui::coin;
    #[test_only]
    use sui::bfc::BFC;
    #[test_only]
    use hello_world::voting_pool::VotingObc;
    //#[test_only]
    //use sui::obc;

    #[test_only]
    //use hello_world::voting_pool;


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
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        //let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner];
            obc_dao::create_dao(admins, test_scenario::ctx( &mut scenario_val));
        };


        //set new delay, votingPeriod, quorum, min_action_delay
        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );

            let old_delay = obc_dao::voting_delay(&mut dao);
            debug::print(&old_delay);
            obc_dao::set_voting_delay(&mut dao, &key, 99999);
            let new_deley = obc_dao::voting_delay(&mut dao);
            debug::print(&new_deley);
            assert!(new_deley == 99999, 0);

            let old_votingPeriod = obc_dao::voting_period(&mut dao);
            debug::print(&old_votingPeriod);
            obc_dao::set_voting_period(&mut dao, &key, 99999);
            let new_votingPeriod = obc_dao::voting_period(&mut dao);
            debug::print(&new_votingPeriod);
            assert!(new_votingPeriod == 99999, 0);


            let old_quorum = obc_dao::voting_quorum_rate(&mut dao);
            debug::print(&old_quorum);
            obc_dao::set_voting_quorum_rate(&mut dao, &key, 9);
            let new_quorum = obc_dao::voting_quorum_rate(&mut dao);
            debug::print(&new_quorum);
            assert!(new_quorum == 9, 0);


            let old_action_delay = obc_dao::min_action_delay(&mut dao);
            debug::print(&old_action_delay);
            obc_dao::set_min_action_delay(&mut dao, &key, 99999);
            let new_action_delay = obc_dao::min_action_delay(&mut dao);
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
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        //let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner];
            obc_dao::create_dao(admins, test_scenario::ctx(&mut scenario_val));
        };
        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );

            let action = obc_dao::create_obcdao_action(&mut dao,&key, b"hello world", test_scenario::ctx(&mut scenario_val));
            let actionId = getOBCDaoActionId(action);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));


            let coin =  coin::mint_for_testing<OBC>(200000000000, test_scenario::ctx(&mut scenario_val));
            //let coin =  coin::mint_for_testing<OBC>(100000000000, test_scenario::ctx(&mut scenario_val));

            obc_dao::propose(&mut dao,&key, coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };

        //get propose info, get propose status
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            obc_dao::proposal_info(&mut p);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));

            obc_dao::proposal_state(&mut p, &clock);
            test_scenario::return_shared(p);
            clock::destroy_for_testing(clock);


        };



        test_scenario::end(scenario_val);
    }

    //test_scenario
    #[test]
    //#[expected_failure(abort_code = hello_world::obc_dao::ERR_NOT_AUTHORIZED)]
    public fun test_propose_voting(){
        use sui::test_scenario;
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        let user1 = @0xA1;
        let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            obc_dao::create_dao(admins,test_scenario::ctx(&mut scenario_val));
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

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );

            let action = obc_dao::create_obcdao_action(&mut dao,&key, b"hello world", test_scenario::ctx(&mut scenario_val));
            let actionId = getOBCDaoActionId(action);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));


            let coin =  coin::mint_for_testing<OBC>(200000000000, test_scenario::ctx(&mut scenario_val));
            //let coin =  coin::mint_for_testing<OBC>(100000000000, test_scenario::ctx(&mut scenario_val));

            obc_dao::propose(&mut dao,&key, coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //voting for
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vObc = test_scenario::take_from_sender<VotingObc>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal(&mut p, ACTIVE, &clock);


            //let amount = voting_pool::voting_obc_amount(&mut vObc);
            obc_dao::cast_vote(&mut p, vObc, 1, &clock, test_scenario::ctx(&mut scenario_val));
            //debug::print(&amount);

            let (forvote, agaistvote) = obc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);

        };

        //create votingobc agaist for user1
        test_scenario::next_tx(&mut scenario_val, user1);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let coin =  coin::mint_for_testing<OBC>(100000000000, test_scenario::ctx(&mut scenario_val));
            obc_dao::create_voting_obc(&mut dao,  coin, test_scenario::ctx(&mut scenario_val));

            test_scenario::return_shared(dao);
        };
        //create votingobc agaist for user2
        test_scenario::next_tx(&mut scenario_val, user2);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let coin =  coin::mint_for_testing<OBC>(110000000000, test_scenario::ctx(&mut scenario_val));
            obc_dao::create_voting_obc(&mut dao,  coin, test_scenario::ctx(&mut scenario_val));

            test_scenario::return_shared(dao);
        };



        //voting against : user1
        test_scenario::next_tx(&mut scenario_val, user1);
        {
            let vObc = test_scenario::take_from_sender<VotingObc>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal(&mut p, ACTIVE, &clock);


            obc_dao::cast_vote(&mut p, vObc, 0, &clock, test_scenario::ctx(&mut scenario_val));

            let (forvote, agaistvote) = obc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);

        };

        //has vote: true for user1
        test_scenario::next_tx(&mut scenario_val, user1);
        {
            let vote = test_scenario::take_from_sender<Vote>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let has = obc_dao::has_vote(&vote, &mut p);
            debug::print(&has);


            test_scenario::return_shared(p);
            test_scenario::return_to_sender(&mut scenario_val, vote);
        };


        //create action, create propose, user2
        test_scenario::next_tx(&mut scenario_val, user2);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );

            let action = obc_dao::create_obcdao_action(&mut dao,&key, b"hello world", test_scenario::ctx(&mut scenario_val));
            let actionId = getOBCDaoActionId(action);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));


            let coin =  coin::mint_for_testing<OBC>(200000000000, test_scenario::ctx(&mut scenario_val));
            //let coin =  coin::mint_for_testing<OBC>(100000000000, test_scenario::ctx(&mut scenario_val));

            obc_dao::propose(&mut dao,&key, coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        // //has vote: false for user2's propose
        test_scenario::next_tx(&mut scenario_val, user2);
        {
            let vote = test_scenario::take_from_address<Vote>(&mut scenario_val, user1);
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let has = obc_dao::has_vote(&vote, &mut p);
            debug::print(&has);


            test_scenario::return_shared(p);
            test_scenario::return_to_address(user1, vote);
        };


        test_scenario::end(scenario_val);
    }


    #[test]
    public fun test_change_voting(){
        use sui::test_scenario;
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            obc_dao::create_dao(admins,test_scenario::ctx(&mut scenario_val));
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

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );

            let action = obc_dao::create_obcdao_action(&mut dao,&key, b"hello world", test_scenario::ctx(&mut scenario_val));
            let actionId = getOBCDaoActionId(action);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));


            let coin =  coin::mint_for_testing<OBC>(200000000000, test_scenario::ctx(&mut scenario_val));
            //let coin =  coin::mint_for_testing<OBC>(100000000000, test_scenario::ctx(&mut scenario_val));

            obc_dao::propose(&mut dao,&key, coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //voting for
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vObc = test_scenario::take_from_sender<VotingObc>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal(&mut p, ACTIVE, &clock);


            //let amount = voting_pool::voting_obc_amount(&mut vObc);
            obc_dao::cast_vote(&mut p, vObc, 1, &clock, test_scenario::ctx(&mut scenario_val));
            //debug::print(&amount);

            let (forvote, agaistvote) = obc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);

        };

        //change vote
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vote = test_scenario::take_from_sender<Vote>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);

            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );

            let (forvote, agaistvote) = obc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);
            assert!(forvote == 200000000000, 0);
            assert!(agaistvote == 0, 0);

            obc_dao::change_vote(&mut vote, &mut p, false, &clock, test_scenario::ctx(&mut scenario_val));


            let (forvote, agaistvote) = obc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);
            assert!(forvote == 0, 0);
            assert!(agaistvote == 200000000000, 0);



            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);
            test_scenario::return_to_sender(&mut scenario_val, vote);
        };

        test_scenario::end(scenario_val);
    }


    #[test]
    public fun test_revoke_voting(){
        use sui::test_scenario;
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            obc_dao::create_dao(admins,test_scenario::ctx(&mut scenario_val));
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

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );

            let action = obc_dao::create_obcdao_action(&mut dao,&key, b"hello world", test_scenario::ctx(&mut scenario_val));
            let actionId = getOBCDaoActionId(action);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));


            let coin =  coin::mint_for_testing<OBC>(200000000000, test_scenario::ctx(&mut scenario_val));
            //let coin =  coin::mint_for_testing<OBC>(100000000000, test_scenario::ctx(&mut scenario_val));

            obc_dao::propose(&mut dao,&key, coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //voting for
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vObc = test_scenario::take_from_sender<VotingObc>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal(&mut p, ACTIVE, &clock);

            obc_dao::cast_vote(&mut p, vObc, 1, &clock, test_scenario::ctx(&mut scenario_val));

            let (forvote, agaistvote) = obc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);


            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);


        };
        //revoke voting
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vote = test_scenario::take_from_sender<Vote>(&mut scenario_val );

            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal(&mut p, ACTIVE, &clock);


            obc_dao::vote_of(&vote, &mut p,test_scenario::ctx(&mut scenario_val));

            obc_dao::revoke_vote(&mut p, vote, 1000000000, &clock, test_scenario::ctx(&mut scenario_val));


            let (forvote, agaistvote) = obc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);

        };

        test_scenario::end(scenario_val);
    }

    #[test]
    public fun test_unvote_voting(){
        use sui::test_scenario;
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            obc_dao::create_dao(admins,test_scenario::ctx(&mut scenario_val));
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

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );

            let action = obc_dao::create_obcdao_action(&mut dao,&key, b"hello world", test_scenario::ctx(&mut scenario_val));
            let actionId = getOBCDaoActionId(action);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));


            let coin =  coin::mint_for_testing<OBC>(200000000000, test_scenario::ctx(&mut scenario_val));
            //let coin =  coin::mint_for_testing<OBC>(100000000000, test_scenario::ctx(&mut scenario_val));

            obc_dao::propose(&mut dao,&key, coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //voting for
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vObc = test_scenario::take_from_sender<VotingObc>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            //let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );
            //change status
            modify_proposal(&mut p, ACTIVE, &clock);

            obc_dao::cast_vote(&mut p, vObc, 1, &clock, test_scenario::ctx(&mut scenario_val));

            let (forvote, agaistvote) = obc_dao::proposal_info(&mut p);
            debug::print(&forvote);
            debug::print(&agaistvote);

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);


        };
        //unstake voting
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let vote = test_scenario::take_from_sender<Vote>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);

            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );

            //change status
            modify_proposal(&mut p, AGREED, &clock);


            obc_dao::unvote_votes(&mut p, vote, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);
            test_scenario::return_shared(p);
        };
        test_scenario::end(scenario_val);

    }

    #[test]
    public fun test_other_proposal_action(){
        use sui::test_scenario;
        use hello_world::obc_dao;
        let owner = @0xC0FFEE;
        //let user1 = @0xA1;
        let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        //create dao
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let admins = vector[owner,user2];
            obc_dao::create_dao(admins,test_scenario::ctx(&mut scenario_val));
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

        //create action, create propose
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);
            //let dao = test_scenario::take_from_sender<Dao>(&mut scenario_val);
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );

            let action = obc_dao::create_obcdao_action(&mut dao,&key, b"hello world", test_scenario::ctx(&mut scenario_val));
            let actionId = getOBCDaoActionId(action);
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));


            let coin =  coin::mint_for_testing<OBC>(200000000000, test_scenario::ctx(&mut scenario_val));
            //let coin =  coin::mint_for_testing<OBC>(100000000000, test_scenario::ctx(&mut scenario_val));

            obc_dao::propose(&mut dao,&key, coin,  actionId, 1000 * 60 * 60 * 24 * 7 + 1000, &clock, test_scenario::ctx(&mut scenario_val));

            clock::destroy_for_testing(clock);

            test_scenario::return_shared(dao);
            test_scenario::return_to_sender(&mut scenario_val, key);


        };
        //other proposal action::queue_proposal_action
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);

            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );

            modify_proposal(&mut p, AGREED, &clock);
            obc_dao::queue_proposal_action(&key, &mut p, &clock);
            test_scenario::return_shared(p);
            test_scenario::return_to_sender(&mut scenario_val, key);
            clock::destroy_for_testing(clock);
        };

        //other proposal action::terminal
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let key = test_scenario::take_from_sender<OBCDaoManageKey>(&mut scenario_val );
            let p = test_scenario::take_shared<Proposal>(&mut scenario_val);
            let dao = test_scenario::take_shared<Dao>(&mut scenario_val);

            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario_val));
            clock::set_for_testing(&mut clock, 1000000000 * 60 );

            modify_proposal(&mut p, EXTRACTED, &clock);

            //obc_dao::queue_proposal_action(&key, &mut p, &clock);
            obc_dao::destroy_terminated_proposal(&mut dao, &key, &mut p, &clock);


            test_scenario::return_shared(dao);
            test_scenario::return_shared(p);
            test_scenario::return_to_sender(&mut scenario_val, key);
            clock::destroy_for_testing(clock);
        };


        test_scenario::end(scenario_val);
    }
}

//sui move test test_dao_init
//


