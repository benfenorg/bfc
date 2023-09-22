module obc_system::obc_dao {
    use std::option;
    use sui::object::{Self, UID};
    use sui::coin::{Self, Coin};
    use sui::vec_map::{Self, VecMap};
    use sui::clock::{Self, Clock};
    use std::string;
    use sui::event;
    use sui::tx_context::TxContext;
    use sui::tx_context;
    use sui::transfer;
    use obc_system::voting_pool::{VotingObc, voting_obc_amount, pool_id};
    use obc_system::voting_pool;
    use obc_system::obc_dao_manager::{OBCDaoManageKey};
    use std::vector;
    use obc_system::obc_dao_manager;
    use sui::obc::OBC;
    use sui::balance;
    //use sui::balance::Balance;

    friend obc_system::obc_system;
    friend obc_system::obc_system_state_inner;

    #[test_only]
    friend obc_system::obc_dao_test;

    #[test_only]
    friend obc_system::obc_dao_voting_pool_test;

    spec module{
        pragma verify;
        pragma aborts_if_is_strict;
    }

    const DEFAULT_TOKEN_ADDRESS:address=  @0x0;

    const DEFAULT_VOTE_DELAY: u64      = 1000 * 60 * 60  * 24 * 3; // 3 days || 3 hour for test
    const DEFAULT_VOTE_PERIOD: u64     = 1000 * 60 * 60  * 24 * 7; // 7 days || 7 hour for test
    const DEFAULT_MIN_ACTION_DELAY: u64 = 1000 * 60 * 60 * 24 * 7; // 7 days || 7 hour for test
    const DEFAULT_VOTE_QUORUM_RATE: u8 = 40; // 40% default quorum rate
    const DEFAULT_START_PROPOSAL_VERSION_ID : u64 = 19;
    const MAX_TIME_PERIOD: u64 = 1000 * 60 * 60 * 24 * 365 * 100; // 100 years


    const MAX_ADMIN_COUNT: u64 = 1000;

    const DEFAULT_OBC_SUPPLY : u64 = 1_0000_0000 * 1000_000_000; // 1  OBC
    const MIN_NEW_PROPOSE_COST: u64 = 200 * 1000000000; // 200 OBC
    const MIN_NEW_ACTION_COST: u64 = 1 * 1000000000; // 1 OBC
    const MAX_VOTE_AMOUNT: u64 = 10 * 1_0000_0000 * 1000000000 ; // 1 billion max OBC
    const MIN_VOTING_THRESHOLD: u64 = 1_000_000_000; // 1 obc

    /// Proposal state
    const PENDING: u8 = 1;
    const ACTIVE: u8 = 2;
    const DEFEATED: u8 = 3;
    const AGREED: u8 = 4;
    const QUEUED: u8 = 5;
    const EXECUTABLE: u8 = 6;
    const EXTRACTED: u8 = 7;


    ///Error codes
    const ERR_EINSUFFICIENT_FUNDS: u64 = 1001;
    const ERR_NOT_AUTHORIZED: u64 = 1401;
    const ERR_ACTION_DELAY_TOO_SMALL: u64 = 1402;
    const ERR_PROPOSAL_STATE_INVALID: u64 = 1403;
    const ERR_PROPOSAL_ID_MISMATCH: u64 = 1404;
    const ERR_PROPOSER_MISMATCH: u64 = 1405;
    const ERR_QUORUM_RATE_INVALID: u64 = 1406;
    const ERR_CONFIG_PARAM_INVALID: u64 = 1407;
    const ERR_VOTE_STATE_MISMATCH: u64 = 1408;
    const ERR_ACTION_MUST_EXIST: u64 = 1409;
    const ERR_ACTION_ID_ALREADY_INDAO: u64 = 1414;
    const ERR_VOTED_OTHERS_ALREADY: u64 = 1410;
    const ERR_VOTED_ERR_AMOUNT: u64 = 1411;
    const ERR_WRONG_VOTING_POOL: u64 = 1412;
    const ERR_INVALID_STRING: u64 = 1413;
    const ERR_PROPOSAL_NOT_EXIST:u64 = 1415;

    //
    struct DaoEvent has copy, drop, store {
        name: string::String,
    }
    //
    struct DaoManagerEvent has copy, drop, store{
        msg: string::String,
        key: address,
    }
    /// emitted when proposal created.
    struct ProposalCreatedEvent has copy, drop, store {
        /// the proposal id.
        proposal_id: u64,
        /// proposer is the user who create the proposal.
        proposer: address,
    }

    /// emitted when user vote/revoke_vote.
    struct VoteChangedEvent has copy, drop, store {
        /// the proposal id.
        proposal_id: u64,
        /// the voter.
        voter: address,
        /// creator of the proposal.
        proposer: address,
        /// agree with the proposal or not
        agree: bool,
        /// latest vote count of the voter.
        vote: u64,
    }
    struct  ActionCreateEvent has copy, drop, store{
        actionId: u64,
        /// Name for the action
        name: string::String,
        creator: address,
    }

    struct ProposalStateEvent has copy, drop, store{
        proposalId: u64,
        state: u8,
    }
    struct BooleanEvent has copy, drop, store{
        value: bool,
    }

    /// global DAO info of the specified token type `Token`.
    struct DaoGlobalInfo has key, store {
        id: UID,
        /// next proposal id.
        next_proposal_id: u64,

        // next action id
        next_action_id: u64,

        /// proposal creating event.
        proposal_create_event: ProposalCreatedEvent,
        /// voting event.
        vote_changed_event: VoteChangedEvent,
    }

    /// Configuration of the `Token`'s DAO.
    struct DaoConfig has copy, drop, store {
        /// after proposal created, how long use should wait before he can vote (in milliseconds)
        voting_delay: u64,
        /// how long the voting window is (in milliseconds).
        voting_period: u64,
        /// the quorum rate to agree on the proposal.
        /// if 50% votes needed, then the voting_quorum_rate should be 50.
        /// it should between (0, 100].
        voting_quorum_rate: u8,
        /// how long the proposal should wait before it can be executed (in milliseconds).
        min_action_delay: u64,
    }
    spec DaoConfig {
        invariant voting_quorum_rate > 0 && voting_quorum_rate <= 100;
        invariant voting_delay > 0;
        invariant voting_period > 0;
        invariant min_action_delay > 0;
    }

    struct Dao has key, store {
        id: UID,
        admin: address,
        config: DaoConfig,
        info: DaoGlobalInfo,

        proposal_record: VecMap<u64, ProposalInfo>,  //pid -> proposal address
        action_record: VecMap<u64, OBCDaoAction>,    //actionId -> action address
        votes_record: VecMap<u64, u64>,  //pid -> vote count
        voting_pool: voting_pool::VotingPool,

        current_proposal_status:  VecMap<u64, ProposalStatus>,
    }

    struct ProposalStatus has copy, drop, store {
        version_id : u64,
        status : u8,
    }

    struct OBCDaoAction has copy, drop, store{
        action_id: u64,
        /// Name for the action
        name: string::String,
    }

    public(friend) fun getProposalRecord(dao : &mut Dao) :VecMap<u64, ProposalInfo>{
        dao.proposal_record
    }

    public(friend) fun get_obcdao_actionid(obcDaoAction: OBCDaoAction): u64 {
        obcDaoAction.action_id
    }

    struct ProposalInfo has store, copy, drop{
        proposal_uid: address,
        pid: u64,
        /// creator of the proposal
        proposer: address,
        /// when voting begins.
        start_time: u64,
        /// when voting ends.
        end_time: u64,
        /// count of voters who agree with the proposal
        for_votes: u64,
        /// count of voters who're against the proposal
        against_votes: u64,
        /// executable after this time.
        eta: u64,
        /// after how long, the agreed proposal can be executed.
        action_delay: u64,
        /// how many votes to reach to make the proposal pass.
        quorum_votes: u64,
        /// proposal action.
        action: OBCDaoAction,
        /// version id.
        version_id: u64,
    }

    /// Proposal data struct.
    struct Proposal has key {
        /// id of the proposal
        id: UID,
        proposal: ProposalInfo,
    }


    /// User vote info.
    struct Vote has key , store {
        id: UID,
        vid: u64,
        /// vote for the proposal under the `proposer`.
        proposer: address,
        /// how many tokens to vote.
        vote:  VotingObc,
        /// vote for or vote against.
        agree: bool,
    }

    //functions
    public(friend) fun create_obcdao_action(
        dao: &mut Dao,
        payment: Coin<OBC>,
        actionName:vector<u8>,
        ctx: &mut TxContext): OBCDaoAction {
        //auth

        //convert proposal payment to voting_obc
        let sender = tx_context::sender(ctx);
        let balance = coin::into_balance(payment);
        let value = balance::value(&balance);
        // ensure the user pays enough
        assert!(value >= MIN_NEW_ACTION_COST, ERR_EINSUFFICIENT_FUNDS);

        let voting_obc = voting_pool::request_add_voting(&mut dao.voting_pool, balance, ctx);
        transfer::public_transfer(voting_obc, sender);

        let nameString = string::try_utf8(actionName);
        assert!(nameString != option::none(), ERR_INVALID_STRING);

        let name_ref = option::extract(&mut nameString);
        // sender address
        let sender = tx_context::sender(ctx);
        let action_id = generate_next_action_id(dao);

        let action = OBCDaoAction{
            action_id: action_id,
            name: name_ref,
        };

        event::emit(
            ActionCreateEvent{
                actionId: action_id,
                name: name_ref,
                creator: sender,
            }
        );

        assert!(vec_map::contains(&dao.action_record, &action_id) == false, ERR_ACTION_ID_ALREADY_INDAO);
        vec_map::insert(&mut dao.action_record, action_id, copy action);
        action
    }

    // Part 3: transfer the OBC Dao object to the sender
    public(friend) fun create_dao(        admins: vector<address>,
                                  ctx: &mut TxContext ) : Dao {


        assert!( vector::length(&admins) <= MAX_ADMIN_COUNT, ERR_CONFIG_PARAM_INVALID );
        assert!( vector::length(&admins) > 0, ERR_CONFIG_PARAM_INVALID );


        let daoConfig = new_dao_config(DEFAULT_VOTE_DELAY,
            DEFAULT_VOTE_PERIOD,
            DEFAULT_VOTE_QUORUM_RATE,
            DEFAULT_MIN_ACTION_DELAY);


        let daoInfo = DaoGlobalInfo{
            id: object::new(ctx),
            next_proposal_id: DEFAULT_START_PROPOSAL_VERSION_ID,
            next_action_id: 0,
            proposal_create_event: ProposalCreatedEvent{
                proposal_id: DEFAULT_START_PROPOSAL_VERSION_ID,
                proposer: DEFAULT_TOKEN_ADDRESS,
            },
            vote_changed_event: VoteChangedEvent{
                proposal_id: DEFAULT_START_PROPOSAL_VERSION_ID,
                voter: DEFAULT_TOKEN_ADDRESS,
                proposer: DEFAULT_TOKEN_ADDRESS,
                agree: false,
                vote: 0,
            }
        };

        let votingPool = voting_pool::new(ctx);
        let rootAdmin = vector::borrow(&admins, 0);
        let dao_obj = Dao{
            id: object::new(ctx),
            admin: *rootAdmin,  //using the first of the admins as the admin of the dao
            config: daoConfig,
            info: daoInfo,
            proposal_record: vec_map::empty(),
            action_record: vec_map::empty(),
            votes_record: vec_map::empty(),
            voting_pool: votingPool,
            current_proposal_status: vec_map::empty(),
        };


        set_admins(admins,  ctx);

        dao_obj
    }

    fun getDaoActionByActionId(dao:&mut Dao, actionId: u64) : OBCDaoAction {
        let data = vec_map::get(&dao.action_record, &actionId);
        *data
    }

    public entry fun create_dao_and_share(        admins: vector<address>,
                                                               ctx: &mut TxContext ) {
        // sender address
        //let sender = tx_context::sender(ctx);

        let daoConfig = new_dao_config(DEFAULT_VOTE_DELAY,
            DEFAULT_VOTE_PERIOD,
            DEFAULT_VOTE_QUORUM_RATE,
            DEFAULT_MIN_ACTION_DELAY);


        let daoInfo = DaoGlobalInfo{
            id: object::new(ctx),
            next_proposal_id: 0,
            next_action_id: 0,
            proposal_create_event: ProposalCreatedEvent{
                proposal_id: 0,
                proposer: DEFAULT_TOKEN_ADDRESS,
            },
            vote_changed_event: VoteChangedEvent{
                proposal_id: 0,
                voter: DEFAULT_TOKEN_ADDRESS,
                proposer: DEFAULT_TOKEN_ADDRESS,
                agree: false,
                vote: 0,
            }
        };

        let votingPool = voting_pool::new(ctx);
        let rootAdmin = vector::borrow(&admins, 0);
        let dao_obj = Dao{
            id: object::new(ctx),
            admin: *rootAdmin,  //using the first of the admins as the admin of the dao
            config: daoConfig,
            info: daoInfo,
            proposal_record: vec_map::empty(),
            action_record: vec_map::empty(),
            votes_record: vec_map::empty(),
            voting_pool: votingPool,
            current_proposal_status: vec_map::empty(),
        };

        transfer::share_object(dao_obj);

        set_admins(admins, ctx);
    }

    // create a dao config
    fun new_dao_config(
        voting_delay: u64,
        voting_period: u64,
        voting_quorum_rate: u8,
        min_action_delay: u64,
    ): DaoConfig {
        assert!(voting_delay > 0 && voting_delay <= MAX_TIME_PERIOD, ERR_CONFIG_PARAM_INVALID);
        assert!(voting_period> 0 && voting_period <= MAX_TIME_PERIOD, ERR_CONFIG_PARAM_INVALID);
        assert!(min_action_delay > 0 && min_action_delay <= MAX_TIME_PERIOD, ERR_CONFIG_PARAM_INVALID);
        assert!(voting_quorum_rate >= 1 && voting_quorum_rate <= 100, ERR_CONFIG_PARAM_INVALID);

        DaoConfig { voting_delay, voting_period, voting_quorum_rate, min_action_delay }
    }



    /// propose a proposal.
    /// `action`: the actual action to execute.
    /// `action_delay`: the delay to execute after the proposal is agreed
    public(friend) fun propose (
        dao: &mut Dao,
        version_id: u64,
        payment: Coin<OBC>,
        action_id: u64,
        action_delay: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {

        //convert proposal payment to voting_obc
        let sender = tx_context::sender(ctx);
        let balance = coin::into_balance(payment);
        let value = balance::value(&balance);
        // ensure the user pays enough
        assert!(value >= MIN_NEW_PROPOSE_COST, ERR_EINSUFFICIENT_FUNDS);

        let voting_obc = voting_pool::request_add_voting(&mut dao.voting_pool, balance, ctx);
        transfer::public_transfer(voting_obc, sender);


        let action = getDaoActionByActionId(dao, action_id);

        if (action_delay <= 0 || action_delay <= min_action_delay(dao)) {
            action_delay = min_action_delay(dao);
        };

        let proposal_id = generate_next_proposal_id(dao);

        let sender = tx_context::sender(ctx);
        let start_time = clock::timestamp_ms(clock)  + voting_delay(dao);
        let quorum_votes = quorum_votes(dao);
        let object_id = object::new(ctx);

        let proposalInfo = ProposalInfo {
            proposal_uid: object::uid_to_address(&object_id),
            pid: proposal_id,
            proposer: sender,
            start_time,
            end_time: start_time + voting_period(dao),
            for_votes: 0,
            against_votes: 0,
            eta: 0,
            action_delay,
            quorum_votes,
            action,
            version_id,
        };

        let proposal = Proposal{
            id: object_id,
            proposal: copy proposalInfo,
        };
        vec_map::insert(&mut dao.proposal_record, proposal_id, proposalInfo);


        transfer::share_object(proposal);

        // emit event
        event::emit(
            ProposalCreatedEvent{
                proposal_id,
                proposer: sender,
            }
        );
        //send_obc_dao_event(manager_key,b"proposal_created");
    }

    fun synchronize_proposal_into_dao(proposal: &Proposal, dao:  &mut Dao) {
        if (vec_map::contains( &dao.proposal_record,&proposal.proposal.pid)) {
            let old = vec_map::get_mut(&mut dao.proposal_record,& proposal.proposal.pid);
            *old = proposal.proposal;
        }
    }

    /// votes for a proposal.
    /// User can only vote once, then the vote is locked,
    /// which can only be un vote by user after the proposal is expired, or cancelled, or executed.
    /// So think twice before casting vote.
    public(friend) fun cast_vote(
        dao:  &mut Dao,
        proposal: &mut Proposal,
        coin: VotingObc,
        agreeInt: u8,
        clock: & Clock,
        ctx: &mut TxContext,
    )  {
        let agree = agreeInt == 1;

        {
            let state = proposal_state(proposal,clock);
            // only when proposal is active, use can cast vote.
            assert!(state == ACTIVE, (ERR_PROPOSAL_STATE_INVALID));
        };

        let vote_amount = voting_pool::voting_obc_amount(&coin);
        {
            assert!(vote_amount >= MIN_VOTING_THRESHOLD, ERR_VOTED_ERR_AMOUNT);
            assert!(vote_amount <= MAX_VOTE_AMOUNT, ERR_VOTED_ERR_AMOUNT);
        };

        let sender = tx_context::sender(ctx);

        let total_voted = {

            let voteCoin = coin;

            let my_vote = Vote {
                id: object::new(ctx),
                vid: proposal.proposal.pid,
                proposer: proposal.proposal.proposer,
                vote: voteCoin,
                agree,
            };

            if (agree) {
                proposal.proposal.for_votes = proposal.proposal.for_votes + vote_amount;
            } else {
                proposal.proposal.against_votes = proposal.proposal.against_votes + vote_amount;
            };
            transfer::transfer(my_vote, sender);

            vote_amount
        };

        synchronize_proposal_into_dao(proposal, dao);
        // emit event
        event::emit(
            VoteChangedEvent{
                proposal_id: proposal.proposal.pid,
                voter: sender,
                proposer: proposal.proposal.proposer,
                agree,
                vote: total_voted,
            });
    }



    /// Let user change their vote during the voting time.
    public(friend) fun change_vote(
        dao:  &mut Dao,
        my_vote: &mut Vote,
        proposal: &mut Proposal,
        agree: bool,
        clock: & Clock,
        ctx: &mut TxContext,
    )  {
        {
            let state = proposal_state(proposal, clock);
            // only when proposal is active, user can change vote.
            assert!(state == ACTIVE, (ERR_PROPOSAL_STATE_INVALID));
        };


        let sender = tx_context::sender(ctx);
        //let total_voted = voting_obc_amount(&my_vote.vote);
        {
            assert!(my_vote.proposer == proposal.proposal.proposer, (ERR_PROPOSER_MISMATCH));
            assert!(my_vote.vid == proposal.proposal.pid, (ERR_VOTED_OTHERS_ALREADY));

        };

        // flip the vote
        if (my_vote.agree != agree) {
            let total_voted = do_flip_vote(my_vote, proposal);

            synchronize_proposal_into_dao(proposal, dao);
            // emit event
            event::emit(
                VoteChangedEvent{
                    proposal_id: proposal.proposal.pid,
                    voter: sender,
                    proposer: proposal.proposal.proposer,
                    agree,
                    vote: total_voted, });

        };
    }

    fun do_flip_vote(my_vote: &mut Vote,
                     proposal: &mut Proposal): u64 {
        my_vote.agree = !my_vote.agree;
        let total_voted = voting_obc_amount(&my_vote.vote);
        if (my_vote.agree) {
            assert!(proposal.proposal.against_votes >= total_voted, (ERR_VOTED_ERR_AMOUNT));
            assert!(proposal.proposal.for_votes + total_voted <= MAX_VOTE_AMOUNT , (ERR_VOTED_ERR_AMOUNT));

            proposal.proposal.for_votes = proposal.proposal.for_votes + total_voted;
            proposal.proposal.against_votes = proposal.proposal.against_votes - total_voted;
        } else {
            assert!(proposal.proposal.for_votes >= total_voted, (ERR_VOTED_ERR_AMOUNT));
            assert!(proposal.proposal.against_votes + total_voted <= MAX_VOTE_AMOUNT, (ERR_VOTED_ERR_AMOUNT));

            proposal.proposal.for_votes = proposal.proposal.for_votes - total_voted;
            proposal.proposal.against_votes = proposal.proposal.against_votes + total_voted;
        };
        total_voted
    }

    /// Revoke some voting powers from vote on `proposal_id` of `proposer_address`.
    public(friend) fun revoke_vote(
        dao:  &mut Dao,
        proposal: &mut Proposal,
        my_vote:  Vote,
        voting_power: u64,
        clock: & Clock,
        ctx: &mut TxContext,
    ){
        {
            let state = proposal_state(proposal, clock);
            // only when proposal is active, user can revoke vote.
            assert!(state == ACTIVE, (ERR_PROPOSAL_STATE_INVALID));
        };
        // get proposal

        // get vote
        let sender = tx_context::sender(ctx);
        {
            assert!(my_vote.proposer == proposal.proposal.proposer, (ERR_PROPOSER_MISMATCH));
            assert!(my_vote.vid == proposal.proposal.pid, (ERR_VOTED_OTHERS_ALREADY));
            assert!(voting_obc_amount(&my_vote.vote) >= voting_power, (ERR_VOTED_ERR_AMOUNT));
            assert!(voting_power >= MIN_VOTING_THRESHOLD && voting_power <= MAX_VOTE_AMOUNT, (ERR_VOTED_ERR_AMOUNT));
            assert!(voting_obc_amount(&my_vote.vote) - voting_power >= MIN_VOTING_THRESHOLD, (ERR_VOTED_ERR_AMOUNT));
        };
        // revoke vote on proposal
        do_revoke_vote(proposal, &mut my_vote, voting_power,ctx);

        synchronize_proposal_into_dao(proposal, dao);

        // emit vote changed event
        event::emit(
            VoteChangedEvent{
                proposal_id: proposal.proposal.pid,
                voter: sender,
                proposer: proposal.proposal.proposer,
                agree: my_vote.agree,
                vote: voting_obc_amount(&my_vote.vote),
            }
        );

        if (voting_obc_amount(&my_vote.vote) == 0u64) {
            let Vote {
                proposer: _,
                id: uid,
                vid: _,
                vote,
                agree: _} = my_vote;

            object::delete(uid);
            transfer::public_transfer(vote, sender);
        } else {
            let some_vote = my_vote;
            transfer::transfer(some_vote, sender);
        };

        //todo transfer back
        //reverted_vote
    }


    fun do_revoke_vote(
        proposal: &mut Proposal,
        vote: &mut Vote,
        to_revoke: u64,
        ctx: &mut TxContext,
    ){
        spec {
            assume vote.vote.principal.value >= to_revoke;
        };

        //todo: unlock vote coin or return...
        //// Token::withdraw(&mut vote.vote, to_revoke);
        let reverted_vote = voting_pool::split(&mut vote.vote, to_revoke, ctx);

        if (vote.agree) {
            proposal.proposal.for_votes = proposal.proposal.for_votes - to_revoke;
        } else {
            proposal.proposal.against_votes = proposal.proposal.against_votes - to_revoke;
        };
        spec {
            assert reverted_vote.principal.value == to_revoke;
        };

        //reverted_vote
        transfer::public_transfer(reverted_vote, tx_context::sender(ctx));
    }

    /// Retrieve back my voted token voted for a proposal.
    public(friend) fun unvote_votes(
        proposal: & Proposal,
        vote: Vote,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        // only check state when proposal exists.
        // because proposal can be destroyed after it ends in DEFEATED or EXTRACTED state.
        {
            let state = proposal_state(proposal,clock);
            // Only after vote period end, user can unvote his votes.
            assert!(state > ACTIVE, (ERR_PROPOSAL_STATE_INVALID));
        };

        let sender = tx_context::sender(ctx);


        // delete vote.
        let Vote { proposer, id,vid, vote, agree: _ } = vote;


        object::delete(id);



        // these checks are still required.
        assert!(proposer == proposal.proposal.proposer, (ERR_PROPOSER_MISMATCH));
        assert!(vid == proposal.proposal.pid, (
            ERR_VOTED_OTHERS_ALREADY));

        transfer::public_transfer(vote, sender);
    }
    /// Get voter's vote info on proposal with `proposal_id` of `proposer_address`.
    struct VoteInfoEvent has copy, drop,store{
        proposal_id: u64,
        voter: address,
        proposer: address,
        agree: bool,
        vote: u64,
    }
    public(friend) fun vote_of(
        vote: &Vote,
        proposal: & Proposal,
        ctx: &mut TxContext,
    ){
        assert!(vote.proposer == proposal.proposal.proposer, (ERR_PROPOSER_MISMATCH));
        assert!(vote.vid == proposal.proposal.pid, (ERR_VOTED_OTHERS_ALREADY));
        //(vote.agree, staking_pool::vote_sui_amount(&vote.vote))
        event::emit(
            VoteInfoEvent{
                proposal_id: proposal.proposal.pid,
                voter: tx_context::sender(ctx),
                proposer: proposal.proposal.proposer,
                agree: vote.agree,
                vote: voting_obc_amount(&vote.vote),
            }
        );
    }


    /// Check whether voter has voted on proposal with `proposal_id` of `proposer_address`.

    public(friend) fun has_vote(
        vote: &Vote,
        proposal: &Proposal,
    ): bool  {
        event::emit(
            BooleanEvent{value:
            vote.proposer == proposal.proposal.proposer && vote.vid == proposal.proposal.pid});

        vote.proposer == proposal.proposal.proposer && vote.vid == proposal.proposal.pid
    }


    /// queue agreed proposal to execute.
    public(friend) fun queue_proposal_action(
        dao:  &mut Dao,
        _: &OBCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    )  {

        //let sender = tx_context::sender(ctx);

            // Only agreed proposal can be submitted.
            assert!(
                proposal_state(proposal, clock) == AGREED,
                (ERR_PROPOSAL_STATE_INVALID)
            );
        assert!(proposal.proposal.action_delay <= MAX_TIME_PERIOD, ERR_CONFIG_PARAM_INVALID);

        proposal.proposal.eta =  clock::timestamp_ms(clock)  + proposal.proposal.action_delay;

        synchronize_proposal_into_dao(proposal, dao);
        //send_obc_dao_event(manager_key, b"proposal_queued");
    }

    /// extract proposal action to execute.
    public fun extract_proposal_action(
        proposal: &mut Proposal,
        clock: & Clock,
    ): OBCDaoAction  {
        // Only executable proposal's action can be extracted.
        assert!(
            proposal_state(proposal, clock) == EXECUTABLE,
            (ERR_PROPOSAL_STATE_INVALID),
        );
        let action = proposal.proposal.action;
        action
    }

    /// check whether a proposal exists in `proposer_address` with id `proposal_id`.
    public fun proposal_exists (
        dao : &mut Dao,
        proposal: &Proposal,
    ): bool {
        let result = vec_map::contains(&dao.proposal_record, &proposal.proposal.pid);
        result
    }

    /// Get the proposal state.
    public(friend) fun proposal_state(
        proposal: &Proposal,
        clock: & Clock,
    ): u8  {
        assert!(proposal.proposal.pid == proposal.proposal.pid, (ERR_PROPOSAL_ID_MISMATCH));
        let current_time =  clock::timestamp_ms(clock) ;
        let status = judge_proposal_state(& proposal.proposal, current_time);

        // emit event
        event::emit(
            ProposalStateEvent {
                proposalId: proposal.proposal.pid,
                state: status,
            });
        status
    }

    public(friend) fun judge_proposal_state(
        proposal: &ProposalInfo,
        current_time: u64,
    ): u8 {
        if (current_time < proposal.start_time) {
            // Pending
            PENDING
        } else if (current_time <= proposal.end_time) {
            // Active
            ACTIVE
        } else if (proposal.for_votes <= proposal.against_votes ||
            proposal.for_votes < proposal.quorum_votes) {
            // Defeated
            DEFEATED
        } else if (proposal.eta == 0) {
            // Agreed.
            AGREED
        } else if (current_time < proposal.eta) {
            // Queued, waiting to execute
            QUEUED
        } else if (proposal.action.action_id != 0 ) {
            EXECUTABLE
        } else {
            EXTRACTED
        }
    }

    /// get proposal's information.
    /// return: (id, start_time, end_time, for_votes, against_votes).
    struct ProposalInfoEvent has copy, drop,store{
        proposal_id: u64,
        start_time: u64,
        end_time: u64,
        for_votes: u64,
        against_votes: u64,
    }

    public(friend) fun proposal_info(
        proposal: &Proposal,
    ) : (u64, u64) {
        event::emit(
            ProposalInfoEvent{
                proposal_id: proposal.proposal.pid,
                start_time: proposal.proposal.start_time,
                end_time: proposal.proposal.end_time,
                for_votes: proposal.proposal.for_votes,
                against_votes: proposal.proposal.against_votes,
            }
        );

        (proposal.proposal.for_votes, proposal.proposal.against_votes)
    }

    fun generate_next_proposal_id(dao: &mut Dao): u64 {
        let info = &mut dao.info;
        let proposal_id = info.next_proposal_id;
        info.next_proposal_id = proposal_id + 1;
        proposal_id

    }

    fun generate_next_action_id(dao: &mut Dao): u64 {
        let info = &mut dao.info;
        let action_id = info.next_action_id;
        info.next_action_id = action_id + 1;
        action_id

    }

    //// Helper functions

    /// Quorum votes to make proposal pass.
    /// temply using 4000* 000_0000 as the pass rate.
    fun quorum_votes(dao: &mut Dao): u64 {
        let total_supply_sui: u64 = DEFAULT_OBC_SUPPLY;
        let supply = total_supply_sui;

        let rate = voting_quorum_rate(dao);
        let rate = (rate as u64);
        supply * rate / 100
    }
    /// get default voting delay of the DAO.
    public(friend) fun voting_delay(dao: &mut Dao): u64 {
        get_config(dao).voting_delay
    }

    /// get the default voting period of the DAO.
    public(friend) fun voting_period(dao: &mut Dao): u64 {
        get_config(dao).voting_period
    }

    /// Get the quorum rate in percent.
    public(friend) fun voting_quorum_rate(dao: &mut Dao): u8 {
        get_config(dao).voting_quorum_rate
    }


    /// Get the min_action_delay of the DAO.
    public(friend) fun min_action_delay(dao: &mut Dao): u64 {
        get_config(dao).min_action_delay
    }

    fun get_config(dao: &mut Dao): &mut DaoConfig {
        &mut dao.config
    }

    /// update function, modify dao config.
    /// if any param is 0, it means no change to that param.
    public fun modify_dao_config(
        dao: &mut Dao,
        _: &OBCDaoManageKey,
        voting_delay: u64,
        voting_period: u64,
        voting_quorum_rate: u8,
        min_action_delay: u64,
    ) {

        assert!(voting_delay <= MAX_TIME_PERIOD && voting_delay > 0, (ERR_CONFIG_PARAM_INVALID));
        assert!(voting_period <= MAX_TIME_PERIOD && voting_period > 0, (ERR_CONFIG_PARAM_INVALID));
        assert!(min_action_delay <= MAX_TIME_PERIOD && min_action_delay > 0, (ERR_CONFIG_PARAM_INVALID));
        assert!(voting_quorum_rate>0 && voting_quorum_rate <= 100, (ERR_QUORUM_RATE_INVALID));



        let config = get_config(dao);
        if (voting_period > 0) {
            config.voting_period = voting_period;
        };
        if (voting_delay > 0) {
            config.voting_delay = voting_delay;
        };
        if (voting_quorum_rate > 0 && voting_quorum_rate <= 100) {
            config.voting_quorum_rate = voting_quorum_rate;
        };
        if (min_action_delay > 0) {
            config.min_action_delay = min_action_delay;
        };

        //send_obc_dao_event(manager_key, b"modify_dao_config");
    }

    /// set voting delay
    public(friend) fun set_voting_delay(
        dao: &mut Dao,
        _: &OBCDaoManageKey,
        value: u64,
    ) {

        assert!(value > 0, (ERR_CONFIG_PARAM_INVALID));
        assert!(value <= MAX_TIME_PERIOD, (ERR_CONFIG_PARAM_INVALID));

        let config = get_config(dao);
        config.voting_delay = value;

        //send_obc_dao_event(manager_key, b"set_voting_delay");
    }


    /// set voting period
    public(friend) fun set_voting_period(
        dao: &mut Dao,
        _: &OBCDaoManageKey,
        value: u64,
    ) {

        assert!(value > 0, (ERR_CONFIG_PARAM_INVALID));
        assert!(value <= MAX_TIME_PERIOD, (ERR_CONFIG_PARAM_INVALID));

        let config = get_config(dao);
        config.voting_period = value;

        //send_obc_dao_event(manager_key, b"set_voting_period");
    }

    /// set voting quorum rate: .
    public(friend) fun set_voting_quorum_rate(
        dao: &mut Dao,
        _: &OBCDaoManageKey,
        value: u8,
    ) {
        assert!(value <= 100 && value > 0, (ERR_QUORUM_RATE_INVALID));
        let config = get_config(dao);
        config.voting_quorum_rate = value;

        //send_obc_dao_event(manager_key, b"set_voting_quorum_rate");
    }


    /// set min action delay
    public(friend) fun set_min_action_delay(
        dao: &mut Dao,
        _: &OBCDaoManageKey,
        value: u64,
    ) {
        assert!(value > 0, (ERR_CONFIG_PARAM_INVALID));
        assert!(value <= MAX_TIME_PERIOD, (ERR_CONFIG_PARAM_INVALID));

        let config = get_config(dao);
        config.min_action_delay = value;

        //send_obc_dao_event(manager_key, b"set_min_action_delay");
    }

    fun set_admins(
        new_admins: vector<address>,
        ctx: &mut TxContext,
    ) {
        //let index = 0;
        let count = vector::length(&new_admins);
        assert!(count > 0 && count <= MAX_ADMIN_COUNT, ERR_CONFIG_PARAM_INVALID);

        let i = 0;
        while (i < count) {
            let admin = vector::borrow(&new_admins, i);
            obc_dao_manager::new(*admin, ctx);
            i = i+1;
        };

    }

    public fun add_admin(
        new_admin:address,
        ctx: &mut TxContext,
    ) {
        obc_dao_manager::new(new_admin, ctx);
    }



    public entry fun modify_proposal_obj(dao: &mut Dao, proposal_obj: &mut Proposal, index : u8, clock: &Clock) {
        //let proposal = proposal_obj.proposal;
        if (index == 1) {
            // Pending
            proposal_obj.proposal.start_time = clock::timestamp_ms(clock)  + 1000000000;
        }else if (index == 2) {
            // active
            proposal_obj.proposal.start_time = clock::timestamp_ms(clock)  - 1000000000;
            proposal_obj.proposal.end_time = clock::timestamp_ms(clock) + 1000000000;
        } else if (index == 3){
            //afer voting  Defeated...
            proposal_obj.proposal.start_time = clock::timestamp_ms(clock)  - 2000000000;
            proposal_obj.proposal.end_time = clock::timestamp_ms(clock) - 1000000000;
            proposal_obj.proposal.for_votes = 1;
            proposal_obj.proposal.against_votes = 2;
        } else if (index == 4) {
            //afer voting AGREED
            proposal_obj.proposal.start_time = clock::timestamp_ms(clock)  - 2000000000;
            proposal_obj.proposal.end_time = clock::timestamp_ms(clock) - 1000000000;
            proposal_obj.proposal.for_votes = 3;
            proposal_obj.proposal.against_votes = 2;
            proposal_obj.proposal.quorum_votes = 2;
            proposal_obj.proposal.eta = 0;
        } else if (index == 5) {
            // Queued, waiting to execute
            proposal_obj.proposal.start_time = clock::timestamp_ms(clock)  - 2000000000;
            proposal_obj.proposal.end_time = clock::timestamp_ms(clock) - 1000000000;
            proposal_obj.proposal.for_votes = 3;
            proposal_obj.proposal.against_votes = 2;
            proposal_obj.proposal.quorum_votes = 2;
            proposal_obj.proposal.eta = clock::timestamp_ms(clock)  + 100000000;
        } else if (index == 6) {
            proposal_obj.proposal.start_time = clock::timestamp_ms(clock)  - 2000000000;
            proposal_obj.proposal.end_time = clock::timestamp_ms(clock) - 1000000000;
            proposal_obj.proposal.for_votes = 3;
            proposal_obj.proposal.against_votes = 2;
            proposal_obj.proposal.quorum_votes = 2;
            proposal_obj.proposal.eta = clock::timestamp_ms(clock)  - 100000000;
            proposal_obj.proposal.action.action_id = 1;
        } else if (index == 7) {
            proposal_obj.proposal.start_time = clock::timestamp_ms(clock)  - 2000000000;
            proposal_obj.proposal.end_time = clock::timestamp_ms(clock) - 1000000000;
            proposal_obj.proposal.for_votes = 3;
            proposal_obj.proposal.against_votes = 2;
            proposal_obj.proposal.quorum_votes = 2;
            proposal_obj.proposal.eta = clock::timestamp_ms(clock)  - 100000000;
            proposal_obj.proposal.action.action_id = 0;
        };
        synchronize_proposal_into_dao(proposal_obj, dao);
    }


    public(friend) fun create_voting_obc(dao: &mut Dao,
                                       coin: Coin<OBC>,
                                       ctx: &mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);
        let balance = coin::into_balance(coin);
        let voting_obc = voting_pool::request_add_voting(&mut dao.voting_pool, balance, ctx);

        transfer::public_transfer(voting_obc, sender);
    }

    public(friend) fun withdraw_voting(  dao: &mut Dao,
                                       voting_obc: VotingObc,
                                       ctx: &mut TxContext ,) {
        // sender address
        let sender = tx_context::sender(ctx);
        assert!(pool_id(&voting_obc) == object::id(&dao.voting_pool), ERR_WRONG_VOTING_POOL);
        let voting_obc = voting_pool::request_withdraw_voting(&mut dao.voting_pool, voting_obc);
        let coin = coin::from_balance(voting_obc, ctx);
        transfer::public_transfer(coin, sender);
    }

    /// remove terminated proposal from proposer
    public(friend) fun destroy_terminated_proposal(
        dao: &mut Dao,
        _: &OBCDaoManageKey,
        proposal:  &mut Proposal,
        clock: & Clock,
    )  {


        let proposal_state = proposal_state(proposal,clock);
        assert!(
            proposal_state == DEFEATED || proposal_state == EXTRACTED,
            (ERR_PROPOSAL_STATE_INVALID),
        );



        assert!(vec_map::contains(&dao.proposal_record, &proposal.proposal.pid), (ERR_PROPOSAL_NOT_EXIST));
        vec_map::remove(&mut dao.proposal_record, &proposal.proposal.pid);
        if (proposal_state == DEFEATED) {
            let _ =  proposal.proposal.action;
        };

        // let Proposal {
        //     id: uid,
        //     proposal: ProposalInfo{
        //         pid: _,
        //         proposer: _,
        //         start_time: _,
        //         end_time: _,
        //         for_votes: _,
        //         against_votes: _,
        //         eta: _,
        //         action_delay: _,
        //         quorum_votes: _,
        //         action: _c
        //         } ,
        //     } = proposal;
        //
        //  object::delete(uid);
        //send_obc_dao_event(manager_key, b"ProposalDestroyed");

    }

    public(friend) fun set_current_status_into_dao(dao: &mut Dao, proposalInfo : &ProposalInfo, curProposalStatus: u8) {
        let flag = vec_map::contains(&dao.current_proposal_status, &proposalInfo.pid);
        if (flag) {
            vec_map::remove(&mut dao.current_proposal_status, &proposalInfo.pid);
        };

        let proposal_status = ProposalStatus {
            version_id : proposalInfo.version_id,
            status: curProposalStatus,
        };
        vec_map::insert(&mut (dao.current_proposal_status), proposalInfo.pid, proposal_status);
    }



}
