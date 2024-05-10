module polynet::utils {
    use std::vector;
    use sui::transfer;
    use sui::coin::{Self, Coin};
    use std::string::{String, Self};
    use sui::bcs;
    use sui::table::{Table, Self};

    const EUNSUPPORT_GENERIC_TYPE: u64 = 7003;
    const EINVALID_FROM_BYTES_TO_BOOL: u64 = 7004;
    const EINVALID_FROM_BYTES_TO_U8: u64 = 7005;
    const EINVALID_FROM_BYTES_TO_U64: u64 = 7006;
    const EINVALID_FROM_BYTES_TO_U128: u64 = 7007;

    public fun slice<Element: copy>(v: &vector<Element>, offset: u64, length: u64): vector<Element> {
        let res = vector::empty<Element>();
        while ( length > 0 ) {
            length = length - 1;
            let t = *vector::borrow<Element>(v, offset);
            vector::push_back<Element>(&mut res, t);
            offset = offset + 1;
        };
        return res
    }
    public fun to_bool(v: vector<u8>): bool {
        let data = bcs::new(v);
        return bcs::peel_bool(&mut data)
    }

    public fun to_u8(v: vector<u8>): u8 {
        let data = bcs::new(v);
        return   bcs::peel_u8(&mut data)

    }

    public fun to_u64(v: vector<u8>): u64 {
        let data = bcs::new(v);
        return bcs::peel_u64(&mut data)
    }

    public fun to_u128(v: vector<u8>): u128 {
        let data = bcs::new(v);
        return bcs::peel_u128(&mut data)
    }

    public fun to_address(v: vector<u8>): address {
        let data = bcs::new(v);
        return bcs::peel_address(&mut data)
    }

    public fun to_string(v: vector<u8>): String {
        let data = string::utf8(v);
        return data
    }

    public fun upsert<K: copy + drop + store, V:  drop + store>(tb: &mut Table<K, V>, k: K, v: V) {
        if (table::contains(tb, k)) {  //if return true just update
            table::remove(tb, k);
        };
        table::add(tb, k, v);
    }

    public fun borrow_mut_with_default<K: copy + drop + store, V: drop + store>(table: &mut Table<K, V>, key: K, default: V): &mut V {
        if (!table::contains(table, key)) {
            table::add(table, copy key, default)
        };
        table::borrow_mut(table, key)
    }

     public fun send_coin<CoinType>(
        _coin: Coin<CoinType>,
        _addr: address
    ) {
        let amount = coin::value<CoinType>(&_coin);
        if (amount > 0) {
            transfer::public_transfer<Coin<CoinType>>(_coin,_addr);
        }else {
            coin::destroy_zero<CoinType>(_coin);
        }
    }

    // public fun from_bytes<T>(v: vector<u8>): T {
    //     let type = type_name::into_string(type_name::get<T>());
    //     if (type == string::to_ascii(string::utf8(b"bool")) ) {
    //         let res = from_bcs::to_bool(v);
    //         return any::unpack<T>(any::pack(res))
    //     } else if (type == string::to_ascii(string::utf8(b"u8"))) {
    //         let res = from_bcs::to_u8(v);
    //         return any::unpack<T>(any::pack(res))
    //     } else if (type == string::to_ascii(string::utf8(b"u64"))) {
    //         let res = from_bcs::to_u64(v);
    //         return any::unpack<T>(any::pack(res))
    //     } else if (type == string::to_ascii(string::utf8(b"u128"))) {
    //         let res = from_bcs::to_u128(v);
    //         return any::unpack<T>(any::pack(res))
    //     } else if (type == string::to_ascii(string::utf8(b"address"))) {
    //         let res = from_bcs::to_address(v);
    //         return any::unpack<T>(any::pack(res))
    //     } else if (type == string::to_ascii(string::utf8(b"0x1::string::String"))) {
    //         let res = from_bcs::to_string(v);
    //         return any::unpack<T>(any::pack(res))
    //     } else {
    //         abort EUNSUPPORT_GENERIC_TYPE
    //     }
    // }
}