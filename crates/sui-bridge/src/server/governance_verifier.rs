// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::time::Duration;

use crate::error::{BridgeError, BridgeResult};
use crate::server::handler::ActionVerifier;
use crate::types::{AssetPriceUpdateAction, BridgeAction, BridgeActionDigest};
use reqwest::Client;

const ASSET_PRICE_MAX_DEVIATION_PERCENT: u128 = 10;
const USD_PRICE_DECIMALS: u128 = 100_000_000;

#[derive(Debug)]
pub struct GovernanceVerifier {
    approved_goverance_actions: HashMap<BridgeActionDigest, BridgeAction>,
    price_client: Client,
}

impl GovernanceVerifier {
    pub fn new(approved_actions: Vec<BridgeAction>) -> BridgeResult<Self> {
        // TOOD(audit-blocking): verify chain ids
        let mut approved_goverance_actions = HashMap::new();
        for action in approved_actions {
            if !action.is_governace_action() {
                return Err(BridgeError::ActionIsNotGovernanceAction(action));
            }
            approved_goverance_actions.insert(action.digest(), action);
        }
        Ok(Self {
            approved_goverance_actions,
            price_client: Client::builder()
                .connect_timeout(Duration::from_secs(2))
                .timeout(Duration::from_secs(2))
                .build()
                .expect("failed to create http client"),
        })
    }
    //todo: change to use PriceClient
    fn token_id_to_symbol(token_id: u64) -> BridgeResult<&'static str> {
        match token_id {
            0 => Ok("SUI"),
            1 => Ok("BTC"),
            2 => Ok("ETH"),
            3 => Ok("USDC"),
            4 => Ok("USDT"),
            5 => Ok("BUSD"),
            6 => Ok("BNB"),
            7 => Ok("OP"),
            8 => Ok("POL"),
            _ => Err(BridgeError::UnknownTokenId(token_id)),
        }
    }

    async fn get_reference_price(&self, token_id: u64) -> BridgeResult<u128> {
        if token_id == 3 || token_id == 4 || token_id == 5 {
            return Ok(USD_PRICE_DECIMALS);
        }
        let symbol = Self::token_id_to_symbol(token_id)?;

        let symbol = format!("{symbol}USDT");
        let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={symbol}");
        let resp = self
            .price_client
            .get(&url)
            .send()
            .await
            .map_err(|err| {
                BridgeError::InvalidBridgeClientRequest(format!(
                    "Failed to fetch reference price for {symbol}: {err}"
                ))
            })?
            .json::<serde_json::Value>()
            .await
            .map_err(|err| {
                BridgeError::InvalidBridgeClientRequest(format!(
                    "Failed to decode reference price for {symbol}: {err}"
                ))
            })?;

        resp["price"]
            .as_str()
            .ok_or_else(|| {
                BridgeError::InvalidBridgeClientRequest(format!(
                    "Reference price field missing for {symbol}"
                ))
            })?
            .parse::<f64>()
            .map(|price| (price * USD_PRICE_DECIMALS as f64).round() as u128)
            .map_err(|err| {
                BridgeError::InvalidBridgeClientRequest(format!(
                    "Failed to parse reference price for {symbol}: {err}"
                ))
            })
    }

    fn is_price_within_deviation(action_price: u64, reference_price: u128) -> bool {
        let action_price = action_price as u128;
        if reference_price == 0 {
            return false;
        }

        let diff = action_price.abs_diff(reference_price);
        diff * 100 <= reference_price * ASSET_PRICE_MAX_DEVIATION_PERCENT
    }

    async fn verify_asset_price_update(
        &self,
        action: &AssetPriceUpdateAction,
    ) -> BridgeResult<()> {
        if action.token_id == 3 || action.token_id == 4 || action.token_id == 5 {
            return Ok(());
        }
        let reference_price = self.get_reference_price(action.token_id).await?;
        if Self::is_price_within_deviation(action.new_usd_price, reference_price) {
            return Ok(());
        }

        Err(BridgeError::InvalidBridgeClientRequest(format!(
            "Asset price update out of range for token {}: proposed {}, reference {}",
            action.token_id, action.new_usd_price, reference_price
        )))
    }
}

