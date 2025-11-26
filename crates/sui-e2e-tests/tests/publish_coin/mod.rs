
use std::path::PathBuf;
use anyhow::Error;
use jsonrpsee::http_client::HttpClient;
use sui::client_commands::{SuiClientCommandResult, SuiClientCommands};
use sui_json_rpc_api::IndexerApiClient;
use sui_json_rpc_types::{ObjectChange, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery};
use sui_move_build::BuildConfig;
use sui_sdk::wallet_context::WalletContext;
use sui_test_transaction_builder::TestTransactionBuilder;
use sui_types::base_types::{ObjectID, ObjectRef, ObjectType, SuiAddress};
use sui_types::transaction::{Transaction, TEST_ONLY_GAS_UNIT_FOR_PUBLISH};
use test_cluster::TestCluster;
use sui::client_commands::{PaymentArgs, GasDataArgs, TxProcessingArgs};
use sui_json_rpc_types::SuiTransactionBlockResponse;
#[allow(unused)]
pub async fn do_publish(test_cluster: &mut TestCluster,path:&str) -> Result<(ObjectID, Vec<ObjectChange>), Error> {
    let address = test_cluster.get_address_0();

    let rgp = test_cluster.get_reference_gas_price().await;
    let mut context = &mut test_cluster.wallet;
    let client = context.get_client().await?;
    let object_refs = client
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    // Check log output contains all object ids.
    let gas_obj = object_refs.iter().find(|r: &&SuiObjectResponse| {
        if r.data.as_ref().unwrap().type_.as_ref().unwrap().is_gas_coin() {
            return true;
        }

        false
    }).unwrap().object().unwrap();
    let gas_obj_id = &gas_obj.object_id;
    //step 1: publish coin
    let resp = do_publish_inner(rgp, &mut context, gas_obj_id,path).await?;

    // Print it out to CLI/logs
    // resp.print(true);

    match resp {
        SuiClientCommandResult::TransactionBlock(tx) => {
            for ele in tx.object_changes.clone().unwrap() {
                if let ObjectChange::Published { package_id, .. } = ele {
                    return Ok((package_id, tx.object_changes.unwrap().clone()));
                }
            }
            Err(anyhow::anyhow!("no package found"))
        }
        _ => {
            Err(anyhow::anyhow!("resp type error"))
        }
    }

}

async fn do_publish_inner(rgp: u64, context: &mut WalletContext, gas_obj_id: &ObjectID,path:&str) -> Result<SuiClientCommandResult, Error> {
    let mut package_path = PathBuf::from(path);
    package_path.push("sources");
    let build_config = BuildConfig::new_for_testing().config;
    let resp = SuiClientCommands::Publish {
        package_path: package_path.clone(),
        build_config,
        skip_dependency_verification: false,
        with_unpublished_dependencies: false,
        verify_deps: true,
        payment: PaymentArgs {
            gas: vec![*gas_obj_id],
        },
        gas_data: GasDataArgs {
            gas_budget: Some(rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH),
            ..Default::default()
        },
        processing: TxProcessingArgs::default(),
    }
        .execute(context)
        .await?;
    Ok(resp)
}

#[allow(unused)]
pub async fn do_mint(test_cluster: &mut TestCluster, package: ObjectID) {
    let context = &test_cluster.wallet;
    let address = test_cluster.get_address_0();
    let gas = context
        .get_one_gas_object_owned_by_address(address)
        .await
        .unwrap()
        .unwrap();
    let cap = get_cap(&test_cluster.rpc_client().clone(), address).await;
    let cap_obj_ref = cap.unwrap().object().unwrap().object_ref();
    let mint_tx = make_mint_test_coin_transaction(context, address, gas, package, cap_obj_ref, 10000000000000).await;
    test_cluster.execute_transaction(mint_tx).await;
}

#[allow(unused)]
pub async fn do_mint_anonymous(test_cluster: &mut TestCluster, package: ObjectID, mint_amount: u64) -> SuiTransactionBlockResponse {
    let context = &test_cluster.wallet;
    let address = test_cluster.get_address_0();
    let gas = context
        .get_one_gas_object_owned_by_address(address)
        .await
        .unwrap()
        .unwrap();
    let cap = get_cap(&test_cluster.rpc_client().clone(), address).await;
    let cap_obj_ref = cap.unwrap().object().unwrap().object_ref();
    let mint_tx = make_mint_test_anonymous_coin_transaction(context, address, gas, package, cap_obj_ref, mint_amount).await;
    test_cluster.execute_transaction(mint_tx).await
}

