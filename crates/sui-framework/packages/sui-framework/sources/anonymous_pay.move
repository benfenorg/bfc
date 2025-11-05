// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// This module provides handy functionality for wallets and `sui::Coin` management.
module sui::anonymous_pay;

use sui::anonymous_coin::Anonymous_Coin;

#[allow(lint(self_transfer))]
/// Transfer `c` to the sender of the current transaction
public fun keep<T>(c: Anonymous_Coin<T>, ctx: &TxContext) {
    transfer::public_transfer(c, ctx.sender())
}

/// Split coin `self` to two coins, one with balance `split_amount`,
/// and the remaining balance is left is `self`.
public entry fun split<T>(coin: &mut Anonymous_Coin<T>, split_amount: u64, ctx: &mut TxContext) {
    keep(coin.split(split_amount, ctx), ctx)
}

/// Split coin `self` to two coins, one with balance `split_amount`,
/// and the remaining balance is left is `self`.
public entry fun split_anonymous<T>(
    coin: &mut Anonymous_Coin<T>,
    value1: vector<u8>,
    value2: vector<u8>,
    ctx: &mut TxContext) {
    keep(coin.split_anonymous(value1, value2, ctx), ctx)
}





public entry fun split_and_transfer_anonymous<T>(
    c: &mut Anonymous_Coin<T>,
    value1: vector<u8>,
    value2: vector<u8>,
    recipient: address,
    ctx: &mut TxContext,
) {
    transfer::public_transfer(c.split_anonymous(value1, value2, ctx), recipient)
}



/// Join `coin` into `self`. Re-exports `coin::join` function.
/// Deprecated: you should call `coin.join(other)` directly.
public entry fun join<T>(self: &mut Anonymous_Coin<T>, coin: Anonymous_Coin<T>, ctx: &mut TxContext) {
    self.join(coin, ctx)
}

/// Join everything in `coins` with `self`
public entry fun join_vec<T>(self: &mut Anonymous_Coin<T>, mut coins: vector<Anonymous_Coin<T>>, ctx: &mut TxContext) {
    let ( mut i, len) = (0, coins.length());
    while (i < len) {
        let coin = coins.pop_back();
        self.join(coin, ctx);
        i = i + 1
    };
    // safe because we've drained the vector
    coins.destroy_empty()
}

/// Join a vector of `Coin` into a single object and transfer it to `receiver`.
public entry fun join_vec_and_transfer<T>( mut _coins: vector<Anonymous_Coin<T>>, _receiver: address) {
    abort 99
}

/// Split coin `self` into multiple coins, each with balance specified
/// in `split_amounts`. Remaining balance is left in `self`.
public entry fun split_vec<T>(_self: &mut Anonymous_Coin<T>, _split_amounts: vector<u64>, _ctx: &mut TxContext) {
    abort 99
}
/// Send `amount` units of `c` to `recipient`
/// Aborts with `EVALUE` if `amount` is greater than or equal to `amount`
public entry fun split_and_transfer<T>(
    _c: &mut Anonymous_Coin<T>,
    _amount: u64,
    _recipient: address,
    _ctx: &mut TxContext,
) {
    //should not use this
    abort 99
}
