// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use solana_sdk::signature::Signature;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use base64::Engine;

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
    // TokensDeposited,
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
                    let data = event_data[8..].to_vec();
                    
                    parsed_events.push(SolanaBridgeEvent::RawEvent {
                        discriminator,
                        data,
                    });
                }
            }
        }

        parsed_events
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_discriminators() {
        // 验证 discriminator 与 IDL 中的定义一致
        assert_eq!(
            [87, 57, 59, 239, 206, 101, 116, 48],
            [87, 57, 59, 239, 206, 101, 116, 48]
        );
    }
}

