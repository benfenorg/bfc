use std::collections::hash_map::DefaultHasher;
use hex;
use sha2::{Digest, Sha256};




#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_convert_sample(){
        println!("Hello, world!");

        let Prefix = "OBC";
        let evmAddress = "0xffc044c1cba10650f59e502a38330b6b00ed74f045e23478a5e2b98467041ba1";

        let result =  convertToOBAddress(Prefix, evmAddress);

        println!("the ob convert result is {}", result);


        ////===============

        let evmAddress = convert_to_evm_address(result);
        println!("the evm convert result is {}", evmAddress);
    }




    #[test]
    fn test_sha256_string() {
        let input = "d62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b76857";
        let result =  sha256_string(input);
        println!("the sha256 result is {}", result);
    }
}





fn sha256_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}


//OBCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768576433
fn convertToOBAddress(prefix: &str, evm_address: &str) -> String {
    let mut address = evm_address.to_string();

    let result = sha256_string(address.as_str());
    let mut hex = hex::encode(result);
    hex.truncate(4);

    let mut address = prefix.to_string();
    address.push_str(&evm_address[2..]);
    address.push_str(hex.as_str());


    return address;
}

// 0x99ec891ff6602457efc2c5086c8926f4fe78cebc02a79a55485a6c56aca2b572
pub fn convert_to_evm_address(ob_address: String) -> String {
    if ob_address.len()==0 {
        return String::from("")
    }

    let mut address = ob_address[3..].to_string();
    let evm_prefix = String::from("0x");
    address.insert_str(0, evm_prefix.as_str());
    address.truncate(address.len()-4);

    let result = sha256_string(address.as_str());
    let mut hex = hex::encode(result);
    hex.truncate(4);


    let verify_code = ob_address[ob_address.len()-4..].to_string();

    return address

    // return if verify_code == hex {
    //     address
    // } else {
    //     //todo, throw error
    //     String::from("")
    // }


    //return address.to_string();
}