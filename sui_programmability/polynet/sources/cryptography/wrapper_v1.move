#[allow(unused_function)]
module polynet::wrapper_v1 {
    use sui::bfc::BFC;
    use std::type_name::{Self};
    use sui::clock::Clock;
    use polynet::events;
    use polynet::lock_proxy::{Treasury, LockProxyManager, Self};
    use polynet::cross_chain_manager::{CrossChainManager};
    use sui::coin::{Coin, Self};
    // use sui::object::{UID, Self};
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    // friend polynet::cross_chain_manager;
    friend polynet::controller ;
    friend polynet::config;
    friend polynet::wrapper_v1_test;
    friend polynet::tools_test;
    friend polynet::lock_proxy_test;


    const DEPRECATED: u64 = 4001;
    const EINVALID_ADMIN: u64 = 4015;
    const EINVALID_SYSTEM_IS_PAUSED: u64 = 4019;



    const DECIMALS: u8 = 8;
    const MAX_AMOUNT: u64 = 100*0000*100000000; //1 million.

    const ONE_DAY : u64 = 24*60*60*1000; //24*60*60*1000


    struct WrapperStore has store{
        // id: UID,
        fee_collector: address,
    }

    public(friend) fun new(_ctx: &mut TxContext): WrapperStore {

        WrapperStore{
            // id: object::new(_ctx),
            fee_collector:tx_context::sender(_ctx) //maybe should set at config file
        }
    }


    public(friend) fun setFeeCollector(
        _wrapperstore:&mut WrapperStore, 
        _new_fee_collector: address, 
        _ctx: &mut TxContext
    ) {
      
        _wrapperstore.fee_collector = _new_fee_collector;
    }

    public fun feeCollector(_wrapperstore: &WrapperStore): address {
        //let config_ref = borrow_global<WrapperStore>(POLY_BRIDGE);
        return _wrapperstore.fee_collector
    }
    

    public(friend) fun lock_and_pay_fee_with_fund<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        wrapperstore:&mut WrapperStore,
        account: address,
        fund: Coin<CoinType>, 
        fee: Coin<BFC>,
        toChainId: u64, 
        toAddress: &vector<u8>,
        clock:&Clock,
        ctx: &mut TxContext
    )  {
        let amount = coin::value(&fund);
        let fee_amount = coin::value(&fee);

        //coin::deposit<BFC>(feeCollector(), fee);

        let feeCollector = feeCollector(wrapperstore);
        transfer::public_transfer(fee, feeCollector);

        lock_proxy::lock(ccManager,lpManager,treasury_ref, account, fund, toChainId, toAddress,clock, ctx);
        //let config_ref = borrow_global_mut<WrapperStore>(POLY_BRIDGE);
        events::lock_with_fee_event(
                         type_name::get<Coin<CoinType>>(),
                         account,
                         toChainId,
                         *toAddress,
                         amount,
                         fee_amount,
                        );
    }


}