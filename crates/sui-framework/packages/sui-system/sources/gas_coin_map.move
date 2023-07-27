module sui_system::gas_coin_set {
    use sui::coin::Coin;
    use sui::object;
    use sui::vec_map::{Self, VecMap};
    use std::vector;

    struct GasCoinMap has store {
        ///The current active gas coin
        active_gas_coins: VecMap<address, GasCoin>,
    }

    struct GasCoin has store, drop {
        id_address: address
    }
    /// Init gas coin map
    public(friend) fun new(init_gas_coins: VecMap<address, GasCoin>): GasCoinMap {
        let active_gas_coins = vec_map::empty<address, GasCoin>();
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

    public(friend) fun request_add_gas_coin<CoinType>(
        self: &mut GasCoinMap,
        gas_coin: &Coin<CoinType>,) {
        let id_address = object::id_address<Coin<CoinType>>(gas_coin);
        vec_map::insert(&mut self.active_gas_coins, id_address, GasCoin {
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
