// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::error::BridgeResult;
use crate::types::{BridgeAction, SolanaToSuiBridgeAction, SolanaToSuiTokenBridgeV1};
use base64::Engine;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::Signature;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use std::io::Cursor;
use std::io::Read;
use std::str::FromStr;
use tracing;

// 声明程序以生成事件类型
anchor_lang::declare_program!(benfen_bridge);

// 使用 anchor-lang 生成的事件类型
// 注意：这些类型由 declare_program! 宏自动生成
// 如果编译错误，可能需要检查 IDL 文件中的事件定义

/// Solana 交易日志包装器
#[derive(Debug, Clone)]
pub struct SolanaLog {
    pub signature: Signature,
    pub slot: u64,
    pub log_messages: Vec<String>,
}

/// TokensDeposited 事件结构
/// 
/// 对应 Solana Anchor 事件：
/// ```rust
/// #[event]
/// pub struct TokensDeposited {
///   pub nonce: u64,
///   pub source_chain_id: u8,
///   pub target_chain_id: u8,
///   pub token_id: u64,
///   pub amount: u64,
///   pub sender_address: Pubkey,   // 32 bytes
///   pub recipient_address: Vec<u8>, // u32 LE length + bytes
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokensDeposited {
    pub discriminator: [u8; 8],
    pub nonce: u64,
    pub source_chain_id: u8,
    pub target_chain_id: u8,
    pub token_id: u64,
    pub amount: u64,
    pub sender_pubkey: [u8; 32],
    pub recipient_length: u32,
    pub recipient_bytes: Vec<u8>,
}

impl TokensDeposited {
    /// 从字节数据解析 TokensDeposited 事件
    pub fn parse_from_bytes(data: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(data);
        
        // 读取 8-byte Anchor event discriminator
        let mut discriminator = [0u8; 8];
        cursor.read_exact(&mut discriminator)
            .map_err(|e| format!("read discriminator: {}", e))?;
        
        // 读取 nonce (u64, little-endian)
        let mut nonce_bytes = [0u8; 8];
        cursor.read_exact(&mut nonce_bytes)
            .map_err(|e| format!("read nonce: {}", e))?;
        let nonce = u64::from_le_bytes(nonce_bytes);
        
        // 读取 source_chain_id (u8)
        let mut source_chain_id_bytes = [0u8; 1];
        cursor.read_exact(&mut source_chain_id_bytes)
            .map_err(|e| format!("read source_chain_id: {}", e))?;
        let source_chain_id = source_chain_id_bytes[0];
        
        // 读取 target_chain_id (u8)
        let mut target_chain_id_bytes = [0u8; 1];
        cursor.read_exact(&mut target_chain_id_bytes)
            .map_err(|e| format!("read target_chain_id: {}", e))?;
        let target_chain_id = target_chain_id_bytes[0];
        
        // 读取 token_id (u64, little-endian)
        let mut token_id_bytes = [0u8; 8];
        cursor.read_exact(&mut token_id_bytes)
            .map_err(|e| format!("read token_id: {}", e))?;
        let token_id = u64::from_le_bytes(token_id_bytes);
        
        // 读取 amount (u64, little-endian)
        let mut amount_bytes = [0u8; 8];
        cursor.read_exact(&mut amount_bytes)
            .map_err(|e| format!("read amount: {}", e))?;
        let amount = u64::from_le_bytes(amount_bytes);
        
        // 读取 sender_pubkey (32 bytes)
        let mut sender_pubkey = [0u8; 32];
        cursor.read_exact(&mut sender_pubkey)
            .map_err(|e| format!("read sender pubkey: {}", e))?;
        
        // 读取 recipient_length (u32, little-endian)
        let mut recipient_length_bytes = [0u8; 4];
        cursor.read_exact(&mut recipient_length_bytes)
            .map_err(|e| format!("read recipient length: {}", e))?;
        let recipient_length = u32::from_le_bytes(recipient_length_bytes);
        
        // 检查长度是否合理
        if recipient_length > 10_000_000 {
            return Err(format!("recipient length too large: {}", recipient_length));
        }
        
        // 读取 recipient_bytes
        let mut recipient_bytes = vec![0u8; recipient_length as usize];
        cursor.read_exact(&mut recipient_bytes)
            .map_err(|e| format!("read recipient bytes: {}", e))?;
        
        // 检查是否还有剩余数据
        if cursor.position() != cursor.get_ref().len() as u64 {
            return Err(format!(
                "unexpected trailing bytes: {}",
                cursor.get_ref().len() - cursor.position() as usize
            ));
        }
        
        Ok(TokensDeposited {
            discriminator,
            nonce,
            source_chain_id,
            target_chain_id,
            token_id,
            amount,
            sender_pubkey,
            recipient_length,
            recipient_bytes,
        })
    }
    
