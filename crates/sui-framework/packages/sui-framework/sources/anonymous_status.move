// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// This module provides functionality for anonymous.
module sui::anonymous;

//use sui::object;
use sui::versioned::{Self, Versioned};

// Sender is not @0x0 the system address.
const ENotSystemAddress: u64 = 0;
const EWrongInnerVersion: u64 = 1;
//const EInvalidUpdate: u64 = 2;
//const EInvalidRange: u64 = 3;
//const EInvalidLength: u64 = 4;

const CURRENT_VERSION: u64 = 1;
//const U16_MAX: u64 = 0xFFFF;



/// Singleton shared object which stores the global Anonymous state.
/// The actual state is stored in a versioned inner field.
public struct Anonymous has key {
    id: UID,
    inner: Versioned,
}

public struct AnonymousInner has store {
    version: u64,
    epoch: u64,
    value: u64,
}

#[allow(unused_function)]
/// Create and share the Anonymous object. This function is called exactly once, when
/// the Anonymous object is first created.
/// Can only be called by genesis or change_epoch transactions.
fun create(ctx: &mut TxContext) {
    assert!(ctx.sender() == @0x0, ENotSystemAddress);

    let version = CURRENT_VERSION;

    let inner = AnonymousInner {
        version,
        epoch: ctx.epoch(),
        value: 0,
    };

    let self = Anonymous {
        //id: object::anonymous_state_id(),
        id: object::new(ctx),
        inner: versioned::create(version, inner, ctx),
    };
    transfer::share_object(self);
}



#[test_only]
public fun create_for_testing(ctx: &mut TxContext) {
    let version = CURRENT_VERSION;

    let inner = AnonymousInner {
        version,
        epoch: ctx.epoch(),
        value: 0,
    };

    let self = Anonymous {
        id: object::new(ctx),
        inner: versioned::create(version, inner, ctx),
    };
    transfer::share_object(self);
}

#[test_only]
public fun destroy_for_testing(r: Anonymous) {
    let Anonymous {id, inner} = r;
    object::delete(id);
    let AnonymousInner {
        version: _,
        epoch: _,
        value: _,
    } = versioned::destroy(inner);
}

fun load_inner_mut(self: &mut Anonymous): &mut AnonymousInner {
    let version = versioned::version(&self.inner);

    // Replace this with a lazy update function when we add a new version of the inner object.
    assert!(version == CURRENT_VERSION, EWrongInnerVersion);
    let inner: &mut AnonymousInner = versioned::load_value_mut(&mut self.inner);
    assert!(inner.version == version, EWrongInnerVersion);
    inner
}
#[allow(unused_function)]
fun load_inner(self: &Anonymous): &AnonymousInner {
    let version = versioned::version(&self.inner);

    // Replace this with a lazy update function when we add a new version of the inner object.
    assert!(version == CURRENT_VERSION, EWrongInnerVersion);
    let inner: &AnonymousInner = versioned::load_value(&self.inner);
    assert!(inner.version == version, EWrongInnerVersion);
    inner
}


#[allow(unused_function)]
fun update_annoymous_state(
    self: &mut Anonymous,
    new_value: u64,
    ctx: &TxContext,
) {
    // Validator will make a special system call with sender set as 0x0.
    assert!(ctx.sender() == @0x0, ENotSystemAddress);

    let inner = self.load_inner_mut();


    inner.epoch = ctx.epoch();
    inner.value = new_value;
}

#[test_only]
public fun update_anonymous_state_for_testing(
    self: &mut Anonymous,
    new_value: u64,
    ctx: &TxContext,
) {
    self.update_annoymous_state(new_value, ctx);
}

