

/***************************************************************************************************
 * native fun

 */
use hmac::{Hmac, Mac};
use sha2::Sha256;
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_charge_gas_early_exit;
use move_vm_runtime::native_functions::NativeContext;
use smallvec::smallvec;
use crate::NativesCostTable;
use mpc_transmission::two_party_share::{mul_two_shared_secrets, sub_two_shared_secrets, recover_two_shares, recover_value, split_to_two_value, add_two_shared_secrets};

use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg,values::Value
};
use move_core_types::account_address::AccountAddress;

use std::collections::VecDeque;
use std::error::Error;
use std::time::Duration;
use move_core_types::gas_algebra::InternalGas;
use serde_json::{json};
use serde_json::Value as JsonValue;
use tracing::info;
use anyhow::anyhow;
use attohttpc::post;
//use fastcrypto::hash::HashFunction;
use mpc_transmission::get_user_address_salt;

#[derive(Clone)]
pub struct AnonymousComputeCostParams {
    /// Base cost for invoking the `anonymous compute` function
    pub anonymous_compute_cost_base: InternalGas,
}

#[allow(unused)]
type HmacSha256 = Hmac<Sha256>;
pub const ARITHMETIC_OVERFLOW_ERROR: u64 = 1;
pub const INVALID_PARAMS_ERROR:  u64 = 2;

pub const INVALID_INPUT_ERROR:  u64 = 3;

pub const NOT_FOUND_ANONYMOUS_RPC_ADDRESS: u64 = 4;

pub const INVALID_SERVER_RESPONSE_ERROR: u64 = 5;

//const THRESHOLD: usize = 2;
//const TOTAL_SHARES: usize = 2;
const MASK_SECRET: &str =  "0x1111ffff0000";


pub fn hfe_ops_add(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{

    let anonymous_compute_cost = &context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost.anonymous_compute_cost_base
    );

    let anonymous_privatekey = context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_privatekey
        .clone().unwrap_or_default();
    let enable_anonymous_rpc = &context
        .extensions()
        .get::<NativesCostTable>()?
        .enable_anonymous_rpc
        .clone();
    let anonymous_rpc = context.extensions().get::<NativesCostTable>()?.anonymous_rpc.clone();

    info!("anonymous_rpc{:?}", anonymous_rpc.clone());
    info!("enable-anonymous-rpc{:?}", enable_anonymous_rpc.clone());


    let owner = pop_arg!(args, AccountAddress);
    let number4 = pop_arg!(args, Vec<u8>);
    let number3 = pop_arg!(args, Vec<u8>);
    let number2 = pop_arg!(args, Vec<u8>);
    let number1 = pop_arg!(args, Vec<u8>);

    let num1 = String::from_utf8(number1).unwrap_or_default();
    let num2 = String::from_utf8(number2).unwrap_or_default();
    let num3 = String::from_utf8(number3).unwrap_or_default();
    let num4 = String::from_utf8(number4).unwrap_or_default();
    let cost = context.gas_used();
    if num1.is_empty() || num2.is_empty() || num3.is_empty() || num4.is_empty() {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT_ERROR,
        ));
    }

    if *enable_anonymous_rpc == Some(true) {
        info!("hfe_ops_add calculate in remote");
        match anonymous_rpc {
            Some(mut v) => {
                if v.is_empty() {
                   return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS));
                }
                let client = AnonymousClient::new( v.pop().unwrap_or_default().as_str());
                let result = client.add(num1, num2, num3, num4);
                match result.success {
                    true => {
                        Ok(NativeResult::ok(
                            cost,
                            smallvec![Value::vector_u8(result.value1.into_bytes()), Value::vector_u8(result.value2.into_bytes())],
                        ))
                    }
                    false => {
                        Ok(NativeResult::err(cost, INVALID_SERVER_RESPONSE_ERROR))
                    }
                }
            },
            None => return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS)),
        }
    } else {
        let mask = get_mask_secret_from_anonymous_privatekey(anonymous_privatekey)
            .unwrap_or(get_mask_secret_from_anonymous_privatekey(MASK_SECRET.to_string()).unwrap());

        info!("hfe_ops_add calculate in local");
        let value1_share = recover_two_shares(num1, num2, mask);
        let value2_share = recover_two_shares(num3, num4, mask);
        if value1_share.is_err() || value2_share.is_err() {
            return Ok(NativeResult::err(
                cost,
                INVALID_PARAMS_ERROR,
            ));
        }
        match add_two_shared_secrets(value1_share.unwrap(), value2_share.unwrap(), mask){
            Ok(result) => {
                let (result1, result2) = split_to_two_value(result, get_user_address_salt(owner), mask);
                Ok(NativeResult::ok(
                    cost,
                    smallvec![Value::vector_u8(result1.into_bytes()),Value::vector_u8(result2.into_bytes())]
                ))
            }
            Err(_e) => {
                Ok(NativeResult::err(
                    cost,
                    ARITHMETIC_OVERFLOW_ERROR,
                ))
            }
        }
    }
}

