module poly_bridge::bridge_coin {
    use std::option;
    //use std::string::{String};

    use sui::coin;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use poly_bridge::lock_proxy;

    const ENOT_BRIDGE_ADMIN: u64 = 4001;

    const HUGE_U64: u64 = 10000000000000000000;

    struct BFC_ETH has drop {}
    struct BFC_BTC has drop {}
    struct BFC_USDT has drop {}
    struct BFC_USDC has drop {}

    public entry fun initialize(
        admin: address,
        decimals: u8,
        ctx: &mut TxContext,
    ) {
        only_admin(admin);

        build_usdt(decimals, admin, ctx);
        build_usdc(decimals, admin, ctx);



    }



    public fun build_usdt(decimals: u8, admin: address, ctx: &mut TxContext){
        let (cap, metadata) = coin::create_currency(
            BFC_USDT{},
            decimals,
            b"BFC_USDT",
            b"Benfen USD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock = coin::mint<BFC_USDT>(&mut cap, HUGE_U64, ctx);
        lock_proxy::initTreasury<BFC_USDT>(admin, ctx);
        lock_proxy::deposit<BFC_USDT>( initial_lock);

        transfer::public_transfer(cap, tx_context::sender(ctx));

    }
    public fun build_usdc(decimals: u8, admin: address, ctx: &mut TxContext){
        let (cap, metadata) = coin::create_currency(
            BFC_USDC{},
            decimals,
            b"BFC_USDC",
            b"Benfen USD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock = coin::mint<BFC_USDC>(&mut cap, HUGE_U64, ctx);
        lock_proxy::initTreasury<BFC_USDC>(admin, ctx);
        lock_proxy::deposit<BFC_USDC>(initial_lock);

        transfer::public_transfer(cap, tx_context::sender(ctx));
    }
    public fun build_eth(){

    }
    public fun build_btc(){

    }







    fun only_admin(account: address) {
        assert!(lock_proxy::is_admin(account), ENOT_BRIDGE_ADMIN);
    }
}