    /// 获取 sender_address 的 base58 编码
    pub fn sender_base58(&self) -> String {
        bs58::encode(&self.sender_pubkey).into_string()
    }
    
    /// 获取 recipient_address 的十六进制编码
    pub fn recipient_hex(&self) -> String {
        hex::encode(&self.recipient_bytes)
    }
    
    /// 获取 discriminator 的十六进制编码
    pub fn discriminator_hex(&self) -> String {
        hex::encode(&self.discriminator)
    }
}

/// Solana Bridge 事件枚举
/// 
/// 注意：事件类型由 anchor_lang::declare_program! 宏生成
/// 如果编译时找不到这些类型，请检查：
/// 1. IDL 文件中是否定义了 events 字段
/// 2. declare_program! 宏是否正确调用
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SolanaBridgeEvent {
    // 这些类型需要根据实际生成的代码调整
    // TokenAddedEvent,
    TokensDeposited(TokensDeposited),
    // TokensClaimed,
    // BlocklistUpdatedEvent,
    // ChainLimitUpdated,
    // SingleTransferLimitUpdated,
    // TokenPriceUpdated,
    // EmergencyOperation,
    // ProgramUpgradeEvent,
    
    // 临时使用原始数据，直到确认生成的事件类型
    RawEvent {
        discriminator: [u8; 8],
        data: Vec<u8>,
    },
}

impl SolanaBridgeEvent {
    /// 从交易日志中解析事件
    pub fn try_from_logs(log: &SolanaLog) -> Vec<SolanaBridgeEvent> {
        let mut parsed_events = Vec::new();

        // Anchor 事件格式: "Program data: <base64>"
        // 或者: "Program log: <message>"
        for log_msg in &log.log_messages {
            // 查找程序日志中的事件数据
            if let Some(event_data) = Self::extract_event_data(log_msg) {
                if event_data.len() >= 8 {
                    let discriminator: [u8; 8] = event_data[..8].try_into().unwrap();
                    
                    // 尝试解析为 TokensDeposited 事件
                    // TokensDeposited discriminator: [196, 217, 199, 88, 35, 117, 60, 96]
                    if discriminator == [196, 217, 199, 88, 35, 117, 60, 96] {
                        match TokensDeposited::parse_from_bytes(&event_data) {
                            Ok(tokens_deposited) => {
                                parsed_events.push(SolanaBridgeEvent::TokensDeposited(tokens_deposited));
                            }
                            Err(e) => {
                                // 如果解析失败，回退到 RawEvent
                                tracing::warn!("Failed to parse TokensDeposited: {}", e);
                                parsed_events.push(SolanaBridgeEvent::RawEvent {
                                    discriminator,
                                    data: event_data[8..].to_vec(),
                                });
                            }
                        }
                    } else {
                        // 其他事件类型，使用 RawEvent
                        let data = event_data[8..].to_vec();
                        parsed_events.push(SolanaBridgeEvent::RawEvent {
                            discriminator,
                            data,
                        });
                    }
                }
            }
        }

        parsed_events
    }

    pub fn try_from_client_transaction(
        tx: &crate::solana_client::SolanaTransaction,
    ) -> Vec<SolanaBridgeEvent> {
        let signature = tx
            .transaction
            .as_ref()
            .and_then(|t| t.signatures.get(0))
            .and_then(|s| Signature::from_str(s).ok())
            .unwrap_or_else(|| Signature::default());
        let slot = tx.slot.unwrap_or_default();
        let log_messages = tx
            .meta
            .as_ref()
            .map(|m| m.log_messages.clone())
            .unwrap_or_default();
        let sol_log = SolanaLog {
            signature,
            slot,
            log_messages,
        };
        Self::try_from_logs(&sol_log)
    }

    pub fn test_try_from_logs(log_msg: &str) -> Vec<SolanaBridgeEvent> {
        let log = SolanaLog {
            signature: Signature::default(),
            slot: 0,
            log_messages: vec![log_msg.to_string()],
        };
        let events = Self::try_from_logs(&log);
        // tracing::info!("events: {:?}", events);
        println!("events: {:?}", events);
        events
    }

