/// Module: anonymous_vault
module anonymous_vault::anonymous_vault;

use sui::clock::{Self, Clock};
use sui::dynamic_field;
use sui::anonymous_coin::{Anonymous_Coin,join};
use sui::bag::{Self, Bag};
use std::string::{Self};
use sui::anonymous_coin::TreasuryCap;
use sui::package::UpgradeCap;
use sui::event;



const ENOT_ADMIN: u64 = 0;
const ADMIN_ALREADY_EXISTS: u64 = 1;
const ADMIN_REACH_MAX: u64 = 2;
const NOT_INIT_ADMIN_STATUS: u64 = 3;
const ADMIN_ALREADY_VOTED: u64 = 4;
const EObjectNotFound: u64 = 6;
const EObjectAlreadyExists: u64 = 7;
const EACTION_STATUE :u64 = 8;
const EACTION_OVERTIME :u64 = 9;

const DEFAULT_ACTION_PERIOD: u64     = 1000 * 60 * 60  * 24 * 7; // 7 days || 7 hour for test


const ADMIN_MAX_COUNT: u64 = 5;
const THRESHOLD_FOR_ACTION: u64 = 3;


const ACTION_STATUS_PENDING: u8 = 0;
const ACTION_STATUS_COMPLETED: u8 = 1;


const ACTION_TYPE_ADD_ADMIN: u8 = 0;
const ACTION_TYPE_REMOVE_ADMIN: u8 = 1;
const ACTION_TYPE_WITHDRAW_TREASURY_CAP_OBJECT: u8 = 2;
const ACTION_TYPE_WITHDRAW_UPGRADE_CAP_OBJECT: u8 = 3;

const ACTION_TYPE_WITHDRAW_TOKEN1: u8 = 4;
const ACTION_TYPE_WITHDRAW_TOKEN2: u8 = 5;

const VAULT_TOKEN_KEY : vector<u8> = b"anonymous_vault_token_key";

public struct AnonymousVault has key, store {
    id: UID,
    latest_action_num: u64,
    admins: vector<address>,
    can_init_admin_status: bool,
    actions: vector<VaultAction>,

    dynamic_keys: vector<string::String>,

    //1. use dynamic field to store objects: upgradeCap, treasuryCap, etc
    //2. use Bag to store tokens [token don't have key and store ability]
    vault_token_pool: Bag,
}

public fun get_latest_action_num(vault: &AnonymousVault): u64 {
    vault.latest_action_num
}

public struct VaultTokenPool<phantom T1, phantom T2> has store, key {
    id: UID,
    anonymous_coin_1: Anonymous_Coin<T1>,
    anonymous_coin_2: Anonymous_Coin<T2>
}



public struct VaultCreateEvent has copy, drop, store {
    creator: address
}
public struct InitTokenPoolEvent has copy, drop, store {
    creator: address
}
public struct InitAdminEvent has copy, drop, store {
    creator: address
}
public struct DepositToken1Event has copy, drop, store {
    creator: address,
}
public struct DepositToken2Event has copy, drop, store {
    creator: address,
}

public struct DepositUpgradeCapEvent has copy, drop, store {
    creator: address,
    object_key: string::String,
}
public struct DepositTreasuryCapEvent has copy, drop, store {
    creator: address,
    object_key: string::String,
}


public struct CreateActionEvent has copy, drop, store {
    creator: address,
    action_type: u8,
    action_index: u64,
    action_receipt: address,
    action_key: string::String,
}

public struct AdminVoteForActionEvent has copy, drop, store {
    creator: address,
    action_index: u64,
}

public struct WithdrawUpgradeCapEvent has copy, drop, store {
    creator: address,
    object_key: string::String,
}

public struct WithdrawTreasuryCapEvent has copy, drop, store {
    creator: address,
    object_key: string::String,
}

public struct WithdrawToken1Event has copy, drop, store {
    receiver: address,
}
public struct AddAdminEvent has copy, drop, store {
    new_admin: address,
}

public struct RemoveAdminEvent has copy, drop, store {
    removed_admin: address,
}


fun init(ctx: &mut TxContext) {
    let vault = create_anonymous_vault(ctx);
    transfer::share_object(vault);
}


