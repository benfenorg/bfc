module polynet::bfc_eth {
    use std::option;
    //use std::string::{String};

    use sui::coin;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use polynet::lock_proxy;

    const ENOT_BRIDGE_ADMIN: u64 = 4001;

    const HUGE_U64: u64 = 10000000000000000000;

    struct BFC_ETH has drop {}

    const DECIMALS: u8 = 8;

    fun init(witness: BFC_ETH, ctx: &mut TxContext){

        build_eth(witness, DECIMALS, tx_context::sender(ctx), ctx);
    }

    public fun build_eth<T:drop>(witness: T, decimals: u8, admin: address, ctx: &mut TxContext){
        let (cap, metadata) = coin::create_currency(
            witness,
            decimals,
            b"BFC_ETH",
            b"Benfen eth",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock = coin::mint<T>(&mut cap, HUGE_U64, ctx);
        //lock_proxy::initTreasury<BFC_USDT>(admin, ctx);
        let treasury = lock_proxy::initTreasury<T>(admin, ctx);


        lock_proxy::deposit<T>(&mut treasury, initial_lock);

        lock_proxy::lock_proxy_transfer(treasury, admin);
        transfer::public_transfer(cap, tx_context::sender(ctx));

    }


}