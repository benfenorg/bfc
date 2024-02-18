module polynet::wrapper_v1 {
    use sui::bfc::BFC;
    use sui::event;
    use std::type_name::{Self, TypeName};
    use polynet::utils;
    use polynet::lock_proxy::{Treasury, LockProxyManager};
    use polynet::cross_chain_manager::{CrossChainManager};
    use sui::coin::{Coin, Self};
    use sui::object;
    use sui::object::UID;
    use sui::transfer;
    use sui::transfer::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use polynet::lock_proxy;

    const DEPRECATED: u64 = 4001;
    const EINVALID_ADMIN: u64 = 4015;



    struct WrapperStore has key, store{
        id: UID,
        fee_collector: address,
    }

    struct LockWithFeeEvent has store, drop, copy{
        from_asset: TypeName,
        from_address: address,
        to_chain_id: u64,
        to_address: vector<u8>,
        amount: u64,
        fee_amount: u64
    }

    // for admin
    public entry fun init_wrapper( ctx: &mut TxContext) {

        // sender address
        let sender = tx_context::sender(ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN);

        transfer(WrapperStore{
            id: object::new(ctx),
            fee_collector: sender,
        }, sender);
    }

    public entry fun setFeeCollector(wrapperstore:&mut WrapperStore, new_fee_collector: address, ctx: &mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN);


        wrapperstore.fee_collector = new_fee_collector;
    }

    public fun feeCollector(wrapperstore:&mut WrapperStore): address {
        //let config_ref = borrow_global<WrapperStore>(POLY_BRIDGE);
        return wrapperstore.fee_collector
    }
    
    // for relayer 
    public entry fun relay_unlock_tx<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        proof: vector<u8>, 
        rawHeader: vector<u8>, 
        headerProof: vector<u8>, 
        curRawHeader: vector<u8>, 
        headerSig: vector<u8>,
        ctx: &mut TxContext
    ) {
        lock_proxy::relay_unlock_tx<CoinType>(
            ccManager, lpManager,treasury_ref,
            proof, rawHeader, headerProof, curRawHeader, headerSig, ctx);
    }

    // for user
    public entry fun lock_and_pay_fee<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        wrapperstore:&mut WrapperStore,
        account: address,
        fund: Coin<CoinType>,
        fee: Coin<BFC>,
        toChainId: u64, 
        toAddress: vector<u8>,
        ctx: &mut TxContext
    )  {

        //any user can lock bfc assets and transfer to evm

        lock_and_pay_fee_with_fund<CoinType>(ccManager,
            lpManager,treasury_ref,wrapperstore,
            account, fund, fee, toChainId, &toAddress,ctx);
    }

    public fun lock_and_pay_fee_with_fund<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        wrapperstore:&mut WrapperStore,
        account: address,
        fund: Coin<CoinType>, 
        fee: Coin<BFC>,
        toChainId: u64, 
        toAddress: &vector<u8>,
        ctx: &mut TxContext
    )  {
        let amount = coin::value(&fund);
        let fee_amount = coin::value(&fee);

        //coin::deposit<BFC>(feeCollector(), fee);

        let feeCollector = feeCollector(wrapperstore);
        transfer::public_transfer(fee, feeCollector);

        lock_proxy::lock(ccManager,lpManager,treasury_ref, account, fund, toChainId, toAddress,ctx);
        //let config_ref = borrow_global_mut<WrapperStore>(POLY_BRIDGE);
        event::emit(
            LockWithFeeEvent{
                from_asset: type_name::get<Coin<CoinType>>(),
                from_address: (account),
                to_chain_id: toChainId,
                to_address: *toAddress,
                amount: amount,
                fee_amount: fee_amount,
            },
        );
    }


}