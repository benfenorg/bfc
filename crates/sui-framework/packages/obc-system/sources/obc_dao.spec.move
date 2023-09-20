spec obc_system::obc_dao{
    spec voting_delay {
        aborts_if false;
    }


    spec voting_period {
        aborts_if false;
    }


    spec min_action_delay {
        aborts_if false;
        //ensures result == spec_dao_config.min_action_delay;
    }

    spec revoke_vote {
        aborts_if false;
    }

    spec schema NewDaoConfigParamSchema {
        voting_delay: u64;
        voting_period: u64;
        voting_quorum_rate: u8;
        min_action_delay: u64;

        aborts_if voting_delay > MAX_TIME_PERIOD || voting_delay == 0;
        aborts_if voting_period > MAX_TIME_PERIOD || voting_period == 0 ;
        aborts_if voting_quorum_rate == 0 || voting_quorum_rate > 100;
        aborts_if min_action_delay > MAX_TIME_PERIOD || min_action_delay == 0;
    }

    spec new_dao_config{
        include NewDaoConfigParamSchema;
    }

    spec withdraw_voting {
        aborts_if false;
        aborts_if pool_id(voting_obc) != object::id(dao.voting_pool);
        aborts_if ctx.ids_created + 1 > MAX_U64;

    }

    spec create_obcdao_action{
        aborts_if false;

        let action_id = dao.info.next_action_id + 1;
        aborts_if action_id  > MAX_U64;
        //aborts_if vec_map::contains(dao.action_record, action_id) == true;
    }

    spec add_admin {
        aborts_if false;
        aborts_if ctx.ids_created + 1 > MAX_U64;
    }
    spec create_dao {
        aborts_if false;
        aborts_if vector::length(admins) > MAX_ADMIN_COUNT || vector::length(admins) == 0;
        aborts_if ctx.ids_created + 3 > MAX_U64;
    }

    spec set_admins {

        aborts_if vector::length(new_admins) > MAX_ADMIN_COUNT || vector::length(new_admins) == 0;
        aborts_if ctx.ids_created + vector::length(new_admins) > MAX_U64;

    }


    spec vote_of{
        aborts_if false;
        aborts_if vote.proposer != proposal.proposal.proposer;
        aborts_if vote.vid != proposal.proposal.pid;
    }
    spec unvote_votes {
        aborts_if false;
        aborts_if vote.proposer != proposal.proposal.proposer;
        aborts_if vote.vid != proposal.proposal.pid;
        let current_time = clock.timestamp_ms;
        aborts_if judge_proposal_state(proposal.proposal,current_time) <= ACTIVE;
    }
    spec synchronize_proposal_into_dao{
        aborts_if false;
        pragma aborts_if_is_partial = true;
    }
    spec set_voting_quorum_rate{
        aborts_if false;
        aborts_if value > 100 || value == 0;
    }
    spec set_voting_period{
        aborts_if false;
        aborts_if value == 0;
        aborts_if value > MAX_TIME_PERIOD;
    }

    spec set_voting_delay{
        aborts_if false;
        aborts_if value == 0;
        aborts_if value > MAX_TIME_PERIOD;
    }
    spec set_min_action_delay{
        aborts_if false;
        aborts_if value == 0;
        aborts_if value > MAX_TIME_PERIOD;
    }
    spec set_current_status_into_dao{
        aborts_if false;
        pragma aborts_if_is_partial = true;
    }



    spec change_vote {
        aborts_if false;
        let current_time = clock.timestamp_ms;
        aborts_if judge_proposal_state(proposal.proposal,current_time) != ACTIVE;
        aborts_if my_vote.proposer != proposal.proposal.proposer;
        aborts_if my_vote.vid != proposal.proposal.pid;



    }
    spec do_flip_vote {
        aborts_if false;
        aborts_if my_vote.agree == false && proposal.proposal.against_votes < my_vote.vote.principal.value;
        aborts_if my_vote.agree == false && proposal.proposal.for_votes + my_vote.vote.principal.value > MAX_VOTE_AMOUNT;

        aborts_if my_vote.agree == true && proposal.proposal.for_votes < my_vote.vote.principal.value;
        aborts_if my_vote.agree == true && proposal.proposal.against_votes + my_vote.vote.principal.value > MAX_VOTE_AMOUNT;


    }
    spec revoke_vote{
        aborts_if false;
        let current_time = clock.timestamp_ms;
        aborts_if judge_proposal_state(proposal.proposal,current_time) != ACTIVE;
        aborts_if my_vote.proposer != proposal.proposal.proposer;
        aborts_if my_vote.vid != proposal.proposal.pid;
        aborts_if my_vote.vote.principal.value < voting_power;
        aborts_if voting_power < MIN_VOTING_THRESHOLD || voting_power > MAX_VOTE_AMOUNT;

        aborts_if my_vote.agree && proposal.proposal.for_votes < voting_power;
        aborts_if my_vote.agree==false && proposal.proposal.against_votes < voting_power;
        aborts_if my_vote.vote.principal.value - voting_power < MIN_VOTING_THRESHOLD;

        aborts_if ctx.ids_created + 1 > MAX_U64;
    }

    spec queue_proposal_action{
        aborts_if false;
        let current_time = clock.timestamp_ms;
        aborts_if judge_proposal_state(proposal.proposal,current_time) != AGREED;
        aborts_if proposal.proposal.action_delay > MAX_TIME_PERIOD;
        aborts_if proposal.proposal.action_delay + current_time > MAX_U64;
    }

    spec propose{
        aborts_if false;
        let obc =  payment.balance;
        let count = balance::value(obc);
        aborts_if count < MIN_NEW_PROPOSE_COST;
        aborts_if ctx.ids_created + 1 > MAX_U64;
    }

    spec modify_proposal_obj{
        aborts_if false;
        pragma aborts_if_is_partial = true;
    }

    spec modify_dao_config{
        aborts_if false;
        include NewDaoConfigParamSchema;
    }
    spec generate_next_action_id {
        aborts_if false;
        aborts_if dao.info.next_action_id >= MAX_U64;

    }
    spec generate_next_proposal_id {
        aborts_if false;
        aborts_if dao.info.next_proposal_id >= MAX_U64;
    }

    // spec set_voting_delay {
    //     //include CheckModifyConfigWithCap;
    //     //aborts_if false;
    //     aborts_if value == 0;
    // }


    // spec set_voting_period {
    //     //include CheckModifyConfigWithCap;
    //     aborts_if value == 0;
    // }
    //
    //
    //
    //
    //
    // spec set_voting_quorum_rate {
    //     aborts_if !(value > 0 && value <= 100);
    //     //include CheckModifyConfigWithCap;
    // }
    //
    //
    // spec set_min_action_delay {
    //     aborts_if value == 0;
    //     //include CheckModifyConfigWithCap;
    // }


}