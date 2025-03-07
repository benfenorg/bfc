module test_oracle_price::test_oracle {
    use sui::dynamic_field;
    use sui::vec_map;
    use sui::coin::{Coin};
    use sui::transfer;
    use sui::balance::Balance;

    public struct TestOraclePrice has key, store {
        id: UID,
    }

    public struct PriceIdentifier has copy, drop, store {
        coin_type_a: vector<u8>,
        coin_type_b: vector<u8>,
    }

    public struct Pool<phantom A, phantom B> has key, store {
        id: UID,
        a_balance: Balance<A>,
        b_balance: Balance<B>,
    }

    public fun new_pool<A, B>(
        coin_a: Coin<A>,
        ctx: &mut TxContext
    ) {
        transfer::public_transfer(
            Pool{
                id: object::new(ctx),
                a_balance: coin_a.into_balance(),
                b_balance: balance::zero(),
            },
            tx_context::sender(ctx)
        );
    }

    public fun empty_test(_ctx: &mut TxContext) {}

    public fun stable_coin_test_swap<A, B>(p: &mut Pool<A, B>, coin_b: Coin<B>, ctx: &mut TxContext) {
        p.b_balance.join(coin_b.into_balance());
        let coin = p.a_balance.take(1);

        transfer::public_transfer(coin, ctx.sender());
    }

    public fun oracle(ctx: &mut TxContext) {
        let mut uid: UID = object::new(ctx);

        let mut price_map = vec_map::empty<PriceIdentifier, u64>();
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            100,
            );
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            6400000,
            );
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::mgg::MGG", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            6400000,
            );
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::bkrw::BKRW", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            6400000,
            );
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::baud::BAUD", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            6400000,
            );
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::bars::BARS", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            6400000,
            );
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::bbrl::BBRL", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            6400000,
            );
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::bcad::BCAD", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            6400000,
            ); 
        // Illegal data  
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::error::ERROR", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            1000,
            );
        // Illegal data  
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            1000,
            );
                // Illegal data  
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY" }, 
            1000,
            );
        dynamic_field::add(&mut uid, b"Test oracle price", price_map);

        let obj = TestOraclePrice { id: uid };
        transfer::transfer(obj, ctx.sender());
    }

    public fun oracle_with_test_coin(c :vector<u8>, ctx: &mut TxContext) {
        let mut uid: UID = object::new(ctx);

        let mut price_map = vec_map::empty<PriceIdentifier, u64>();
         price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::beur::BEUR", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            647300000,
            );
        price_map.insert(PriceIdentifier { 
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::xxxx::XXXX", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            647300000,
        );
        price_map.insert(PriceIdentifier { 
                coin_type_a: c, 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            647300000,
            );
     
        dynamic_field::add(&mut uid, b"Test oracle price", price_map);

        let obj = TestOraclePrice { id: uid };
        transfer::transfer(obj, ctx.sender());
    }
}