pub fn hfe_ops_minus(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{


    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
    );

    let anonymous_rpc = context.extensions().get::<NativesCostTable>()?.anonymous_rpc.clone();
    let anonymous_privatekey = context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_privatekey
        .clone().unwrap_or_default();
    let enable_anonymous_rpc = &context
        .extensions()
        .get::<NativesCostTable>()?
        .enable_anonymous_rpc
        .clone();
    let cost = context.gas_used();

    let owner = pop_arg!(args, AccountAddress);

    let number4 = pop_arg!(args, Vec<u8>);
    let number3 = pop_arg!(args, Vec<u8>);
    let number2 = pop_arg!(args, Vec<u8>);
    let number1 = pop_arg!(args, Vec<u8>);
    let num1 = String::from_utf8(number1).unwrap_or_default();
    let num2 = String::from_utf8(number2).unwrap_or_default();
    let num3 = String::from_utf8(number3).unwrap_or_default();
    let num4 = String::from_utf8(number4).unwrap_or_default();
    if num1.is_empty() || num2.is_empty() || num3.is_empty() || num4.is_empty() {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT_ERROR,
        ));
    }

    if *enable_anonymous_rpc == Some(true) {
        info!("hfe_ops_minus calculate in remote");
        match anonymous_rpc {
            Some(mut v) => {
                if v.is_empty() {
                    return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS));
                }
                let client = AnonymousClient::new( v.pop().unwrap_or_default().as_str());
                let result = client.minus(num1, num2, num3, num4);
                match result.success {
                    true => {
                        Ok(NativeResult::ok(
                            cost,
                            smallvec![Value::vector_u8(result.value1.into_bytes()), Value::vector_u8(result.value2.into_bytes())],
                        ))
                    }
                    false => {
                        Ok(NativeResult::err(cost, INVALID_SERVER_RESPONSE_ERROR))
                    }
                }
            },
            None => return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS)),
        }
    } else {
        let mask = get_mask_secret_from_anonymous_privatekey(anonymous_privatekey)
            .unwrap_or(get_mask_secret_from_anonymous_privatekey(MASK_SECRET.to_string()).unwrap());

        info!("hfe_ops_minus calculate in local");
        let value1_share = recover_two_shares(num1, num2, mask);
        let value2_share = recover_two_shares(num3, num4, mask);

        if value1_share.is_err() || value2_share.is_err() {
                return Ok(NativeResult::err(
                    cost,
                    INVALID_PARAMS_ERROR,
                ));
        }

        match sub_two_shared_secrets(
            value1_share.unwrap(),
            value2_share.unwrap(),
            mask
        ) {
            Ok(result) => {
                let (result1, result2) = split_to_two_value(result, get_user_address_salt(owner), mask);

                Ok(NativeResult::ok(
                    cost,
                    smallvec![Value::vector_u8(result1.into_bytes()),Value::vector_u8(result2.into_bytes())]
                ))
            }
            Err(_e) => {
               Ok(NativeResult::err(
                    cost,
                    ARITHMETIC_OVERFLOW_ERROR,
                ))
            }
        }
    }
}

