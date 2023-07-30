
use anyhow::{anyhow, Result};
use clap::*;
use sui_json_rpc_types::SuiObjectDataOptions;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::{ObjectID, ObjectRef};
use sui_types::transaction::{CallArg, ObjectArg};
use crate::validator_commands::call_0x5;
use serde::Serialize;

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
                    call_0x5(context, "request_add_gas_coin", args, gas_budget).await?;
                GasCoinCommandResponse::AddGasCoin
            },
            GasCoinCommand::RemoveGasCoin { coin_address,gas_budget} => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let coin_to_merge_ref = get_object_ref(context, coin_address).await?;
                let args = vec![
                    CallArg::Object(ObjectArg::ImmOrOwnedObject(coin_to_merge_ref))
                ];
                let _response =
                    call_0x5(context, "request_remove_gas_coin", args, gas_budget).await?;
                GasCoinCommandResponse::RemoveGasCoin
            }
        });
        ret
    }
}
/// Get gas coin object ref.
async fn get_object_ref(
    context: &mut WalletContext,
    coin_id: ObjectID,
) -> Result<ObjectRef> {
    let sui_client = context.get_client().await?;
    let gas_obj_ref = sui_client
        .read_api()
        .get_object_with_options(
            coin_id,
            SuiObjectDataOptions::default().with_owner(),
        )
        .await?
        .object_ref_if_exists()
        .ok_or_else(|| anyhow!("gas coin {} does not exist", coin_id))?;
    Ok::<ObjectRef, anyhow::Error>(
        gas_obj_ref,
    )
}