module sui_system::stable_coin {

    use sui::coin::Coin;
    use sui::obc::OBC;
    use sui::coin;
    use sui::tx_context::TxContext;

    struct DummyCoin<phantom T> has store, drop {
    }

    /// Request of swap obc coin
    public(friend) fun request_swap_obc<CoinType>(
        _stable_coin: DummyCoin<CoinType>, ctx: &mut TxContext): Coin<OBC>  {
       // mock for rust test
       coin::zero<OBC>(ctx)
    }

    /// Request of swap two stable coin
    public(friend) fun request_swap<CoinX, CoinY>(
        _stable_coin: DummyCoin<CoinX>, _stable2: DummyCoin<CoinY>)  {
    }

    /// Request of price of two stable coin
    public(friend) fun request_price<CoinX, CoinY>(
        _stable_coin: DummyCoin<CoinX>, _stable2: DummyCoin<CoinY>)  {
    }

}
