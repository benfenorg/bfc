module polynet::bfc_usdc {
    use std::option;
    use polynet::acl::{ Self};
    use sui::coin::{TreasuryCap, Self, Coin};
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;
    use polynet::lock_proxy;
    use polynet::consts;

    friend polynet::controller;


    // Errors
    const EINVALID_ADMIN: u64 = 2001;

    const HUGE_U64: u64 = 10000000000000000000;

    struct BFC_USDC has drop {}


    fun init(witness: BFC_USDC, ctx: &mut TxContext){

        build_usdc(witness, consts::get_decimal(), tx_context::sender(ctx), ctx);
    }

    public fun build_usdc<T:drop>(witness: T, decimals: u8, admin: address, ctx: &mut TxContext){
        assert!(acl::is_admin(admin), EINVALID_ADMIN);

        let (cap, metadata) = coin::create_currency(
            witness,
            decimals,
            b"BFC_USDC",
            b"Benfen USDC",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock = coin::mint<T>(&mut cap, HUGE_U64, ctx);
        //lock_proxy::initTreasury<BFC_USDT>(admin, ctx);
        let treasury = lock_proxy::init_treasury<T>(ctx);


        lock_proxy::deposit<T>(&mut treasury, initial_lock);

        lock_proxy::lock_proxy_transfer(treasury);
        transfer::public_transfer(cap, tx_context::sender(ctx));

    }

    public(friend) fun mint_treasury<T>(
       _cap: &mut TreasuryCap<T>,
       _amount: u64,
       _ctx: &mut TxContext
    ):  Coin<T>{
      coin::mint<T>(_cap, _amount, _ctx)
    }


}