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

    ///Default exchange rate
    const DEFAULT_EXCHANGE_RATE: u64 = 1_000_000_000;

    struct GasCoinMap has store {
        ///The current active gas coin
        active_gas_coins: VecMap<address, GasCoinEntity>,
    }

    struct GasCoinEntity has store, drop {
        id_address: address,
        exchange_rate: u64
    }
    /// Init gas coin map
    public(friend) fun new(init_gas_coins: VecMap<address, GasCoinEntity>, _ctx: &mut TxContext): GasCoinMap {
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

    public(friend) fun new_default_entity(id_address: address): GasCoinEntity {
        GasCoinEntity {
            id_address,
            exchange_rate:DEFAULT_EXCHANGE_RATE
        }
    }

    public(friend) fun new_entity(id_address: address, exchange_rate: u64): GasCoinEntity {
        GasCoinEntity {
            id_address,
            exchange_rate
        }
    }

    public(friend) fun map_size(self: &GasCoinMap): u64 {
        vec_map::size(&self.active_gas_coins)
    }

    public(friend) fun request_add_gas_coin<CoinType>(
        self: &mut GasCoinMap,
        gas_coin: &Coin<CoinType>) {
        let id_address = object::id_address<Coin<CoinType>>(gas_coin);
        vec_map::insert(&mut self.active_gas_coins, id_address, GasCoinEntity {
            id_address,
            exchange_rate: DEFAULT_EXCHANGE_RATE
        });
    }

    public(friend) fun request_update_gas_coin<CoinType>(
        self: &mut GasCoinMap,
        gas_coin: &Coin<CoinType>, exchange_rate: u64) {
        let id_address = object::id_address<Coin<CoinType>>(gas_coin);
        let entity = vec_map::get_mut(&mut self.active_gas_coins, &id_address);
        entity.exchange_rate = exchange_rate
    }

    public(friend) fun requst_get_exchange_rate<CoinType>(
        self: &GasCoinMap,
        gas_coin: &Coin<CoinType>): u64 {
        let id_address = object::id_address<Coin<CoinType>>(gas_coin);
        let gas_entity = vec_map::get(&self.active_gas_coins, &id_address);
        gas_entity.exchange_rate
    }

    public(friend) fun request_remove_gas_coin<CoinType>(
        self: &mut GasCoinMap,
        gas_coin: &Coin<CoinType>,) {
        let id_address = object::id_address<Coin<CoinType>>(gas_coin);
        vec_map::remove(&mut self.active_gas_coins, &id_address);
    }
}
