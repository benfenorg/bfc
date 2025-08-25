/*
/// Module: normal_usd
module normal_usd::normal_usd;
*/
module normal_usd::normal_usd;


use sui::coin;
use sui::coin::TreasuryCap;
//use sui::balance::Supply;

public struct NORMAL_USD has drop {}

//spec module { pragma verify = false; }


#[allow(unused_function)]
fun init(witness: NORMAL_USD,  ctx: &mut TxContext) {
    let (treasury, metadata) = coin::create_currency(
        witness,
        9,
        b"NORMAL_USD",
        b"Benfen  USD",
        b"",
    option::none(),
    ctx
    );
    transfer::public_freeze_object(metadata);
    transfer::public_transfer(treasury, tx_context::sender(ctx))
}

public entry fun mint(treasury_cap: &mut TreasuryCap<NORMAL_USD>,  amount: u64,   ctx: &mut TxContext) {
    let coin = coin::mint<NORMAL_USD>(treasury_cap, amount, ctx);
    transfer::public_transfer(coin, tx_context::sender(ctx));
}

public entry fun transfer(c: coin::Coin<NORMAL_USD>, recipient: address) {
    transfer::public_transfer(c, recipient)
}

#[test_only]
public fun new_for_test(ctx: &mut TxContext): Supply<NORMAL_USD> {
    let  (cap, metadata) = coin::create_currency(
        NORMAL_USD {},
        9,
        b"ABUSD",
        b"Benfen  USD",
        b"",
        option::none(),
        ctx
        );
    transfer::public_freeze_object(metadata);
    coin::treasury_into_supply(cap)
}

