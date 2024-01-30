module poly_bridge::wrapper_v1 {
    use sui::bfc::BFC;
    use sui::event;
    use sui::type_info::{TypeInfo, Self};
    use sui::coin::{Coin, Self};

    use poly_bridge::lock_proxy;

    const DEPRECATED: u64 = 1;
    const EINVALID_SIGNER: u64 = 2;

    struct WrapperStore has key, store{
        fee_collector: address,
        lock_with_fee_event: event::EventHandle<LockWithFeeEvent>
    }

    struct LockWithFeeEvent has store, drop, copy{
        from_asset: TypeInfo,
        from_address: address,
        to_chain_id: u64,
        to_address: vector<u8>,
        amount: u64,
        fee_amount: u64
    }

    // for admin
    public entry fun init(admin: address) {
        assert!((admin) == @poly_bridge, EINVALID_SIGNER);
        move_to(admin, WrapperStore{
            fee_collector: @poly_bridge,
            lock_with_fee_event: account::new_event_handle<LockWithFeeEvent>(admin)
        });
    }

    public entry fun setFeeCollector(admin: address, new_fee_collector: address) acquires WrapperStore {
        assert!((admin) == @poly_bridge, EINVALID_SIGNER);
        let config_ref = borrow_global_mut<WrapperStore>(@poly_bridge);
        config_ref.fee_collector = new_fee_collector;
    }

    public fun feeCollector(): address acquires WrapperStore {
        let config_ref = borrow_global<WrapperStore>(@poly_bridge);
        return config_ref.fee_collector
    }
    
    // for relayer 
    public entry fun relay_unlock_tx<CoinType>(
        proof: vector<u8>, 
        rawHeader: vector<u8>, 
        headerProof: vector<u8>, 
        curRawHeader: vector<u8>, 
        headerSig: vector<u8>
    ) {
        lock_proxy::relay_unlock_tx<CoinType>(proof, rawHeader, headerProof, curRawHeader, headerSig);
    }

    // for user
    public entry fun lock_and_pay_fee<CoinType>(
        account: address,
        amount: u64, 
        fee_amount: u64,
        toChainId: u64, 
        toAddress: vector<u8>
    ) acquires WrapperStore {
        let fund = coin::withdraw<CoinType>(account, amount);
        let fee = coin::withdraw<AptosCoin>(account, fee_amount);
        lock_and_pay_fee_with_fund<CoinType>(account, fund, fee, toChainId, &toAddress);
    }

    public fun lock_and_pay_fee_with_fund<CoinType>(
        account: address,
        fund: Coin<CoinType>, 
        fee: Coin<BFC>,
        toChainId: u64, 
        toAddress: &vector<u8>
    ) acquires WrapperStore { 
        let amount = coin::value(&fund);
        let fee_amount = coin::value(&fee);
        coin::deposit<BFC>(feeCollector(), fee);
        lock_proxy::lock(account, fund, toChainId, toAddress);
        let config_ref = borrow_global_mut<WrapperStore>(@poly_bridge);
        event::emit(
            LockWithFeeEvent{
                from_asset: type_info::type_of<Coin<CoinType>>(),
                from_address: (account),
                to_chain_id: toChainId,
                to_address: *toAddress,
                amount: amount,
                fee_amount: fee_amount,
            },
        );
    }

    public entry fun register_coin<CoinType>(account: address) {
        coin::register<CoinType>(account);
    }
}