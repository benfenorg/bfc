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
    use crate::dynamic_field::Field;
    use crate::sui_system_state::sui_system_state_inner_v1::SuiSystemStateInnerV1;
    //use crate::base_types_bfc::bfc_address_util::convert_to_bfc_address;
    use super::*;

    #[test]
    fn test_convert_util() {
        let result = bfc_address_util::convert_to_evm_address(String::from(
            "BFCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768579ede",
        ));
        println!("the evm convert result is {}", result);

        let sui_address = bfc_address_util::convert_to_bfc_address(
            "BFC",
            "0xd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b76857",
        );
        println!("the bfc convert result is {}", sui_address);
    }

    #[test]
    fn test_convert_sample() {
        let prefix = "BFC";
        let evm_address = "0xd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b76857";

        let bfc_address = local_convert_to_obaddress(prefix, evm_address);

        //BFCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768579ede
        println!("the ob convert result is {}", bfc_address);

        ////===============

        //let evm_address = convert_to_evm_address(result);
        //println!("the evm convert result is {}", evm_address);
    }

    #[test]
    fn test_convert_sample2() {
        ////===============
        let result = "BFCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768579ede";
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
        let expected_json_address_bfc = format!("\"BFC{}{}\"", SAMPLE_ADDRESS, check_sum);

        assert_eq!(
            expected_json_address == json_serialized
                || expected_json_address_bfc == json_serialized,
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
        let expected_json_address_bfc = format!("\"BFC{}{}\"", SAMPLE_ADDRESS, check_sum);

        assert_eq!(
            expected_json_address == json_serialized
                || expected_json_address_bfc == json_serialized,
            true
        );
        //assert_eq!(expected_json_address, json_serialized);
        assert_eq!(object_id_vec, bcs_serialized);
    }

    #[test]
    fn test_load_sui_system(){
        let system_data_str = "6af2a2b7ca60bf76174adfd3e9c4957f8e937759603182f9b46c7f6c5f19c6d2010000000000000000000000000000001700000000000000010000000000000000008d49fd1a0700019f476b0637370817249aa76eda122cc2150993af32dd5702ff02b187f83c12116099f25ef61f8032b914636460982c5cc6f134ef1ddae76657f2cbfec1ebfc8d097374080df6fcf0dcb8bc4b0d8e0af5d80ebbff2b4c599f54f42d6312dfc314276078c1cc347ebbbec5198be258513f386b930d02c2749a803e2330955ebd1a1020307918d53a3f541e98ead66a3d84bdbf8fed40069e8bb32694ebea2ce19fa5d520675a748ae2a2cb9f80acc38add51758e605a973aff1b259b555ae221c45a206630b04535c403666395af8c23249d1d28a4b782fc59be06f377c9d1b5e6ae1fbb0462a4e3433705ac7ee75239ab25df59970b76616c696461746f722d300000001d2f6970342f3132372e302e302e312f7463702f35383239362f68747470182f6970342f3132372e302e302e312f7564702f3538323938182f6970342f3132372e302e302e312f7564702f3538333034182f6970342f3132372e302e302e312f7564702f353833303600000000000000006fdca7ea2fc9e50e320cd49a7caa04c80ee65989a722f6cb5db600643366975700000000000000001027000000000000b9ee53b5e2e101ffcfe1cf744179da8a270bf679358d08712edbae916deeb4176400000000000000645f336a5a5be0f45c8b035530c52813abeb0d5700005c7e10594f323dfb26620100000000000000000000008d49fd1a0700000000000000000000008d49fd1a07006bb381f3ffe3a4ff27cc854954299e9546ab79c010180b0f7743de5c06400cf00100000000000000000000000000000000000000000000000000000000000000874a783a4ff6e58f68e70587313e68d55b9406033a028dc456213ae0d5e2a3e90000000000000000b8506a56001c8f6d52a62e96c4f9f8e5a66eecfc61493b0d4c8cc2d95249441801000000000000000000000000000000000000000000000000000000000000000000097a50cce5d4da4927946bbd74294e10b0eed38bf517f492eef75b4250709ee30100000000000000000000000000000000000000000000000000000000000000d9ad5070beb0720e8139568522e2bb7a82fb35ec7c8f3d402a7fba4701f3d2ca0000000000000000c80000000000000000008d49fd1a070000000000000000006400000000000000c8000000000000003d60bb0b8ddb3c6fc135f775d5d6bc97a646e50eb39b3fb19cd27fae448d050a00000000000000006d7c17d15392845c5741265bc68f2e9d3fd2698e11af28f4fe360903ac4a1eb900000000000000000056b6d799eb1a826ce33fbfedf55c549c873dab07c19c6cac26b21fa24f8113480100000000000000ef6ed038208c9b4b34f8a3a314055f2beaf614f019d46b196d6bb008fed165480100000000000000c26359e394860d9f3d9621b3f340df13d6b570f4493da64f4aca1386bf859cb100000000000000007334e64bd1b1a90cd822d09a4ba89f9237e4ccafccbcf1ea863b88833fddf01f0000000000000000006a01667ddb9e826dd1e144b1f35eff073682687e186691fa0f520c9c0841fe1c000000000000000000000000000000000000000000000000005c26050000000000000000000000009600000000000000008053ee7ba80a0000008d49fd1a070000c029f73d5405000700000000000000aad7cfb69e3f590a31c19a95f0406c31c8894dc448c9bae42202a4ed53150fd600000000000000006400000000000000000000aa55c623d40500000000000000000080c6a47e8d03000a00000000000000e803a84025828b77bc4d09cb7865c3265a80e57b5cfe86429a3fcc0fc502a5ad269000000000000000000000000000000000000000000000000000000000000000000000000000000000006396a2b28b010000bf16dcaa084631e4b3af0eae8725a2a2c15e46d176e66c908ffd372fb25767f10000000000000000";
        let system_data = hex::decode(system_data_str).unwrap();
        let system_state_innner=bcs::from_bytes::<Field<u64, SuiSystemStateInnerV1>>(system_data.as_slice()).unwrap();
    }
}

fn sha256_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

//BFCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768579ede
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