pub fn hfe_ops_multiplied(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{

    //todo: add result overflow defense.

    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
    );

    let cost = context.gas_used();
    let anonymous_privatekey = context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_privatekey
        .clone().unwrap_or_default();
    let enable_anonymous_rpc = &context
        .extensions()
        .get::<NativesCostTable>()?
        .enable_anonymous_rpc
        .clone();
    let anonymous_rpc = context.extensions().get::<NativesCostTable>()?.anonymous_rpc.clone();

    let owner = pop_arg!(args, AccountAddress);

    let number4 = pop_arg!(args, Vec<u8>);
    let number3 = pop_arg!(args, Vec<u8>);
    let number2 = pop_arg!(args, Vec<u8>);
    let number1 = pop_arg!(args, Vec<u8>);
    let num1 = String::from_utf8(number1).unwrap_or_default();
    let num2 = String::from_utf8(number2).unwrap_or_default();
    let num3 = String::from_utf8(number3).unwrap_or_default();
    let num4 = String::from_utf8(number4).unwrap_or_default();
    if num1.is_empty() || num2.is_empty() || num3.is_empty() || num4.is_empty() {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT_ERROR,
        ));
    }

    if *enable_anonymous_rpc == Some(true) {
        info!("hfe_ops_minus calculate in remote");
        match anonymous_rpc {
            Some(mut v) => {
                if v.is_empty() {
                    return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS));
                }
                let client = AnonymousClient::new( v.pop().unwrap_or_default().as_str());
                let result = client.multiply(num1, num2, num3, num4);
                match result.success {
                    true => {
                        Ok(NativeResult::ok(
                            cost,
                            smallvec![Value::vector_u8(result.value1.into_bytes()), Value::vector_u8(result.value2.into_bytes())],
                        ))
                    }
                    false => {
                        Ok(NativeResult::err(cost, INVALID_SERVER_RESPONSE_ERROR))
                    }
                }
            },
            None => return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS)),
        }
    } else {
        info!("hfe_ops_minus calculate in local");
        let mask = get_mask_secret_from_anonymous_privatekey(anonymous_privatekey)
            .unwrap_or(get_mask_secret_from_anonymous_privatekey(MASK_SECRET.to_string()).unwrap());

        let value1_share =  recover_two_shares(num1, num2, mask);
        let value2_share =  recover_two_shares(num3, num4, mask);

        if value1_share.is_err() || value2_share.is_err() {
            return Ok(NativeResult::err(
                cost,
                INVALID_PARAMS_ERROR,
            ));
        }

        match mul_two_shared_secrets(value1_share.unwrap(), value2_share.unwrap(), mask) {
            Ok(result) => {
                let (result1, result2) = split_to_two_value(result,  get_user_address_salt(owner), mask);

                Ok(NativeResult::ok(
                    cost,
                    smallvec![Value::vector_u8(result1.into_bytes()),Value::vector_u8(result2.into_bytes())]
                ))
            }
            Err(_e) => {
                Ok(NativeResult::err(
                    cost,
                    ARITHMETIC_OVERFLOW_ERROR,
                ))
            }
        }
    }


}

pub fn hfe_ops_encode_data(context: &mut NativeContext,
                           _ty_args: Vec<Type>,
                           mut args: VecDeque<Value>) -> PartialVMResult<NativeResult> {

    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_compute_cost_params
        .clone();



    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
    );

    let owner = pop_arg!(args, AccountAddress);
    let value = pop_arg!(args, u64);
    let cost = context.gas_used();


    let anonymous_privatekey = context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_privatekey
        .clone().unwrap_or_default();
    let enable_anonymous_rpc = &context
        .extensions()
        .get::<NativesCostTable>()?
        .enable_anonymous_rpc
        .clone();
    let anonymous_rpc = context.extensions().get::<NativesCostTable>()?.anonymous_rpc.clone();

    if *enable_anonymous_rpc == Some(true) {
        match anonymous_rpc {
            Some(mut v) => {
                if v.is_empty() {
                    return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS));
                }
                let client = AnonymousClient::new( v.pop().unwrap_or_default().as_str());
                let result = client.encode_data(value);
                match result.success {
                    true => {
                        Ok(NativeResult::ok(
                            cost,
                            smallvec![Value::vector_u8(result.value1.into_bytes()), Value::vector_u8(result.value2.into_bytes())],
                        ))
                    }
                    false => {
                        Ok(NativeResult::err(cost, INVALID_SERVER_RESPONSE_ERROR))
                    }
                }
            },
            None => return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS)),
        }
    } else {
        let mask = get_mask_secret_from_anonymous_privatekey(anonymous_privatekey)
            .unwrap_or(get_mask_secret_from_anonymous_privatekey(MASK_SECRET.to_string()).unwrap());

        let (result1, result2) = split_to_two_value(value, get_user_address_salt(owner), mask);
        Ok(NativeResult::ok(
            cost,
            smallvec![Value::vector_u8(result1.into_bytes()), Value::vector_u8(result2.into_bytes())]
        ))
    }
}

