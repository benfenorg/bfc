#[allow(unused_function)]
module polynet::wrapper_v1 {
    use sui::bfc::BFC;
    use std::type_name::{Self};
    use sui::clock::Clock;
    use polynet::events;
    use polynet::utils;
    use polynet::lock_proxy::{Treasury, LockProxyManager, Self};
    use polynet::cross_chain_manager::{CrossChainManager};
    use sui::coin::{Coin, Self};
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    friend polynet::controller ;
    friend polynet::config;
    friend polynet::tools;

    #[test_only]
    friend polynet::wrapper_v1_test;
   

    const DECIMALS: u8 = 8;
    const MAX_AMOUNT: u64 = 100*0000*100000000; //1 million.

    const ONE_DAY : u64 = 24*60*60*1000; //24*60*60*1000


    struct WrapperStore has store{
        fee_collector: address,
        need_fee: bool
    }

    public(friend) fun new(_ctx: &mut TxContext): WrapperStore {

        WrapperStore{
            fee_collector:tx_context::sender(_ctx), //maybe should set at config file
            need_fee: false //default false
        }
    }


    public(friend) fun set_fee_collector(
        _wrapperstore:&mut WrapperStore, 
        _new_fee_collector: address, 
        _ctx: &mut TxContext
    ) {
      
        _wrapperstore.fee_collector = _new_fee_collector;
    }

    public fun fee_collector(_wrapperstore: &WrapperStore): address {
        return _wrapperstore.fee_collector
    }
    

    public(friend) fun lock_and_pay_fee_with_fund<CoinType>(
        cc_manager:&mut CrossChainManager,
        lp_manager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        wrapper_store:&mut WrapperStore,
        account: address,
        fund: Coin<CoinType>, 
        amount: u64,
        fee: Coin<BFC>,
        to_chain_id: u64, 
        to_address: &vector<u8>,
        clock:&Clock,
        ctx: &mut TxContext
    )  {
        // let amount = coin::value(&fund);
        let fee_amount = 0;

        //coin::deposit<BFC>(feeCollector(), fee);
        if (wrapper_store.need_fee) {
            let fee_amount = coin::value(&fee);
            let collector = fee_collector(wrapper_store);
            transfer::public_transfer(fee, collector); 
        } else {
            utils::send_coin(fee,account);
        };

        lock_proxy::lock(
                cc_manager,
                lp_manager,
                treasury_ref, 
                account, 
                fund, 
                amount, 
                to_chain_id, 
                to_address,
                clock, 
                ctx
            );
        events::lock_with_fee_event(
                         type_name::get<Coin<CoinType>>(),
                         account,
                         to_chain_id,
                         *to_address,
                         amount,
                         fee_amount
                        );
    }


}