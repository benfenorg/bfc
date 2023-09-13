module obc_system::obc_system {
    use sui::balance;
    use obc_system::obc_dao;
    use obc_system::voting_pool::VotingObc;
    use sui::balance::{Balance, Supply};
    use sui::coin;
    use sui::coin::Coin;
    use sui::clock::{Clock};
    use sui::dynamic_field;
    use sui::clock::{Self};

    use sui::obc::OBC;
    use sui::object::UID;
    use sui::stable::STABLE;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use obc_system::usd::USD;
    use obc_system::obc_dao_manager::{OBCDaoManageKey};
    use obc_system::obc_dao::{Proposal, Vote};
    use obc_system::obc_system_state_inner;
    use obc_system::obc_system_state_inner::{ObcSystemStateInner, ObcSystemParameters};

    #[test_only]
    friend obc_system::obc_system_tests;

    struct ObcSystemState has key {
        id: UID,
        version: u64
    }

    const OBC_SYSTEM_STATE_VERSION_V1: u64 = 1;

    public fun create(
        id: UID,
        usd_supply: Supply<USD>,
        parameters: ObcSystemParameters,
        ctx: &mut TxContext
    ) {
        let inner_state = obc_system_state_inner::create_inner_state(
            usd_supply,
            parameters,
            ctx,
        );
        let self = ObcSystemState {
            id,
            version: OBC_SYSTEM_STATE_VERSION_V1
        };

        dynamic_field::add(&mut self.id, OBC_SYSTEM_STATE_VERSION_V1, inner_state);

        transfer::share_object(self);
    }

    public fun obc_round(
        wrapper: &mut ObcSystemState,
        clock: &Clock,
        round: u64,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(wrapper);
        obc_system_state_inner::update_round(inner_state, round);
        //exchange all stable to obc.
        obc_system_state_inner::request_exchange_all(inner_state, ctx);
        // //update inner exchange rate from stable-swap.
        let stable = coin::zero<STABLE>(ctx);
        //todo read rate from stable-swap.
        let rate = 1000000000;
        obc_system_state_inner::request_update_gas_coin(inner_state, &stable, rate);
        balance::destroy_zero(coin::into_balance(stable));

        // X-treasury rebalance
        obc_system_state_inner::rebalance(inner_state, clock, ctx);

        judge_proposal_state(wrapper, clock::timestamp_ms(clock));
    }

    public entry fun update_round(
        wrapper: &mut ObcSystemState,
	clock: &Clock, 
        ctx: &mut TxContext,
    ){
        obc_round(wrapper,  clock,200, ctx);
    }

    fun load_system_state(
        self: &ObcSystemState,
    ): &ObcSystemStateInner {
        dynamic_field::borrow(&self.id, self.version)
    }

    fun load_system_state_mut(
        self: &mut ObcSystemState
    ): &mut ObcSystemStateInner {
        dynamic_field::borrow_mut(&mut self.id, self.version)
    }

    /// Getter of the gas coin exchange pool rate.
    public entry fun request_get_exchange_rate(
        self: &ObcSystemState,
        stable: &Coin<STABLE>
    ): u64 {
        let inner_state = load_system_state(self);
        obc_system_state_inner::requst_get_exchange_rate<STABLE>(inner_state, stable)
    }

    public entry fun request_add_gas_coin(
        self: &mut ObcSystemState,
        gas_coin: &Coin<STABLE>,
        rate: u64,
    ) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::request_add_gas_coin(inner_state, gas_coin, rate)
    }

    public entry fun request_update_gas_coin(
        self: &mut ObcSystemState,
        gas_coin: &Coin<STABLE>,
        rate: u64,
    ) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::request_update_gas_coin(inner_state, gas_coin, rate)
    }

    public entry fun request_remove_gas_coin(
        self: &mut ObcSystemState,
        gas_coin: &Coin<STABLE>,
    ) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::request_remove_gas_coin(inner_state, gas_coin)
    }

    /// Request exchange stable coin to obc.
    public entry fun request_exchange_stable(
        self: &mut ObcSystemState,
        stable: Coin<STABLE>,
        ctx: &mut TxContext,
    ) {
        let balance = request_exchange_stable_no_entry(self, stable, ctx);
        transfer::public_transfer(coin::from_balance(balance, ctx), tx_context::sender(ctx));
    }

    fun request_exchange_stable_no_entry(
        self: &mut ObcSystemState,
        stable: Coin<STABLE>,
        ctx: &mut TxContext,
    ): Balance<OBC> {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::request_exchange_stable(inner_state, stable, ctx)
    }

    /// Request exchange all stable coin to obc.
    public entry fun request_exchange_all(
        self: &mut ObcSystemState,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::request_exchange_all(inner_state, ctx)
    }

    /// Request withdraw stable coin.
    public entry fun request_withdraw_stable(
        self: &mut ObcSystemState,
        ctx: &mut TxContext,
    ) {
        let stables = request_withdraw_stable_no_entry(self);
        transfer::public_transfer(coin::from_balance(stables, ctx), tx_context::sender(ctx));
    }

    fun request_withdraw_stable_no_entry(
        self: &mut ObcSystemState,
    ): Balance<STABLE> {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::request_withdraw_stable(inner_state)
    }

    /// Init exchange pool by add obc coin.
    public entry fun init_exchange_pool(
        self: &mut ObcSystemState,
        coin: Coin<OBC>,
    ) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::init_exchange_pool(inner_state, coin)
    }

    public(friend) fun obc_system_stat_parameter(
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        initialize_price: u128,
        time_interval: u32,
        base_point: u64,
        max_counter_times: u32,
        chain_start_timestamp_ms: u64,
    ): ObcSystemParameters {
        obc_system_state_inner::obc_system_stat_parameter(
            position_number,
            tick_spacing,
            spacing_times,
            initialize_price,
            time_interval,
            base_point,
            max_counter_times,
            chain_start_timestamp_ms,
        )
    }

    public entry fun destroy_terminated_proposal(
        wrapper: &mut ObcSystemState,
        manager_key: &OBCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::destroy_terminated_proposal(system_state, manager_key, proposal, clock);
    }

    public entry fun propose(
        wrapper: &mut ObcSystemState,
        manager_key: &OBCDaoManageKey,
        version_id : u64,
        payment: Coin<OBC>,
        action_id: u64,
        action_delay: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::propose(system_state, manager_key, version_id, payment, action_id, action_delay, clock, ctx);
    }

    public entry fun create_obcdao_action(
        wrapper: &mut ObcSystemState,
        _: &OBCDaoManageKey,
        actionName: vector<u8>,
        ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::create_obcdao_action(system_state, _, actionName, ctx);
    }

    public entry fun judge_proposal_state(wrapper: &mut ObcSystemState, current_time: u64) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::judge_proposal_state(system_state, current_time);
    }

    public entry fun set_voting_period(
        wrapper: &mut ObcSystemState,
        manager_key: &OBCDaoManageKey,
        value: u64,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::set_voting_period(system_state, manager_key, value);
    }

    public entry fun modify_proposal(wrapper: &mut ObcSystemState, proposal_obj: &mut Proposal, index: u8, clock: &Clock) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::modify_proposal(system_state, proposal_obj, index, clock);
    }

    public entry fun set_voting_quorum_rate(wrapper: &mut ObcSystemState, manager_key: &OBCDaoManageKey, value: u8,){
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::set_voting_quorum_rate(system_state, manager_key, value);
    }

    public entry fun set_min_action_delay(
        wrapper: &mut ObcSystemState,
        manager_key: &OBCDaoManageKey,
        value: u64,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::set_min_action_delay(system_state, manager_key, value);
    }

    public entry fun withdraw_voting(   wrapper: &mut ObcSystemState,
                                 voting_obc: VotingObc,
                                 ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::withdraw_voting(system_state, voting_obc, ctx);
    }

    public entry fun create_voting_obc(wrapper: &mut ObcSystemState,
                                 coin: Coin<OBC>,
                                 ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::create_voting_obc(system_state, coin, ctx);
    }

    /// X treasury  swap obc to stablecoin
    public entry fun swap_obc_to_stablecoin<StableCoinType>(
        wrapper: &mut ObcSystemState,
        coin_obc: Coin<OBC>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::swap_obc_to_stablecoin<StableCoinType>(system_state, coin_obc, amount, ctx);
    }

    /// X treasury  swap stablecoin to obc
    public entry fun swap_stablecoin_to_obc<StableCoinType>(
        wrapper: &mut ObcSystemState,
        coin_sc: Coin<StableCoinType>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::swap_stablecoin_to_obc<StableCoinType>(system_state, coin_sc, amount, ctx);
    }

    public fun get_stablecoin_by_obc<StableCoinType>(
        wrapper: &ObcSystemState,
        amount: u64,
    ): u64
    {
        let system_state = load_system_state(wrapper);
        obc_system_state_inner::get_stablecoin_by_obc<StableCoinType>(system_state, amount)
    }

    public fun get_obc_by_stablecoin<StableCoinType>(
        wrapper: &ObcSystemState,
        amount: u64,
    ): u64
    {
        let system_state = load_system_state(wrapper);
        obc_system_state_inner::get_obc_by_stablecoin<StableCoinType>(system_state, amount)
    }

    public fun next_epoch_obc_required(wrapper: &ObcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        obc_system_state_inner::next_epoch_obc_required(system_state)
    }

    public fun treasury_balance(wrapper: &ObcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        obc_system_state_inner::treasury_balance(system_state)
    }

    public entry fun deposit_to_treasury(self: &mut ObcSystemState, coin: Coin<OBC>) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::deposit_to_treasury(inner_state, coin)
    }

    public entry fun set_voting_delay(
        self: &mut ObcSystemState,
        manager_key: &OBCDaoManageKey,
        value: u64,
    ) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::set_voting_delay(inner_state, manager_key, value);
    }

    public entry fun cast_vote(
        self: &mut ObcSystemState,
        proposal: &mut Proposal,
        coin: VotingObc,
        agreeInt: u8,
        clock: & Clock,
        ctx: &mut TxContext,
    )  {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::cast_vote(inner_state, proposal, coin, agreeInt, clock, ctx);
    }

    public entry fun change_vote(
        self: &mut ObcSystemState,
        my_vote: &mut Vote,
        proposal: &mut Proposal,
        agree: bool,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::change_vote(inner_state, my_vote, proposal, agree, clock, ctx);
    }

    public entry fun queue_proposal_action(
        self: &mut ObcSystemState,
        manager_key: &OBCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::queue_proposal_action(inner_state, manager_key, proposal, clock);
    }

    public entry fun revoke_vote(
        self: &mut ObcSystemState,
        proposal: &mut Proposal,
        my_vote:  Vote,
        voting_power: u64,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(self);
        obc_system_state_inner::revoke_vote(inner_state, proposal, my_vote, voting_power, clock, ctx);
    }

    public entry fun unvote_votes(
        proposal: &mut Proposal,
        vote: Vote,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        obc_dao::unvote_votes(proposal, vote, clock, ctx);
    }

    public entry fun vote_of(
        vote: &Vote,
        proposal: &Proposal,
        ctx: &mut TxContext,
    ) {
        obc_dao::vote_of(vote, proposal, ctx);
    }

    public entry fun has_vote(
        vote: &Vote,
        proposal: &Proposal,
    ) {
        obc_dao::has_vote(vote, proposal);
    }

}