pub fn hfe_ops_compare_value1_and_value2(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
    );


    let cost = context.gas_used();

    let anonymous_privatekey = context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_privatekey
        .clone().unwrap_or_default();
    let enable_anonymous_rpc = &context
        .extensions()
        .get::<NativesCostTable>()?
        .enable_anonymous_rpc
        .clone();
    let anonymous_rpc = context.extensions().get::<NativesCostTable>()?.anonymous_rpc.clone();


    let _owner = pop_arg!(args, AccountAddress);

    let number4 = pop_arg!(args, Vec<u8>);
    let number3 = pop_arg!(args, Vec<u8>);
    let number2 = pop_arg!(args, Vec<u8>);
    let number1 = pop_arg!(args, Vec<u8>);
    let num1 = String::from_utf8(number1).unwrap_or_default();
    let num2 = String::from_utf8(number2).unwrap_or_default();
    let num3 = String::from_utf8(number3).unwrap_or_default();
    let num4 = String::from_utf8(number4).unwrap_or_default();
    if num1.is_empty() || num2.is_empty() || num3.is_empty() || num4.is_empty(){
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT_ERROR,
        ));
    }
    if *enable_anonymous_rpc == Some(true) {
        match anonymous_rpc {
            Some(mut v) => {
                if v.is_empty() {
                    return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS));
                }
                let client = AnonymousClient::new( v.pop().unwrap_or_default().as_str());
                let result = client.compare_value1_and_value2(num1, num2, num3, num4);
                match result.success {
                    true => {
                        let result = result.value1.parse::<u8>();
                        match result {
                            Ok(result) => {
                                Ok(NativeResult::ok(
                                    cost,
                                    smallvec![Value::u8(result)],
                                ))
                            }
                            Err(_e) => {
                                Ok(NativeResult::err(cost, INVALID_SERVER_RESPONSE_ERROR))
                            }
                        }
                    }
                    false => {
                        Ok(NativeResult::err(cost, INVALID_SERVER_RESPONSE_ERROR))
                    }
                }
            },
            None => return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS)),
        }
    } else {
        let mask = get_mask_secret_from_anonymous_privatekey(anonymous_privatekey)
            .unwrap_or(get_mask_secret_from_anonymous_privatekey(MASK_SECRET.to_string()).unwrap());

        let new_number1 = recover_value(num1, num2, mask);
        let new_number2 = recover_value(num3, num4, mask);

        if new_number1.is_err() || new_number2.is_err() {
            return Ok(NativeResult::err(
                cost,
                ARITHMETIC_OVERFLOW_ERROR,
            ));
        }

        let value_a = new_number1.unwrap();
        let value_b = new_number2.unwrap();
        let comparison = if value_a > value_b {
            1
        } else if value_a < value_b {
            2
        } else {
            0
        };
        Ok(NativeResult::ok(
            cost,
            smallvec![Value::u8(comparison)],
        ))
    }
}

