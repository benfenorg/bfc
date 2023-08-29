module obc_system::obc_system {
    use sui::tx_context::TxContext;
    use obc_system::obc::length;

    struct ObcSystemStateInner has store {
        /// The current epoch ID, starting from 0.
        round: u64,
    }

    public(friend) fun create(
        ctx: &mut TxContext,
    ){
        //let exchange_gas_coin_pool =  exchange_inner::new_exchange_pool(ctx, 0);
    }

    fun request_exchange_stable_obc(
        ctx: &mut TxContext
    ){

    }

    #[allow(unused_function)]
    fun obc_round(ctx: &mut TxContext):u64 {
        length()
    }
}