module anonymous_vault::testausd;


use sui::anonymous_coin;
//use sui::anonymous_balance::Anonymous_Balance;
use sui::anonymous_coin::TreasuryCap;

public struct TESTAUSD has drop {}

//spec module { pragma verify = false; }

//const TOTAL_SUPPLY_MIST: u64 = 1_000_000_000_000_000_000;

#[allow(unused_function)]
fun init(witness: TESTAUSD, ctx: &mut TxContext) {
    let (treasury, metadata) = anonymous_coin::create_currency(
        witness,
        9,
        b"TESTAUSD",
        b"Benfen Anonymous TESTAUSD",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    transfer::public_transfer(treasury, tx_context::sender(ctx))
}


public entry fun mint(treasury_cap: &mut TreasuryCap<TESTAUSD>,  amount: u64,   ctx: &mut TxContext) {
    let coin = anonymous_coin::mint<TESTAUSD>(treasury_cap, amount, ctx);
    transfer::public_transfer(coin, tx_context::sender(ctx));
}

public entry fun transfer(c: anonymous_coin::Anonymous_Coin<TESTAUSD>, recipient: address) {
    transfer::public_transfer(c, recipient)
}

#[test_only]
public fun new_for_testing(ctx: &mut TxContext): TreasuryCap<TESTAUSD>{
    let (treasury, metadata) = anonymous_coin::create_currency(
    TESTAUSD {},
        9,
        b"TESTAUSD",
        b"TESTAUSD Anonymous USD",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    treasury
}