entry fun init_token_pool<T1, T2>(anonymous_coin_1: Anonymous_Coin<T1>,
                                    anonymous_coin_2: Anonymous_Coin<T2>,
                                    vault: &mut AnonymousVault,
                                    ctx: &mut TxContext) {
    //use bag or table to store pool
    let pool = VaultTokenPool {
        id: object::new(ctx),
        anonymous_coin_1,
        anonymous_coin_2,
    };
    let vault_token_key = string::utf8(VAULT_TOKEN_KEY);
    bag::add<string::String, VaultTokenPool<T1, T2>>(&mut vault.vault_token_pool, vault_token_key, pool);

    event::emit(InitTokenPoolEvent {
        creator: tx_context::sender(ctx),
    });

}

entry fun init_admin(admin: address, vault: &mut AnonymousVault) {

    assert!(vault.can_init_admin_status == true, NOT_INIT_ADMIN_STATUS);

    assert!(vector_contains(&vault.admins, &admin) == false, ADMIN_ALREADY_EXISTS);
    assert!(vector::length(&vault.admins) <= ADMIN_MAX_COUNT, ADMIN_REACH_MAX);

    vector::push_back(&mut vault.admins, admin);
    if (vector::length(&vault.admins) == ADMIN_MAX_COUNT) {
        vault.can_init_admin_status = false;
    };

    event::emit(InitAdminEvent {
        creator: admin,
    });
}


entry fun deposit_token1_to_vault_pool<T1, T2>( anonymous_coin: Anonymous_Coin<T1>,
                                                vault: &mut AnonymousVault,
                                                 ctx: &mut TxContext){
    let vault_key = string::utf8(VAULT_TOKEN_KEY);
    let vault_token_pool = bag::borrow_mut<string::String, VaultTokenPool<T1, T2>>(&mut vault.vault_token_pool, vault_key);
    join(&mut vault_token_pool.anonymous_coin_1, anonymous_coin, ctx);

    event::emit(DepositToken1Event {
        creator: tx_context::sender(ctx),
    });
}


entry fun deposit_token2_to_vault_pool<T1, T2>( anonymous_coin: Anonymous_Coin<T2>,
                                                vault: &mut AnonymousVault,
                                                ctx: &mut TxContext){
    let vault_key = string::utf8(VAULT_TOKEN_KEY);
    let vault_token_pool = bag::borrow_mut<string::String, VaultTokenPool<T1, T2>>(&mut vault.vault_token_pool, vault_key);

    join(&mut vault_token_pool.anonymous_coin_2, anonymous_coin, ctx);

    event::emit(DepositToken2Event {
        creator: tx_context::sender(ctx),
    });
}

entry fun deposit_contract_upgrade_cap_to_vault( obj: UpgradeCap, object_key: string::String,  vault: &mut AnonymousVault, ctx: &TxContext){
    //transfer object to sub vault for objects
    let sender = tx_context::sender(ctx);
    assert!(vector_contains(&vault.admins, &sender), ENOT_ADMIN);


    assert!(!dynamic_field::exists_(&vault.id, object_key), EObjectAlreadyExists);
    dynamic_field::add(&mut vault.id, object_key, obj);
    vector::push_back(&mut vault.dynamic_keys,object_key);
    event::emit(DepositUpgradeCapEvent {
        creator: sender,
        object_key,
    });
}


entry fun deposit_treasury_cap_to_vault<T>( obj: TreasuryCap<T>, object_key: string::String,  vault: &mut AnonymousVault, ctx: &TxContext){
    //transfer object to sub vault for objects
    let sender = tx_context::sender(ctx);
    assert!(vector_contains(&vault.admins, &sender), ENOT_ADMIN);

    assert!(!dynamic_field::exists_(&vault.id, object_key), EObjectAlreadyExists);
    dynamic_field::add(&mut vault.id, object_key, obj);
    vector::push_back(&mut vault.dynamic_keys,object_key);

    event::emit(DepositTreasuryCapEvent {
        creator: sender,
        object_key,
    });
}

