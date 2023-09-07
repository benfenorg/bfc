module obc_system::obc_system {
    use sui::balance::{Balance, Supply};
    use sui::coin;
    use sui::coin::Coin;
    use sui::dynamic_object_field;

    use sui::obc::OBC;
    use sui::object::UID;
    use sui::stable::STABLE;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use obc_system::usd::USD;
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

        dynamic_object_field::add(&mut self.id, OBC_SYSTEM_STATE_VERSION_V1, inner_state);

        transfer::share_object(self);
    }

    public fun obc_round(
        wrapper: &mut ObcSystemState,
        round: u64,
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
    }

    public entry fun update_round(
        wrapper: &mut ObcSystemState,
        ctx: &mut TxContext,
    ) {
        obc_round(wrapper, 200, ctx)
    }

    fun load_system_state(
        self: &ObcSystemState
    ): &ObcSystemStateInner {
        dynamic_object_field::borrow(&self.id, self.version)
    }

    fun load_system_state_mut(
        self: &mut ObcSystemState
    ): &mut ObcSystemStateInner {
        dynamic_object_field::borrow_mut(&mut self.id, self.version)
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
}