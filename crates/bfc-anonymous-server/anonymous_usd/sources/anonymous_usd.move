module anonymous_usd::anonymous_usd;


use sui::anonymous_coin;
//use sui::anonymous_balance::Anonymous_Balance;
use sui::anonymous_coin::TreasuryCap;

public struct ANONYMOUS_USD has drop {}

//spec module { pragma verify = false; }

//const TOTAL_SUPPLY_MIST: u64 = 1_000_000_000_000_000_000;

#[allow(unused_function)]
fun init(witness: ANONYMOUS_USD, ctx: &mut TxContext) {
    let (treasury, metadata) = anonymous_coin::create_currency(
        witness,
        9,
        b"ANONYMOUS_USD",
        b"Benfen Anonymous USD",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    transfer::public_transfer(treasury, tx_context::sender(ctx))
}


public entry fun mint(treasury_cap: &mut TreasuryCap<ANONYMOUS_USD>,  amount: u64,   ctx: &mut TxContext) {
    let coin = anonymous_coin::mint<ANONYMOUS_USD>(treasury_cap, amount, ctx);
    transfer::public_transfer(coin, tx_context::sender(ctx));
}

public entry fun transfer(c: anonymous_coin::Anonymous_Coin<ANONYMOUS_USD>, recipient: address) {
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

