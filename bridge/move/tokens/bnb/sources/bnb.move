module bridged_bnb::bnb {
    use std::option;

    use sui::coin;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    struct BNB has drop {}

    const DECIMAL: u8 = 8;

    fun init(otw: BNB, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            otw,
            DECIMAL,
            b"BNB",
            b"BSC",
            b"Bridged BSC token",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx))
    }
}