fun withdraw_upgrade_cap_object_to_receiver(
    object_key: string::String,
    vault: &mut AnonymousVault,
    receiver: address,
){
    assert!(vector_contains(&vault.admins, &receiver), ENOT_ADMIN);
    assert!(dynamic_field::exists_(&vault.id, object_key), EObjectNotFound);

    let  obj: UpgradeCap = dynamic_field::remove(&mut vault.id, object_key);
    let index = find_index_string(&vault.dynamic_keys, object_key);
    vector::remove(&mut vault.dynamic_keys, index);

    transfer::public_transfer(obj, receiver);
    event::emit(WithdrawUpgradeCapEvent {
        creator: receiver,
        object_key,
    });
}

fun withdraw_treasure_cap_object_to_receiver<T>(
                                    object_key: string::String,
                                    vault: &mut AnonymousVault,
                                    receiver: address,
                                    ){

    assert!(vector_contains(&vault.admins, &receiver), ENOT_ADMIN);
    assert!(dynamic_field::exists_(&vault.id, object_key), EObjectNotFound);

    let  obj: TreasuryCap<T> = dynamic_field::remove(&mut vault.id, object_key);
    let index = find_index_string(&vault.dynamic_keys, object_key);
    vector::remove(&mut vault.dynamic_keys, index);
    transfer::public_transfer(obj, receiver);
    event::emit(WithdrawTreasuryCapEvent {
        creator: receiver,
        object_key,
    });
}

entry fun admin_vote_for_action<T,T1, T2,>(action: &mut VaultAction, vault:& mut AnonymousVault, clock: &Clock, ctx: & TxContext){

    let sender = tx_context::sender(ctx);
    let current_time = clock::timestamp_ms(clock);

    assert!(vector_contains(&vault.admins, &sender), ENOT_ADMIN);
    assert!(action.action_status == ACTION_STATUS_PENDING, EACTION_STATUE);
    assert!(vector_contains(&action.approve_admins, &sender) == false, ADMIN_ALREADY_VOTED);
    assert!(current_time - action.start_time < DEFAULT_ACTION_PERIOD, EACTION_OVERTIME);

    vector::push_back(&mut action.approve_admins, tx_context::sender(ctx));
    if(vector::length(&action.approve_admins) >= THRESHOLD_FOR_ACTION){
        do_action_if_reach_threshold<T, T1, T2>(action, vault);
    };

    event::emit(AdminVoteForActionEvent {
        creator: sender,
        action_index: action.action_index,
    });
}

fun remove_admin(admin: address, vault:& mut AnonymousVault){
    assert!(vector_contains(&vault.admins, &admin), ENOT_ADMIN);
    let index = find_index_address(&vault.admins, admin);

    vector::remove(&mut vault.admins, index);

    //in case we remove too many admins, allow to init again
    if(vector::length(&vault.admins) < THRESHOLD_FOR_ACTION){
        vault.can_init_admin_status = true;
    };
    event::emit(RemoveAdminEvent {
        removed_admin: admin,
    });
}
fun add_admin(admin: address, vault:& mut AnonymousVault){
    assert!(vector_contains(&vault.admins, &admin), ADMIN_ALREADY_EXISTS);
    assert!(vector::length(&vault.admins) < ADMIN_MAX_COUNT, ADMIN_REACH_MAX);
    vector::push_back(&mut vault.admins, admin);
    event::emit(AddAdminEvent {
        new_admin: admin,
    });
}


fun do_action_if_reach_threshold<T, T1, T2>(action: &mut VaultAction, vault:& mut AnonymousVault){
    let receiver = action.action_receipt;
    if(action.action_type == ACTION_TYPE_ADD_ADMIN){
        add_admin(receiver, vault);
    }
    else if(action.action_type == ACTION_TYPE_REMOVE_ADMIN){
        remove_admin(receiver, vault);
    }
    else if(action.action_type == ACTION_TYPE_WITHDRAW_TREASURY_CAP_OBJECT){
        withdraw_treasure_cap_object_to_receiver<T>(action.action_key, vault, receiver);
    }
    else if(action.action_type == ACTION_TYPE_WITHDRAW_UPGRADE_CAP_OBJECT){
        withdraw_upgrade_cap_object_to_receiver(action.action_key, vault, receiver);
    }
    else if(action.action_type == ACTION_TYPE_WITHDRAW_TOKEN1){
        withdraw_token1<T1, T2>(vault,receiver);
    }
    else if(action.action_type == ACTION_TYPE_WITHDRAW_TOKEN2){
        //withdraw_token2<T1, T2>(vault,receiver, ctx);
    };

    action.action_status = ACTION_STATUS_COMPLETED;

}

