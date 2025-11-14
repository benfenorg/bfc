use anchor_lang::prelude::*;
use solana_program::keccak::hashv;
use crate::errors::MessageError;
use crate::errors::BridgeConvertError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Debug)]
pub struct Message {
    pub message_type: u8,
    pub version: u8,
    pub nonce: u64,
    pub chain_id: u8,
    pub payload: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone,Debug)]
pub struct TokenTransferPayload {
    pub sender_address_length: u8,
    pub sender_address:  Vec<u8>,
    pub target_chain_id: u8,
    pub recipient_address_length: u8,
    pub recipient_address: Pubkey,
    pub token_id: u64,
    pub amount: u64,
    pub tx_hash: Vec<u8>,
    pub event_idx: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize,Clone,Debug)]
pub struct BlocklistPayload {
    pub addresses: Vec<[u8;20]>,
    pub is_blocklisted: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize,Debug,Clone)]
pub struct AddTokenPayload {
    pub native: bool,
    pub token_id: u64,
    pub token_address: Pubkey,
    pub benfen_decimal: u8,
    pub token_price: u64,
}



pub const MESSAGE_PREFIX: &[u8] = b"SUI_BRIDGE_MESSAGE";

// Message types
pub const TOKEN_TRANSFER: u8 = 0;
pub const BLOCKLIST: u8 = 1;
pub const EMERGENCY_OP: u8 = 2;
pub const UPDATE_BRIDGE_LIMIT: u8 = 3;
pub const UPDATE_TOKEN_PRICE: u8 = 4;
pub const UPGRADE: u8 = 5;
// pub const ADD_EVM_TOKENS: u8 = 7;

pub const UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT: u8 = 19;
pub const ADD_SVM_TOKENS: u8 = 20;




// Required stakes
pub const TRANSFER_STAKE_REQUIRED: u32 = 3334;
pub const FREEZING_STAKE_REQUIRED: u32 = 450;
pub const UNFREEZING_STAKE_REQUIRED: u32 = 5001;
pub const UPGRADE_STAKE_REQUIRED: u32 = 5001;
pub const BLOCKLIST_STAKE_REQUIRED: u32 = 5001;
pub const BRIDGE_LIMIT_STAKE_REQUIRED: u32 = 5001;
pub const UPDATE_TOKEN_PRICE_STAKE_REQUIRED: u32 = 5001;
pub const ADD_EVM_TOKENS_STAKE_REQUIRED: u32 = 5001;
pub const ADD_SVM_TOKENS_STAKE_REQUIRED: u32 = 5001;

pub const UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT_STAKE_REQUIRED: u32 = 5001;

// Token IDs
pub const SUI: u64 = 0;
pub const BTC: u64 = 1;
pub const ETH: u64 = 2;
pub const USDC: u64 = 3;
pub const USDT: u64 = 4;
pub const BNB: u64 = 6;
pub const OP: u64 = 7;
pub const ARB: u64 = 8;
pub const POL: u64 = 9;
pub const AVAX: u64 = 10;
pub const SOL: u64 = 14;


pub fn create_message(
    message_type: u8,
    version: u8,
    nonce: u64,
    chain_id: u8,
    payload: Vec<u8>,
) -> Message {
        Message {
            message_type,
            version,
            nonce,
            chain_id,
            payload,
        }
}

pub fn encode_message(message: &Message) -> Vec<u8> {
    let mut encoded = Vec::new();
    
    // Add prefix, type and version
    encoded.extend_from_slice(MESSAGE_PREFIX);
    encoded.push(message.message_type);
    encoded.push(message.version);
    
    // Add nonce
    encoded.extend_from_slice(&message.nonce.to_be_bytes());
    
    // Add chain ID
    encoded.push(message.chain_id);
    
    // Add payload
    encoded.extend_from_slice(&message.payload);
    
    encoded
}
pub fn keccak256_hash(data: &[u8]) -> [u8; 32] {
    hashv(&[data]).0
}
/// Computes the hash of a bridge message
pub fn compute_message_hash(message: &Message) -> [u8; 32] {
    let encoded = encode_message(message);
    keccak256_hash(encoded.as_slice())
}


