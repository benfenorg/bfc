module obc_system::obc_dao_manager {
    use sui::object::UID;
    use sui::tx_context::TxContext;
    use sui::object;
    use sui::transfer;

    friend obc_system::obc_dao;
    friend obc_system::voting_pool;
    struct OBCDaoManageKey has key, store {
        id: UID,
    }

    spec module{
        pragma verify;
    }
    /// Create a new key.
    public(friend) fun new(sender: address, ctx: &mut TxContext)  {
        let key = OBCDaoManageKey {
            id: object::new(ctx),

        };
        transfer::transfer(key, sender);
    }
    spec new {
        aborts_if false;
        aborts_if ctx.ids_created + 1 > MAX_U64;
    }

    public(friend) fun getKeyAddress(key: &OBCDaoManageKey) : address {
        object::uid_to_address(&key.id)
    }
}