use crate::gas_coin_commands::{GasCoinCommandResponse, GasCoinCommand};
use crate::sui_commands::SuiCommand::GasCoin;
use sui_types::gas_coin::GasCoin as GasCoinObj;
use test_utils::network::TestClusterBuilder;
use sui_types::base_types::{ObjectID};

#[tokio::test]
async fn test_gas_coin_config() -> Result<(), anyhow::Error> {
    let test_cluster = TestClusterBuilder::new().build().await?;
    let mut context = test_cluster.wallet;
    let sui_client = context.get_client().await?;
    //execute add gas coin
    let id = ObjectID::random();
    let value = 10000;
    // let coin = GasCoinObj::new(id, value);

    let response = GasCoinCommand::AddGasCoin { coin_address: id, gas_budget: Some(value) }.execute(&mut context).await?;
    Ok(())
}