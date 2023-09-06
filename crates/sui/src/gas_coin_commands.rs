
use anyhow::Result;
use clap::*;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::transaction::{CallArg, ObjectArg};
use crate::{call_0x200, get_object_ref};
use serde::Serialize;

#[path = "unit_tests/gas_coin_tests.rs"]
#[cfg(test)]
mod gas_coin_tests;

const DEFAULT_GAS_BUDGET: u64 = 100_000_000; // 0.1 SUI

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum GasCoinCommand {
    #[clap(name = "make-validator-info")]
    AddGasCoin {
        /// Address of the gas coin.
        #[clap(name = "coin-address")]
        coin_address: ObjectID,
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
    },
    RemoveGasCoin {
        /// Address of the gas coin.
        #[clap(name = "coin-address")]
        coin_address: ObjectID,
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum GasCoinCommandResponse {
    AddGasCoin,
    RemoveGasCoin,
}
impl GasCoinCommandResponse {
    pub fn print(&self, _pretty: bool) {
        match self {
            // Don't print empty responses
            GasCoinCommandResponse::AddGasCoin
            | GasCoinCommandResponse::RemoveGasCoin => {}
        }
    }
}

impl GasCoinCommand {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> Result<GasCoinCommandResponse, anyhow::Error> {
        let ret = Ok(match self {
            GasCoinCommand::AddGasCoin { coin_address, gas_budget} => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let coin_to_merge_ref = get_object_ref(context,coin_address).await?;
                let args = vec![
                    CallArg::Object(ObjectArg::ImmOrOwnedObject(coin_to_merge_ref))
                ];
                let _response =
                    call_0x200(context, "request_add_gas_coin", args, gas_budget).await?;
                GasCoinCommandResponse::AddGasCoin
            },
            GasCoinCommand::RemoveGasCoin { coin_address,gas_budget} => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let coin_to_merge_ref = get_object_ref(context, coin_address).await?;
                let args = vec![
                    CallArg::Object(ObjectArg::ImmOrOwnedObject(coin_to_merge_ref))
                ];
                let _response =
                    call_0x200(context, "request_remove_gas_coin", args, gas_budget).await?;
                GasCoinCommandResponse::RemoveGasCoin
            }
        });
        ret
    }
}