
module sui_crosschain::sui_vault {

    use std::string;
    use std::vector;
    use sui_crosschain::sui_vault_cap;
    use sui::object;
    use sui::object::UID;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;



    //3-2 MODEL
    const DEFAULT_WATCH_NODE_COUNT: u32 = 3;
    const DEFAULT_WATCH_NODE_THRESHOLD: u32 = 2;

    //5-3 MODEL
    //const DEFAULT_WATCH_NODE_COUNT: u32= 5;
    //const DEFAULT_WATCH_NODE_THRESHOLD: u32 = 3;



    const DEFAULT_ADMIN_NODES: vector<address> = vector[
        @0x7419050e564485685f306e20060472fca1b3a4453b41bdace0010624801b11ea,
        @0x63db8b52040f30ef0a617f2fa3571ad3406ee19cf4619638bf6ed9ac6a7cd7dc,
        @0x17e80b7c1b29b65688a074834fb6e26e840eba0804a26e6ac831722f4b1638de,
        @0x8c28c69e8d4d280a57def81fc76eb0fd73c00cc511d8f070fbd242b6fa22186c,
        @0x199bd1ef1fdac4bca4a3a60e29c8611630e00b81a864fd3cdfe79140766813b3,
    ];

    struct Sui_vault has key {
        id: UID,
        /// Name for the vault type
        name: string::String,
        token_chain: string::String,
        token_address: string::String,

        //
        balance: u64,

        //add map for operation.
    }


    // Part 3: transfer the BFC Dao object to the sender
    entry public fun create_bfc_vault(ctx: &mut TxContext ) {
        let sender = tx_context::sender(ctx);

        let vault = Sui_vault {
            id: object::new(ctx),
            name: string::utf8(b"sui_vault"),
            token_chain: string::utf8(b"sui"),
            token_address: string::utf8(b"0x0001"),
            balance: 0,
            //TODO
        };

        create_cap_for_watch_node(ctx);


        transfer::transfer(vault, sender);
    }


    public fun create_cap_for_watch_node( ctx: &mut TxContext){

        let count = vector::length(&DEFAULT_ADMIN_NODES);
        let i = 0;
        while (i < count) {
            let admin = vector::borrow(&DEFAULT_ADMIN_NODES, i);
            sui_vault_cap::new(*admin, ctx);
            i = i+1;
        };
    }
}