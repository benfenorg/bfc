/// Module: anonymous_vault
module anonymous_vault::anonymous_vault;

use sui::vec_map::{Self, VecMap};
use std::ascii;
use sui::clock::{Self, Clock};
use sui::dynamic_field;

const ENOT_ADMIN: u64 = 0;
const ADMIN_ALREADY_EXISTS: u64 = 1;
const ADMIN_REACH_MAX: u64 = 2;
const NOT_INIT_ADMIN_STATUS: u64 = 3;
const ADMIN_ALREADY_VOTED: u64 = 4;
const MAP_KEY_IS_EXIST: u64 = 5;
const EObjectNotFound: u64 = 6;
const EObjectAlreadyExists: u64 = 7;



const ADMIN_MAX_COUNT: u64 = 5;
const THRESHOLD_FOR_ACTION: u64 = 3;


const ACTION_STATUS_PENDING: u8 = 0;
const ACTION_STATUS_COMPLETED: u8 = 1;


const ACTION_TYPE_ADD_ADMIN: u8 = 0;
const ACTION_TYPE_REMOVE_ADMIN: u8 = 1;
const ACTION_TYPE_WITHDRAW_OBJECT: u8 = 2;



public struct AnonymousVault has key, store {
    id: UID,
    latest_action_num: u64,
    admins: vector<address>,
    can_init_admin_status: bool,
    actions: vector<VaultAction>,



}

fun init(ctx: &mut TxContext) {
    let vault = create_anonymous_vault(ctx);

    transfer::share_object(vault);

}

entry fun init_admin(admin: address, vault: &mut AnonymousVault) {

    assert!(vault.can_init_admin_status == false, NOT_INIT_ADMIN_STATUS);

    assert!(vector_contains(&vault.admins, &admin) , ADMIN_ALREADY_EXISTS);
    assert!(vector::length(&vault.admins) < ADMIN_MAX_COUNT, ADMIN_REACH_MAX);

    vector::push_back(&mut vault.admins, admin);
    if (vector::length(&vault.admins) == ADMIN_MAX_COUNT) {
        vault.can_init_admin_status = false;
    }
}


entry fun deposit_object_to_vault<T: key + store>( obj: T, object_key: ascii::String,  vault: &mut AnonymousVault, ctx: &mut TxContext){
    //transfer object to sub vault for objects
    let sender = tx_context::sender(ctx);
    assert!(vector_contains(&vault.admins, &sender), ENOT_ADMIN);
    let obj_id = object::id(&obj);

    // 检查对象是否已存在
    assert!(!dynamic_field::exists_(&vault.id, object_key), EObjectAlreadyExists);
    // 存入对象
    dynamic_field::add(&mut vault.id, object_key, obj);
}


fun withdraw_object_to_sender<T: key + store>(obj_id: ID, object_key: ascii::String,  vault: &mut AnonymousVault, ctx: &mut TxContext){
    let sender = tx_context::sender(ctx);
    assert!(vector_contains(&vault.admins, &sender), ENOT_ADMIN);
    assert!(dynamic_field::exists_(&vault.id, object_key), EObjectNotFound);

    let  obj: T = dynamic_field::remove(&mut vault.id, object_key);
    transfer::public_transfer(obj, sender);

}

entry fun admin_vote_for_action(action: &mut VaultAction, vault:& mut AnonymousVault, ctx: &mut TxContext){

    let sender = tx_context::sender(ctx);
    assert!(vector_contains(&vault.admins, &sender), ENOT_ADMIN);
    assert!(vector_contains(&action.approve_admins, &sender), ADMIN_ALREADY_VOTED);

    vector::push_back(&mut action.approve_admins, tx_context::sender(ctx));
    if(vector::length(&action.approve_admins) >= THRESHOLD_FOR_ACTION){
        do_action_if_reach_threshold(action, vault);
    }

}

fun remove_admin(admin: address, vault:& mut AnonymousVault){
    assert!(vector_contains(&vault.admins, &admin), ENOT_ADMIN);
    let index = find_index_address(&vault.admins, admin);

    vector::remove(&mut vault.admins, index);
}
fun add_admin(admin: address, vault:& mut AnonymousVault){
    assert!(vector_contains(&vault.admins, &admin), ADMIN_ALREADY_EXISTS);
    assert!(vector::length(&vault.admins) < ADMIN_MAX_COUNT, ADMIN_REACH_MAX);
    vector::push_back(&mut vault.admins, admin);
}


fun do_action_if_reach_threshold(action: &mut VaultAction, vault:& mut AnonymousVault){

    if(action.action_type == ACTION_TYPE_ADD_ADMIN){
        add_admin(action.action_receipt, vault);
    }
    else if(action.action_type == ACTION_TYPE_REMOVE_ADMIN){
        remove_admin(action.action_receipt, vault);
    }
    else if(action.action_type == ACTION_TYPE_WITHDRAW_OBJECT){

    };

    action.action_status = ACTION_STATUS_COMPLETED;

}

public fun create_anonymous_vault(ctx: &mut TxContext): AnonymousVault {
    AnonymousVault {
        id: object::new(ctx),
        latest_action_num: 0,
        admins: vector::empty<address>(),
        can_init_admin_status: true,
        actions: vector::empty<VaultAction>(),
    }
}



public struct VaultAction has key, store {
    id: UID,
    action_type: u8,
    action_index: u64,
    start_time: u64,
    approve_admins: vector<address>,
    action_status: u8,
    action_receipt: address,

}


entry fun create_action(action_type: u8, vault: &mut AnonymousVault, clock: &Clock, ctx: &mut TxContext) {
    let action = VaultAction {
        id: object::new(ctx),
        action_type: action_type,
        action_index: vault.latest_action_num,
        start_time: clock::timestamp_ms(clock),
        approve_admins: vector::empty<address>(),
        action_status: ACTION_STATUS_PENDING,
        action_receipt: tx_context::sender(ctx),
    };
    //vector::push_back(&mut vault.actions, action);
    vault.latest_action_num = vault.latest_action_num + 1;

    transfer::share_object(action);
}


fun find_index_address(vec: &vector<address>, addr: address): u64 {
    let mut i = 0;
    let len = vector::length(vec);

    while (i < len) {
        if (*vector::borrow(vec, i) == addr) {
            return i
        };
        i = i + 1;
    };
    len
}
fun vector_contains(vec: &vector<address>, addr: &address): bool {
    let mut i = 0;
    let len = vector::length(vec);

    while (i < len) {
        if (*vector::borrow(vec, i) == *addr) {
            return true
        };
        i = i + 1;
    };
    false
}