module polynet::bfc_usdt {
    use std::option;
    //use std::string::{String};

    use sui::coin;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use polynet::lock_proxy;

    const ENOT_BRIDGE_ADMIN: u64 = 4001;

    const HUGE_U64: u64 = 10000000000000000000;

    struct BFC_USDT has drop {}

    const DECIMALS: u8 = 8;

    fun init(witness: BFC_USDT, ctx: &mut TxContext){

        build_usdt(witness, DECIMALS, tx_context::sender(ctx), ctx);
    }

    public fun build_usdt<T:drop>(witness: T, decimals: u8, admin: address, ctx: &mut TxContext){
        let (cap, metadata) = coin::create_currency(
            witness,
            decimals,
            b"BFC_USDT",
            b"Benfen USD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock = coin::mint<T>(&mut cap, HUGE_U64, ctx);
        //lock_proxy::initTreasury<BFC_USDT>(admin, ctx);
        let treasury = lock_proxy::initTreasury<T>(ctx);


        lock_proxy::deposit<T>(&mut treasury, initial_lock);

        lock_proxy::lock_proxy_transfer(treasury, admin);
        transfer::public_transfer(cap, tx_context::sender(ctx));

    }


}