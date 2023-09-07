module obc_system::obc_system_state_inner {
    use sui::object::{Self, UID};
    use sui::balance::{Balance, Supply};
    use sui::coin::Coin;
    use sui::obc::OBC;
    use sui::stable::STABLE;
    use sui::tx_context::TxContext;
    use sui::vec_map;
    use sui::dynamic_object_field;

    use obc_system::exchange_inner;
    use obc_system::exchange_inner::ExchangePool;
    use obc_system::gas_coin_map;
    use obc_system::gas_coin_map::{GasCoinMap, GasCoinEntity};
    use obc_system::treasury;
    use obc_system::usd::USD;

    friend obc_system::obc_system;

    const OBC_SYSTEM_STATE_START_ROUND: u64 = 0;

    spec module { pragma verify = false; }

    struct ObcSystemStateInner has key, store {
        id: UID,

        round: u64,
        /// Contains gas coin information
        gas_coin_map: GasCoinMap,
        /// Exchange gas coin pool
        exchange_pool: ExchangePool<STABLE>,
    }

    struct TreasuryParameters has drop, copy {
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        initialize_price: u128,
        time_interval: u32,
        base_point: u64,
    }

    struct ObcSystemParameters has drop, copy {
        treasury_parameters: TreasuryParameters,
        chain_start_timestamp_ms: u64,
    }

    const OBC_SYSTEM_TREASURY_KEY: u64 = 1;

    public(friend) fun create_inner_state(
        usd_supply: Supply<USD>,
        parameters: ObcSystemParameters,
        ctx: &mut TxContext,
    ): ObcSystemStateInner {
        // init gas coin mappings
        let init_gas_coins_map = vec_map::empty<address, GasCoinEntity>();
        let gas_coin_map = gas_coin_map::new(init_gas_coins_map, ctx);
        let exchange_pool = exchange_inner::new_exchange_pool<STABLE>(ctx, 0);

        let inner = ObcSystemStateInner {
            id: object::new(ctx),
            round: OBC_SYSTEM_STATE_START_ROUND,
            gas_coin_map,
            exchange_pool,
        };

        create_treasury(
            &mut inner,
            usd_supply,
            parameters.treasury_parameters,
            parameters.chain_start_timestamp_ms,
            ctx
        );

        inner
    }

    public(friend) fun update_round(
        inner: &mut ObcSystemStateInner,
        round: u64,
    ) {
        inner.round = round;
    }

    public(friend) fun request_exchange_stable(
        inner: &mut ObcSystemStateInner,
        stable: Coin<STABLE>,
        ctx: &mut TxContext,
    ): Balance<OBC> {
        //get exchange rate
        let rate = gas_coin_map::requst_get_exchange_rate<STABLE>(&inner.gas_coin_map, &stable);
        exchange_inner::request_exchange_stable<STABLE>(rate, &mut inner.exchange_pool, stable, ctx)
    }

    public(friend) fun request_exchange_all(
        inner: &mut ObcSystemStateInner,
        ctx: &mut TxContext
    ) {
        exchange_inner::request_exchange_all<STABLE>(&mut inner.exchange_pool, ctx)
    }

    ///Request withdraw stable coin.
    public(friend) fun request_withdraw_stable(
        inner: &mut ObcSystemStateInner,
    ): Balance<STABLE> {
        exchange_inner::request_withdraw_stable(&mut inner.exchange_pool)
    }

    /// Getter of the gas coin exchange pool rate.
    public(friend) fun requst_get_exchange_rate<CoinType>(
        self: &ObcSystemStateInner,
        stable: &Coin<CoinType>
    ): u64 {
        gas_coin_map::requst_get_exchange_rate<CoinType>(&self.gas_coin_map, stable)
    }

    public(friend) fun request_add_gas_coin<CoinType>(
        self: &mut ObcSystemStateInner,
        gas_coin: &Coin<CoinType>,
        rate: u64,
    ) {
        gas_coin_map::request_add_gas_coin<CoinType>(&mut self.gas_coin_map, gas_coin, rate)
    }

    public(friend) fun request_update_gas_coin<CoinType>(
        self: &mut ObcSystemStateInner,
        gas_coin: &Coin<CoinType>,
        rate: u64,
    ) {
        gas_coin_map::request_update_gas_coin(&mut self.gas_coin_map, gas_coin, rate)
    }

    public(friend) fun request_remove_gas_coin<CoinType>(
        self: &mut ObcSystemStateInner,
        gas_coin: &Coin<CoinType>,
    ) {
        gas_coin_map::request_remove_gas_coin<CoinType>(&mut self.gas_coin_map, gas_coin)
    }

    /// Init exchange pool by add obc coin.
    public fun init_exchange_pool(
        self: &mut ObcSystemStateInner,
        coin: Coin<OBC>,
    ) {
        exchange_inner::add_obc_to_pool(&mut self.exchange_pool, coin)
    }

    fun create_treasury(
        inner: &mut ObcSystemStateInner,
        supply: Supply<USD>,
        treasury_parameters: TreasuryParameters,
        ts: u64,
        ctx: &mut TxContext
    ) {
        let t = treasury::create_treasury(ctx);
        dynamic_object_field::add(&mut inner.id, OBC_SYSTEM_TREASURY_KEY, t);

        let mut_t = dynamic_object_field::borrow_mut<u64, treasury::Treasury>(
            &mut inner.id,
            OBC_SYSTEM_TREASURY_KEY
        );

        // create obc-usd pool
        treasury::create_vault<OBC, USD, USD>(
            mut_t,
            supply,
            treasury_parameters.position_number,
            treasury_parameters.tick_spacing,
            treasury_parameters.initialize_price,
            ts,
            ctx,
        );
        // init positions
        treasury::init_positions<OBC, USD>(
            mut_t,
            treasury_parameters.tick_spacing,
            treasury_parameters.spacing_times,
            ctx
        );
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
        let treasury_parameters = TreasuryParameters {
            position_number,
            tick_spacing,
            spacing_times,
            initialize_price,
            time_interval,
            base_point,
        };
        ObcSystemParameters {
            treasury_parameters,
            chain_start_timestamp_ms,
        }
    }
}