
use crate::gas_coin_commands::{GasCoinCommand};
use test_cluster::TestClusterBuilder;
use sui_types::base_types::ObjectID;
use sui_types::object::Object;

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