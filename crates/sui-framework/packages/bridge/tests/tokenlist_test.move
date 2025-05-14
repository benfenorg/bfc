#[test_only]
module bridge::tokenlist_tests {

    use sui::test_scenario;
    use bridge::tokenlist;
    use bridge::chain_ids::eth_mainnet;
    use std::unit_test::assert_eq;
    use bridge::bridge_env::eth_id;
    use sui::test_utils;


    public struct TokenListObject has key,store {
        id: UID
    }


    public fun new(ctx: &mut TxContext) : TokenListObject{
        TokenListObject {
            id: object::new(ctx),
        }
    }

    #[test]
    fun test_add_token_to_benfen(){
        //init
        let mut scenario = test_scenario::begin(@0xABC);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        tokenlist::new_tokenlist_registry_for_testing(&mut obj.id,ctx);
        assert_eq!(tokenlist::is_supported_to_benfen(&obj.id, eth_mainnet() as u64, eth_id()), false);
        scenario.next_tx(@0xABCD);
        //add
        tokenlist::add_token_to_benfen(&mut obj.id, eth_mainnet() as u64, eth_id() ,  scenario.ctx());

        scenario.next_tx(@0xABCD);
        //check
        assert_eq!(tokenlist::is_supported_to_benfen(&obj.id, eth_mainnet() as u64, eth_id()), true);

        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_remove_token_to_benfen(){
         //init
        let mut scenario = test_scenario::begin(@0xABC);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        tokenlist::new_tokenlist_registry_for_testing(&mut obj.id,ctx);
        assert_eq!(tokenlist::is_supported_to_benfen(&obj.id, eth_mainnet() as u64, eth_id()), false);
        scenario.next_tx(@0xABCD);
        //add
        tokenlist::add_token_to_benfen(&mut obj.id, eth_mainnet() as u64, eth_id() ,  scenario.ctx());

        scenario.next_tx(@0xABCD);
        //check
        assert_eq!(tokenlist::is_supported_to_benfen(&obj.id, eth_mainnet() as u64, eth_id()), true);
        //remove
        tokenlist::remove_token_to_benfen(&mut obj.id, eth_mainnet() as u64, eth_id());
        //check
        assert_eq!(tokenlist::is_supported_to_benfen(&obj.id, eth_mainnet() as u64, eth_id()), false);
        test_utils::destroy(obj);
        test_scenario::end(scenario);

    }

    #[test]
     fun test_add_token_from_benfen(){
        //init
        let mut scenario = test_scenario::begin(@0xABC);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        tokenlist::new_tokenlist_registry_for_testing(&mut obj.id,ctx);
        assert_eq!(tokenlist::is_supported_from_benfen(&obj.id, eth_mainnet() as u64, eth_id()), false);
        scenario.next_tx(@0xABCD);
        //add
        tokenlist::add_token_from_benfen(&mut obj.id, eth_mainnet() as u64, eth_id() ,  scenario.ctx());

        scenario.next_tx(@0xABCD);
        //check
        assert_eq!(tokenlist::is_supported_from_benfen(&obj.id, eth_mainnet() as u64, eth_id()), true);

        test_utils::destroy(obj);
        test_scenario::end(scenario);
     }


    #[test]
    fun test_remove_token_from_benfen(){
         //init
        let mut scenario = test_scenario::begin(@0xABC);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        tokenlist::new_tokenlist_registry_for_testing(&mut obj.id,ctx);
        assert_eq!(tokenlist::is_supported_from_benfen(&obj.id, eth_mainnet() as u64, eth_id()), false);
        scenario.next_tx(@0xABCD);
        //add
        tokenlist::add_token_from_benfen(&mut obj.id, eth_mainnet() as u64, eth_id() ,  scenario.ctx());

        scenario.next_tx(@0xABCD);
        //check
        assert_eq!(tokenlist::is_supported_from_benfen(&obj.id, eth_mainnet() as u64, eth_id()), true);
        //remove
        tokenlist::remove_token_from_benfen(&mut obj.id, eth_mainnet() as u64, eth_id());
        //check
        assert_eq!(tokenlist::is_supported_from_benfen(&obj.id, eth_mainnet() as u64, eth_id()), false);
        test_utils::destroy(obj);
        test_scenario::end(scenario);

    }





}