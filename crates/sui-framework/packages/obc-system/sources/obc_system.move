module obc_system::obc_system {
    use sui::dynamic_field;
    use sui::object::UID;
    use sui::transfer;
    use sui::tx_context::TxContext;
    use obc_system::obc::length;

    struct ObcSystemState has key {
        id: UID,
        version:u64
    }

    struct ObcSystemStateInner has store {
        round: u64,
    }

    const OBC_SYSTEM_STATE_VERSION_V1: u64 = 1;

    #[allow(unused_function)]
    fun create(
        id: UID,
        ctx: &mut TxContext,
    ){
        //let exchange_gas_coin_pool =  exchange_inner::new_exchange_pool(ctx, 0);
        let system_state = ObcSystemStateInner{
             round:0,
        };

        let self = ObcSystemState {
            id,
            version:OBC_SYSTEM_STATE_VERSION_V1
        };

        dynamic_field::add(&mut self.id,OBC_SYSTEM_STATE_VERSION_V1, system_state);

        transfer::share_object(self);
    }

    fun request_exchange_stable_obc(
        ctx: &mut TxContext
    ){

    }

    #[allow(unused_function)]
    fun obc_round(wrapper: &mut ObcSystemState,round:u64,ctx: &mut TxContext){
        let system_state = load_system_state_mut(wrapper);
        system_state.round = round;
    }

    fun load_system_state_mut(self: &mut ObcSystemState): &mut ObcSystemStateInner {
        let inner = dynamic_field::borrow_mut(&mut self.id, self.version);
        inner
    }

}