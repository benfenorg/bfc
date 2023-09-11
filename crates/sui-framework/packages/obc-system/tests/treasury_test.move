#[test_only]
module obc_system::treasury_test {
    use sui::test_scenario;
    use std::debug;
    use sui::coin;
    use sui::obc::OBC;
    use sui::transfer;
    use obc_system::treasury::{Self, Treasury};

    #[test]
    public fun test_treasury() {
        let owner = @0x0;
        let scenario_val = test_scenario::begin(owner);

        //create treasury
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = treasury::create_treasury(
                3600 * 4,
                test_scenario::ctx(&mut scenario_val),
            );
            transfer::public_share_object(t);
        };

        // check info
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            assert!(treasury::index(&t) == 0, 0);
            assert!(treasury::get_balance(&t) == 0, 1);
            test_scenario::return_shared(t);
        };

        // create vault
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let t = test_scenario::take_shared<Treasury>(&mut scenario_val);
            test_scenario::return_shared(t);
        };

        test_scenario::end(scenario_val);
    }
}


