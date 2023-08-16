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
    use sui_system::gas_coin_map::{GasCoinMap, requst_get_exchange_rate};

    friend sui_system::genesis;
    friend sui_system::sui_system_state_inner;


    const ENotActivePool: u64 = 1;
    const EZeroAmount: u64 = 2;
    const EOBCZeroAmount: u64 = 3;
    const ELackOfOBC: u64 = 4;


    struct ExchangePool<phantom T: key + store> has key, store {
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
        stable_pool: Balance<T>,
    }

    /// Init exchange pool for gas coin exchange
    public(friend) fun new_exchange_pool<T: key + store>(ctx: &mut TxContext) : ExchangePool<T> {
        ExchangePool<T> {
            id: object::new(ctx),
            activation_epoch: option::none(),
            obc_balance: 0,
            obc_pool: balance::zero(),
            stable_token_balance: 0,
            stable_pool: balance::zero<T>(),
        }
    }

    /// Returns true if the input exchange pool is active.
    public fun is_active<T: key + store>(pool: &ExchangePool<T>): bool {
        option::is_some(&pool.activation_epoch)
    }

    /// Get obc amount by exchange rate
    public(friend) fun get_obc_amount(exchange_rate: u64, token_amount: u64): u64 {
        let res = (token_amount as u128) / (exchange_rate as u128);
        (res as u64)
    }

    /// Request for exchange gas coin to default coin.
    public(friend) fun request_exchange_gas<T: key + store>(
        map: & GasCoinMap,
        pool: &mut ExchangePool<T>,
        stable_coin: Coin<T>,
        ctx: &mut TxContext): Coin<OBC> {
        assert!(coin::value(&stable_coin) > 0, EZeroAmount);
        let rate = requst_get_exchange_rate<T>(map, &stable_coin);
        let tok_balance = coin::into_balance(stable_coin);
        let stable_amount = balance::value(&tok_balance);

        let obc_amount= get_obc_amount(rate, stable_amount);
        assert!(obc_amount > 0, EOBCZeroAmount);
        assert!(pool.obc_balance > obc_amount, ELackOfOBC);
        balance::join(&mut pool.stable_pool, tok_balance);
        coin::take(&mut pool.obc_pool, obc_amount, ctx)
    }

    /// Exchange all stable gas coins to default coins
    public(friend) fun request_exchange_all<T: key + store>(
        pool: &mut ExchangePool<T>,
        ctx: &mut TxContext) {
        assert!(is_active(pool), ENotActivePool);
        if(pool.stable_token_balance > 0) {
            // call stable swap interface
            // let obc = stable_coin::request_swap_obc<CoinType>(coin::from_balance<CoinType>(pool.stable_pool, ctx), ctx);
            let obc = stable_coin::request_swap_obc<T>(stable_coin::new_dummy<T>(), ctx);
            // store obc to exchange pool
            balance::join(&mut pool.obc_pool, coin::into_balance(obc));
        }
    }

    /// Withdraw the stable gas coins.
    public(friend) fun request_withdraw_stable_gas<T: key + store>(
        pool: &mut ExchangePool<T>,
        ctx: &mut TxContext): Coin<T> {
        let balance = balance::withdraw_all<T>(&mut pool.stable_pool);
        coin::from_balance<T>(balance, ctx)
    }
}
