
#[test_only]

module sui_system::gas_coin_map_tests {
    use sui::vec_map;
    use sui_system::gas_coin_map::GasCoinEntity;
    use sui::test_scenario;
    use sui::balance;
    use sui::obc::OBC;
    use sui::url;
    use sui::coin;
    use sui::object;
    use std::option;
    use sui_system::gas_coin_map;
    use sui::test_utils;
    use sui::pay;
    use sui::transfer;
    use sui::tx_context;

    struct COIN_TESTS has drop {}

    #[test]
    fun test_gas_coin_map_flow() {
        let scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);
        //init gas coin map
        let init_gas_coins_map = vec_map::empty<address, GasCoinEntity>();
        let dummy_coin = balance::zero<OBC>();
        let init_coin = coin::from_balance(dummy_coin, ctx);
        let coin_id_address = object::id_address(&init_coin);
        vec_map::insert(&mut init_gas_coins_map, coin_id_address, gas_coin_map::new_entity(coin_id_address));
        let gas_coin_map = gas_coin_map::new(init_gas_coins_map, ctx);
        assert!(gas_coin_map::map_size(&gas_coin_map) == 1, 100);
        pay::keep(init_coin, ctx);

        //add gas coin to map
        let witness = COIN_TESTS{};
        let (treasury, metadata) = coin::create_currency(witness, 6, b"COIN_TESTS", b"coin_name", b"description", option::some(url::new_unsafe_from_bytes(b"icon_url")), ctx);
        let balance = coin::mint_balance<COIN_TESTS>(&mut treasury, 1000);
        let coin1 = coin::from_balance(balance, ctx);
        gas_coin_map::request_add_gas_coin<COIN_TESTS>(&mut gas_coin_map, &coin1);
        assert!(gas_coin_map::map_size(&gas_coin_map) == 2, 101);
        pay::keep(coin1, ctx);

        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury, tx_context::sender(ctx));
        test_utils::destroy(gas_coin_map);
        test_scenario::end(scenario_val);
    }
}
