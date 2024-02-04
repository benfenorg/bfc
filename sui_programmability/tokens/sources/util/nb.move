module tokens::nb {
    use std::option;
    use tokens::utils::get_poly_address;
    //use std::string;

    use sui::coin::{Self, Coin, TreasuryCap};
    use sui::transfer;
    //use sui::transfer::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    const ENOT_ADMIN: u64 = 4001;

    struct NBCOIN has drop {}



    #[allow(unused_function)]
    public fun new(ctx: &mut TxContext) {
        //todo: check sender....
        //only_admin(admin);
        let (cap, metadata) = coin::create_currency(
            NBCOIN {},
            6,
            b"NBCOIN",
            b"NBCOIN USD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)
        transfer::public_transfer(cap, tx_context::sender(ctx))

    }





    public entry fun mint(treasury_cap: &mut TreasuryCap<NBCOIN>,
                          amount: u64,
                          dst_addr: address,
                          ctx: &mut TxContext) {
        let coin = coin::mint<NBCOIN>(treasury_cap,amount, ctx);
        transfer::public_transfer(coin, dst_addr);
    }



    public entry fun burn(
        treasury_cap: &mut TreasuryCap<NBCOIN>,
        amount: Coin<NBCOIN>,
    )  {
        //only_admin(admin);

        //let admin_addr = (admin);
        coin::burn(treasury_cap, amount);
    }

    public entry fun freeze_coin_store(
        admin: address,
        _freeze_addr: address,
    )  {
        only_admin(admin);

        //let freeze_cap = &borrow_global<NBCapStore>((admin)).freeze_cap;
        //coin::freeze_coin_store<NBCoin>(freeze_addr, freeze_cap);
    }

    public entry fun unfreeze_coin_store(
        admin: address,
        _unfreeze_addr: address,
    )  {
        only_admin(admin);

        //let freeze_cap = &borrow_global<NBCapStore>((admin)).freeze_cap;
        //coin::unfreeze_coin_store<NBCoin>(unfreeze_addr, freeze_cap);
    }

    fun only_admin(account: address) {
        assert!((account) == get_poly_address(), ENOT_ADMIN);
    }
}