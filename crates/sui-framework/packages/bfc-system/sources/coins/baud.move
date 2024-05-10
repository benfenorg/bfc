module bfc_system::baud {
    use std::option;
    use sui::transfer;
    use sui::coin;
    use sui::balance::Supply;
    use sui::tx_context::{Self, TxContext};

    struct BAUD has drop {}

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    spec module { pragma verify = false; }


    #[allow(unused_function)]
    public fun new(ctx: &mut TxContext): Supply<BAUD> {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        assert!(tx_context::epoch(ctx) == 0, EAlreadyMinted);
        let (cap, metadata) = coin::create_currency(
            BAUD {},
            9,
            b"BAUD",
            b"Benfen AUD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }

    public entry fun transfer(c: coin::Coin<BAUD>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }

    #[test_only]
    public fun new_for_test(ctx: &mut TxContext): Supply<BAUD> {
        let (cap, metadata) = coin::create_currency(
            BAUD {},
            9,
            b"BAUD",
            b"Benfen AUD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }
}
