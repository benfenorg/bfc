use crate::NativesCostTable;
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::{native_charge_gas_early_exit, native_functions::NativeContext};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};
use std::collections::VecDeque;
use move_core_types::gas_algebra::InternalGas;
use smallvec::smallvec;
#[allow(unused)]
const MATH_E: f64 = 2.718f64;
const Q64: u128 = 18446744073709551616;

#[derive(Clone, Debug)]
pub struct CurveDxCostParams {
    /// u128 and u256 are constant size, so base cost suffices
    pub curve_dx_cost_base: InternalGas
}

/***************************************************************************************************
 * native fun curve_dx
 * Implementation of the Move native function `curve::curve_dx(numerator: u128, denominator: u128): u128`
 *   gas cost:  curve_dx_cost_base                   | u128 is constant size, so base cost suffices
 **************************************************************************************************/
pub fn curve_dx(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 2, "invalid args");

    let curve_dx_cost_params = context
        .extensions_mut()
        .get::<NativesCostTable>()
        .curve_dx_cost_params
        .clone();
    // Charge base fee
    native_charge_gas_early_exit!(
        context,
        curve_dx_cost_params.curve_dx_cost_base
    );

    let denominator = pop_arg!(args, u128);
    let numerator = pop_arg!(args, u128);
    let cost = context.gas_used();
    let x: f64 = (numerator as f64) / (denominator as f64);

    //println!("{:?}", x);

    //let value = 5.0 * MATH_E.powf(-10.0f64 * x) * (Q64 as f64);
    let value = (0.6 * x + 1.0f64 / (x + 0.16) - 1.3) * (Q64 as f64);
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::u128(value as u128)]
    ))
}
