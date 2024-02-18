#[test_only]
module polynet::wrapper_v1_test {
    use polynet::wrapper_v1::{init_wrapper, feeCollector, WrapperStore, setFeeCollector};
    use polynet::utils;
    use sui::test_scenario;
    use sui::test_scenario::return_to_sender;

    #[test]
    fun test_wrapper_init(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_wrapper(ctx);

        };
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let wrapper_store = test_scenario::take_from_sender<WrapperStore>(&mut scenario_val );

            let fee_collector =  feeCollector(&mut wrapper_store);
            assert!(fee_collector==owner, 4002);

            return_to_sender(&mut scenario_val, wrapper_store);
        };

        let new_fee_collecotr = @0x01;
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let wrapper_store = test_scenario::take_from_sender<WrapperStore>(&mut scenario_val );


            let ctx = test_scenario::ctx(&mut scenario_val);
            setFeeCollector(&mut wrapper_store, new_fee_collecotr,ctx);
            return_to_sender(&mut scenario_val, wrapper_store);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let wrapper_store = test_scenario::take_from_sender<WrapperStore>(&mut scenario_val );

            let fee_collector =  feeCollector(&mut wrapper_store);
            assert!(fee_collector==new_fee_collecotr, 4002);

            return_to_sender(&mut scenario_val, wrapper_store);
        };





        test_scenario::end(scenario_val);
    }
}