pub fn hfe_ops_compare_value(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
    );


    let cost = context.gas_used();

    let anonymous_privatekey = context
        .extensions()
        .get::<NativesCostTable>()?
        .anonymous_privatekey
        .clone().unwrap_or_default();
    let enable_anonymous_rpc = &context
        .extensions()
        .get::<NativesCostTable>()?
        .enable_anonymous_rpc
        .clone();

    let anonymous_rpc = context.extensions().get::<NativesCostTable>()?.anonymous_rpc.clone();

    let _owner = pop_arg!(args, AccountAddress);

    let number3 = pop_arg!(args, u64);

    let number2 = pop_arg!(args, Vec<u8>);
    let number1 = pop_arg!(args, Vec<u8>);
    let num1 = String::from_utf8(number1).unwrap_or_default();
    let num2 = String::from_utf8(number2).unwrap_or_default();

    if num1.is_empty() || num2.is_empty() {
        return Ok(NativeResult::err(
            cost,
            INVALID_INPUT_ERROR,
        ));
    }
    if *enable_anonymous_rpc == Some(true) {
        match anonymous_rpc {
            Some(mut v) => {
                if v.is_empty() {
                    return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS));
                }
                let client = AnonymousClient::new( v.pop().unwrap_or_default().as_str());
                let result = client.compare_value(num1, num2, number3);
                match result.success {
                    true => {
                        let result = result.value1.parse::<u8>();
                        match result {
                            Ok(result) => {
                                Ok(NativeResult::ok(
                                    cost,
                                    smallvec![Value::u8(result)],
                                ))
                            }
                            Err(_e) => {
                                Ok(NativeResult::err(cost, INVALID_SERVER_RESPONSE_ERROR))
                            }
                        }
                    }
                    false => {
                        Ok(NativeResult::err(cost, INVALID_SERVER_RESPONSE_ERROR))
                    }
                }
            },
            None => return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS)),
        }
    } else {
        let mask = get_mask_secret_from_anonymous_privatekey(anonymous_privatekey)
            .unwrap_or(get_mask_secret_from_anonymous_privatekey(MASK_SECRET.to_string()).unwrap());

        match recover_value(num1, num2, mask) {
            Ok(value_a) => {
                let value_b = number3;
                let comparison = if value_a > value_b {
                    1
                } else if value_a < value_b {
                    2
                } else {
                    0
                };
                Ok(NativeResult::ok(
                    cost,
                    smallvec![Value::u8(comparison)],
                ))
            }
            Err(_e) => {
                Ok(NativeResult::err(
                    cost,
                    ARITHMETIC_OVERFLOW_ERROR,
                ))
            }
        }
    }
}

// pub fn hfe_ops_restore_value(context: &mut NativeContext,
//                              _ty_args: Vec<Type>,
//                               mut args: VecDeque<Value>) -> PartialVMResult<NativeResult> {
//
//     let anonymous_compute_cost_params = &context
//         .extensions()
//         .get::<NativesCostTable>()
//         .anonymous_compute_cost_params
//         .clone();
//
//
//
//     // Charge the base cost for this oper
//     native_charge_gas_early_exit!(
//         context,
//         anonymous_compute_cost_params.anonymous_compute_cost_base
//     );
//
//     let owner = pop_arg!(args, AccountAddress);
//
//     let publickey = pop_arg!(args, Vec<u8>);
//     let id = pop_arg!(args, AccountAddress);
//     let signature= pop_arg!(args, Vec<u8>);
//     let number2 = pop_arg!(args, Vec<u8>);
//     let number1 = pop_arg!(args, Vec<u8>);
//
//     let cost = context.gas_used();
//
//
//     let anonymous_privatekey = context
//         .extensions()
//         .get::<NativesCostTable>()
//         .anonymous_privatekey
//         .clone().unwrap_or_default();
//     let enable_anonymous_rpc = &context
//         .extensions()
//         .get::<NativesCostTable>()
//         .enable_anonymous_rpc
//         .clone();
//     let anonymous_rpc = context.extensions().get::<NativesCostTable>().anonymous_rpc.clone();
//
//     let num1 = String::from_utf8(number1).unwrap_or_default();
//     let num2 = String::from_utf8(number2).unwrap_or_default();
//     if num1.is_empty() || num2.is_empty() {
//         return Ok(NativeResult::err(
//             cost,
//             INVALID_INPUT_ERROR,
//         ));
//     }
//
//     if *enable_anonymous_rpc == Some(true) {
//         match anonymous_rpc {
//             Some(mut v) => {
//                 if v.is_empty() {
//                     return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS));
//                 }
//                 let client = AnonymousClient::new( v.pop().unwrap_or_default().as_str());
//                 let result = client.restore_value(num1, num2, signature, id, publickey);
//                 match result.success {
//                     true => {
//                         Ok(NativeResult::ok(
//                             cost,
//                             smallvec![Value::u64(result.value1.parse::<u64>().expect("Failed to parse number"))],
//                         ))
//                     }
//                     false => {
//                         Ok(NativeResult::err(cost, INVALID_SERVER_RESPONSE_ERROR))
//                     }
//                 }
//             },
//             None => return Ok(NativeResult::err(cost, NOT_FOUND_ANONYMOUS_RPC_ADDRESS)),
//         }
//     } else {
//         let mask = get_mask_secret_from_anonymous_privatekey(anonymous_privatekey, owner)
//             .unwrap_or(get_mask_secret_from_anonymous_privatekey(MASK_SECRET.to_string(), owner).unwrap());
//
//         match recover_value(num1, num2, mask) {
//             Ok(value) => {
//                 info!("hfe_ops_restore_value calculate in local restore");
//
//                 Ok(NativeResult::ok(
//                     cost,
//                     smallvec![Value::u64(value)],
//                 ))
//             }
//             Err(_e) => {
//                 Ok(NativeResult::err(
//                     cost,
//                     ARITHMETIC_OVERFLOW_ERROR,
//                 ))
//             }
//         }
//     }
// }

