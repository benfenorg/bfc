// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::base_types::ObjectID;
use crate::error::SuiError;
use fastcrypto::encoding::{Encoding, Hex};
use std::fmt;
//use serde::{Deserialize};
use schemars::JsonSchema;
use serde_with::serde_as;
//use crate::sui_serde::Readable;
use std::convert::TryFrom;
//use hex;
//use sha2::{Digest, Sha256};

#[cfg(test)]
#[path = "unit_tests/base_type_bfc_tests.rs"]
mod base_type_bfc_tests;

pub const BFC_ADDRESS_LENGTH: usize = ObjectID::LENGTH;

#[serde_as]
#[derive(Eq, Default, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, JsonSchema)]
#[cfg_attr(feature = "fuzzing", derive(proptest_derive::Arbitrary))]
struct BfcAddress(
    #[schemars(with = "Hex")]
    #[serde_as(as = "Readable<Hex, _>")]
    [u8; BFC_ADDRESS_LENGTH],
);

impl BfcAddress {
    /// Parse a SuiAddress from a byte array or buffer.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, SuiError> {
        <[u8; BFC_ADDRESS_LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| SuiError::InvalidAddress)
            .map(BfcAddress)
    }
}

impl fmt::Debug for BfcAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

impl AsRef<[u8]> for BfcAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl TryFrom<&[u8]> for BfcAddress {
    type Error = SuiError;

    /// Tries to convert the provided byte array into a SuiAddress.
    fn try_from(bytes: &[u8]) -> Result<Self, SuiError> {
        Self::from_bytes(bytes)
    }
}

pub mod bfc_address_util {
    //use std::collections::hash_map::DefaultHasher;
    use sha2::{Digest, Sha256};
    use crate::base_types::{ObjectID, SuiAddress};

    pub fn sha256_string(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn convert_to_evm_address(ob_address: String) -> String {
        if ob_address.len() == 0 {
            return String::from("");
        }

        let mut address = ob_address[3..].to_string();
        let evm_prefix = String::from("0x");
        address.insert_str(0, evm_prefix.as_str());
        address.truncate(address.len() - 4);

        let result = sha256_string(&address[2..]);
        let check_sum = result.get(0..4).unwrap();

        let verify_code = ob_address[ob_address.len() - 4..].to_string();

        return if verify_code == check_sum {
            address
        } else {
            println!("verify_code: {}, check_sum: {}", verify_code, check_sum);
            String::from("")
        };

        //return address.to_string();
    }

    pub fn convert_to_bfc_address( evm_address: &str) -> String {
        let prefix = "BFC";
        //let mut address = evm_address.to_string();
        let result = sha256_string(&evm_address[2..]);
        let check_sum = result.get(0..4).unwrap();

        let mut address = prefix.to_string();
        address.push_str(&evm_address[2..]);
        address.push_str(check_sum);

        return address;
    }

    pub fn objects_id_to_bfc_address(object_id: ObjectID) ->String{
        let gas_suiaddress: SuiAddress = object_id.into() ;
        let bfc_address = convert_to_bfc_address(gas_suiaddress.to_string().as_str());

        bfc_address
    }

    pub fn sui_address_to_bfc_address(suiaddress: SuiAddress) ->String{
        let bfc_address = convert_to_bfc_address(suiaddress.to_string().as_str());

        bfc_address
    }
}
