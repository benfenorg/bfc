module ausd::ausd;


use sui::anonymous_coin;
//use sui::anonymous_balance::Anonymous_Balance;
use sui::anonymous_coin::TreasuryCap;

public struct AUSD has drop {}

//spec module { pragma verify = false; }

//const TOTAL_SUPPLY_MIST: u64 = 1_000_000_000_000_000_000;

#[allow(unused_function)]
fun init(witness: AUSD, ctx: &mut TxContext) {
    let (treasury, metadata) = anonymous_coin::create_currency(
        witness,
        9,
        b"AUSD",
        b"Benfen Anonymous USD",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    transfer::public_transfer(treasury, tx_context::sender(ctx))
}


public entry fun mint(treasury_cap: &mut TreasuryCap<AUSD>,  amount: u64,   ctx: &mut TxContext) {
    let coin = anonymous_coin::mint<AUSD>(treasury_cap, amount, ctx);
    transfer::public_transfer(coin, tx_context::sender(ctx));
}

public entry fun transfer(c: anonymous_coin::Anonymous_Coin<AUSD>, recipient: address) {
    transfer::public_transfer(c, recipient)
}

#[test_only]
public fun new_for_testing(ctx: &mut TxContext): TreasuryCap<AUSD>{
    let (treasury, metadata) = anonymous_coin::create_currency(
        AUSD {},
        9,
        b"AUSD",
        b"AUSD Anonymous USD",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    treasury
}

