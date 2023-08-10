
use crate::gas_coin_commands::{GasCoinCommand};
use test_cluster::TestClusterBuilder;
use sui_types::base_types::ObjectID;
use sui_types::coin::Coin;
use sui_types::id::UID;
use sui_types::object::Object;
use crate::inner_swap::InnerSwap;

#[tokio::test]
async fn test_gas_coin_config() -> Result<(), anyhow::Error> {
    let gas_id = ObjectID::random();
    let test_cluster = TestClusterBuilder::new()
        .with_objects([Object::immutable_with_id_for_testing(gas_id)])
        .build()
        .await;
    let mut context = test_cluster.wallet;
    //execute add gas coin
    let value = 10000000;
    GasCoinCommand::AddGasCoin { coin_address: gas_id, gas_budget: Some(value) }.execute(&mut context).await?.print(true);
    //execute remove gas coin
    GasCoinCommand::RemoveGasCoin { coin_address: gas_id, gas_budget: Some(value) }.execute(&mut context).await?.print(true);
    Ok(())
}

#[tokio::test]
async fn test_inner_swap()  -> Result<(), anyhow::Error> {
    let gas_id = ObjectID::random();
    let test_cluster = TestClusterBuilder::new()
        .with_objects([Object::immutable_with_id_for_testing(gas_id)])
        .build()
        .await;
    let mut context = test_cluster.wallet;

    let _result = InnerSwap::exchange_obc(&mut context, Coin::new(UID::new(gas_id), 1000)).await;
    Ok(())
}