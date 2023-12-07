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
        let system_data_str = "6af2a2b7ca60bf76174adfd3e9c4957f8e937759603182f9b46c7f6c5f19c6d2010000000000000000000000000000001700000000000000010000000000000000008d49fd1a07000170199b3e4da44b8a109f7d3c8722df90125a4e0511627d6020fc8f4819bec3ff6099f25ef61f8032b914636460982c5cc6f134ef1ddae76657f2cbfec1ebfc8d097374080df6fcf0dcb8bc4b0d8e0af5d80ebbff2b4c599f54f42d6312dfc314276078c1cc347ebbbec5198be258513f386b930d02c2749a803e2330955ebd1a1020eae0f82fb57f1513e19d00635498793571300a862f3dae1307a0c36894466a2b202aa7c056ecdc52ca3a1812c204f20915bb8cd56bccffe4eba96f58275486766030abc53caab5698bde6fd3c7d15f2c565976f71f91ba9c708c368d58561bae9b1ea7e95da7d22be5f00877015aa565c2650b76616c696461746f722d300000001d2f6970342f3132372e302e302e312f7463702f36323932332f68747470182f6970342f3132372e302e302e312f7564702f3632393235182f6970342f3132372e302e302e312f7564702f3632393331182f6970342f3132372e302e302e312f7564702f36323933330000000000000000779f68dc9dce78ff21a3b10d2195d61a812ea0da50e4a88ee737b963360bf411000000000000000010270000000000000778d2fd0b89247b327402b1ede0691a77f45c8f1d0799e33b15ca81be2ee251640000000000000022eb9b7c8271f8f6020cc2b9613b651746be82aca8329db2e604578708dfdd690100000000000000000000008d49fd1a0700000000000000000000008d49fd1a07000e8769d9e963d830210ff7b88442521521c0bdf81b14adadeb587e890b96cdaf010000000000000000000000000000000000000000000000000000000000000063f0081aa3f00d471ef57b0e16e85b0260758adbf139e78b06e5858ffa7f9de600000000000000007ebc6857b83026d56fe7e2698bb6bd3fea0ac14d4ec187fc064d5cf133f94416010000000000000000000000000000000000000000000000000000000000000000004514819cfde0dae5e441a00ea5efc3f151d4ca03c7bf0a21aab5d1ee8378cd0501000000000000000000000000000000000000000000000000000000000000001a4bf749a4d966e262cff121321b5131ad9767c1b162583af626036e077db2930000000000000000c80000000000000000008d49fd1a070000000000000000006400000000000000c800000000000000682f9794f4307b555cb9b537c8f1d7943171613e2644b77e4e5754b31b46f7a1000000000000000026fffa8ad71b6aff595d7c9f238c9a973801dbdec33feaba5dcf6918fd860f6f0000000000000000000c7443f778e79a126a3f84bcbc34b4296e40cc3dc5629398120541e43c06a2990100000000000000566f91a875261993fb9ef0db5beb884ece86a9f8803e182df3eaeefaf64a88c4010000000000000037782e3d16a6bd58d3a39b53a31a98d34d2fcc034351a4ab37e5515dffafe5c00000000000000000e04bfec862469f23021dd47b64e2f16b9309d6c0d7f2be82d9f9991e3fe02dc900000000000000000072a7eb3c35448cb09bf5246d453d9a027a658f4b5187d50a374a5db68bc7e48b000000000000000000000000000000000000000000000000005c26050000000000000000000000009600000000000000008053ee7ba80a0000008d49fd1a070000c029f73d54050007000000000000003135dc5e86342f8bb2529305b48d938b92d4e0fe795f47146656c2440525670900000000000000006400000000000000000000aa55c623d40500000000000000000080c6a47e8d03000a00000000000000e803361c3136a96546e6c9da5b4ad02e2e2bfb3931fa34c4b2b0e648fff13f46efd7000000000000000000000000000000000000000000000000000000000000000000000000000000000039c3f1b28b0100000ac9df5cfd5032b21ccd1e7139c7f19e28d1dc617f20993a9ebcbb004343e67a0000000000000000";
        let system_data = hex::decode(system_data_str).unwrap();
        let system_state_innner=bcs::from_bytes::<Field<u64, SuiSystemStateInnerV1>>(system_data.as_slice());
        println!("inner is {:?}",system_state_innner);
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