pub fn pubkey_to_eth_address(pubkey: [u8;64]) -> [u8; 20] {
    let keccak = hashv(&[&pubkey]).0;
    let mut eth_address = [0u8; 20];
    eth_address.copy_from_slice(&keccak[12..32]);
    eth_address
}
/// Computes the required stake for a message type
pub fn compute_required_stake(message: &Message) -> u32 {
    match message.message_type {
        TOKEN_TRANSFER => TRANSFER_STAKE_REQUIRED,
        BLOCKLIST => BLOCKLIST_STAKE_REQUIRED,
        EMERGENCY_OP => {
            let op_code = decode_emergency_op_payload(&message.payload).unwrap();
            if op_code { FREEZING_STAKE_REQUIRED } else { UNFREEZING_STAKE_REQUIRED }
        },
        UPDATE_BRIDGE_LIMIT => BRIDGE_LIMIT_STAKE_REQUIRED,
        UPDATE_TOKEN_PRICE => UPDATE_TOKEN_PRICE_STAKE_REQUIRED,
        UPGRADE => UPGRADE_STAKE_REQUIRED,
        ADD_SVM_TOKENS => ADD_SVM_TOKENS_STAKE_REQUIRED,
        UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT => UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT_STAKE_REQUIRED,
        _ => 0,
    }
}

/// Converts a token amount from SLP decimal precision to Benfen decimal precision
pub fn convert_slp_to_benfen_decimal(
    slp_decimal: u8,
    benfen_decimal: u8,
    amount: u64,
) -> Result<u64> {
    require!(amount > 0, BridgeConvertError::InsufficientAmount);

    if slp_decimal == benfen_decimal {
        return Ok(amount);
    }

    let adjusted_amount = if slp_decimal > benfen_decimal {
        let factor = 10u64.pow((slp_decimal - benfen_decimal) as u32);
        //require!(amount % factor == 0, BridgeError::AmountTooSmall);
        amount.checked_div(factor).ok_or(BridgeConvertError::AmountTooSmall)?
    } else {
        let factor = 10u64.pow((benfen_decimal - slp_decimal) as u32);
        amount.checked_mul(factor).ok_or(BridgeConvertError::AmountTooLarge)?
    };

    require!(adjusted_amount > 0, BridgeConvertError::AmountTooSmall);
    Ok(adjusted_amount)
}

/// Converts a token amount from Sui decimal precision to SLP decimal precision
pub fn convert_benfen_to_slp_decimal(
    benfen_decimal: u8,
    slp_decimal: u8,
    amount: u64,
) -> Result<u64> {
    require!(amount > 0, BridgeConvertError::InsufficientAmount);

    if benfen_decimal == slp_decimal {
        return Ok(amount);
    }

    let adjusted_amount = if slp_decimal > benfen_decimal {
        let factor = 10u64.pow((slp_decimal - benfen_decimal) as u32);
        amount.checked_mul(factor).ok_or(BridgeConvertError::AmountTooLarge)?
    } else {
        let factor = 10u64.pow((benfen_decimal - slp_decimal) as u32);
        amount.checked_div(factor).ok_or(BridgeConvertError::AmountTooSmall)?
    };

    require!(adjusted_amount > 0, BridgeConvertError::AmountTooSmall);
    Ok(adjusted_amount)
}

/// Decodes a token transfer payload from bytes to a TokenTransferPayload struct
pub fn decode_token_transfer_payload(payload: &[u8]) -> Result<TokenTransferPayload> {
    require!(payload.len() >= 83, MessageError::InvalidPayloadLength); 

    let sender_address_length = payload[0] as u8;
    require!(
        sender_address_length == 32,
        MessageError::InvalidSenderAddressLength
    );

    // Extract sender address (bytes 1-32)
    let mut sender_address = Vec::with_capacity(sender_address_length as usize);
    sender_address.extend_from_slice(&payload[1..33]);

    // Get target chain (byte 33)
    let target_chain_id = payload[33];

    // Get recipient address length (byte 34)
    let recipient_address_length = payload[34] as u8;
    require!(
        recipient_address_length == 32,
        MessageError::InvalidRecipientAddressLength
    );

    // Extract recipient address (bytes 35-66)
    let recipient_address = Pubkey::new_from_array(payload[35..67].try_into().unwrap());

    // Extract token ID (bytes 67-74)
    let token_id = u64::from_be_bytes(payload[67..75].try_into().unwrap());

    // Extract amount (bytes 75-82)
    let amount = u64::from_be_bytes(payload[75..83].try_into().unwrap());

    // Extract tx hash
    let tx_hash_start = 83;
    let tx_hash_end = payload.len() - 2; // -2 for event_idx
    let mut tx_hash = Vec::new();
    tx_hash.extend_from_slice(&payload[tx_hash_start..tx_hash_end]);

    // Extract event index (last 2 bytes)
    let event_idx = u16::from_be_bytes(payload[tx_hash_end..].try_into().unwrap());

    Ok(TokenTransferPayload {
        sender_address_length,
        sender_address,
        target_chain_id,
        recipient_address_length,
        recipient_address,
        token_id,
        amount,
        tx_hash,
        event_idx,
    })
}
/// Decodes a blocklist payload from bytes to a BlocklistPayload struct
pub fn decode_blocklist_payload(payload: &[u8]) -> Result<BlocklistPayload> {
    require!(payload.len() >= 2, MessageError::InvalidPayloadLength);

    // Extract is_blocklist flag (first byte)
    //blocklistType: 0 = blocklist, 1 = unblocklist
    let is_blocklist = payload[0] == 0;

    // Extract number of members (second byte)
    let num_members = payload[1] as usize;
    let offset=2;
    let expected_length = offset + (num_members * 20); // 2 bytes header + (20 bytes per address)
    require!(payload.len() >= expected_length, MessageError::InvalidPayloadLength);

    // Extract member addresses
    let mut members = Vec::with_capacity(num_members);
    for i in 0..num_members {
        let start = offset + (i * 20);
        let end = start + 20;
        let member = payload[start..end].try_into().unwrap();
        members.push(member);
    }

    Ok(BlocklistPayload {
        addresses: members,
        is_blocklisted: is_blocklist,
    })
}

