use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use hex;



#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_convert_sample(){
        println!("Hello, world!");

        let Prefix = "OBC";
        let evmAddress = "0xd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b76857";

        let result =  convertToOBAddress(Prefix, evmAddress);

        println!("the ob convert result is {}", result);


        ////===============

        let evmAddress = convertToEvmAddress(result);
        println!("the evm convert result is {}", evmAddress);
    }


}



fn sha256(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let result = hasher.finish();

    format!("{:x}", result)
}


//OBCd62ca040aba24f862a763851c54908cd2a0ee7d709c11b93d4a2083747b768576433
fn convertToOBAddress(prefix: &str, evm_address: &str) -> String {
    let mut address = evm_address.to_string();

    let result = sha256(address.as_str());
    let mut hex = hex::encode(result);
    hex.truncate(4);

    let mut address = prefix.to_string();
    address.push_str(&evm_address[3..]);
    address.push_str(hex.as_str());


    return address;
}

// 0x99ec891ff6602457efc2c5086c8926f4fe78cebc02a79a55485a6c56aca2b572
pub fn convertToEvmAddress(ob_address: String) -> String {

    let mut address = ob_address[3..].to_string(); //remove obc
    let evm_prefix = String::from("0x");
    address.insert_str(0, evm_prefix.as_str());
    address.truncate(address.len()-4);

    let result = sha256(address.as_str());
    let mut hex = hex::encode(result);
    hex.truncate(4);


    let verify_code = ob_address[ob_address.len()-4..].to_string();

    return if verify_code == hex {
        address
    } else {
        String::from("")
    }
}