#[allow(unused)]
fn derive_shard_key(shard_id: usize, key: Vec<u8>) -> Vec<u8> {
    let mut hmac = HmacSha256::new_from_slice(&key)
        .expect("HMAC can take key of any size");
    hmac.update(&shard_id.to_be_bytes());
    hmac.finalize().into_bytes().to_vec()
}

#[derive(Debug)]
#[allow(unused)]
struct AnonymousResult {
    success: bool,
    error: Option<String>,
    value1: String,
    value2: String,
}

struct AnonymousClient {
    base_url: String,
}

impl AnonymousClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub fn add(&self, value1: String, value2: String, value3: String, value4: String) -> AnonymousResult {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4
        });

        match self.atto_http_post("bfcx_getAnonymousAdd", params, 1) {

            Ok(response) => {
                let result1 = response["result"]["result1"].as_str().unwrap();
                let result2 = response["result"]["result2"].as_str().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1.to_string(),
                    value2: result2.to_string(),
                }
            }
            Err(e) => {
                AnonymousResult {
                    success: false,
                    error: Some(e.to_string()),
                    value1: "0".to_string(),
                    value2: "0".to_string(),
                }
            },
        }
    }

    pub fn minus(&self, value1: String, value2: String, value3: String, value4: String) -> AnonymousResult {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self.atto_http_post("bfcx_getAnonymousMinus", params, 2) {
            Ok(response) => {
                let result1 = response["result"]["result1"].as_str().unwrap();
                let result2 = response["result"]["result2"].as_str().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1.to_string(),
                    value2: result2.to_string(),
                }
            },
            Err(e) => AnonymousResult {
                success: false,
                error: Some(e.to_string()),
                value1: "0".to_string(),
                value2: "0".to_string(),
            },
        }
    }

    // pub fn restore_value(&self, value1: String, value2: String, signature : Vec<u8>, id: AccountAddress, publickey: Vec<u8>) -> AnonymousResult  {
    //     let params = json!({
    //         "value1": value1,
    //         "value2": value2,
    //         "signature": signature,
    //         "objectid" : id.to_hex_literal(),
    //         "publickey": publickey,
    //     });
    //
    //     match self.atto_http_post("bfcx_getAnonymousRestoreValue", params, 3) {
    //         Ok(response) => {
    //
    //             let result1 = response["result"]["result1"].as_u64().unwrap();
    //             AnonymousResult {
    //                 success: true,
    //                 error: None,
    //                 value1: result1.to_string(),
    //                 value2: "0".to_string(),
    //             }
    //         },
    //         Err(e) => AnonymousResult {
    //             success: false,
    //             error: Some(e.to_string()),
    //             value1: "0".to_string(),
    //             value2: "0".to_string(),
    //         },
    //     }
    // }

    pub fn restore_anonymous_value_internal(&self, value1: String, value2: String, owner: AccountAddress) -> AnonymousResult  {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "owner": owner,
        });

        match self.atto_http_post("bfcx_getAnonymousRestoreValueInternal", params, 3) {
            Ok(response) => {

                let result1 = response["result"]["result1"].as_u64().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1.to_string(),
                    value2: "0".to_string(),
                }
            },
            Err(e) => AnonymousResult {
                success: false,
                error: Some(e.to_string()),
                value1: "0".to_string(),
                value2: "0".to_string(),
            },
        }
    }

    pub fn encode_data(&self, value1: u64) -> AnonymousResult  {
        let params = json!({
            "value": value1,
        });

        match self.atto_http_post("bfcx_getAnonymousEncodeData", params, 3) {
            Ok(response) => {

                let result1 = response["result"]["result1"].as_str().unwrap();
                let result2 = response["result"]["result2"].as_str().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1.to_string(),
                    value2: result2.to_string()
                }
            },
            Err(e) => AnonymousResult {
                success: false,
                error: Some(e.to_string()),
                value1: "0".to_string(),
                value2: "0".to_string(),
            },
        }
    }

    pub fn compare_value(&self, value1: String, value2: String, value3: u64) -> AnonymousResult  {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
        });

        match self.atto_http_post("bfcx_getAnonymousCompare", params, 3) {
            Ok(response) => {
                let result1 = response["result"]["result1"].as_str().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1.to_string(),
                    value2: "0".to_string(),
                }
            },
            Err(e) => AnonymousResult {
                success: false,
                error: Some(e.to_string()),
                value1: "0".to_string(),
                value2: "0".to_string(),
            },
        }
    }

    pub fn compare_value1_and_value2(&self, value1: String, value2: String, value3: String, value4: String) -> AnonymousResult  {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,

        });

        match self.atto_http_post("bfcx_getAnonymousCompareValue1AndValue2", params, 3) {
            Ok(response) => {
                let result1 = response["result"]["result1"].as_str().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1.to_string(),
                    value2: "0".to_string(),
                }
            },
            Err(e) => AnonymousResult {
                success: false,
                error: Some(e.to_string()),
                value1: "0".to_string(),
                value2: "0".to_string(),
            },
        }
    }

    pub fn multiply(&self, value1: String, value2: String, value3: String, value4: String) -> AnonymousResult  {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self.atto_http_post("bfcx_getAnonymousMultiply", params, 3) {
            Ok(response) => {

                let result1 = response["result"]["result1"].as_str().unwrap();
                let result2 = response["result"]["result2"].as_str().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1.to_string(),
                    value2: result2.to_string()
                }
            },
            Err(e) => AnonymousResult {
                success: false,
                error: Some(e.to_string()),
                value1: "0".to_string(),
                value2: "0".to_string(),
            },
        }
    }

    pub fn atto_http_post(
        &self,
        method: &str,
        params: JsonValue,
        id: u64,
    ) -> Result<JsonValue, Box<dyn Error>> {
        let data = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id
        });

        let resp = post(&format!("{}/rpc", self.base_url)).timeout(Duration::from_secs(10))
            .json(&data)?
            .send();
        match resp {
            Ok(resp) => {
                let response_json: serde_json::Value = resp.json()?;
                println!("Server responded with: {}", response_json);
                Ok(response_json)
            },
            Err(e) => {
                eprintln!("POST request failed with status: {:?},", e.to_string());
                Err(anyhow!("post request failed").into())
            }
        }
    }
}

