module polynet::wrapper_v1 {
    use sui::bfc::BFC;
    use sui::event;
    //use sui::type_info::{TypeInfo, Self};
    use std::type_name::{Self, TypeName};
    use polynet::utils;
    use polynet::lock_proxy::{Treasury, LockProxyManager};
    use polynet::cross_chain_manager::{CrossChainManager};
    use sui::coin::{Coin, Self};
    use sui::object;
    use sui::object::UID;
    use sui::transfer;
    use sui::transfer::transfer;
    use sui::tx_context::TxContext;

    use polynet::lock_proxy;

    const DEPRECATED: u64 = 1;
    const EINVALID_SIGNER: u64 = 2;
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
    public entry fun init_wrapper(admin: address , ctx: &mut TxContext) {
        assert!((admin) == utils::get_bridge_address(), EINVALID_SIGNER);

        transfer(WrapperStore{
            id: object::new(ctx),
            fee_collector: utils::get_bridge_address(),
        }, admin);
    }

    public entry fun setFeeCollector(wrapperstore:&mut WrapperStore, admin: address, new_fee_collector: address) {
        assert!((admin) == utils::get_bridge_address(), EINVALID_SIGNER);
        //let config_ref = borrow_global_mut<WrapperStore>(POLY_BRIDGE);
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
        toAddress: vector<u8>
    )  {

        //todo: add check?

        //let fund = coin::withdraw<CoinType>(account, amount);
        //let fee = coin::withdraw<BFC>(account, fee_amount);
        lock_and_pay_fee_with_fund<CoinType>(ccManager, lpManager,treasury_ref,wrapperstore, account, fund, fee, toChainId, &toAddress);
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
        toAddress: &vector<u8>
    )  {
        let amount = coin::value(&fund);
        let fee_amount = coin::value(&fee);

        //coin::deposit<BFC>(feeCollector(), fee);

        let feeCollector = feeCollector(wrapperstore);
        transfer::public_transfer(fee, feeCollector);

        lock_proxy::lock(ccManager,lpManager,treasury_ref, account, fund, toChainId, toAddress);
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

    // public entry fun register_coin<CoinType>(account: address) {
    //     //coin::register<CoinType>(account);
    // }
}