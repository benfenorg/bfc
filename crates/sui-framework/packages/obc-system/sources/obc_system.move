module obc_system::obc_system {
    use sui::balance::{Balance, Supply};
    use sui::coin;
    use sui::coin::Coin;
    use sui::clock::{Clock};
    use sui::dynamic_field;

    use sui::obc::OBC;
    use sui::object::UID;
    use sui::stable::STABLE;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use obc_system::usd::USD;
    use obc_system::obc_dao_manager::{OBCDaoManageKey};
    use obc_system::obc_dao::{Proposal};
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
        round: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(wrapper);
        obc_system_state_inner::update_round(inner_state, round);
        //exchange all stable to obc.
        obc_system_state_inner::request_exchange_all(inner_state, ctx);
        // //update inner exchange rate from stable-swap.
        // let stable = coin::zero<STABLE>(ctx);
        //todo read rate from stable-swap.
        // let rate = 1000000000;
        // obc_system_state_inner::request_update_gas_coin(inner_state, &stable, rate);
        // balance::destroy_zero(coin::into_balance(stable));

        // X-treasury rebalance
        obc_system_state_inner::rebalance(inner_state, clock, ctx);
    }

    public entry fun update_round(
        wrapper: &mut ObcSystemState,
	clock: &Clock, 
        ctx: &mut TxContext,
    ){
        obc_round(wrapper, 200, clock, ctx);
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
        chain_start_timestamp_ms: u64,
    ): ObcSystemParameters {
        obc_system_state_inner::obc_system_stat_parameter(
            position_number,
            tick_spacing,
            spacing_times,
            initialize_price,
            time_interval,
            base_point,
            chain_start_timestamp_ms,
        )
    }

    entry public fun destroy_terminated_proposal(
        wrapper: &mut ObcSystemState,
        manager_key: &OBCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::destroy_terminated_proposal(system_state, manager_key, proposal, clock);
    }

    entry public fun propose(
        wrapper: &mut ObcSystemState,
        manager_key: &OBCDaoManageKey,
        payment: Coin<OBC>,
        action_id: u64,
        action_delay: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::propose(system_state, manager_key, payment, action_id, action_delay, clock, ctx);
    }

    entry public fun create_obcdao_action(
        wrapper: &mut ObcSystemState,
        _: &OBCDaoManageKey,
        actionName: vector<u8>,
        ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::create_obcdao_action(system_state, _, actionName, ctx);
    }

    entry public fun judge_proposal_state(wrapper: &mut ObcSystemState, current_time: u64) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::judge_proposal_state(system_state, current_time);
    }

    entry public fun modify_proposal(wrapper: &mut ObcSystemState, index: u8, clock: &Clock) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::modify_proposal(system_state, index, clock);
    }

    /// X treasury  swap obc to stablecoin
    public entry fun mint<StableCoinType>(
        wrapper: &mut ObcSystemState,
        coin_obc: Coin<OBC>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::mint<StableCoinType>(system_state, coin_obc, amount, ctx);
    }

    /// X treasury  swap stablecoin to obc
    public entry fun redeem<StableCoinType>(
        wrapper: &mut ObcSystemState,
        coin_sc: Coin<StableCoinType>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        obc_system_state_inner::redeem<StableCoinType>(system_state, coin_sc, amount, ctx);
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
}