#[allow(unused)]
pub async fn do_mint_ausd(test_cluster: &mut TestCluster, package: ObjectID, mint_amount: u64) -> SuiTransactionBlockResponse {
    let context = &test_cluster.wallet;
    let address = test_cluster.get_address_0();
    let gas = context
        .get_one_gas_object_owned_by_address(address)
        .await
        .unwrap()
        .unwrap();
    let cap = get_cap(&test_cluster.rpc_client().clone(), address).await;
    let cap_obj_ref = cap.unwrap().object().unwrap().object_ref();
    let mint_tx = make_mint_test_ausd_transaction(context, address, gas, package, cap_obj_ref, mint_amount).await;
    test_cluster.execute_transaction(mint_tx).await
}

async fn make_mint_test_coin_transaction(
    context: &WalletContext,
    sender :SuiAddress,
    gas_object :ObjectRef,
    package_id:ObjectID,
    treasury_cap: ObjectRef,
    amount:u64,
) -> Transaction {
    let addresses= context.get_addresses();
    let recipient= addresses.first().unwrap();
    let gas_price = context.get_reference_gas_price().await.unwrap();
    println!("sender:{:?} recipient:{:?}",sender,recipient);
    context.sign_transaction(
        &TestTransactionBuilder::new(sender, gas_object, gas_price)
            .call_mint_test_coin(package_id,treasury_cap,amount,*recipient)
            .build(),
    )
}

async fn make_mint_test_anonymous_coin_transaction(
    context: &WalletContext,
    sender :SuiAddress,
    gas_object :ObjectRef,
    package_id:ObjectID,
    treasury_cap: ObjectRef,
    amount:u64,
) -> Transaction {
    let addresses= context.get_addresses();
    let recipient= addresses.first().unwrap();
    let gas_price = context.get_reference_gas_price().await.unwrap();
    println!("sender:{:?} recipient:{:?}",sender,recipient);
    context.sign_transaction(
        &TestTransactionBuilder::new(sender, gas_object, gas_price)
            .call_mint_test_anonymous_coin(package_id,treasury_cap,amount,*recipient)
            .build(),
    )
}

async fn make_mint_test_ausd_transaction(
    context: &WalletContext,
    sender :SuiAddress,
    gas_object :ObjectRef,
    package_id:ObjectID,
    treasury_cap: ObjectRef,
    amount:u64,
) -> Transaction {
    let addresses= context.get_addresses();
    let recipient= addresses.first().unwrap();
    let gas_price = context.get_reference_gas_price().await.unwrap();
    println!("sender:{:?} recipient:{:?}",sender,recipient);
    context.sign_transaction(
        &TestTransactionBuilder::new(sender, gas_object, gas_price)
            .call_mint_test_ausd_coin(package_id,treasury_cap,amount,*recipient)
            .build(),
    )
}

//默认返回第一个TreasuryCap
#[allow(unused)]
async fn get_cap(http_client: &HttpClient, address: SuiAddress) -> Result<SuiObjectResponse, anyhow::Error> {
    let objects = get_coin_object(http_client, address).await?;
    let cap = objects.into_iter().find(|ele| {
        if let Some(data) = &ele.data {
            if let Some(ObjectType::Struct(tp)) = &data.type_ {
                if tp.name().to_string() == "TreasuryCap" {
                    return true;
                }
            }
        }
        false
    });
    if let Some(cap) = cap {
        Ok(cap)
    } else {
        Err(anyhow::anyhow!("no cap found"))
    }
}

#[allow(unused)]
async fn get_coin_object(http_client: &HttpClient, address: SuiAddress) -> Result<Vec<SuiObjectResponse>, anyhow::Error> {
    let data_option = SuiObjectDataOptions::new()
        .with_type()
        .with_owner()
        .with_previous_transaction()
        .with_content();
    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new(
                None,
                Some(data_option),
            )),
            None,
            None,
        )
        .await?
        .data;
    // println!("objects {:?}",objects);
    Ok(objects)
}