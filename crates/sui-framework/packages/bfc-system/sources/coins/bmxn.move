module bfc_system::bmxn {
    use sui::coin;
    use sui::balance::Supply;

    public struct BMXN has drop {}

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    //spec module { pragma verify = false; }


    #[allow(unused_function)]
    public fun new(ctx: &mut TxContext): Supply<BMXN> {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        assert!(tx_context::epoch(ctx) == 0, EAlreadyMinted);
        let (cap, metadata) = coin::create_currency(
            BMXN {},
            9,
            b"BMXN",
            b"Benfen MXN",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }

    public entry fun transfer(c: coin::Coin<BMXN>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }

    #[test_only]
    public fun new_for_test(ctx: &mut TxContext): Supply<BMXN> {
        let (cap, metadata) = coin::create_currency(
            BMXN {},
            9,
            b"BMXN",
            b"Benfen MXN",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }
}