    /// 从交易元数据中解析事件
    pub fn try_from_transaction(
        signature: Signature,
        _transaction: &EncodedConfirmedTransactionWithStatusMeta,
    ) -> Vec<SolanaBridgeEvent> {
        // 从 transaction 中提取日志消息
        // 注意：solana-transaction-status 的 API 可能不同，需要根据实际版本调整
        let log_messages = Vec::new(); // TODO: 根据实际 API 实现
        
        let solana_log = SolanaLog {
            signature,
            slot: 0, // TODO: 从 transaction 中提取 slot
            log_messages,
        };

        Self::try_from_logs(&solana_log)
    }

    /// 提取事件数据（从 "Program data: <base64>" 格式中提取）
    fn extract_event_data(log_msg: &str) -> Option<Vec<u8>> {
        // Anchor 事件格式: "Program data: <base64>"
        if let Some(data_str) = log_msg.strip_prefix("Program data: ") {
            base64::engine::general_purpose::STANDARD
                .decode(data_str.trim())
                .ok()
        } else {
            None
        }
    }

    /// 根据 discriminator 识别事件类型
    pub fn event_name(&self) -> Option<&'static str> {
        match self {
            SolanaBridgeEvent::TokensDeposited(_) => Some("TokensDeposited"),
            SolanaBridgeEvent::RawEvent { discriminator, .. } => {
                match discriminator {
                    [87, 57, 59, 239, 206, 101, 116, 48] => Some("TokenAddedEvent"),
                    [196, 217, 199, 88, 35, 117, 60, 96] => Some("TokensDeposited"),
                    [25, 128, 244, 55, 241, 136, 200, 91] => Some("TokensClaimed"),
                    [163, 84, 230, 167, 35, 82, 88, 113] => Some("BlocklistUpdatedEvent"),
                    [245, 55, 93, 189, 176, 28, 196, 37] => Some("ChainLimitUpdated"),
                    [186, 104, 0, 221, 229, 130, 121, 64] => Some("SingleTransferLimitUpdated"),
                    [173, 108, 54, 131, 134, 152, 186, 141] => Some("TokenPriceUpdated"),
                    [64, 209, 113, 233, 105, 207, 91, 160] => Some("EmergencyOperation"),
                    [206, 150, 127, 84, 112, 102, 128, 219] => Some("ProgramUpgradeEvent"),
                    _ => None,
                }
            }
        }
    }

    pub fn try_into_bridge_action(
        &self,
        solana_tx_signature: String,
        event_index: u16,
    ) -> BridgeResult<Option<BridgeAction>> {
        match self {
            SolanaBridgeEvent::TokensDeposited(tokens_deposited) => {
                let mut bridge_event = SolanaToSuiTokenBridgeV1::try_from(tokens_deposited)?;
                
                bridge_event.set_tx_signature(solana_tx_signature.as_bytes().to_vec());
                bridge_event.set_event_idx(event_index);
                
                let action = SolanaToSuiBridgeAction {
                    solana_tx_signature,
                    solana_event_index: event_index,
                    solana_bridge_event: bridge_event,
                };
                
                Ok(Some(BridgeAction::SolanaToSuiBridgeAction(action)))
            }
            SolanaBridgeEvent::RawEvent { discriminator, .. } => {
                tracing::debug!(
                    "Skipping RawEvent with discriminator: {:?}, event_name: {:?}",
                    discriminator,
                    self.event_name()
                );
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // xNnHWCN1PGABAAAAAAAAAD0CBAAAAAAAAAAAypo7AAAAAOXaYE6RS0pYLywuTnxDVWpNC5vNpLb2ZR5oXqMkmrq8IAAAAK6o6kznyCufMoNfXO4QV6GdxnPPcbMT248r8B8cx6ke
    #[test]
    fn test_try_from_logs_with_sample() {
        // The base64 sample string from the instructions
        let base64_str = "xNnHWCN1PGABAAAAAAAAAD0CBAAAAAAAAAAAypo7AAAAAOXaYE6RS0pYLywuTnxDVWpNC5vNpLb2ZR5oXqMkmrq8IAAAAK6o6kznyCufMoNfXO4QV6GdxnPPcbMT248r8B8cx6ke";
        let log_msg = format!("Program data: {}", base64_str);

        let event_result = SolanaBridgeEvent::test_try_from_logs(&log_msg.as_str());

        assert!(!event_result.is_empty(), "Event parse should succeed");
        
        // 验证解析结果
        match &event_result[0] {
            SolanaBridgeEvent::TokensDeposited(tokens_deposited) => {
                println!("== TokensDeposited ==");
                println!("discriminator(hex): {}", tokens_deposited.discriminator_hex());
                println!("nonce: {}", tokens_deposited.nonce);
                println!("source_chain_id: {}", tokens_deposited.source_chain_id);
                println!("target_chain_id: {}", tokens_deposited.target_chain_id);
                println!("token_id: {}", tokens_deposited.token_id);
                println!("amount: {}", tokens_deposited.amount);
                println!("sender_address(base58): {}", tokens_deposited.sender_base58());
                println!("recipient_address_len: {}", tokens_deposited.recipient_length);
                println!("recipient_address(hex): {}", tokens_deposited.recipient_hex());
                
                // 验证 discriminator
                assert_eq!(
                    tokens_deposited.discriminator,
                    [196, 217, 199, 88, 35, 117, 60, 96],
                    "Discriminator should match TokensDeposited"
                );
            }
            SolanaBridgeEvent::RawEvent { discriminator, .. } => {
                panic!("Expected TokensDeposited event, got RawEvent with discriminator: {:?}", discriminator);
            }
        }
    }

    /// Test parsing real Solana log data
    /// Real log from user:
    /// Program FVaTThSeeX4G5dHXqhTdqby9W6u77WDRMtBnfUpQasHm invoke [1]
    /// Program log: Instruction: MockCross
    /// In Program log: emit TokensDeposited
    /// Program data: xNnHWCN1PGABAAAAAAAAAD0CAwAAAAAAAABAQg8AAAAAAOXaYE6RS0pYLywuTnxDVWpNC5vNpLb2ZR5oXqMkmrq8IAAAAK6o6kznyCufMoNfXO4QV6GdxnPPcbMT248r8B8cx6ke
    /// Program FVaTThSeeX4G5dHXqhTdqby9W6u77WDRMtBnfUpQasHm consumed 1333 of 200000 compute units
    /// Program FVaTThSeeX4G5dHXqhTdqby9W6u77WDRMtBnfUpQasHm success
    #[test]
    fn test_parse_real_solana_log_data() {
        let base64_str = "xNnHWCN1PGABAAAAAAAAAD0CAwAAAAAAAABAQg8AAAAAAOXaYE6RS0pYLywuTnxDVWpNC5vNpLb2ZR5oXqMkmrq8IAAAAK6o6kznyCufMoNfXO4QV6GdxnPPcbMT248r8B8cx6ke";
        let log_msg = format!("Program data: {}", base64_str);

        let events = SolanaBridgeEvent::test_try_from_logs(&log_msg);
        assert_eq!(events.len(), 1, "Should parse exactly one event");

        match &events[0] {
            SolanaBridgeEvent::TokensDeposited(tokens_deposited) => {
                // Verify discriminator
                assert_eq!(
                    tokens_deposited.discriminator,
                    [196, 217, 199, 88, 35, 117, 60, 96],
                    "Discriminator should match TokensDeposited"
                );

                // Verify parsed fields
                assert_eq!(tokens_deposited.nonce, 1);
                assert_eq!(tokens_deposited.source_chain_id, 61); // 0x3D
                assert_eq!(tokens_deposited.target_chain_id, 2);
                assert_eq!(tokens_deposited.token_id, 3);
                assert_eq!(tokens_deposited.amount, 1000000);
                assert_eq!(tokens_deposited.recipient_length, 32);
                assert_eq!(tokens_deposited.recipient_bytes.len(), 32);

                // Verify sender pubkey (base58)
                assert_eq!(
                    tokens_deposited.sender_base58(),
                    "GUFVktRxvzKofrHb8htuAKB5gWj3sdbXchznjro9aVU7"
                );

                // Verify recipient address (hex)
                assert_eq!(
                    tokens_deposited.recipient_hex(),
                    "aea8ea4ce7c82b9f32835f5cee1057a19dc673cf71b313db8f2bf01f1cc7a91e"
                );

                println!("== Real Solana Log Parsed Successfully ==");
                println!("nonce: {}", tokens_deposited.nonce);
                println!("source_chain_id: {}", tokens_deposited.source_chain_id);
                println!("target_chain_id: {}", tokens_deposited.target_chain_id);
                println!("token_id: {}", tokens_deposited.token_id);
                println!("amount: {}", tokens_deposited.amount);
                println!("sender(base58): {}", tokens_deposited.sender_base58());
                println!("recipient(hex): {}", tokens_deposited.recipient_hex());
            }
            SolanaBridgeEvent::RawEvent { discriminator, .. } => {
                panic!("Expected TokensDeposited, got RawEvent with discriminator: {:?}", discriminator);
            }
        }
    }
}
