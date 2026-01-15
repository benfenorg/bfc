

/***************************************************************************************************
 * native fun

 */
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_functions::NativeContext;
use crate::crypto::hfe_ops_v2;
use crate::crypto::hfe_ops_v1;
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, values::Value
};
use std::collections::VecDeque;
use crate::NativesCostTable;

pub fn hfe_ops_add(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
    let enable_anonymous_version_v2 = context
        .extensions()
        .get::<NativesCostTable>()
        .enable_anonymous_version_v2
        .clone();

    if enable_anonymous_version_v2 {
        hfe_ops_v2::hfe_ops_add_v2(context, _ty_args, args)
    } else {
        hfe_ops_v1::hfe_ops_add_v1(context, _ty_args, args)
    }
}

pub fn hfe_ops_minus(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
    let enable_anonymous_version_v2 = context
        .extensions()
        .get::<NativesCostTable>()
        .enable_anonymous_version_v2
        .clone();

    if enable_anonymous_version_v2 {
        hfe_ops_v2::hfe_ops_minus_v2(context, _ty_args, args)
    } else {
        hfe_ops_v1::hfe_ops_minus_v1(context, _ty_args, args)
    }
}

pub fn hfe_ops_multiplied(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
    let enable_anonymous_version_v2 = context
        .extensions()
        .get::<NativesCostTable>()
        .enable_anonymous_version_v2
        .clone();

    if enable_anonymous_version_v2 {
        hfe_ops_v2::hfe_ops_multiplied_v2(context, _ty_args, args)
    } else {
        hfe_ops_v1::hfe_ops_multiplied_v1(context, _ty_args, args)
    }
}

pub fn hfe_ops_encode_data(context: &mut NativeContext,
                           _ty_args: Vec<Type>,
                           args: VecDeque<Value>) -> PartialVMResult<NativeResult> {
    let enable_anonymous_version_v2 = context
        .extensions()
        .get::<NativesCostTable>()
        .enable_anonymous_version_v2
        .clone();

    if enable_anonymous_version_v2 {
        hfe_ops_v2::hfe_ops_encode_data_v2(context, _ty_args, args)
    } else {
        hfe_ops_v1::hfe_ops_encode_data_v1(context, _ty_args, args)
    }
}

pub fn hfe_ops_compare_value1_and_value2(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
    let enable_anonymous_version_v2 = context
        .extensions()
        .get::<NativesCostTable>()
        .enable_anonymous_version_v2
        .clone();

    if enable_anonymous_version_v2 {
        hfe_ops_v2::hfe_ops_compare_value1_and_value2_v2(context, _ty_args, args)
    } else {
        hfe_ops_v1::hfe_ops_compare_value1_and_value2_v1(context, _ty_args, args)
    }
}

pub fn hfe_ops_compare_value(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
    let enable_anonymous_version_v2 = context
        .extensions()
        .get::<NativesCostTable>()
        .enable_anonymous_version_v2
        .clone();

    if enable_anonymous_version_v2 {
        hfe_ops_v2::hfe_ops_compare_value_v2(context, _ty_args, args)
    } else {
        hfe_ops_v1::hfe_ops_compare_value_v1(context, _ty_args, args)
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