/// Decodes an emergency operation payload from bytes to an operation code

pub fn decode_emergency_op_payload(payload: &[u8]) -> Result<bool> {
    require!(payload.len() == 1, MessageError::InvalidPayloadLength);

    let op_code = payload[0];
    require!(op_code <= 1, MessageError::InvalidOpCode);
    // 0 = blocklist, 1 = unblocklist 
    // Keep consistent with EVM contract 
    Ok(op_code == 0)
}

/// Decodes an update limit payload from bytes to a chain ID and a new limit
pub fn decode_update_limit_payload(payload: &[u8]) -> Result<(u8, u64)> {
    require!(payload.len() == 9, MessageError::InvalidPayloadLength);
    
    let sender_chain_id = payload[0];
    let new_limit = u64::from_be_bytes(payload[1..9].try_into().unwrap());
    
    Ok((sender_chain_id, new_limit))
}


pub fn decode_update_single_transfer_limit_payload(payload: &[u8]) -> Result<(u8, u64)> {
    require!(payload.len() == 9, MessageError::InvalidPayloadLength);
    
    let sender_chain_id = payload[0];
    let new_limit = u64::from_be_bytes(payload[1..9].try_into().unwrap());
    
    Ok((sender_chain_id, new_limit))
}

/// Decodes an upgrade payload from bytes to proxy and implementation addresses and version
pub fn decode_upgrade_payload(payload: &[u8]) -> Result<(Pubkey, Pubkey, u8)> {
    require!(payload.len() >= 65, MessageError::InvalidPayloadLength);
    
    let proxy = Pubkey::new_from_array(payload[0..32].try_into().unwrap());
    let implementation = Pubkey::new_from_array(payload[32..64].try_into().unwrap());

    let version=payload[64];
    

    Ok((proxy, implementation, version))
}

/// Decodes an update token price payload from bytes to a token ID and a new price
pub fn decode_update_token_price_payload(payload: &[u8]) -> Result<(u64, u64)> {
    require!(payload.len() == 16, MessageError::InvalidPayloadLength);
    
    let token_id = u64::from_be_bytes(payload[0..8].try_into().unwrap());
    let token_price = u64::from_be_bytes(payload[8..16].try_into().unwrap());
    
    Ok((token_id, token_price))
}


/// Decodes an add tokens payload from bytes
pub fn decode_add_token_payload(payload: &[u8]) -> Result<AddTokenPayload> {
    // Check minimum length: 1 + 8 + 32 + 1 + 8 = 50 bytes
    require!(payload.len() >= 50, MessageError::InvalidPayloadLength);
    
    let native: bool = payload[0] != 0;
    let mut offset = 1;
    
    // Read token ID (8 bytes)
    require!(offset + 8 <= payload.len(), MessageError::InvalidPayloadLength);
    let token_id_bytes: [u8; 8] = payload[offset..offset + 8]
        .try_into()
        .map_err(|_| MessageError::InvalidTokenIdNotSupported)?;
    let token_id = u64::from_be_bytes(token_id_bytes);
    offset += 8;
    
    // Read token address (32 bytes)
    require!(offset + 32 <= payload.len(), MessageError::InvalidPayloadLength);
    let pubkey_bytes: [u8; 32] = payload[offset..offset + 32]
        .try_into()
        .map_err(|_| MessageError::InvalidMintAddress)?;
    let token_address = Pubkey::new_from_array(pubkey_bytes);
    offset += 32;
    
    // Read Sui decimals (1 byte)
    require!(offset < payload.len(), MessageError::InvalidPayloadLength);
    let sui_decimal = payload[offset];
    offset += 1;
    
    // Read token price (8 bytes)
    require!(offset + 8 <= payload.len(), MessageError::InvalidPayloadLength);
    let price_bytes: [u8; 8] = payload[offset..offset + 8]
        .try_into()
        .map_err(|_| MessageError::InvalidTokenPrice)?;
    let token_price = u64::from_be_bytes(price_bytes);
    
    Ok(AddTokenPayload {
        native,
        token_id,
        token_address,
        benfen_decimal: sui_decimal,
        token_price,
    })
}






