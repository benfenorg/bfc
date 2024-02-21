module polynet::bfc_eth {
    use std::option;
    use polynet::utils;
    //use std::string::{String};

    use sui::coin;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use polynet::lock_proxy;
    
    // Errors
    const EINVALID_ADMIN: u64 = 4001;


    const HUGE_U64: u64 = 10000000000000000000;

    struct BFC_ETH has drop {}

    const DECIMALS: u8 = 8;

    fun init(witness: BFC_ETH, ctx: &mut TxContext){

        build_eth(witness, DECIMALS, tx_context::sender(ctx), ctx);
    }

    public fun build_eth<T:drop>(witness: T, decimals: u8, admin: address, ctx: &mut TxContext){
        assert!(utils::is_admin(admin), EINVALID_ADMIN);

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
        let treasury = lock_proxy::initTreasury<T>(ctx);


        lock_proxy::deposit<T>(&mut treasury, initial_lock);

        lock_proxy::lock_proxy_transfer(treasury, admin);
        transfer::public_transfer(cap, tx_context::sender(ctx));

    }
    #[test_only]
    public fun new_for_test(ctx: &mut TxContext, owner: address) {

        let (cap, metadata) = coin::create_currency(
            BFC_ETH {},
            6,
            b"BFC_ETH",
            b"Benfen eth",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)

        let initial_lock = coin::mint<BFC_ETH>(&mut cap, HUGE_U64, ctx);
        let remain = coin::split<BFC_ETH>(&mut initial_lock, 100000, ctx);
        //lock_proxy::initTreasury<BFC_USDT>(admin, ctx);
        let treasury = lock_proxy::initTreasury<BFC_ETH>(ctx);

        lock_proxy::deposit<BFC_ETH>(&mut treasury, initial_lock);
        lock_proxy::lock_proxy_transfer(treasury, owner);

        transfer::public_transfer(cap, tx_context::sender(ctx));
        transfer::public_transfer(remain, owner);
    }

}