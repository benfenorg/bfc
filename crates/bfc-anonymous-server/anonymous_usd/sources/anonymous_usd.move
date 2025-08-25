module anonymous_usd::anonymous_usd;


use sui::anonymous_coin;
use sui::anonymous_balance::Anonymous_Balance;

public struct ABUSD has drop {}

//spec module { pragma verify = false; }

const TOTAL_SUPPLY_MIST: u64 = 1_000_000_000_000_000_000;

#[allow(unused_function)]
public fun new(ctx: &mut TxContext): Anonymous_Balance<ABUSD> {
    let (treasury, metadata) = anonymous_coin::create_currency(
        ABUSD {},
        9,
        b"ABUSD",
        b"Benfen Anonymous USD",
        b"",
        option::none(),
        ctx
    );
    transfer::public_freeze_object(metadata);
    //coin::treasury_into_supply(cap)
    let mut supply = treasury.treasury_into_supply();
    let total_anonymous_usd = supply.increase_supply(TOTAL_SUPPLY_MIST);
    supply.destroy_supply();
    total_anonymous_usd
}

public entry fun transfer(c: anonymous_coin::Anonymous_Coin<ABUSD>, recipient: address) {
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