#[cfg(test)]
pub mod bridge_utils_test{
    use super::*;

    const USD_VALUE_MULTIPLIER: u64 =100000000;
    #[test]
    fn test_convert_slp_to_benfen_decimal_amount_is_max() {
        let amount = u64::MAX;
        let benfen_decimal = 18;
        let token_decimal = 6;
        let converted_amount = convert_slp_to_benfen_decimal(token_decimal, benfen_decimal, amount);
        assert_eq!(converted_amount.unwrap_err(), BridgeConvertError::AmountTooLarge.into());
    }

    #[test]
    fn test_convert_benfen_to_slp_decimal_amount_is_zero() {
        let amount = 0;
        let token_decimal = 6;
        let benfen_decimal = 9;
        let converted_amount = convert_benfen_to_slp_decimal(token_decimal, benfen_decimal, amount);
        assert_eq!(converted_amount.unwrap_err(), BridgeConvertError::InsufficientAmount.into());
    }

    #[test]
    fn test_convert_slp_to_benfen_decimal_slp_lt_benfen() {
        let amount = 1*1000000;
        let token_decimal = 6;
        let benfen_decimal = 9;
        let converted_amount = convert_slp_to_benfen_decimal(token_decimal, benfen_decimal, amount).unwrap();
        assert_eq!(converted_amount, 1_000_000_000);
    }


     #[test]
    fn test_convert_slp_to_benfen_decimal_slp_gt_benfen() {
        let amount = 1*1_000_000_000_000_000_000;
        let token_decimal = 18;
        let benfen_decimal = 9;
        let converted_amount = convert_slp_to_benfen_decimal(token_decimal, benfen_decimal, amount).unwrap();
        assert_eq!(converted_amount, 1_000_000_000);
    }

    #[test]
    fn test_convert_slp_to_benfen_decimal_slp_eq_benfen(){
        let amount = 1*1_000_000_000_000_000_000;
        let token_decimal = 6;
        let benfen_decimal = 6;
        let converted_amount = convert_slp_to_benfen_decimal(token_decimal, benfen_decimal, amount).unwrap();
        assert_eq!(converted_amount, amount);
    }
    //testConvertSuiToERC20DecimalWithETHgtSui
    #[test]
    fn test_convert_benfen_to_slp_decimal_slp_gt_benfen() {
        let amount = 1*1_000_000_000;
        let token_decimal = 18;
        let benfen_decimal = 9;
        let converted_amount = convert_benfen_to_slp_decimal(benfen_decimal,token_decimal, amount).unwrap();
        assert_eq!(converted_amount, 1_000_000_000_000_000_000);
    }

    #[test]
    fn test_convert_benfen_to_slp_decimal_slp_lt_benfen() {
       let amount = 1*1_000_000_000;
        let token_decimal = 6;
        let benfen_decimal = 9;
        let converted_amount = convert_benfen_to_slp_decimal(benfen_decimal,token_decimal, amount).unwrap();
        assert_eq!(converted_amount, 1_000_000);
    }

    #[test]
    fn test_convert_benfen_to_slp_decimal_benfen_eq_slp() {
        let amount = 1*1_000_000_000;
        let token_decimal = 9;
        let benfen_decimal = 9;
        let converted_amount = convert_benfen_to_slp_decimal(benfen_decimal,token_decimal, amount).unwrap();
        assert_eq!(converted_amount, amount);
    }

    


    #[test]
    fn test_compute_message_hash() {
        use ethers::utils::keccak256;
        let message = Message{
            message_type: 1,
            version: 1,
            nonce: 1,
            chain_id: 1,
            payload: vec![1u8;0],
        };
        let hash = compute_message_hash(&message);
        let expected_hash = keccak256(&encode_message(&message));
        assert_eq!(hash,expected_hash);
    }

