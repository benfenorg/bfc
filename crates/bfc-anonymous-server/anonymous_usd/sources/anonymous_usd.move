module anonymous_usd::anonymous_usd;


use sui::coin;
use sui::balance::Supply;

public struct ABUSD has drop {}

//spec module { pragma verify = false; }


#[allow(unused_function)]
public fun new(ctx: &mut TxContext): Supply<ABUSD> {
    let (cap, metadata) = coin::create_currency(
        ABUSD {},
        9,
        b"ABUSD",
        b"Benfen Anonymous USD",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    coin::treasury_into_supply(cap)
}

public entry fun transfer(c: coin::Coin<ABUSD>, recipient: address) {
    transfer::public_transfer(c, recipient)
}

#[test_only]
public fun new_for_test(ctx: &mut TxContext): Supply<ABUSD> {
    let (cap, metadata) = coin::create_currency(
        ABUSD {},
        9,
        b"ABUSD",
        b"Benfen Anonymous USD",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    coin::treasury_into_supply(cap)
}

