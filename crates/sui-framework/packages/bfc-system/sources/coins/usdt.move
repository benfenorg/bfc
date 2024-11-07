module bfc_system::usdt {
    use sui::coin;
    use sui::balance::Supply;

    public struct USDT has drop {}

    #[allow(unused_function)]
    public fun new(ctx: &mut TxContext): Supply<USDT> {
        let (cap, metadata) = coin::create_currency(
            USDT {},
            9,
            b"USDT",
            b"Benfen USDT",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }

    public entry fun transfer(c: coin::Coin<USDT>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }

    #[test_only]
    public fun new_for_test(ctx: &mut TxContext): Supply<USDT> {
        let (cap, metadata) = coin::create_currency(
            USDT {},
            9,
            b"USDT",
            b"Benfen USDT",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }
}
