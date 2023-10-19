module hello_world::bfc_dao_manager {
    use sui::object::UID;
    use sui::tx_context::TxContext;
    use sui::object;
    use sui::transfer;

    friend hello_world::bfc_dao;
    friend hello_world::voting_pool;
    struct BFCDaoManageKey has key, store {
        id: UID,
    }


    /// Create a new key.
    public(friend) fun new(sender: address, ctx: &mut TxContext)  {
        let key = BFCDaoManageKey {
            id: object::new(ctx),

        };
        transfer::transfer(key, sender);
    }

    public(friend) fun getKeyAddress(key: &BFCDaoManageKey) : address {
        object::uid_to_address(&key.id)
    }
}
