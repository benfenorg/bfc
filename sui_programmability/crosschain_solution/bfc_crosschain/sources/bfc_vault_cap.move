module bfc_crosschain::bfc_vault_cap {
    use sui::object::UID;
    use sui::tx_context::TxContext;
    use sui::object;
    use sui::transfer;

    friend bfc_crosschain::bfc_vault;
    struct Bfc_vault_cap has key, store {
        id: UID,
    }



    /// Create a new key.
    public(friend) fun new(sender: address, ctx: &mut TxContext)  {
        let key = Bfc_vault_cap {
            id: object::new(ctx),

        };
        transfer::transfer(key, sender);
    }

    public(friend) fun getKeyAddress(key: &Bfc_vault_cap) : address {
        object::uid_to_address(&key.id)
    }



}
