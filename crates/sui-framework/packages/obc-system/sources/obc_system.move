module obc_system::obc_system {
    use sui::tx_context::TxContext;

    fun request_exchange_stable_obc(
        ctx: &mut TxContext
    ){

    }

    public entry fun length(): u64 {
        32
    }
}