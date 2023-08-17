module sui_system::exchange_inner {
    use sui::coin::Coin;
    use sui::object;
    use sui::tx_context::TxContext;
    use sui::object::UID;
    use std::option::Option;
    use sui::balance::Balance;
    use sui::obc::OBC;
    use std::option;
    use sui::balance;
    use sui::coin;
    use sui_system::stable_coin;
    use sui::stable::STABLE;

    friend sui_system::sui_system;
    friend sui_system::genesis;
    friend sui_system::sui_system_state_inner;

    #[test_only]
    friend sui_system::exchange_inner_tests;

    const ENotActivePool: u64 = 1;
    const EZeroAmount: u64 = 2;
    const EOBCZeroAmount: u64 = 3;
    const ELackOfOBC: u64 = 4;


    struct ExchangePool has key, store {
        id: UID,
        /// The epoch at which this pool became active.
        /// The value is `None` if the pool is pre-active and `Some(<epoch_number>)` if active or inactive.
        activation_epoch: Option<u64>,
        /// The total number of SUI coins in this pool
        obc_balance: u64,
        /// The epoch stake rewards will be added here at the end of each epoch.
        obc_pool: Balance<OBC>,
        /// Total number of pool stable coins issued by the pool.
        stable_token_balance: u64,
        /// The epoch stable gas coins
        //todo replace generic types
        stable_pool: Balance<STABLE>,
    }

    /// Init exchange pool for gas coin exchange
    public(friend) fun new_exchange_pool(ctx: &mut TxContext, epoch: u64) : ExchangePool {
        ExchangePool {
            id: object::new(ctx),
            activation_epoch: option::some(epoch),
            obc_balance: 0,
            obc_pool: balance::zero(),
            stable_token_balance: 0,
            stable_pool: balance::zero<STABLE>(),
        }
    }

    /// Add obc to pool for gas exchange.
    public(friend) fun add_obc(pool: &mut ExchangePool, coin: Coin<OBC>) {
        let amount = coin::value(&coin);
        assert!( amount > 0, EZeroAmount);
        pool.obc_balance = pool.obc_balance + amount;
        let balance = coin::into_balance(coin);
        balance::join(&mut pool.obc_pool, balance);
    }

    /// Returns true if the input exchange pool is active.
    public fun is_active(pool: &ExchangePool): bool {
        option::is_some(&pool.activation_epoch)
    }

    public(friend) fun get_obc_amount(pool: &ExchangePool): u64 {
        pool.obc_balance
    }

    public(friend) fun get_stable_amount(pool: &ExchangePool): u64 {
        pool.stable_token_balance
    }

    /// Get obc amount by exchange rate
    public(friend) fun exchange_obc_amount(exchange_rate: u64, token_amount: u64): u64 {
        let res = (token_amount as u128) / (exchange_rate as u128);
        (res as u64)
    }

    /// Request for exchange gas coin to default coin.
    public(friend) fun request_exchange_gas(
        exchange_rate: u64,
        pool: &mut ExchangePool,
        stable_coin: Coin<STABLE>,
        ctx: &mut TxContext
    ): Balance<OBC> {
        assert!(coin::value(&stable_coin) > 0, EZeroAmount);
        let tok_balance = coin::into_balance(stable_coin);
        let stable_amount = balance::value(&tok_balance);
        let obc_amount= exchange_obc_amount(exchange_rate, stable_amount);
        assert!(obc_amount > 0, EOBCZeroAmount);
        assert!(pool.obc_balance > obc_amount, ELackOfOBC);
        balance::join(&mut pool.stable_pool, tok_balance);
        let result = coin::take(&mut pool.obc_pool, obc_amount, ctx);
        pool.obc_balance = pool.obc_balance - obc_amount;
        pool.stable_token_balance = pool.stable_token_balance + stable_amount;
        coin::into_balance(result)
    }

    /// Exchange all stable gas coins to default coins
    public(friend) fun request_exchange_all(
        pool: &mut ExchangePool,
        ctx: &mut TxContext
    ) {
        assert!(is_active(pool), ENotActivePool);
        if(pool.stable_token_balance > 0) {
            // call stable swap interface
            // let obc = stable_coin::request_swap_obc<CoinType>(coin::from_balance<CoinType>(pool.stable_pool, ctx), ctx);
            let obc = stable_coin::request_swap_obc<STABLE>(stable_coin::new_dummy<STABLE>(), ctx);
            // store obc to exchange pool
            pool.obc_balance = pool.obc_balance + coin::value(&obc);
            balance::join(&mut pool.obc_pool, coin::into_balance(obc));
            pool.stable_token_balance = 0;
        }
    }

    /// Withdraw the stable gas coins.
    public(friend) fun request_withdraw_stable_gas(
        pool: &mut ExchangePool,
    ): Balance<STABLE> {
        pool.stable_token_balance = 0;
         balance::withdraw_all<STABLE>(&mut pool.stable_pool)
    }
}
