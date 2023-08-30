module obc_system::obc_system {
    use sui::object::UID;
    use sui::transfer;
    use sui::tx_context::TxContext;
    use obc_system::obc::length;

    struct ObcSystemState has key {
        id: UID,
    }

    struct ObcSystemStateInner has store {
        round: u64,
    }

    #[allow(unused_function)]
    fun create(
        id: UID,
        ctx: &mut TxContext,
    ){
        //let exchange_gas_coin_pool =  exchange_inner::new_exchange_pool(ctx, 0);
        // let _system_state = ObcSystemStateInner{
        //     round,
        // };

        let self = ObcSystemState {
            id,
        };

        //dynamic_field::add(&mut self.id,1, system_state);

        transfer::share_object(self);
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