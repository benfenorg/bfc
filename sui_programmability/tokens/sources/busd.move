#[allow(unused_use)]
module tokens::busd {
    use std::option;
    use sui::transfer;
    use sui::coin;
    use sui::balance::Supply;
    use sui::coin::TreasuryCap;
    use sui::tx_context;
    use sui::tx_context::{TxContext};

    struct COIN_TESTS has drop {}

    const EAlreadyMinted: u64 = 0;
    const ENotSystemAddress: u64 = 1;

    spec module { pragma verify = false; }


    #[allow(unused_function)]
    public fun new(ctx: &mut TxContext) {
        let (cap, metadata) = coin::create_currency(
            COIN_TESTS {},
            9,
            b"KKUSD",
            b"Benfen USD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        //coin::treasury_into_supply(cap)
        transfer::public_transfer(cap, tx_context::sender(ctx))

    }




    public entry fun mint(treasury_cap: &mut TreasuryCap<COIN_TESTS>, amount: u64, ctx: &mut TxContext) {
        let coin = coin::mint<COIN_TESTS>(treasury_cap,amount, ctx);
        transfer::public_transfer(coin, tx_context::sender(ctx));
    }

    public entry fun transfer(c: coin::Coin<COIN_TESTS>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }

    #[test_only]
    public fun new_for_test(ctx: &mut TxContext): Supply<COIN_TESTS> {
        let (cap, metadata) = coin::create_currency(
            COIN_TESTS {},
            9,
            b"COIN_TESTS",
            b"Benfen Test USD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }
}
