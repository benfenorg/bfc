

/***************************************************************************************************
 * native fun

 */
use std::collections::VecDeque;
use rand_chacha::ChaChaRng;
use rand::Rng;
use rand::SeedableRng;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_charge_gas_early_exit;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::loaded_data::runtime_types::Type;
use move_vm_types::natives::function::NativeResult;
use move_vm_types::pop_arg;
use move_vm_types::values::Value;
use smallvec::smallvec;
use crate::NativesCostTable;
use move_vm_types::{
    values::{VectorRef},
};


type HmacSha256 = Hmac<Sha256>;
pub fn hfe_ops_add(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
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

    let number4 = pop_arg!(args, u64);
    let number3 = pop_arg!(args, u64);
    let number2 = pop_arg!(args, u64);
    let number1 = pop_arg!(args, u64);


    let data1 = number1 + number2;
    let data2 = number3 + number4;

    let result = data1 + data2;
    let result1 = result / 2;
    let result2 = result - result1;

    let cost = context.gas_used();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::u64(result)]
    ))

    //Err(_) => Ok(NativeResult::err(context.gas_used(), INVALID_INPUT)),
}

pub fn hfe_ops_minus(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
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

    let number4 = pop_arg!(args, u64);
    let number3 = pop_arg!(args, u64);
    let number2 = pop_arg!(args, u64);
    let number1 = pop_arg!(args, u64);

    let data1 = number1 + number2;
    let data2 = number3 + number4;
    let result = data1 - data2;


    let result1 = result / 2;
    let result2 = result - result1;

    let cost = context.gas_used();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::u64(result)]
    ))

}
pub fn hfe_ops_multiplied(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
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

    let number4 = pop_arg!(args, u64);
    let number3 = pop_arg!(args, u64);
    let number2 = pop_arg!(args, u64);
    let number1 = pop_arg!(args, u64);


    let data1 = number1 + number2;
    let data2 = number3 + number4;
    let result = data1 * data2;

    let result1 = result / 2;
    let result2 = result - result1;


    let cost = context.gas_used();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::u64(result1)]
    ))
}


pub fn hfe_ops_compare(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
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


    let number4 = pop_arg!(args, u64);
    let number3 = pop_arg!(args, u64);
    let number2 = pop_arg!(args, u64);
    let number1 = pop_arg!(args, u64);


    let data1 = number1 + number2;
    let data2 = number3 + number4;
    let mut result = 0;
    if data1 > data2 {
        result = 1;
    }
    if data1 < data2 {
        result = 2;
    }

    let cost = context.gas_used();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::u8(result)],
    ))
}




pub fn split_data(context: &mut NativeContext,
                  ty_args: Vec<Type>,
                  mut args: VecDeque<Value>) -> PartialVMResult<NativeResult> {

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

    let index = pop_arg!(args, u8);
    let number = pop_arg!(args, u8);
    let msg = pop_arg!(args, VectorRef);
    let bytes = msg.as_bytes_ref().to_vec();
    let ans = split_data_to_shard(&*bytes, number as usize);
    let cost = context.gas_used();

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u8(ans[index as usize].to_vec())],
    ))
    // Ok(NativeResult::ok(
    //     cost,
    //     smallvec![Value::u8(number2 + number3)],
    // ))
}

pub fn split_data_to_shard(data: &[u8], n: usize) -> Vec<Vec<u8>> {
    assert!(n >= 2, "n must be at least 2");
    assert!(!data.is_empty(), "data cannot be empty");
    let mut seed = [0u8; 32];
    // todo get key from private key
    let mut key = [0u8; 32];
    let key_hash = Sha256::digest(&key);
    seed.copy_from_slice(&key_hash);
    let mut rng = ChaChaRng::from_seed(seed);

    // 1. 生成n-1个确定性随机分片
    let mut shards: Vec<Vec<u8>> = (0..n-1)
        .map(|_| {
            rng.clone().sample_iter(rand::distributions::Standard)
                .take(data.len())
                .collect()
        })
        .collect();

    // 2. 计算最后一个分片：原始数据 XOR 所有随机分片 XOR 所有分片密钥
    let last_shard = shards.iter()
        .enumerate()
        .fold(data.to_vec(), |acc, (i, shard)| {
            let shard_key = derive_shard_key(i, key.into());
            acc.iter()
                .zip(shard.iter())
                .zip(shard_key.iter().cycle())
                .map(|((&a, &b), &k)| a ^ b ^ k)
                .collect()
        });

    // 最后一个分片还需要与自己的分片密钥XOR
    let last_shard_key = derive_shard_key(n-1, key.into());
    let last_shard = last_shard.iter()
        .zip(last_shard_key.iter().cycle())
        .map(|(&a, &k)| a ^ k)
        .collect();

    shards.push(last_shard);
    shards
}

fn derive_shard_key(shard_id: usize, key: Vec<u8>) -> Vec<u8> {
    let mut hmac = HmacSha256::new_from_slice(&key)
        .expect("HMAC can take key of any size");
    hmac.update(&shard_id.to_be_bytes());
    hmac.finalize().into_bytes().to_vec()
}