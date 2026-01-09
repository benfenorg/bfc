use crate::id::UID;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::balance::AnonymousBalance;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct AnonymousCoin {
    pub id: UID,
    pub balance: AnonymousBalance,
}

impl AnonymousCoin {
    /// Create a coin from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}
