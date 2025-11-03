module test_anonymous_coin::testabfc {
    use sui::anonymous_balance::Anonymous_Balance;
    use sui::anonymous_coin;
    use sui::anonymous_coin::{Anonymous_Coin, TreasuryCap};

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    /// The amount of Mist per Sui token based on the the fact that mist is
    /// 10^-9 of a Sui token
    //const MIST_PER_SUI: u64 = 1_000_000_000;
    #[allow(unused_const)]
    /// The amount of Mist per Sui token based on the fact that mist is
    /// 10^-9 of a Sui token
    const MIST_PER_SUI: u64 = 1_000_000_000;

    /// The total supply of Sui denominated in whole Sui tokens (10 Billion)
    //const TOTAL_SUPPLY_SUI: u64 = 1_000_000_000;

    const TOTAL_SUPPLY_MIST: u64 = 1_0000_0000__000_000_000;

    /// Name of the coin
    public struct TESTABFC has drop {}


    fun init(witness: TESTABFC, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = anonymous_coin::create_currency<TESTABFC>(
            witness,
            8, // decimals
            b"TESTABFC",
            b"TESTABFC",
            b"TESTABFC for testing",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }

    public fun mint(
        treasury_cap: &mut TreasuryCap<TESTABFC>,
        amount: u64,
        recipient: address,
        ctx: &mut TxContext
    ) {
        anonymous_coin::mint_and_transfer(treasury_cap, amount, recipient, ctx);
    }

//    public fun burn(treasury_cap: &mut TreasuryCap<TESTABFC>, coin: Anonymous_Coin<TESTABFC>, signatures: vector<u8>, anonymous_coin_id: address, publickey: vector<u8>) {
//        anonymous_coin::burn(treasury_cap, coin, signatures, anonymous_coin_id, publickey);
//    }
}
