// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module examples::move_random {
    use std::vector;
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::TxContext;

    struct Object has key, store {
        id: UID,
        data: vector<u64>,
    }

    // simple infinite loop to go out of gas in computation
    public entry fun loopy() {
        loop { }
    }

    // for test: delete two objects
    public entry fun delete(o: Object, o2: Object) {
        let Object { id, data: _ } = o;
        object::delete(id);

        let Object { id, data: _ } = o2;
        object::delete(id);
    }

    // create an object with a vector of size `size` and transfer to recipient
    public entry fun storage_heavy(size: u64, recipient: address, ctx: &mut TxContext) {
        let data = vector::empty();
        while (size > 0) {
            vector::push_back(&mut data, size);
            size = size - 1;
        };
        transfer::public_transfer(
            Object { id: object::new(ctx), data },
            recipient
        )
    }
}
