module bfc_system::bfc_dao_manager {
    use sui::balance;
    use sui::balance::Balance;
    use sui::coin;
    use sui::bfc::BFC;
    use sui::object::UID;
    use sui::tx_context::TxContext;
    use sui::object;
    use sui::transfer;
    use sui::tx_context;

    friend bfc_system::bfc_dao;
    friend bfc_system::voting_pool;


    const FREE_KEY : u64 = 0;
    const STAKE_KEY: u64 = 1;

    const ERROR_KEY_TYPE: u64 = 1400;
    const ERROR_KEY_NOT_MATCH: u64 = 1401;


    struct BFCDaoManageKey has key, store {
        id: UID,
        key_type: u64,
        amount: u64,
    }

    spec module{
        pragma verify;
    }
    /// Create a new key.
    public(package) fun new(sender: address, ctx: &mut TxContext)  {
        let key = BFCDaoManageKey {
            id: object::new(ctx),
            key_type: FREE_KEY,
            amount: 0,
        };
        transfer::transfer(key, sender);
    }


    struct ManagerKeyBfc has key, store {
        id: UID,
        principal: Balance<BFC>,
    }

    //create stake key
    public(package) fun create_stake_key(sender: address,
                                        payment: Balance<BFC>,
                                        ctx: &mut TxContext)  {
        let key = BFCDaoManageKey {
            id: object::new(ctx),
            key_type: STAKE_KEY,
            amount: balance::value(&payment)
        };

        let managerBfc = ManagerKeyBfc {
            id: object::new(ctx),
            principal: payment,
        };


        transfer::transfer(key, sender);
        transfer::transfer(managerBfc, sender);

    }

    public (package) fun unstake_key(key:BFCDaoManageKey, token: ManagerKeyBfc, ctx: &mut TxContext){

        assert!(key.key_type == STAKE_KEY, ERROR_KEY_TYPE);
        assert!(key.amount == balance::value(&token.principal), ERROR_KEY_NOT_MATCH);

        //convert proposal payment to voting_bfc
        let sender = tx_context::sender(ctx);

        let BFCDaoManageKey{id:uid,
                            key_type: _key_type,
                            amount: _amount,}= key;
        object::delete(uid);

        let ManagerKeyBfc{id:uid,
            principal:bfc}= token;

        object::delete(uid);

        let coin = coin::from_balance(bfc, ctx);
        transfer::public_transfer(coin, sender);

    }




    spec new {
        aborts_if false;
        aborts_if ctx.ids_created + 1 > MAX_U64;
    }

    public(package) fun getKeyAddress(key: &BFCDaoManageKey) : address {
        object::uid_to_address(&key.id)
    }



}