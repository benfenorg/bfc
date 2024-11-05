module bfc_system::usdc {
    use sui::coin;
    use sui::balance::Supply;

    public struct USDC has drop {}

    #[allow(unused_function)]
    public fun new(ctx: &mut TxContext): Supply<USDC> {
        let (cap, metadata) = coin::create_currency(
            USDC {},
            9,
            b"USDC",
            b"Benfen USDC",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }

    public entry fun transfer(c: coin::Coin<USDC>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }

    #[test_only]
    public fun new_for_test(ctx: &mut TxContext): Supply<USDC> {
        let (cap, metadata) = coin::create_currency(
            USDC {},
            9,
            b"USDC",
            b"Benfen USDC",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }
}