#[async_trait::async_trait]
impl ActionVerifier<BridgeAction> for GovernanceVerifier {
    fn name(&self) -> &'static str {
        "GovernanceVerifier"
    }

    async fn verify(&self, key: BridgeAction) -> BridgeResult<BridgeAction> {
        // TODO: an optimization would be to check the current nonce on chain and err for older ones
        if !key.is_governace_action() {
            return Err(BridgeError::ActionIsNotGovernanceAction(key));
        }
        if let BridgeAction::AssetPriceUpdateAction(action) = &key {
            self.verify_asset_price_update(action).await?;
            return Ok(key);
        }
        if let Some(approved_action) = self.approved_goverance_actions.get(&key.digest()) {
            assert_eq!(
                &key, approved_action,
                "Mismatched action found in approved_actions"
            );
            return Ok(key);
        }
        return Err(BridgeError::GovernanceActionIsNotApproved);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_utils::get_test_sui_to_eth_bridge_action,
        types::{BridgeAction, EmergencyAction, EmergencyActionType, LimitUpdateAction},
    };
    use sui_types::bridge::BridgeChainId;

    #[tokio::test]
    async fn test_governance_verifier() {
        let action_1 = BridgeAction::EmergencyAction(EmergencyAction {
            chain_id: BridgeChainId::EthCustom,
            nonce: 1,
            action_type: EmergencyActionType::Pause,
        });
        let action_2 = BridgeAction::LimitUpdateAction(LimitUpdateAction {
            chain_id: BridgeChainId::EthCustom,
            sending_chain_id: BridgeChainId::SuiCustom,
            nonce: 1,
            new_usd_limit: 10000,
        });

        let verifier = GovernanceVerifier::new(vec![action_1.clone(), action_2.clone()]).unwrap();
        assert_eq!(
            verifier.verify(action_1.clone()).await.unwrap(),
            action_1.clone()
        );
        assert_eq!(
            verifier.verify(action_2.clone()).await.unwrap(),
            action_2.clone()
        );

        let action_3 = BridgeAction::LimitUpdateAction(LimitUpdateAction {
            chain_id: BridgeChainId::EthCustom,
            sending_chain_id: BridgeChainId::SuiCustom,
            nonce: 2,
            new_usd_limit: 10000,
        });
        assert_eq!(
            verifier.verify(action_3).await.unwrap_err(),
            BridgeError::GovernanceActionIsNotApproved
        );

        // Token transfer action is not allowed
        let action_4 = get_test_sui_to_eth_bridge_action(None, None, None, None, None, None, None);
        assert!(matches!(
            GovernanceVerifier::new(vec![action_1, action_2, action_4.clone()]).unwrap_err(),
            BridgeError::ActionIsNotGovernanceAction(..)
        ));

        // Token transfer action will be rejected
        assert!(matches!(
            verifier.verify(action_4).await.unwrap_err(),
            BridgeError::ActionIsNotGovernanceAction(..)
        ));
    }

    #[test]
    fn test_token_id_to_symbol() {
        assert_eq!(GovernanceVerifier::token_id_to_symbol(2).unwrap(), "ETH");
        assert_eq!(
            GovernanceVerifier::token_id_to_symbol(999).unwrap_err(),
            BridgeError::UnknownTokenId(999)
        );
    }

    #[test]
    fn test_asset_price_deviation_check() {
        assert!(GovernanceVerifier::is_price_within_deviation(
            100_000_000,
            100_000_000
        ));
        assert!(GovernanceVerifier::is_price_within_deviation(
            110_000_000,
            100_000_000
        ));
        assert!(!GovernanceVerifier::is_price_within_deviation(
            111_000_000,
            100_000_000
        ));
    }
}
