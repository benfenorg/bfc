module anonymous_vault::testausc;


use sui::anonymous_coin;
//use sui::anonymous_balance::Anonymous_Balance;
use sui::anonymous_coin::TreasuryCap;

public struct TESTAUSC has drop {}

//spec module { pragma verify = false; }

//const TOTAL_SUPPLY_MIST: u64 = 1_000_000_000_000_000_000;

#[allow(unused_function)]
fun init(witness: TESTAUSC, ctx: &mut TxContext) {
    let (treasury, metadata) = anonymous_coin::create_currency(
        witness,
        9,
        b"TESTAUSC",
        b"Benfen Anonymous TESTAUSC",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    transfer::public_transfer(treasury, tx_context::sender(ctx))
}


public entry fun mint(treasury_cap: &mut TreasuryCap<TESTAUSC>,  amount: u64,   ctx: &mut TxContext) {
    let coin = anonymous_coin::mint<TESTAUSC>(treasury_cap, amount, ctx);
    transfer::public_transfer(coin, tx_context::sender(ctx));
}

public entry fun transfer(c: anonymous_coin::Anonymous_Coin<TESTAUSC>, recipient: address) {
    transfer::public_transfer(c, recipient)
}

#[test_only]
public fun new_for_testing(ctx: &mut TxContext): TreasuryCap<TESTAUSC>{
    let (treasury, metadata) = anonymous_coin::create_currency(
TESTAUSC {},
        9,
        b"TESTAUSC",
        b"TESTAUSC Anonymous USDC",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    treasury
}

