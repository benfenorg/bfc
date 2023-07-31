module sui_system::gas_coin_map {
    use sui::coin::Coin;
    use sui::object;
    use sui::vec_map::{Self, VecMap};
    use std::vector;
    use sui::tx_context::TxContext;

    friend sui_system::genesis;
    friend sui_system::sui_system_state_inner;

    #[test_only]
    friend sui_system::gas_coin_map_tests;

    struct GasCoinMap has store {
        ///The current active gas coin
        active_gas_coins: VecMap<address, GasCoinEntity>,
    }

    struct GasCoinEntity has store, drop {
        id_address: address
    }
    /// Init gas coin map
    public(friend) fun new(init_gas_coins: VecMap<address, GasCoinEntity>, ctx: &mut TxContext): GasCoinMap {
        let active_gas_coins = vec_map::empty<address, GasCoinEntity>();
        let init_keys = vec_map::keys(&init_gas_coins);
        let num_coins = vector::length(&init_keys);
        let i = 0;
        while (i < num_coins) {
            let (id, gasCoin) = vec_map::pop(&mut init_gas_coins);
            vec_map::insert(&mut active_gas_coins, id, gasCoin);
            i = i + 1;
        };
       let map = GasCoinMap {
            active_gas_coins
        };
        map
    }

    public(friend) fun new_entity(id_address: address): GasCoinEntity {
        GasCoinEntity {
            id_address,
        }
    }

    public(friend) fun map_size(self: &GasCoinMap): u64 {
        vec_map::size(&self.active_gas_coins)
    }

    public(friend) fun request_add_gas_coin<CoinType>(
        self: &mut GasCoinMap,
        gas_coin: &Coin<CoinType>,) {
        let id_address = object::id_address<Coin<CoinType>>(gas_coin);
        vec_map::insert(&mut self.active_gas_coins, id_address, GasCoinEntity {
            id_address,
        });
    }

    public(friend) fun request_remove_gas_coin<CoinType>(
        self: &mut GasCoinMap,
        gas_coin: &Coin<CoinType>,) {
        let id_address = object::id_address<Coin<CoinType>>(gas_coin);
        vec_map::remove(&mut self.active_gas_coins, &id_address);
    }
}
