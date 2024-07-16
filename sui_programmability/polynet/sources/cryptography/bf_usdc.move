module polynet::bf_usdc {
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

    struct BF_USDC has drop {}

    fun init(witness: BF_USDC, ctx: &mut TxContext){

        build_usdc(witness, consts::get_decimal(), tx_context::sender(ctx), ctx);
    }

    //todo: change token name and symbol
    //todo: treasure admin address use config address
    public fun build_usdc<T:drop>(witness: T, decimals: u8, admin: address, ctx: &mut TxContext){
        assert!(acl::is_admin(admin), EINVALID_ADMIN);

        let (cap, metadata) = coin::create_currency(
            witness,
            decimals,
            b"BF_USDC",
            b"Benfen USDC",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock_token = coin::mint<T>(&mut cap, consts::get_huge(), ctx);
     
        let remain = coin::split<T>(&mut initial_lock_token, consts::get_local_amount(), ctx);
        let treasury = lock_proxy::init_treasury<T>(ctx);

        lock_proxy::deposit<T>(&mut treasury, initial_lock_token);
        lock_proxy::lock_proxy_transfer(treasury);

        transfer::public_transfer(cap, @treasure_admin);
        transfer::public_transfer(remain, @treasure_admin);

    }

    public(friend) fun mint_treasury<T>(
       _cap: &mut TreasuryCap<T>,
       _amount: u64,
       _ctx: &mut TxContext
    ):  Coin<T>{
      coin::mint<T>(_cap, _amount, _ctx)
    }

    #[test_only]
    public fun new_for_test(ctx: &mut TxContext, owner: address) {

        let (cap, metadata) = coin::create_currency(
            BF_USDC {},
            consts::get_decimal(),
            b"BF_USDC",
            b"Benfen usdc",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock = coin::mint<BF_USDC>(&mut cap, consts::get_huge(), ctx);
        let remain = coin::split<BF_USDC>(&mut initial_lock, 100000, ctx);
        //lock_proxy::initTreasury<BFC_USDT>(admin, ctx);
        let treasury = lock_proxy::init_treasury<BF_USDC>(ctx);

        lock_proxy::deposit<BF_USDC>(&mut treasury, initial_lock);
        lock_proxy::lock_proxy_transfer(treasury);

        transfer::public_transfer(cap, @treasure_admin);
        transfer::public_transfer(remain, @treasure_admin);
    }



}