module obc_system::obc_system_state_inner {

    use sui::balance::Balance;
    use obc_system::exchange_inner;
    use obc_system::exchange_inner::ExchangePool;
    use sui::coin::Coin;
    use sui::obc::OBC;
    use sui::stable::STABLE;
    use sui::tx_context::TxContext;
    use sui::vec_map;
    use obc_system::gas_coin_map;
    use obc_system::gas_coin_map::{GasCoinMap, GasCoinEntity};

    friend obc_system::obc_system;

    const OBC_SYSTEM_STATE_START_ROUND: u64 = 0;

    spec module { pragma verify = false; }

    struct ObcSystemStateInner has store {
        round: u64,
        /// Contains gas coin information
        gas_coin_map: GasCoinMap,
        /// Exchange gas coin pool
        exchange_pool: ExchangePool<STABLE>,
    }

    public(friend) fun create_inner_state(
        ctx: &mut TxContext,
    ): ObcSystemStateInner {
        // init gas coin mappings
        let init_gas_coins_map = vec_map::empty<address, GasCoinEntity>();
        let gas_coin_map = gas_coin_map::new(init_gas_coins_map, ctx);
        let exchange_pool =  exchange_inner::new_exchange_pool<STABLE>(ctx, 0);
        ObcSystemStateInner {
            round: OBC_SYSTEM_STATE_START_ROUND,
            gas_coin_map,
            exchange_pool,
        }
    }

    public(friend) fun update_round(
        inner: &mut ObcSystemStateInner,
        round: u64,
    ){
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
}