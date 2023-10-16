// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use hex;
use sha2::{Digest, Sha256};

const SAMPLE_ADDRESS: &str = "af306e86c74e937552df132b41a6cb3af58559f5342c6e82a98f7d1f7a4a9f30";
const SAMPLE_ADDRESS_VEC: [u8; 32] = [
    175, 48, 110, 134, 199, 78, 147, 117, 82, 223, 19, 43, 65, 166, 203, 58, 245, 133, 89, 245, 52,
    44, 110, 130, 169, 143, 125, 31, 122, 74, 159, 48,
];
#[cfg(test)]
mod tests {
    //use crate::base_types_obc::obc_address_util::convert_to_obc_address;
    use super::*;

    #[test]
    fn test_convert_util() {
        let result = obc_address_util::convert_to_evm_address(String::from(
            "OBCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768579ede",
        ));
        println!("the evm convert result is {}", result);

        let sui_address = obc_address_util::convert_to_obc_address(
            "BFC",
            "0xd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b76857",
        );
        println!("the obc convert result is {}", sui_address);
    }

    #[test]
    fn test_convert_sample() {
        let prefix = "BFC";
        let evm_address = "0xd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b76857";

        let obc_address = local_convert_to_obaddress(prefix, evm_address);

        //OBCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768579ede
        println!("the ob convert result is {}", obc_address);

        ////===============

        //let evm_address = convert_to_evm_address(result);
        //println!("the evm convert result is {}", evm_address);
    }

    #[test]
    fn test_convert_sample2() {
        ////===============
        let result = "OBCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768579ede";
        let evm_address = local_convert_to_evm_address(result.to_string());
        println!("the evm convert result is {}", evm_address);
    }
    //0xd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b76857

    #[test]
    fn test_sha256_string() {
        let object_id_vec = SAMPLE_ADDRESS_VEC.to_vec();
        let object_id = ObjectID::try_from(object_id_vec.clone()).unwrap();
        let json_serialized = serde_json::to_string(&object_id).unwrap();
        let bcs_serialized = bcs::to_bytes(&object_id).unwrap();

        let expected_json_address = format!("\"0x{}\"", SAMPLE_ADDRESS);
        let check_sum = get_check_sum(SAMPLE_ADDRESS.to_string());
        let expected_json_address_obce = format!("\"OBC{}{}\"", SAMPLE_ADDRESS, check_sum);

        assert_eq!(
            expected_json_address == json_serialized
                || expected_json_address_obce == json_serialized,
            true
        );
        assert_eq!(object_id_vec, bcs_serialized);
    }

    #[test]
    fn test_object_id_serde_with_expected_value() {
        let object_id_vec = SAMPLE_ADDRESS_VEC.to_vec();
        let object_id = ObjectID::try_from(object_id_vec.clone()).unwrap();
        let json_serialized = serde_json::to_string(&object_id).unwrap();
        let bcs_serialized = bcs::to_bytes(&object_id).unwrap();

        let expected_json_address = format!("\"0x{}\"", SAMPLE_ADDRESS);
        let check_sum = get_check_sum(SAMPLE_ADDRESS.to_string());
        let expected_json_address_obce = format!("\"OBC{}{}\"", SAMPLE_ADDRESS, check_sum);

        assert_eq!(
            expected_json_address == json_serialized
                || expected_json_address_obce == json_serialized,
            true
        );
        //assert_eq!(expected_json_address, json_serialized);
        assert_eq!(object_id_vec, bcs_serialized);
    }
}

fn sha256_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

//OBCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768579ede
fn local_convert_to_obaddress(prefix: &str, evm_address: &str) -> String {
    //let mut address = evm_address.to_string();

    let result = sha256_string(&evm_address[2..]);
    //let mut hex = hex::encode(result);
    //result.truncate(4);
    let check_sum = result.get(0..4).unwrap();
    println!("the check_sum is {}", check_sum);

    let mut address = prefix.to_string();
    address.push_str(&evm_address[2..]);
    address.push_str(check_sum);

    return address;
}

pub fn local_convert_to_evm_address(ob_address: String) -> String {
    if ob_address.len() == 0 {
        return String::from("");
    }

    let mut address = ob_address[3..].to_string();
    let evm_prefix = String::from("0x");
    address.insert_str(0, evm_prefix.as_str());
    address.truncate(address.len() - 4);

    let result = sha256_string(address.as_str());
    let mut hex = hex::encode(result);
    hex.truncate(4);

    let verify_code = ob_address[ob_address.len() - 4..].to_string();

    return if verify_code == hex {
        address
    } else {
        //todo, throw error
        String::from("")
    };

    //return address
}

fn get_check_sum(input: String) -> String {
    let result = sha256_string(&input.clone());
    let check_sum = result.get(0..4).unwrap();
    return check_sum.to_string();
}
