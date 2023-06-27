use std::{fmt};
use fastcrypto::encoding::{Encoding, Hex};
use crate::base_types::ObjectID;
use crate::error::SuiError;
//use serde::{Deserialize};
use schemars::JsonSchema;
use serde_with::serde_as;
//use crate::sui_serde::Readable;
use std::convert::TryFrom;
use hex;
use sha2::{Digest, Sha256};

#[cfg(test)]
#[path = "unit_tests/base_type_obc_tests.rs"]
mod base_type_obc_tests;

pub const OBC_ADDRESS_LENGTH: usize = ObjectID::LENGTH;


#[serde_as]
#[derive(
Eq, Default, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, JsonSchema,
)]
#[cfg_attr(feature = "fuzzing", derive(proptest_derive::Arbitrary))]
struct ObcAddress(
    #[schemars(with = "Hex")]
    #[serde_as(as = "Readable<Hex, _>")]
    [u8; OBC_ADDRESS_LENGTH],


);




impl  ObcAddress {




    /// Parse a SuiAddress from a byte array or buffer.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, SuiError> {
        <[u8; OBC_ADDRESS_LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| SuiError::InvalidAddress)
            .map(ObcAddress)
    }




}

impl fmt::Debug for ObcAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

impl AsRef<[u8]> for ObcAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl TryFrom<&[u8]> for ObcAddress {
    type Error = SuiError;

    /// Tries to convert the provided byte array into a SuiAddress.
    fn try_from(bytes: &[u8]) -> Result<Self, SuiError> {
        Self::from_bytes(bytes)
    }
}

pub mod obc_address_util {
    use std::collections::hash_map::DefaultHasher;
    use sha2::{Digest, Sha256};


    fn sha256_string(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }


    pub fn convert_to_evm_address(ob_address: String) -> String {
        if ob_address.len()==0 {
            return String::from("")
        }

        let mut address = ob_address[3..].to_string();
        let evm_prefix = String::from("0x");
        address.insert_str(0, evm_prefix.as_str());
        address.truncate(address.len()-4);

        let mut result = sha256_string(&address[2..]);
        let checkSum = result.get(0..4).unwrap();



        let verify_code = ob_address[ob_address.len()-4..].to_string();


        return if verify_code == checkSum {
            address
        } else {
            //todo, throw error
            println!("verify_code: {}, checkSum: {}", verify_code, checkSum);
            String::from("")
        }


        //return address.to_string();
    }


    pub fn convert_to_obc_address(prefix: &str, evm_address: &str) -> String {

        //let mut address = evm_address.to_string();
        let mut result = sha256_string(&evm_address[2..]);
        let checkSum = result.get(0..4).unwrap();

        let mut address = prefix.to_string();
        address.push_str(&evm_address[2..]);
        address.push_str(checkSum);


        return address;
    }

}