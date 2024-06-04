/// Example coin with a trusted owner responsible for minting/burning (e.g., a stablecoin)
module tokens::peace {
    use sui::coin::{Self, TreasuryCap};


    /// This type is intended to be used only once.
    public struct PEACE has drop {}

    /// Register the trusted currency to acquire its `TreasuryCap`. Because
    /// this is a module initializer, it ensures the currency only gets
    /// registered once.
    fun init(witness: PEACE, ctx: &mut TxContext) {
        // Get a treasury cap for the coin and give it to the transaction
        // sender
        let (treasury_cap, metadata) = coin::create_currency<PEACE>(
            witness,
            2,
            b"PEACE",
            b"",
            b"", option::none(), ctx);
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx))
    }

    public entry fun mint(treasury_cap: &mut TreasuryCap<PEACE>, amount: u64, ctx: &mut TxContext) {
        let coin = coin::mint<PEACE>(treasury_cap,amount,  ctx);
        transfer::public_transfer(coin, tx_context::sender(ctx));
    }


    public entry fun add_update(){

    }

    #[test_only]
    /// Wrapper of module initializer for testing
    public fun test_init(ctx: &mut TxContext) {
        init(PEACE{},ctx)
    }
}