public fun create_anonymous_vault(ctx: &mut TxContext): AnonymousVault {

    let vault = AnonymousVault {
        id: object::new(ctx),
        latest_action_num: 0,
        admins: vector::empty<address>(),
        can_init_admin_status: true,
        actions: vector::empty<VaultAction>(),
        vault_token_pool: bag::new(ctx),
        dynamic_keys: vector::empty<string::String>(),
    };
    event::emit(VaultCreateEvent {
        creator: tx_context::sender(ctx),
    });

    vault

}

fun withdraw_token1<T1, T2>( vault: &mut AnonymousVault,
                    receiver: address,
                    ){
    //only admin receive tokens
    assert!(vector_contains(&vault.admins, &receiver), ENOT_ADMIN);

    let vault_key = string::utf8(VAULT_TOKEN_KEY);
    let vault_token_pool = bag::remove<string::String, VaultTokenPool<T1, T2>>(&mut vault.vault_token_pool, vault_key);
    let VaultTokenPool {
                        anonymous_coin_1: token1,
                        anonymous_coin_2: token2, id} = vault_token_pool;

    transfer::public_transfer(token1, receiver);
    transfer::public_transfer(token2, receiver);
    object::delete(id);
    event::emit(WithdrawToken1Event {
        receiver: receiver,
    });
    //re-add poo

}

//fun withdraw_token2<T1, T2>(vault: &mut AnonymousVault,
//                    receiver: address,
//                    ctx: & TxContext) {
//    //only admin receive tokens
//    assert!(vector_contains(&vault.admins, &receiver), ENOT_ADMIN);
//    let vault_key = string::utf8(VAULT_TOKEN_KEY);
//    let vault_token_pool = bag::borrow_mut<string::String, VaultTokenPool<T1, T2>>(&mut vault.vault_token_pool, vault_key);
//
//}

public struct VaultAction has key, store {
    id: UID,
    action_type: u8,
    action_index: u64,
    start_time: u64,
    approve_admins: vector<address>,
    action_status: u8,
    action_receipt: address,
    action_key: string::String,

}


entry fun create_action(action_type: u8,
                        action_address: address,
                        action_key: string::String,
                        vault: &mut AnonymousVault,
                        clock: &Clock, ctx: &mut TxContext) {
    let action = VaultAction {
        id: object::new(ctx),
        action_type: action_type,
        action_index: vault.latest_action_num,
        start_time: clock::timestamp_ms(clock),
        approve_admins: vector::empty<address>(),
        action_status: ACTION_STATUS_PENDING,
        action_receipt: action_address,
        action_key: action_key,

    };
    //vector::push_back(&mut vault.actions, action);
    vault.latest_action_num = vault.latest_action_num + 1;

    event::emit(CreateActionEvent {
        creator: tx_context::sender(ctx),
        action_type,
        action_index: action.action_index,
        action_receipt: action_address,
        action_key,
    });
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
fun find_index_string(vec: &vector<string::String>, key_string: string::String): u64 {
    let mut i = 0;
    let len = vector::length(vec);

    while (i < len) {
        if (*vector::borrow(vec, i) == key_string) {
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




#[test_only]
public fun new_for_testing(ctx: &mut TxContext): AnonymousVault{
    create_anonymous_vault(ctx)
}

#[test_only]
public fun create_testing_action(action_type: u8,
                          action_address: address,
                            action_key: string::String,
                            vault: &mut AnonymousVault,
                            clock: &Clock, ctx: &mut TxContext): VaultAction {
    let action = VaultAction {
        id: object::new(ctx),
        action_type: action_type,
        action_index: vault.latest_action_num,
        start_time: clock::timestamp_ms(clock),
        approve_admins: vector::empty<address>(),
        action_status: ACTION_STATUS_PENDING,
        action_receipt: action_address,
        action_key: action_key,

    };
    //vector::push_back(&mut vault.actions, action);
    vault.latest_action_num = vault.latest_action_num + 1;

    action
}



