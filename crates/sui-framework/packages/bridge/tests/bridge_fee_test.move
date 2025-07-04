#[test_only]
module bridge::bridge_fee_tests {

    use sui::test_scenario;
    use bridge::bridge_fee;
    use bridge::chain_ids::eth_mainnet;
    use std::unit_test::assert_eq;
    use bridge::bridge_env::usdc_id;
    use sui::test_utils;

    public struct BridgeFeeObject has key,store {
        id: UID
    }


    public fun new(ctx: &mut TxContext) : BridgeFeeObject{
        BridgeFeeObject {
            id: object::new(ctx),
        }
    }


    #[test]
    fun test_set_fee_cross_in(){
        //init
        let mut scenario = test_scenario::begin(@0xABC);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        bridge_fee::new_bridge_fee_registry(&mut obj.id,ctx);
        let amount =5*1_000_000_000;
        let fee=2_500_000;
        assert_eq!(bridge_fee::calculate_cross_in_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==0, true);
        scenario.next_tx(@0xABCD);
        //set 1
        bridge_fee::set_fee_in_cross_in(&mut obj.id, eth_mainnet() as u64, usdc_id() ,1,500,  scenario.ctx());
        scenario.next_tx(@0xABCD);
        //check
        assert_eq!(bridge_fee::calculate_cross_in_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==fee, true);
        assert_eq!(bridge_fee::get_cross_in_amount_after_fee(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==amount-fee, true);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_set_fee_cross_in_with_twice_mode(){
         //init
        let mut scenario = test_scenario::begin(@0xABC);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        bridge_fee::new_bridge_fee_registry(&mut obj.id,ctx);
        let amount =5*1_000_000_000;
        let fee=2_500_000;
        assert_eq!(bridge_fee::calculate_cross_in_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==0, true);
        scenario.next_tx(@0xABCD);
        //set 1
        bridge_fee::set_fee_in_cross_in(&mut obj.id, eth_mainnet() as u64, usdc_id() ,1,500,  scenario.ctx());
        scenario.next_tx(@0xABCD);
        //check
        assert_eq!(bridge_fee::calculate_cross_in_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==fee, true);
        assert_eq!(bridge_fee::get_cross_in_amount_after_fee(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==amount-fee, true);
        scenario.next_tx(@0xABCEF);

        //set 0
        let amount =6*1_000_000_000;
        let fee=5*1_000_000_000;
        bridge_fee::set_fee_in_cross_in(&mut obj.id, eth_mainnet() as u64, usdc_id() ,0,fee,  scenario.ctx());

        scenario.next_tx(@0xABCDE);
        //check
        assert_eq!(bridge_fee::calculate_cross_in_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==fee, true);
        assert_eq!(bridge_fee::get_cross_in_amount_after_fee(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==amount-fee, true);
        test_utils::destroy(obj);
        test_scenario::end(scenario);

    }


    #[test]
    fun test_set_fee_cross_out(){
        //init
        let mut scenario = test_scenario::begin(@0xABC);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        bridge_fee::new_bridge_fee_registry(&mut obj.id,ctx);
        let amount =5*1_000_000_000;
        let fee=2_500_000; //0.05%
        //assert_eq!(bridge_fee::calculate_cross_out_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==0, true);
        scenario.next_tx(@0xABCD);
        //set 1
        bridge_fee::set_fee_in_cross_out(&mut obj.id, eth_mainnet() as u64, usdc_id() ,1,500,  scenario.ctx());
        scenario.next_tx(@0xABCD);
        //check
        assert_eq!(bridge_fee::calculate_cross_out_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==fee, true);
        assert_eq!(bridge_fee::get_cross_out_amount_after_fee(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==amount-fee, true);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }


    #[test]
    fun test_set_fee_cross_out_with_twice_mode(){
         //init
        let mut scenario = test_scenario::begin(@0xABC);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        bridge_fee::new_bridge_fee_registry(&mut obj.id,ctx);
        let amount =5*1_000_000_000;
        let fee=2_500_000;
        let fee1=3_000_000;
        assert_eq!(bridge_fee::calculate_cross_out_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==fee, true);
        scenario.next_tx(@0xABCD);
        //set 1
        bridge_fee::set_fee_in_cross_out(&mut obj.id, eth_mainnet() as u64, usdc_id() ,1,600,  scenario.ctx());
        scenario.next_tx(@0xABCD);
        //check
        std::debug::print(&bridge_fee::calculate_cross_out_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount));
        assert_eq!(bridge_fee::calculate_cross_out_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==fee1, true);
        assert_eq!(bridge_fee::get_cross_out_amount_after_fee(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==amount-fee1, true);
        scenario.next_tx(@0xABCEF);

        //set 0
        let amount =6*1_000_000_000;
        let fee=5*1_000_000_000;
        bridge_fee::set_fee_in_cross_out(&mut obj.id, eth_mainnet() as u64, usdc_id() ,0,fee,  scenario.ctx());

        scenario.next_tx(@0xABCDE);
        //check
        assert_eq!(bridge_fee::calculate_cross_out_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==fee, true);
        assert_eq!(bridge_fee::get_cross_out_amount_after_fee(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==amount-fee, true);

        scenario.next_tx(@0xABCDE10);

        let amount =6*1_000_000_000;
        let fee=1*1_000_000_000;
        bridge_fee::set_fee_in_cross_out(&mut obj.id, eth_mainnet() as u64, usdc_id() ,0,fee,  scenario.ctx());

        scenario.next_tx(@0xABCDE2);
        //check
        assert_eq!(bridge_fee::calculate_cross_out_fee_amount(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==fee, true);
        assert_eq!(bridge_fee::get_cross_out_amount_after_fee(&obj.id, eth_mainnet() as u64, usdc_id(),amount)==amount-fee, true);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

}