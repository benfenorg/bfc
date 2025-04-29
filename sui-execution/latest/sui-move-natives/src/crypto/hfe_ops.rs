

/***************************************************************************************************
 * native fun

 */
use std::collections::VecDeque;
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_charge_gas_early_exit;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::loaded_data::runtime_types::Type;
use move_vm_types::natives::function::NativeResult;
use move_vm_types::values::Value;
use smallvec::smallvec;
use sui_protocol_config::ProtocolConfigValue::u64;
use crate::NativesCostTable;

pub fn hfe_ops_add(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{

    //todo: design hfe ops cost table.
    // Load the cost parameters from the protocol config
    let ed25519_verify_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .ed25519_verify_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        ed25519_verify_cost_params.ed25519_ed25519_verify_cost_base
    );


    let cost = context.gas_used();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::bool(false)],
    ))

    //Err(_) => Ok(NativeResult::err(context.gas_used(), INVALID_INPUT)),
}