module obc_system::stable_coin {

    use sui::coin::Coin;
    use sui::obc::OBC;
    use sui::coin;
    use sui::tx_context::TxContext;

    struct DummyCoin<phantom T> has store, drop {
    }

    public fun new_dummy<T>(): DummyCoin<T> {
        DummyCoin{}
    }

    /// Request of swap obc coin
    public fun request_swap_obc<CoinType>(
        _stable_coin: DummyCoin<CoinType>, ctx: &mut TxContext): Coin<OBC>  {
       // mock for rust test
       coin::zero<OBC>(ctx)
    }

    /// Request of swap two stable coin
    public fun request_swap<CoinX, CoinY>(
        _stable_coin: DummyCoin<CoinX>, _stable2: DummyCoin<CoinY>)  {
    }

    /// Request of price of two stable coin
    public fun request_price<CoinX, CoinY>(
        _stable_coin: DummyCoin<CoinX>, _stable2: DummyCoin<CoinY>)  {
    }

}
