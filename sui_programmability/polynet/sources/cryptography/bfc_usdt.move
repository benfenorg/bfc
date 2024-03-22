module polynet::bfc_usdt {
    use std::option;
    use polynet::acl::{ Self};
    use polynet::consts;
    use sui::coin;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;
    use polynet::lock_proxy;

    // Errors
    const EINVALID_ADMIN: u64 = 2002;

    struct BFC_USDT has drop {}

    fun init(witness: BFC_USDT, ctx: &mut TxContext){

        build_usdt(witness, consts::get_decimal(), tx_context::sender(ctx), ctx);
    }

    public fun build_usdt<T:drop>(witness: T, decimals: u8, admin: address, ctx: &mut TxContext){
        assert!(acl::is_admin(admin), EINVALID_ADMIN);

        let (cap, metadata) = coin::create_currency(
            witness,
            decimals,
            b"BFC_USDT",
            b"Benfen USDT",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock = coin::mint<T>(&mut cap, consts::get_huge(), ctx);

        let remain = coin::split<T>(&mut initial_lock, consts::get_local_amount(), ctx);
        let treasury = lock_proxy::init_treasury<T>(ctx);

        lock_proxy::deposit<T>(&mut treasury, initial_lock);
        lock_proxy::lock_proxy_transfer(treasury);

        transfer::public_transfer(cap, tx_context::sender(ctx));
        transfer::public_transfer(remain, admin);

    }

      #[test_only]
    public fun new_test(ctx: &mut TxContext, owner: address) {

        let (cap, metadata) = coin::create_currency(
            BFC_USDT {},
            consts::get_decimal(),
            b"BFC_USDT",
            b"Benfen usdt",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock = coin::mint<BFC_USDT>(&mut cap, consts::get_huge(), ctx);
        let remain = coin::split<BFC_USDT>(&mut initial_lock, 100000, ctx);
        //lock_proxy::initTreasury<BFC_USDT>(admin, ctx);
        let treasury = lock_proxy::init_treasury<BFC_USDT>(ctx);

        lock_proxy::deposit<BFC_USDT>(&mut treasury, initial_lock);
        lock_proxy::lock_proxy_transfer(treasury);

        transfer::public_transfer(cap, tx_context::sender(ctx));
        transfer::public_transfer(remain, owner);
    }


}