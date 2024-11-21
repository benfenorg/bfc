module test_oracle_price::test_oracle {
    use sui::dynamic_field;
    use sui::vec_map;

    public struct TestOraclePrice has key, store {
        id: UID,
    }

    public struct PriceIdentifier has copy, drop, store {
        coin_type_a: vector<u8>,
        coin_type_b: vector<u8>,
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
                coin_type_a: b"00000000000000000000000000000000000000000000000000000000000000c8::error::ERROR", 
                coin_type_b: b"00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD" }, 
            6400000,
            );
        dynamic_field::add(&mut uid, b"Test oracle price", price_map);

        let obj = TestOraclePrice { id: uid };
        transfer::transfer(obj, ctx.sender());
    }
}
