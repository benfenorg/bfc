module obc_system::obc_dao_manager {
    use sui::balance;
    use sui::balance::Balance;
    use sui::coin;
    use sui::obc::OBC;
    use sui::object::UID;
    use sui::tx_context::TxContext;
    use sui::object;
    use sui::transfer;
    use sui::tx_context;

    friend obc_system::obc_dao;
    friend obc_system::voting_pool;


    const FREE_KEY : u64 = 0;
    const STAKE_KEY: u64 = 1;

    const ERROR_KEY_TYPE: u64 = 1400;
    const ERROR_KEY_NOT_MATCH: u64 = 1401;


    struct OBCDaoManageKey has key, store {
        id: UID,
        key_type: u64,
        amount: u64,
    }

    spec module{
        pragma verify;
    }
    /// Create a new key.
    public(friend) fun new(sender: address, ctx: &mut TxContext)  {
        let key = OBCDaoManageKey {
            id: object::new(ctx),
            key_type: FREE_KEY,
            amount: 0,
        };
        transfer::transfer(key, sender);
    }


    struct ManagerKeyObc has key, store {
        id: UID,
        principal: Balance<OBC>,
    }

    //create stake key
    public(friend) fun create_stake_key(sender: address,
                                        payment: Balance<OBC>,
                                        ctx: &mut TxContext)  {
        let key = OBCDaoManageKey {
            id: object::new(ctx),
            key_type: STAKE_KEY,
            amount: balance::value(&payment)
        };

        let managerObc = ManagerKeyObc {
            id: object::new(ctx),
            principal: payment,
        };


        transfer::transfer(key, sender);
        transfer::transfer(managerObc, sender);

    }

    public (friend) fun unstake_key(key:OBCDaoManageKey, token: ManagerKeyObc, ctx: &mut TxContext){

        assert!(key.key_type == STAKE_KEY, ERROR_KEY_TYPE);
        assert!(key.amount == balance::value(&token.principal), ERROR_KEY_NOT_MATCH);

        //convert proposal payment to voting_obc
        let sender = tx_context::sender(ctx);

        let OBCDaoManageKey{id:uid,
                            key_type: _key_type,
                            amount: _amount,}= key;
        object::delete(uid);

        let ManagerKeyObc{id:uid,
            principal:obc}= token;

        object::delete(uid);

        let coin = coin::from_balance(obc, ctx);
        transfer::public_transfer(coin, sender);

    }




    spec new {
        aborts_if false;
        aborts_if ctx.ids_created + 1 > MAX_U64;
    }

    public(friend) fun getKeyAddress(key: &OBCDaoManageKey) : address {
        object::uid_to_address(&key.id)
    }



}