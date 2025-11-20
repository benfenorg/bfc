use serde::{Deserialize, Serialize};
use crate::types::{BridgeActionType,AddTokenOnSolanaAction};
use crate::encoding::{ADD_TOKENS_ON_SOLANA_MESSAGE_VERSION, BridgeMessageEncoding};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolanaMessage {
    pub message_type: u8,
    pub version: u8,
    pub nonce: u64,
    pub chain_id: u8,
    pub payload: Vec<u8>,
}

////////////////////////////////////////////////////////////////////////
//                        Solana Message Conversion                   //
////////////////////////////////////////////////////////////////////////
impl From<AddTokenOnSolanaAction> for SolanaMessage {
    fn from(action: AddTokenOnSolanaAction) -> Self {
        SolanaMessage {
            message_type: BridgeActionType::AddTokensOnSolana as u8,
            version: ADD_TOKENS_ON_SOLANA_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().clone(),
        }
    }
}