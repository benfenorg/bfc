module sui_system::gas_coin_set {
    use sui::object::ID;
    use sui::coin::Coin;
    use sui::object;
    use sui::vec_map::{Self, VecMap};
    use std::vector;

    struct GasCoinMap has store {
        ///The current active gas coin
        active_gas_coins: VecMap<ID, GasCoin>,
    }

    struct GasCoin has key, store {
        id: ID
    }
    /// Init gas coin map
    public(friend) fun new(init_gas_coins: vector<GasCoin>): GasCoinMap {
        let active_gas_coins = vec_map::empty<ID, GasCoin>();
        let num_coins = vector::length(&init_gas_coins);
        let i = 0;
        while (i < num_coins) {
            let gasCoin = vector::remove(&mut init_gas_coins, i);
            vec_map::insert(&mut active_gas_coins, gasCoin.id, gasCoin);
            i = i + 1;
        };
        GasCoinMap {
            active_gas_coins
        }
    }

    public(friend) fun request_add_gas_coin<CoinType>(
        self: &mut GasCoinMap,
        gas_coin: Coin<CoinType>,) {
        let uid = object::id(&gas_coin);
        vec_map::insert(&mut self.active_gas_coins, uid, GasCoin {
            id: uid,
        });
    }

    public(friend) fun request_remove_gas_coin<CoinType>(
        self: &mut GasCoinMap,
        gas_coin: Coin<CoinType>,) {
        let uid = object::id(&gas_coin);
        vec_map::remove(&mut self.active_gas_coins, &uid);
    }
}
