use serde::{Deserialize, Serialize};
use crate::types::{
    BridgeActionType,
    AddTokenOnSolanaAction,AssetPriceUpdateAction,
    SingleTransferLimitUpdateAction,
    LimitUpdateAction,
};
use crate::encoding::{
    ADD_TOKENS_ON_SOLANA_MESSAGE_VERSION, 
    ASSET_PRICE_UPDATE_MESSAGE_VERSION,
    SINGLE_TRANSFER_LIMIT_UPDATE_MESSAGE_VERSION,
    LIMIT_UPDATE_MESSAGE_VERSION,
    BridgeMessageEncoding
};
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


impl From<AssetPriceUpdateAction> for SolanaMessage {
    fn from(action: AssetPriceUpdateAction) -> Self {
        SolanaMessage {
            message_type: BridgeActionType::AssetPriceUpdate as u8,
            version: ASSET_PRICE_UPDATE_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().clone(),
        }
    }
}

impl From<SingleTransferLimitUpdateAction> for SolanaMessage {
    fn from(action: SingleTransferLimitUpdateAction) -> Self {
        SolanaMessage {
            message_type: BridgeActionType::SingleTransferLimitUpdate as u8,
            version: SINGLE_TRANSFER_LIMIT_UPDATE_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().clone(),
        }
    }
}


impl From<LimitUpdateAction> for SolanaMessage {
    fn from(action: LimitUpdateAction) -> Self {
        SolanaMessage {
            message_type: BridgeActionType::LimitUpdate as u8,
            version: LIMIT_UPDATE_MESSAGE_VERSION,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload: action.as_payload_bytes().clone(),
        }
    }
}