    #[test]
    fn test_encode_message() {

        let move_encoded_message = hex::decode(
            "5355495f4252494447455f4d45535341474500010000000000000000012080ab1ee086210a3a37355300ca24672e81062fcdb5ced6618dab203f6a3b291c0b14b18f79fe671db47393315ffdb377da4ea1b7af96010084d71700000000"
        ).unwrap();

        let nonce = 0u64;
        let sui_chain_id = 1u8;

        // payload 的十六进制表示
        let payload = hex::decode(
            "2080ab1ee086210a3a37355300ca24672e81062fcdb5ced6618dab203f6a3b291c0b14b18f79fe671db47393315ffdb377da4ea1b7af96010084d71700000000"
        ).unwrap();

        // 创建并编码消息
        let message = Message {
            message_type: TOKEN_TRANSFER,
            version: 1,
            nonce,
            chain_id: sui_chain_id,
            payload,
        };
        let abi_encoded_message = encode_message(&message);
        //let encoded = encode_message(&message);
        assert_eq!(abi_encoded_message,move_encoded_message);
    }

    #[test]
    fn test_decode_transfer_token_payload() {

    }

    #[test]
    fn test_decode_block_list_payload() {
        // encode payload
        let payload = hex::decode("010268b43fd906c0b8f024a18c56e06744f7c6157c65acaef39832cb995c4e049437a3e2ec6a7bad1ab5").unwrap();
        
        // decode payload
        let payload = decode_blocklist_payload(&payload).unwrap();
        //println!("payload {:?}",payload.is_blocklisted as u8);

        // verify
        assert_eq!(payload.addresses.len(), 2);
        let eth_address1: [u8; 20] 
            = hex::decode("0x68B43fD906C0B8F024a18C56e06744F7c6157c65"
            .trim_start_matches("0x"))
            .unwrap()
            .try_into()
            .unwrap();

        let eth_address2: [u8; 20] 
            = hex::decode("0xaCAEf39832CB995c4E049437A3E2eC6a7bad1Ab5"
            .trim_start_matches("0x"))
            .unwrap()
            .try_into()
            .unwrap();
        
        // verify
        assert_eq!(
           payload.addresses[0],
           eth_address1,
        );
        assert_eq!(
            payload.addresses[1],
           eth_address2,
        );
        
        assert!(!payload.is_blocklisted);
    }

    #[test]
    fn test_decode_update_limit_payload() {
        let payload = hex::decode("0c00000002540be400").unwrap();
        
        let (source_chain_id, new_limit) = decode_update_limit_payload(&payload).unwrap();
        
        assert_eq!(source_chain_id, 12);
        assert_eq!(new_limit, 100 * USD_VALUE_MULTIPLIER); 
    }

    #[test]
    fn test_decode_update_token_price_payload(){
        let payload = hex::decode("0000000000000001000000003b9aca00").unwrap();
        let (token_id, new_price) = decode_update_token_price_payload(&payload).unwrap();
        assert_eq!(token_id, 1);
        assert_eq!(new_price, 10 * USD_VALUE_MULTIPLIER);
    }

    #[test]
    fn test_decode_emergency_op_payload() {
        //encode
        let payload = vec![1u8];
        
        //decode
        let pausing = decode_emergency_op_payload(&payload).unwrap();
        
        // verify
        assert!(!pausing);
    }
    #[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
    pub struct UpgradePayload {
        pub proxy: Pubkey,
        pub implementation: Pubkey,
        pub calldata: Vec<u8>,
    }

    #[test]
    fn test_decode_upgrade_payload(){
        let proxy=Pubkey::new_unique();
        let bridge=Pubkey::new_unique();
        // let calldata = "SUI_BRIDGE_MESSAGE050100000000000000000c".as_bytes().to_vec();
        let  version = 1u8;

        let mut encoded_payload = Vec::new();
        encoded_payload.extend_from_slice(proxy.as_ref());
        encoded_payload.extend_from_slice(bridge.as_ref());
        encoded_payload.extend_from_slice(&[version]);

        let decoded = decode_upgrade_payload(&encoded_payload).unwrap();
        assert_eq!(decoded.0, proxy);
        assert_eq!(decoded.1, bridge);
        assert_eq!(decoded.2, version);
    }

    #[test]
    fn test_required_stake_invaild_type(){
        let invalid_type = 100;
        let message=create_message(invalid_type, 0,    0, 0, vec![]);
        let required_stake = compute_required_stake(&message);
        assert_eq!(required_stake,0);
    }


}