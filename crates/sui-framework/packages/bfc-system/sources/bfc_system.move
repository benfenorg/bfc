module bfc_system::bfc_system {
    use sui::balance;
    use bfc_system::bfc_dao;
    use bfc_system::voting_pool::VotingBfc;
    use sui::balance::{Balance, Supply};
    use sui::coin;
    use sui::coin::Coin;
    use sui::clock::{Clock};
    use sui::dynamic_field;
    use sui::clock::{Self};

    use sui::bfc::BFC;
    use sui::object::UID;
    use sui::stable::STABLE;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use bfc_system::busd::{BUSD};
    use bfc_system::vault;
    use bfc_system::vault::VaultInfo;
    use bfc_system::bfc_dao_manager::{BFCDaoManageKey, ManagerKeyBfc};
    use bfc_system::bfc_dao::{Proposal, Vote};
    use bfc_system::bfc_system_state_inner;
    use bfc_system::bfc_system_state_inner::{BfcSystemStateInner, BfcSystemParameters};

    #[test_only]
    friend bfc_system::bfc_system_tests;

    struct BfcSystemState has key {
        id: UID,
        version: u64
    }

    public entry fun create_stake_manager_key( payment: Coin<BFC>,
                                               ctx: &mut TxContext) {
        bfc_system_state_inner::create_stake_manager_key(payment, ctx);
    }

    public entry fun unstake_manager_key(key: BFCDaoManageKey,
                                         token: ManagerKeyBfc,
                                         ctx: &mut TxContext) {
        bfc_system_state_inner::unstake_manager_key(key, token, ctx);
    }


    const BFC_SYSTEM_STATE_VERSION_V1: u64 = 1;

    spec module { pragma verify = false; }

    public fun create(
        id: UID,
        usd_supply: Supply<BUSD>,
        bfc_balance: Balance<BFC>,
        parameters: BfcSystemParameters,
        ctx: &mut TxContext
    ) {
        let inner_state = bfc_system_state_inner::create_inner_state(
            usd_supply,
            bfc_balance,
            parameters,
            ctx,
        );
        let self = BfcSystemState {
            id,
            version: BFC_SYSTEM_STATE_VERSION_V1
        };

        dynamic_field::add(&mut self.id, BFC_SYSTEM_STATE_VERSION_V1, inner_state);

        transfer::share_object(self);
    }

    public entry fun change_round( wrapper: &mut BfcSystemState, round: u64) {
        let inner_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::update_round(inner_state, round);
    }

    public fun bfc_round(
        wrapper: &mut BfcSystemState,
        clock: &Clock,
        round: u64,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::update_round(inner_state, round);
        //exchange all stable to bfc.
        bfc_system_state_inner::request_exchange_all(inner_state, ctx);
        //update inner exchange rate from stable-swap.
        let stable = coin::zero<BUSD>(ctx);
        bfc_system_state_inner::request_update_gas_coin(inner_state, &stable);
        balance::destroy_zero(coin::into_balance(stable));
        // X-treasury rebalance
        bfc_system_state_inner::rebalance(inner_state, clock, ctx);

        judge_proposal_state(wrapper, clock::timestamp_ms(clock));
    }

    //todo close
    public entry fun update_round(
        wrapper: &mut BfcSystemState,
	clock: &Clock, 
        ctx: &mut TxContext,
    ){
        bfc_round(wrapper,  clock,200, ctx);
    }

    fun load_system_state(
        self: &BfcSystemState,
    ): &BfcSystemStateInner {
        dynamic_field::borrow(&self.id, self.version)
    }
    public fun load_bfc_system_state(id: &UID, version: u64): &BfcSystemStateInner {
        dynamic_field::borrow(id, version)
    }

    fun load_system_state_mut(
        self: &mut BfcSystemState
    ): &mut BfcSystemStateInner {
        dynamic_field::borrow_mut(&mut self.id, self.version)
    }

    public fun get_exchange_rate(id: &UID, version: u64): u64 {
        let inner = load_bfc_system_state(id, version);
        bfc_system_state_inner::get_stablecoin_exchange_rate<BUSD>(inner)
    }

    /// Getter of the gas coin exchange pool rate.
    public entry fun request_get_exchange_rate(
        self: &BfcSystemState,
        stable: &Coin<BUSD>
    ): u64 {
        let inner_state = load_system_state(self);
        bfc_system_state_inner::requst_get_exchange_rate<BUSD>(inner_state, stable)
    }

    public entry fun request_add_gas_coin(
        self: &mut BfcSystemState,
        gas_coin: &Coin<BUSD>,
        rate: u64,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::request_add_gas_coin(inner_state, gas_coin, rate)
    }

    public entry fun request_update_gas_coin(
        self: &mut BfcSystemState,
        gas_coin: &Coin<BUSD>,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::request_update_gas_coin(inner_state, gas_coin)
    }

    public entry fun request_remove_gas_coin(
        self: &mut BfcSystemState,
        gas_coin: &Coin<BUSD>,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::request_remove_gas_coin(inner_state, gas_coin)
    }

    /// Request exchange stable coin to bfc.
    public entry fun request_exchange_stable(
        self: &mut BfcSystemState,
        stable: Coin<BUSD>,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(self);
        let balance = bfc_system_state_inner::swap_stablecoin_to_bfc_balance<BUSD>(
            inner_state,
            stable,
            ctx);
        transfer::public_transfer(coin::from_balance(balance, ctx), tx_context::sender(ctx));
    }

    /// Request exchange all stable coin to bfc.
    public entry fun request_exchange_all(
        self: &mut BfcSystemState,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::request_exchange_all(inner_state, ctx)
    }

    /// Request withdraw stable coin.
    public entry fun request_withdraw_stable(
        self: &mut BfcSystemState,
        ctx: &mut TxContext,
    ) {
        let stables = request_withdraw_stable_no_entry(self);
        transfer::public_transfer(coin::from_balance(stables, ctx), tx_context::sender(ctx));
    }

    fun request_withdraw_stable_no_entry(
        self: &mut BfcSystemState,
    ): Balance<BUSD> {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::request_withdraw_stable(inner_state)
    }

    /// Init exchange pool by add bfc coin.
    public entry fun init_exchange_pool(
        self: &mut BfcSystemState,
        coin: Coin<BFC>,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::init_exchange_pool(inner_state, coin)
    }
    public entry fun get_bfc_amount(
    self: &BfcSystemState): u64
    {
        let inner_state = load_system_state(self);
        bfc_system_state_inner::get_bfc_amount(inner_state)
    }

    public(friend) fun bfc_system_stat_parameter(
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        initialize_price: u128,
        time_interval: u32,
        base_point: u64,
        max_counter_times: u32,
        chain_start_timestamp_ms: u64,
    ): BfcSystemParameters {
        bfc_system_state_inner::bfc_system_stat_parameter(
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
        wrapper: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::destroy_terminated_proposal(system_state, manager_key, proposal, clock);
    }

    public entry fun propose(
        wrapper: &mut BfcSystemState,
        version_id : u64,
        payment: Coin<BFC>,
        action_id: u64,
        action_delay: u64,
        description: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::propose(system_state, version_id, payment, action_id, action_delay, description, clock, ctx);
    }

    public entry fun create_bfcdao_action(
        wrapper: &mut BfcSystemState,
        payment: Coin<BFC>,
        actionName: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::create_bfcdao_action(system_state, payment, actionName,clock, ctx);
    }

    public entry fun judge_proposal_state(wrapper: &mut BfcSystemState, current_time: u64) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::judge_proposal_state(system_state, current_time);
    }

    public entry fun set_voting_period(
        wrapper: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::set_voting_period(system_state, manager_key, value);
    }

    public entry fun modify_proposal(wrapper: &mut BfcSystemState, proposal_obj: &mut Proposal, index: u8, clock: &Clock) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::modify_proposal(system_state, proposal_obj, index, clock);
    }

    public entry fun set_voting_quorum_rate(wrapper: &mut BfcSystemState, manager_key: &BFCDaoManageKey, value: u8,){
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::set_voting_quorum_rate(system_state, manager_key, value);
    }

    public entry fun set_min_action_delay(
        wrapper: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::set_min_action_delay(system_state, manager_key, value);
    }

    public entry fun withdraw_voting(   wrapper: &mut BfcSystemState,
                                 voting_bfc: VotingBfc,
                                    clock: &Clock,
                                 ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::withdraw_voting(system_state, voting_bfc,clock, ctx);
    }

    public entry fun create_voting_bfc(wrapper: &mut BfcSystemState,
                                 coin: Coin<BFC>,
                                    clock: &Clock,
                                 ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::create_voting_bfc(system_state, coin,clock, ctx);
    }

    /// X treasury  swap bfc to stablecoin
    public entry fun swap_bfc_to_stablecoin<StableCoinType>(
        wrapper: &mut BfcSystemState,
        native_coin: Coin<BFC>,
        clock: &Clock,
        amount: u64,
        min_amount: u64,
        deadline: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::swap_bfc_to_stablecoin<StableCoinType>(system_state, native_coin, clock, amount, min_amount, deadline, ctx);
    }

    /// X treasury  swap stablecoin to bfc
    public entry fun swap_stablecoin_to_bfc<StableCoinType>(
        wrapper: &mut BfcSystemState,
        stable_coin: Coin<StableCoinType>,
        clock: &Clock,
        amount: u64,
        min_amount: u64,
        deadline: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::swap_stablecoin_to_bfc<StableCoinType>(system_state, stable_coin, clock, amount, min_amount, deadline, ctx);
    }

    public fun get_stablecoin_by_bfc<StableCoinType>(
        wrapper: &BfcSystemState,
        amount: u64,
    ): vault::CalculatedSwapResult
    {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_stablecoin_by_bfc<StableCoinType>(system_state, amount)
    }

    public fun get_bfc_by_stablecoin<StableCoinType>(
        wrapper: &BfcSystemState,
        amount: u64,
    ): vault::CalculatedSwapResult
    {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_bfc_by_stablecoin<StableCoinType>(system_state, amount)
    }

    public fun vault_info<StableCoinType>(wrapper: &BfcSystemState): VaultInfo {
        let inner_state = load_system_state(wrapper);
        bfc_system_state_inner::vault_info<StableCoinType>(inner_state)
    }

    public fun get_bfc_exchange_rate<StableCoinType>(wrapper: &BfcSystemState): u64
    {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_bfc_exchange_rate<StableCoinType>(system_state)
    }

    public fun get_stablecoin_exchange_rate<StableCoinType>(wrapper: &BfcSystemState): u64
    {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_stablecoin_exchange_rate<StableCoinType>(system_state)
    }

    public fun next_epoch_bfc_required(wrapper: &BfcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::next_epoch_bfc_required(system_state)
    }

    public fun treasury_balance(wrapper: &BfcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::treasury_balance(system_state)
    }

    public entry fun deposit_to_treasury(self: &mut BfcSystemState, bfc: Coin<BFC>) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::deposit_to_treasury(inner_state, bfc)
    }

    public entry fun set_voting_delay(
        self: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::set_voting_delay(inner_state, manager_key, value);
    }

    public entry fun cast_vote(
        self: &mut BfcSystemState,
        proposal: &mut Proposal,
        coin: VotingBfc,
        agreeInt: u8,
        clock: & Clock,
        ctx: &mut TxContext,
    )  {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::cast_vote(inner_state, proposal, coin, agreeInt, clock, ctx);
    }

    public entry fun change_vote(
        self: &mut BfcSystemState,
        my_vote: &mut Vote,
        proposal: &mut Proposal,
        agree: bool,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::change_vote(inner_state, my_vote, proposal, agree, clock, ctx);
    }

    public entry fun queue_proposal_action(
        self: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::queue_proposal_action(inner_state, manager_key, proposal, clock);
    }

    public entry fun revoke_vote(
        self: &mut BfcSystemState,
        proposal: &mut Proposal,
        my_vote:  Vote,
        voting_power: u64,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::revoke_vote(inner_state, proposal, my_vote, voting_power, clock, ctx);
    }

    public entry fun unvote_votes(
        proposal: &mut Proposal,
        vote: Vote,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao::unvote_votes(proposal, vote, clock, ctx);
    }

    public entry fun vote_of(
        vote: &Vote,
        proposal: &Proposal,
        ctx: &mut TxContext,
    ) {
        bfc_dao::vote_of(vote, proposal, ctx);
    }

    public entry fun has_vote(
        vote: &Vote,
        proposal: &Proposal,
    ) {
        bfc_dao::has_vote(vote, proposal);
    }


    public entry fun cluster_add_admin(
        new_admin:address,
        ctx: &mut TxContext,
    ) {
        bfc_dao::add_admin(new_admin, ctx);
        //bfc::new(new_admin, ctx);
    }
}
