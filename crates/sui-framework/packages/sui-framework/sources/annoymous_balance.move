// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// A storable handler for Balances in general. Is used in the `Coin`
/// module to allow balance operations and can be used to implement
/// custom coins with `Supply` and `Balance`s.
module sui::annoymous_balance;
use sui::vec_set;

/// Allows calling `.into_coin()` on a `Balance` to turn it into a coin.
public use fun sui::annoymous_coin::from_balance as Annoymos_Balance.into_coin;

/// For when trying to destroy a non-zero balance.
const ENonZero: u64 = 0;
/// For when an overflow is happening on Supply operations.
const EOverflow: u64 = 1;
/// For when trying to withdraw more than there is.
const ENotEnough: u64 = 2;
/// Sender is not @0x0 the system address.
//const ENotSystemAddress: u64 = 3;
/// System operation performed for a coin other than SUI
//const ENotSUI: u64 = 4;


/// A Supply of T. Used for minting and burning.
/// Wrapped into a `TreasuryCap` in the `Coin` module.
public struct Supply<phantom T> has store, drop {
    value: u64,
}

public enum Annoymous_Balance_Type has store, drop {

    BALANCE_TYPE_FHE,
    BALANCE_TYPE_SHARING,
}

/// Storable balance - an inner struct of a Coin type.
/// Can be use
/// d to store coins which don't need the key ability.
public struct Annoymos_Balance<phantom T> has store {
    balance_type:Annoymous_Balance_Type,
    value: u64,
    encode_data: vec_set::VecSet<u8>,
    version: u8,
}

public fun create_by_value<T>(value: u64) : Annoymos_Balance<T> {
    let encode_data = vec_set::empty<u8>();
    let balance_type = Annoymous_Balance_Type::BALANCE_TYPE_FHE;
    let version = 0;
    Annoymos_Balance { value: value,
        encode_data,
        balance_type: balance_type,
        version:version }
}
/// Get the amount stored in a `Balance`.
public fun value<T>(self: &Annoymos_Balance<T>): u64 {
    self.value
}

/// Get the `Supply` value.
public fun supply_value<T>(supply: &Supply<T>): u64 {
    supply.value
}

/// Create a new supply for type T.
public fun create_supply<T: drop>(_: T): Supply<T> {
    Supply { value: 0 }
}

/// Increase supply by `value` and create a new `Balance<T>` with this value.
public fun increase_supply<T>(self: &mut Supply<T>, value: u64): Annoymos_Balance<T> {
    assert!(value < (18446744073709551615u64 - self.value), EOverflow);
    self.value = self.value + value;
    create_by_value(value)
}

/// Burn a Balance<T> and decrease Supply<T>.
public fun decrease_supply<T>(self: &mut Supply<T>, balance: Annoymos_Balance<T>): u64 {
    let Annoymos_Balance {
        encode_data: _,
        version: _,
        balance_type: _,
        value } = balance;
    assert!(self.value >= value, EOverflow);
    self.value = self.value - value;
    value
}

/// Create a zero `Balance` for type `T`.
public fun zero<T>(): Annoymos_Balance<T> {
    create_by_value(0)
}

/// Join two balances together.
public fun join<T>(self: &mut Annoymos_Balance<T>, balance: Annoymos_Balance<T>): u64 {
    let Annoymos_Balance {
        encode_data: _,
        version: _,
        balance_type: _,
        value } = balance;
    self.value = self.value + value;
    self.value
}

/// Split a `Balance` and take a sub balance from it.
public fun split<T>(self: &mut Annoymos_Balance<T>, value: u64): Annoymos_Balance<T> {
    assert!(self.value >= value, ENotEnough);
    self.value = self.value - value;
    create_by_value(value)
}

/// Withdraw all balance. After this the remaining balance must be 0.
public fun withdraw_all<T>(self: &mut Annoymos_Balance<T>): Annoymos_Balance<T> {
    let value = self.value;
    split(self, value)
}

/// Destroy a zero `Balance`.
public fun destroy_zero<T>(balance: Annoymos_Balance<T>) {
    assert!(balance.value == 0, ENonZero);
    let Annoymos_Balance {
        encode_data: _,
        version: _,
        balance_type: _,
        value: _ } = balance;
}


/// Destroy a `Supply` preventing any further minting and burning.
public(package) fun destroy_supply<T>(self: Supply<T>): u64 {
    let Supply { value } = self;
    value
}

#[test_only]
/// Create a `Balance` of any coin for testing purposes.
public fun create_for_testing<T>(value: u64): Annoymos_Balance<T> {
    create_by_value(value)
}

#[test_only]
/// Destroy a `Balance` of any coin for testing purposes.
public fun destroy_for_testing<T>(self: Annoymos_Balance<T>): u64 {
    let Annoymos_Balance {
        encode_data: _,
        version: _,
        balance_type: _,
        value } = self;
    value
}

#[test_only]
/// Create a `Supply` of any coin for testing purposes.
public fun create_supply_for_testing<T>(): Supply<T> {
    Supply { value: 0 }
}
