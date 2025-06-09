

/***************************************************************************************************
 * native fun

 */
use rand_chacha::ChaChaRng;
use rand::Rng;
use rand::SeedableRng;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_charge_gas_early_exit;
use move_vm_runtime::native_functions::NativeContext;
use smallvec::smallvec;
use crate::NativesCostTable;
use move_vm_types::{
    values::{VectorRef},
};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg,values::Value
};

use std::collections::VecDeque;
use std::error::Error;
use move_core_types::gas_algebra::InternalGas;
use serde_json::{json};
use serde_json::Value as JsonValue;
use tracing::info;

#[derive(Clone)]
pub struct AnonymousComputeCostParams {
    /// Base cost for invoking the `anonymous compute` function
    pub anonymous_compute_cost_base: InternalGas,
}

type HmacSha256 = Hmac<Sha256>;
pub const ARITHMETIC_OVERFLOW_ERROR: u64 = 1;



pub fn hfe_ops_add(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{

    //todo: add result overflow defense.

    let anonymous_compute_cost = &context
        .extensions()
        .get::<NativesCostTable>()
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost.anonymous_compute_cost_base
    );

    let anonymous_privatekey = &context
        .extensions()
        .get::<NativesCostTable>()
        .anonymous_privatekey
        .clone();
    let enable_anonymous_rpc = &context
        .extensions()
        .get::<NativesCostTable>()
        .enable_anonymous_rpc
        .clone();
    let anonymous_rpc = context.extensions().get::<NativesCostTable>().anonymous_rpc.clone();
    info!("anonymous_privatekey{:?}", anonymous_privatekey.clone().unwrap());
    info!("anonymous_rpc{:?}", anonymous_rpc.clone());
    info!("enable-anonymous-rpc{:?}", enable_anonymous_rpc.clone().unwrap());
    let number4 = pop_arg!(args, u64);
    let number3 = pop_arg!(args, u64);
    let number2 = pop_arg!(args, u64);
    let number1 = pop_arg!(args, u64);
    let cost = context.gas_used();
    if *enable_anonymous_rpc == Some(true) {
        let client = AnonymousClient::new(anonymous_rpc.unwrap().pop().unwrap().as_str());
        let result = client.add(number1, number2, number3, number4);
        Ok(NativeResult::ok(
            cost,
            smallvec![Value::vector_u64(vec![result.value1, result.value2])]
        ))
    } else {
        let data1 = number1 + number2;
        let data2 = number3 + number4;

        if data1.checked_add(data2) == None {
            return Ok(NativeResult::err(
                cost,
                ARITHMETIC_OVERFLOW_ERROR,
            ));
        }
        let result = data1 + data2;
        let result1 = result / 2;
        let result2 = result - result1;
        Ok(NativeResult::ok(
            cost,
            smallvec![Value::vector_u64(vec![result1, result2])]
        ))
    }
}

pub fn hfe_ops_minus(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{

    ////todo: add result overflow defense.


    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
    );
    let number4 = pop_arg!(args, u64);
    let number3 = pop_arg!(args, u64);
    let number2 = pop_arg!(args, u64);
    let number1 = pop_arg!(args, u64);

    let anonymous_rpc = context.extensions().get::<NativesCostTable>().anonymous_rpc.clone();
    let anonymous_privatekey = &context
        .extensions()
        .get::<NativesCostTable>()
        .anonymous_privatekey
        .clone();
    let enable_anonymous_rpc = &context
        .extensions()
        .get::<NativesCostTable>()
        .enable_anonymous_rpc
        .clone();
    let cost = context.gas_used();

    if *enable_anonymous_rpc == Some(true) {
        let client = AnonymousClient::new(anonymous_rpc.unwrap().pop().unwrap().as_str());
        let result = client.minus(number1, number2, number3, number4);
        Ok(NativeResult::ok(
            cost,
            smallvec![Value::vector_u64(vec![result.value1, result.value2])]
        ))
    } else {
        let data1 = number1 + number2;
        let data2 = number3 + number4;

        if data1.checked_sub(data2) == None {
            return Ok(NativeResult::err(
                cost,
                ARITHMETIC_OVERFLOW_ERROR,
            ));
        }
        let result = data1 - data2;


        let result1 = result / 2;
        let result2 = result - result1;
        Ok(NativeResult::ok(
            cost,
            smallvec![Value::vector_u64(vec![result1, result2])]
        ))
    }
}
pub fn hfe_ops_multiplied(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{

    //todo: add result overflow defense.

    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
    );

    let number4 = pop_arg!(args, u64);
    let number3 = pop_arg!(args, u64);
    let number2 = pop_arg!(args, u64);
    let number1 = pop_arg!(args, u64);

    let cost = context.gas_used();
    let anonymous_privatekey = &context
        .extensions()
        .get::<NativesCostTable>()
        .anonymous_privatekey
        .clone();
    let enable_anonymous_rpc = &context
        .extensions()
        .get::<NativesCostTable>()
        .enable_anonymous_rpc
        .clone();
    let anonymous_rpc = context.extensions().get::<NativesCostTable>().anonymous_rpc.clone();
    if *enable_anonymous_rpc == Some(true) {
        let client = AnonymousClient::new(anonymous_rpc.unwrap().pop().unwrap().as_str());
        let result = client.multiply(number1, number2, number3, number4);
        Ok(NativeResult::ok(
            cost,
            smallvec![Value::vector_u64(vec![result.value1, result.value2])]
        ))
    } else {
        let data1 = number1 + number2;
        let data2 = number3 + number4;

        if data1.checked_mul(data2) == None {
            return Ok(NativeResult::err(
                cost,
                ARITHMETIC_OVERFLOW_ERROR,
            ));
        }

        let result = data1 * data2;

        let result1 = result / 2;
        let result2 = result - result1;
        Ok(NativeResult::ok(
            cost,
            smallvec![Value::vector_u64(vec![result1, result2])]
        ))
    }


}


