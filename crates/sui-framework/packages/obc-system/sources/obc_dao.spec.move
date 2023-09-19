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

        aborts_if voting_delay == 0;
        aborts_if voting_period == 0;
        aborts_if voting_quorum_rate == 0 || voting_quorum_rate > 100;
        aborts_if min_action_delay == 0;
    }

    spec new_dao_config{
        include NewDaoConfigParamSchema;
    }

    spec withdraw_voting {
        aborts_if false;
        aborts_if pool_id(voting_obc) != object::id(dao.voting_pool);
        //aborts_if ctx.ids_created =
    }


    spec vote_of{
        aborts_if false;
        aborts_if vote.proposer != proposal.proposal.proposer;
        aborts_if vote.vid != proposal.proposal.pid;
    }
    spec unvote_votes {
        aborts_if false;
    }
    spec synchronize_proposal_into_dao{
        aborts_if false;
    }
    spec set_voting_quorum_rate{
        aborts_if false;
    }
    spec set_voting_period{
        aborts_if false;
    }

    spec set_voting_delay{
        aborts_if false;
    }
    spec set_min_action_delay{
        aborts_if false;
    }
    spec set_current_status_into_dao{
        aborts_if false;
    }


    spec send_obc_dao_event{
        aborts_if false;
    }

    spec revoke_vote{
        aborts_if false;
    }
    spec queue_proposal_action{
        aborts_if false;
    }

    spec propose{
        aborts_if false;
    }

    spec modify_proposal_obj{
        aborts_if false;
    }

    spec modify_dao_config{
        aborts_if false;
    }
    spec generate_next_action_id {
        aborts_if false;
        //aborts_if  >= MAX_U64;
    }
    spec generate_next_proposal_id {
        aborts_if false;
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