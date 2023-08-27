module obc_system::obc_system {
    use obc_system::exchange_inner;
    use sui::tx_context::TxContext;

    public(friend) fun create(
        ctx: &mut TxContext,
    ){
        //let exchange_gas_coin_pool =  exchange_inner::new_exchange_pool(ctx, 0);
    }

    fun request_exchange_stable_obc(
        ctx: &mut TxContext
    ){

    }
}