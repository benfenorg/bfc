// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// A storable handler for Balances in general. Is used in the `Coin`
/// module to allow balance operations and can be used to implement
/// custom coins with `Supply` and `Balance`s.
module sui::anonymous_balance;
use std::string::{Self, String};
use sui::hfe_ops::{hfe_ops_add, hfe_ops_minus, hfe_ops_split_value, hfe_ops_restore_value, hfe_ops_compare_value};

/// Allows calling `.into_coin()` on a `Balance` to turn it into a coin.
public use fun sui::anonymous_coin::from_balance as Anonymous_Balance.into_coin;

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

public enum Anonymous_Balance_Type has store, drop {

    BALANCE_TYPE_FHE,
    BALANCE_TYPE_SHARING,
}

/// Storable balance - an inner struct of a Coin type.
/// Can be use
/// d to store coins which don't need the key ability.
public struct Anonymous_Balance<phantom T> has store {
    balance_type: Anonymous_Balance_Type,
    value1: u64,
    value2: u64,
    encode_data: String,
    version: u8,
}


public fun get_anonymous_value<T>(self: &Anonymous_Balance<T>, signatures: vector<u8>, id: address, publickey: vector<u8>): u64 {
    //todo : use hfe_ops to get the value from value1 and value2
    hfe_ops_restore_value(self.value1, self.value2, signatures, id, publickey)
}

 public fun convert_to_string(mut value: u64): vector<u8> {
        if (value == 0) {
            return string::utf8(b"0").into_bytes()
        };
        let mut buffer = vector::empty<u8>();
        while (value != 0) {
            vector::push_back(&mut buffer, ((48 + value % 10) as u8));
            value = value / 10;
        };
        vector::reverse(&mut buffer);
        string::utf8(buffer).into_bytes()
    }



public fun create_by_value<T>(value: u64) : Anonymous_Balance<T> {
    let mut encode_data = string::utf8(b"");
    let balance_type = Anonymous_Balance_Type::BALANCE_TYPE_FHE;
    let version = 0;

    //todo: use hfe_ops to split the value into two parts
    // let value1 = value/2;
    // let value2 = value - value1;

    let result =  hfe_ops_split_value(value);
    let value1 = result[0];
    let value2 = result[1];

    string::append_utf8(&mut encode_data, convert_to_string(value1));
    string::append_utf8(&mut encode_data, b",");
    string::append_utf8(&mut encode_data, convert_to_string(value2));



    Anonymous_Balance {
        value1: value1,
        value2: value2,
        encode_data,
        balance_type: balance_type,
        version:version }
}
/// Get the amount stored in a `Balance`.
public fun value<T>(self: &Anonymous_Balance<T>, signatures: vector<u8>, id: address, publickey: vector<u8>): u64 {
    self.get_anonymous_value(signatures, id, publickey)
}

public fun value1<T>(self: &Anonymous_Balance<T>): u64 {
    self.value1
}

public fun value2<T>(self: &Anonymous_Balance<T>): u64 {
    self.value2
}

public fun get_encode_data<T>(self: &Anonymous_Balance<T>): String {
    self.encode_data
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
public fun increase_supply<T>(self: &mut Supply<T>, value: u64): Anonymous_Balance<T> {
    assert!(value < (18446744073709551615u64 - self.value), EOverflow);
    self.value = self.value + value;
    create_by_value(value)
}

/// Burn a Balance<T> and decrease Supply<T>.
public fun decrease_supply<T>(self: &mut Supply<T>, balance: Anonymous_Balance<T>, signatures: vector<u8>, id: address, publickey: vector<u8>): u64 {
    let Anonymous_Balance {
        encode_data: _,
        version: _,
        balance_type: _,
        value1: value1,
        value2: value2} = balance;
    let value = hfe_ops_restore_value(value1, value2, signatures, id, publickey);
    assert!(self.value >= value, EOverflow);
    self.value = self.value - value;
    value
}

/// Create a zero `Balance` for type `T`.
public fun zero<T>(): Anonymous_Balance<T> {
    create_by_value(0)
}

fun update_encode_data<T>(self: &mut Anonymous_Balance<T>) {
    // Update the encode data based on the current value1 and value2
    let mut encode_data = string::utf8(b"");
    string::append_utf8(&mut encode_data, convert_to_string(self.value1));
    string::append_utf8(&mut encode_data, b",");
    string::append_utf8(&mut encode_data, convert_to_string(self.value2));
    self.encode_data = encode_data;
}
/// Join two balances together.
public fun join<T>(self: &mut Anonymous_Balance<T>, balance: Anonymous_Balance<T>): (u64, u64) {
    let Anonymous_Balance {
        encode_data: _,
        version: _,
        balance_type: _,
        value1: value1,
        value2: value2} = balance;

    let result =  hfe_ops_add(self.value1, self.value2, value1, value2);
    self.value1 = result[0];
    self.value2 = result[1];


    self.update_encode_data();
    (self.value1,self.value2)
}

/// Split a `Balance` and take a sub balance from it.
public fun split<T>(self: &mut Anonymous_Balance<T>, value: u64): Anonymous_Balance<T> {
    // todo add compare_value
    let compare_result = hfe_ops_compare_value(self.value1, self.value2, value);
    assert!(compare_result >= 0, ENotEnough);
    let value3 = value/2;
    let value4 = value - value3;
    let result = hfe_ops_minus(self.value1, self.value2, value3, value4);
    self.value1 = result[0];
    self.value2 = result[1];

    self.update_encode_data();

    create_by_value(value)
}

/// Withdraw all balance. After this the remaining balance must be 0.
// public fun withdraw_all<T>(self: &mut Anonymous_Balance<T>): Anonymous_Balance<T> {
//     let value = self.value;
//     split(self, value)
// }

/// Destroy a zero `Balance`.
public fun destroy_zero<T>(balance: Anonymous_Balance<T>) {
    assert!(balance.value1 == 0, ENonZero);
    assert!(balance.value2 == 0, ENonZero);
    let Anonymous_Balance {
        encode_data: _,
        version: _,
        balance_type: _,
        value1: _,
        value2: _,
        } = balance;
}


/// Destroy a `Supply` preventing any further minting and burning.
public(package) fun destroy_supply<T>(self: Supply<T>): u64 {
    let Supply { value } = self;
    value
}

#[test_only]
/// Create a `Balance` of any coin for testing purposes.
public fun create_for_testing<T>(value: u64): Anonymous_Balance<T> {
    create_by_value(value)
}

#[test_only]
/// Destroy a `Balance` of any coin for testing purposes.
public fun destroy_for_testing<T>(self: Anonymous_Balance<T>): u64 {
    let Anonymous_Balance {
        encode_data: _,
        version: _,
        balance_type: _,
        value1: _,
        value2: _,
        value } = self;
    value
}

#[test_only]
/// Create a `Supply` of any coin for testing purposes.
public fun create_supply_for_testing<T>(): Supply<T> {
    Supply { value: 0 }
}