#[allow(unused)]
fn test_get_anonymous_add() -> (){
    let client = AnonymousClient::new("http://localhost:9010");
    let params: JsonValue = json!({
            "value1": 2,
            "value2": 3
        });
    let response = client.atto_http_post("bfcx_getAnonymousAdd", params.clone(), 1);
    match response {
        Ok(value) => {
            println!("Response: {}", value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    let response = client.atto_http_post("bfcx_getAnonymousMinus", params.clone(), 1);
    match response {
        Ok(value) => {
            println!("Response: {}", value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    let response = client.atto_http_post("bfcx_getAnonymousMultiplied", params.clone(), 1);
    match response {
        Ok(value) => {
            println!("Response: {}", value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    let response = client.atto_http_post("bfcx_getAnonymousCompare", params, 1);
    match response {
        Ok(value) => {
            println!("Response: {}", value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub fn get_mask_secret_from_anonymous_privatekey(private_key_str: String) -> Result<u64, Box<dyn std::error::Error>> {
    // Parse the private key string to u64
    // Handle both hex format (0x...) and decimal format
    let mask_secret = if private_key_str.starts_with("0x") || private_key_str.starts_with("0X") {
        // Parse as hexadecimal
        u64::from_str_radix(&private_key_str[2..], 16)
            .map_err(|e| anyhow!("Failed to parse private key as hex: {}", e))?
    } else {
        // Parse as decimal
        private_key_str.parse::<u64>()
            .map_err(|e| anyhow!("Failed to parse private key as decimal: {}", e))?
    };

    Ok(mask_secret)
}


#[test]
pub fn get_account_address(){
    let address = AccountAddress::from_hex_literal("0x847998d7a801abb749e5698865f4d2b1f82a0453c4fc31962878b5e5e4c3406e").unwrap();
   let num = get_user_address_salt(address);
    println!("num:{}", num);

    assert_eq!(num, 2835591981614551602);
}