pub fn hfe_ops_compare(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult>{
    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
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

pub fn split_value(context: &mut NativeContext,
                  ty_args: Vec<Type>,
                  mut args: VecDeque<Value>) -> PartialVMResult<NativeResult> {


    //todo: add result overflow defense.


    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
    );

    let value = pop_arg!(args, u64);

    let value1 = value/2;
    let value2 = value - value1;
    let cost = context.gas_used();

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u64(vec![value1, value2])],
    ))
}

pub fn split_data(context: &mut NativeContext,
                  ty_args: Vec<Type>,
                  mut args: VecDeque<Value>) -> PartialVMResult<NativeResult> {

    let anonymous_compute_cost_params = &context
        .extensions()
        .get::<NativesCostTable>()
        .anonymous_compute_cost_params
        .clone();
    // Charge the base cost for this oper
    native_charge_gas_early_exit!(
        context,
        anonymous_compute_cost_params.anonymous_compute_cost_base
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

    let mut shards: Vec<Vec<u8>> = (0..n-1)
        .map(|_| {
            rng.clone().sample_iter(rand::distributions::Standard)
                .take(data.len())
                .collect()
        })
        .collect();

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

#[derive(Debug)]
struct AnonymousResult {
    success: bool,
    error: Option<String>,
    value1: u64,
    value2: u64,
}

struct AnonymousClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl AnonymousClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn add(&self, value1: u64, value2: u64, value3: u64, value4: u64) -> AnonymousResult {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4
        });

        match self.send_rpc_request("bfcx_getAnonymousAdd", params, 1) {

            Ok(response) => {
                let result1 = response["result"]["result1"].as_u64().unwrap();
                let result2 = response["result"]["result2"].as_u64().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1,
                    value2: result2,
                }
            }
            Err(e) => {
                AnonymousResult {
                    success: false,
                    error: Some(e.to_string()),
                    value1: 0,
                    value2: 0,
                }
            },
        }
    }

    pub fn minus(&self, value1: u64, value2: u64, value3: u64, value4: u64) -> AnonymousResult {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self.send_rpc_request("bfcx_getAnonymousMinus", params, 2) {
            Ok(response) => {
                let result1 = response["result"]["result1"].as_u64().unwrap();
                let result2 = response["result"]["result2"].as_u64().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1,
                    value2: result2,
                }
            },
            Err(e) => AnonymousResult {
                success: false,
                error: Some(e.to_string()),
                value1: 0,
                value2: 0,
            },
        }
    }

    pub fn multiply(&self, value1: u64, value2: u64, value3: u64, value4: u64) -> AnonymousResult  {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self.send_rpc_request("bfcx_getAnonymousMultiply", params, 3) {
            Ok(response) => {

                let result1 = response["result"]["result1"].as_u64().unwrap();
                let result2 = response["result"]["result2"].as_u64().unwrap();
                AnonymousResult {
                    success: true,
                    error: None,
                    value1: result1,
                    value2: result2
                }
            },
            Err(e) => AnonymousResult {
                success: false,
                error: Some(e.to_string()),
                value1: 0,
                value2: 0,
            },
        }
    }

    fn send_rpc_request(
        &self,
        method: &str,
        params: JsonValue,
        id: u64,
    ) -> Result<JsonValue, Box<dyn Error>> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id
        });

        let response = self
            .client
            .post(&format!("{}/rpc", self.base_url))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()?;

        let response_text = response.text()?;
        let response_json: JsonValue = serde_json::from_str(&response_text)?;
        Ok(response_json)
    }
}

fn test_get_anonymous_add() -> (){
    let client = AnonymousClient::new("http://localhost:9010");
    let params: JsonValue = json!({
            "value1": 2,
            "value2": 3
        });
    let response = client.send_rpc_request("bfcx_getAnonymousAdd", params.clone(), 1);
    match response {
        Ok(value) => {
            println!("Response: {}", value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    let response = client.send_rpc_request("bfcx_getAnonymousMinus", params.clone(), 1);
    match response {
        Ok(value) => {
            println!("Response: {}", value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    let response = client.send_rpc_request("bfcx_getAnonymousMultiplied", params.clone(), 1);
    match response {
        Ok(value) => {
            println!("Response: {}", value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    let response = client.send_rpc_request("bfcx_getAnonymousCompare", params, 1);
    match response {
        Ok(value) => {
            println!("Response: {}", value);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}