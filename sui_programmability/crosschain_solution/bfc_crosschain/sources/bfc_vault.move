module bfc_crosschain::bfc_vault{

    use std::string;
    use std::vector;
    use sui::balance;
    use sui::balance::{Balance};
    use sui::bfc::BFC;
    use bfc_crosschain::bfc_valut_eventcenter;
    use bfc_crosschain::bfc_vault_cap::Bfc_vault_cap;
    use sui::clock;
    use sui::clock::Clock;
    use sui::coin;
    use sui::coin::{Coin};
    use bfc_crosschain::bfc_vault_cap;
    use sui::object;
    use sui::object::UID;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;
    use sui::vec_map;
    use sui::vec_map::VecMap;



    const MAX_OPERATION_DELAY :u64 = 60 * 60 * 24  * 1000; //1 days

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
        @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590,
    ];



    const OPERATION_STATUS_INIT: u32 = 0;
    const OPERATION_STATUS_SUCCESS: u32 = 200;
    const OPERATION_STATUS_FAILED: u32 = 404;

    //event content
    const EVENT_OPERATION_IS_ALREADY_SUCCESS: vector<u8> = b"operation is already success";
    const EVENT_OPERATION_IS_OUT_OF_TIME: vector<u8> = b"operation is out of time";
    const EVENT_CAN_NOT_VOTE_TWICE: vector<u8> = b"can not vote twice";

    //error code
    const ERR_MSG_CAN_NOT_VOTE_TWICE: u64 = 40001;
    const ERR_MSG_OPERATION_IS_OUT_OF_TIME: u64 = 40002;
    const ERR_MSG_OPERATION_IS_ALREADY_SUCCESS: u64 = 40003;



    struct Bfc_vault has key {
        id: UID,
        /// Name for the vault type
        name: string::String,
        token_chain: string::String,
        token_address: string::String,

        //
        total_balance: u64,
        token: Balance<BFC>,

        //add map for operation.
    }

    struct Operation has key {
        id: UID,
        // operation of users, need watch nodes to confirm
        //
        operation_type: u32,
        balance: u64,
        create_time: u64,
        node_signed_weight: u32,
        status: u32,

        //record the voting nodes.
        voting_nodes: VecMap<address, u32>,
    }

    entry public fun create_bfc_vault(ctx: &mut TxContext ) {
        //let sender = tx_context::sender(ctx);

        let vault = Bfc_vault {
            id: object::new(ctx),
            name: string::utf8(b"bfc_vault"),
            token_chain: string::utf8(b"bfc"),
            token_address: string::utf8(b"0x0001"),
            total_balance: 0,
            token: balance::zero<BFC>(),
            //TODO
        };

        create_cap_for_watch_node(ctx);


        //todo, change to share model.
        //transfer::transfer(vault, sender);
        transfer::share_object(vault);

    }


    entry public fun create_operation(optype: u32,
                                      clock: & Clock,
                                      ctx: &mut TxContext) {
        //let sender = tx_context::sender(ctx);


        let current_time =  clock::timestamp_ms(clock) ;
        let operation = Operation {
            id: object::new(ctx),
            operation_type: optype,
            create_time: current_time,

            balance: 0,
            node_signed_weight: 0,
            status: OPERATION_STATUS_INIT,
            voting_nodes: vec_map::empty(),
        };

        transfer::share_object(operation);

    }

    entry public fun vote_operation(operation: &mut Operation,
                                    vault: &mut Bfc_vault,
                                    _node_cap:  &mut Bfc_vault_cap,
                                    amount: u64,
                                    clock: & Clock,
                                    ctx: &mut TxContext,) {
        let sender = tx_context::sender(ctx);

        let current_time =  clock::timestamp_ms(clock) ;

        //time check
        if (current_time - operation.create_time > MAX_OPERATION_DELAY){
            bfc_valut_eventcenter::sendEvent(EVENT_OPERATION_IS_OUT_OF_TIME);
            assert!(false, ERR_MSG_OPERATION_IS_OUT_OF_TIME);
        };

        //status check
        if(operation.status == OPERATION_STATUS_SUCCESS){
            //send event.
            bfc_valut_eventcenter::sendEvent(EVENT_OPERATION_IS_ALREADY_SUCCESS);
            assert!(false, ERR_MSG_OPERATION_IS_ALREADY_SUCCESS);
        };

        //voting history check
        if (vec_map::contains(&operation.voting_nodes, &bfc_vault_cap::getKeyAddress(_node_cap))) {
            bfc_valut_eventcenter::sendEvent(EVENT_CAN_NOT_VOTE_TWICE);
            assert!(false, ERR_MSG_CAN_NOT_VOTE_TWICE);
        };


        vec_map::insert(&mut operation.voting_nodes, bfc_vault_cap::getKeyAddress(_node_cap), 1);


        operation.node_signed_weight = operation.node_signed_weight + 1;

        if(operation.status < OPERATION_STATUS_SUCCESS && operation.node_signed_weight >= DEFAULT_WATCH_NODE_THRESHOLD){
            operation.status = OPERATION_STATUS_SUCCESS;
            //local native release token transfer...
            //transfer.
            let userBalance =  balance::split(&mut vault.token, amount);

            transfer::public_transfer(coin::from_balance(userBalance, ctx), sender)
        }



    }



    public fun create_cap_for_watch_node( ctx: &mut TxContext){

        let count = vector::length(&DEFAULT_ADMIN_NODES);
        let i = 0;
        while (i < count) {
            let admin = vector::borrow(&DEFAULT_ADMIN_NODES, i);
            bfc_vault_cap::new(*admin, ctx);
            i = i+1;
        };
    }

    entry public fun deposit( vault: &mut Bfc_vault,input: Coin<BFC>) {
        let inputBalance = coin::into_balance(input);
        //let sender = tx_context::sender(ctx);
        balance::join(&mut vault.token, inputBalance);
        bfc_valut_eventcenter::sendEvent(b"deposit");
    }


    entry public fun withdraw( vault: &mut Bfc_vault, amount: u64, ctx: &mut TxContext){
        let sender = tx_context::sender(ctx);
        let userBalance =  balance::split(&mut vault.token, amount);

        transfer::public_transfer(coin::from_balance(userBalance, ctx), sender)
    }

}