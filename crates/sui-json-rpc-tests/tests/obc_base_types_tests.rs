// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use hex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_sample() {
        println!("Hello, world!");

        let prefix = "OB";
        let evm_address = "0x99ec891ff6602457efc2c5086c8926f4fe78cebc02a79a55485a6c56aca2b572";

        let result = convert_to_obaddress(prefix, evm_address);

        println!("the ob convert result is {}", result);

        ////===============

        let evm_address = convert_to_evm_address(&result);
        println!("the evm convert result is {}", evm_address);
    }
}

fn sha256(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let result = hasher.finish();

    format!("{:x}", result)
}

//OB99ec891ff6602457efc2c5086c8926f4fe78cebc02a79a55485a6c56aca2b5723735
fn convert_to_obaddress(prefix: &str, evm_address: &str) -> String {
    let address = evm_address.to_string();

    let result = sha256(address.as_str());
    let mut hex = hex::encode(result);
    hex.truncate(4);

    let mut address = prefix.to_string();
    address.push_str(&evm_address[2..]);
    address.push_str(hex.as_str());

    return address;
}

// 0x99ec891ff6602457efc2c5086c8926f4fe78cebc02a79a55485a6c56aca2b572
fn convert_to_evm_address(ob_address: &str) -> String {
    let mut address = ob_address[2..].to_string();
    let evm_prefix = String::from("0x");
    address.insert_str(0, evm_prefix.as_str());
    address.truncate(address.len() - 4);

    let result = sha256(address.as_str());
    let mut hex = hex::encode(result);
    hex.truncate(4);

    let verify_code = ob_address[ob_address.len() - 4..].to_string();
    assert_eq!(verify_code, hex);

    return address.to_string();
}
