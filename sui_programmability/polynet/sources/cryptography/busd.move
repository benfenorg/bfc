module bfc_crosschain::busd {
    use std::option;
    use sui::transfer;
    use sui::coin;
    use sui::balance::Supply;
    use sui::tx_context::{TxContext};

    struct BUSD_Test has drop {}

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    spec module { pragma verify = false; }


    #[allow(unused_function)]
    public fun new(ctx: &mut TxContext): Supply<BUSD_Test> {
        let (cap, metadata) = coin::create_currency(
            BUSD_Test {},
            9,
            b"Test_BUSD",
            b"Benfen Test USD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }

    public entry fun transfer(c: coin::Coin<BUSD_Test>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }

    #[test_only]
    public fun new_for_test(ctx: &mut TxContext): Supply<BUSD_Test> {
        let (cap, metadata) = coin::create_currency(
            BUSD_Test {},
            9,
            b"Test_BUSD",
            b"Benfen Test USD",
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        coin::treasury_into_supply(cap)
    }
}
