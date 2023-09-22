module obc_system::obc_dao_manager {
    use sui::object::UID;
    use sui::tx_context::TxContext;
    use sui::object;
    use sui::transfer;

    friend obc_system::obc_dao;
    friend obc_system::voting_pool;


    const FREE_KEY : u64 = 0;
    const STAKE_KEY: u64 = 1;
    struct OBCDaoManageKey has key, store {
        id: UID,
        key_type: u64,
    }

    spec module{
        pragma verify;
    }
    /// Create a new key.
    public(friend) fun new(sender: address, ctx: &mut TxContext)  {
        let key = OBCDaoManageKey {
            id: object::new(ctx),
            key_type: FREE_KEY,
        };
        transfer::transfer(key, sender);
    }

    //create stake key
    public(friend) fun create_stake_key(sender: address, ctx: &mut TxContext)  {
        let key = OBCDaoManageKey {
            id: object::new(ctx),
            key_type: STAKE_KEY,
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