use anyhow::anyhow;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::Utc;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::DeserializeOwned;
use jsonrpsee::http_client::HttpClient;
use move_core_types::account_address::AccountAddress;
use move_core_types::parser::parse_type_tag;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Div;
use sui_json_rpc_api::{IndexerApiClient, ReadApiClient, WriteApiClient};
use sui_json_rpc_types::DevInspectResults;
use sui_json_rpc_types::NFTStakingOverview;
use sui_json_rpc_types::SuiMiningNFTFutureReward;
use sui_json_rpc_types::SuiMiningNFTProfitRate;
use sui_json_rpc_types::SuiObjectDataOptions;
use sui_json_rpc_types::SuiObjectResponse;
use sui_json_rpc_types::SuiOwnedMiningNFTProfit;
use sui_json_rpc_types::SuiParsedData;
use sui_types::base_types::ObjectID;
use sui_types::base_types::SuiAddress;
use sui_types::collection_types::LinkedTable;
use sui_types::dynamic_field::DynamicFieldName;
use sui_types::dynamic_field::Field;
use sui_types::error::SuiError;
use sui_types::gas_coin::MIST_PER_SUI;
use sui_types::id::ID;
use sui_types::id::UID;
use sui_types::object::Object;
use sui_types::transaction::ObjectArg;
use sui_types::{
    transaction::{Argument, CallArg, Command, ProgrammableTransaction, TransactionKind},
    Identifier, TypeTag, BFC_SYSTEM_PACKAGE_ID, BFC_SYSTEM_STATE_OBJECT_ID,
};

use crate::errors::IndexerError;
use crate::IndexerConfig;

// Rust types of VaultInfo in crates/sui-framework/packages/bfc-system/sources/treasury/vault.move
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultInfo {
    pub vault_id: ID,
    pub position_number: u32,
    pub state: u8,
    pub last_rebalance_state: u8,
    pub state_counter: u32,
    pub max_counter_times: u32,
    pub last_sqrt_price: u128,
    pub coin_a_balance: u64,
    pub coin_b_balance: u64,
    pub coin_a_type: String,
    pub coin_b_type: String,
    pub tick_spacing: u32,
    pub spacing_times: u32,
    pub liquidity: u128,
    pub current_sqrt_price: u128,
    pub current_tick_index: u32,
    pub is_pause: bool,
    pub index: u64,
    pub base_point: u64,
    pub coin_market_cap: u64,
    pub last_bfc_rebalance_amount: u64,
}

pub async fn get_bfc_price_in_usd(http_client: HttpClient) -> Result<f64, IndexerError> {
    let price =
        get_bfc_price_in_stable_coin(parse_type_tag("0xc8::busd::BUSD").unwrap(), http_client)
            .await?;
    Ok(price)
}

pub async fn get_bfc_value_of_stable_coin(
    coin: TypeTag,
    amount: u64,
    http_client: HttpClient,
) -> Result<u64, IndexerError> {
    let price = get_bfc_price_in_stable_coin(coin, http_client).await?;
    Ok((amount as f64 / price) as u64)
}

// https://github.com/hellokittyboy-code/obc/blob/1d3e3f066b59c4dd5a7395adf728a4878b8dc48a/apps/explorer/src/hooks/useTokenPrice.ts#L30-L68
pub async fn get_bfc_price_in_stable_coin(
    coin: TypeTag,
    http_client: HttpClient,
) -> Result<f64, IndexerError> {
    let tx = &build_vault_info_tx(coin.clone())?;
    let val: VaultInfo = dev_inspect_tx(http_client, tx).await.map_err(|err| {
        IndexerError::FullNodeReadingError(format!(
            "Failed to read VaultInfo with error: {:?}",
            err
        ))
    })?;
    return calculate_price(val.current_sqrt_price);
}

fn build_vault_info_tx(coin: TypeTag) -> Result<TransactionKind, IndexerError> {
    let tx = ProgrammableTransaction {
        inputs: vec![CallArg::Object(ObjectArg::SharedObject {
            id: BFC_SYSTEM_STATE_OBJECT_ID,
            initial_shared_version: 1u64.into(),
            mutable: false,
        })],
        commands: vec![Command::move_call(
            BFC_SYSTEM_PACKAGE_ID,
            Identifier::new("bfc_system")?,
            Identifier::new("vault_info")?,
            vec![coin.clone()],
            vec![Argument::Input(0)],
        )],
    };
    Ok(TransactionKind::ProgrammableTransaction(tx))
}

async fn dev_inspect_tx<T: DeserializeOwned>(
    http_client: HttpClient,
    tx: &TransactionKind,
) -> Result<T, IndexerError> {
    let tx = Base64::from_bytes(&bcs::to_bytes(tx)?);
    let results: DevInspectResults = http_client
        .dev_inspect_transaction_block(AccountAddress::ZERO.into(), tx, None, None, None)
        .await
        .map_err(|err| {
            IndexerError::FullNodeReadingError(
                format!("Failed to dry run tx with error {:?}", err,),
            )
        })?;
    if let Some(err) = results.error {
        return Err(IndexerError::FullNodeReadingError(format!(
            "Failed to dry run tx with error {:?}",
            err,
        )));
    }

    if let Some(results) = results.results {
        if results.len() > 0 && results[0].return_values.len() > 0 {
            let val = bcs::from_bytes::<T>(&results[0].return_values[0].0)?;
            return Ok(val);
        }
    }
    Err(IndexerError::FullNodeReadingError(format!(
        "No value returned when dry run tx",
    )))
}

fn calculate_price(current_sqrt_price: u128) -> Result<f64, IndexerError> {
    let current_sqrt_price =
        Decimal::from_str_exact(&format!("{}", current_sqrt_price)).map_err(|err| {
            IndexerError::FullNodeReadingError(format!(
                "Failed to convert u128 to decimal with error {:?}",
                err
            ))
        })?;

    current_sqrt_price
        .div(Decimal::from_i128_with_scale(2 << 63, 0))
        .powi(2)
        .powi(-1)
        .try_into()
        .map_err(|err| {
            IndexerError::FullNodeReadingError(format!(
                "Failed to convert decimal to f64 with error {:?}",
                err,
            ))
        })
}

async fn get_mining_nft_cost_in_usd(
    config: IndexerConfig,
    http_client: HttpClient,
) -> Result<f64, IndexerError> {
    let object_id = ObjectID::from_hex_literal(&config.mining_nft_pool_id)?;
    let response: SuiObjectResponse = http_client
        .get_object(
            object_id.clone(),
            Some(SuiObjectDataOptions::bcs_lossless().with_content()),
        )
        .await
        .map_err(|err| {
            IndexerError::FullNodeReadingError(format!(
                "Failed to read object {:?} with error {:?}",
                object_id, err
            ))
        })?;
    let object = response.object()?;
    if let Some(content) = &object.content {
        if let SuiParsedData::MoveObject(move_obj) = content {
            let value: Value = move_obj.fields.clone().to_json_value();
            if let Some(cur_sqrt_price_val) = value.get("current_sqrt_price") {
                if let Some(cur_sqrt_price) = cur_sqrt_price_val.as_str() {
                    return Ok(get_price_from_sqrt_price(cur_sqrt_price)?);
                }
            }
        }
    }
    Err(IndexerError::FullNodeReadingError(format!(
        "Failed to extract current_sqrt_price from object: {:?}",
        object_id
    )))
}

fn get_price_from_sqrt_price(sqrt_price: &str) -> Result<f64, IndexerError> {
    let two: Decimal = 2u64.into();
    let div = two.powi(64); // 2^64
    let coin_a_price = Decimal::from_str_exact(sqrt_price)
        .map_err(|err| IndexerError::UncategorizedError(err.into()))?
        .div(&div)
        .powi(2);
    let one: Decimal = 1.into();
    Ok(one.div(coin_a_price).try_into().map_err(|err| {
        IndexerError::FullNodeReadingError(format!(
            "Failed to convert decimal to f64 with error {:?}",
            err,
        ))
    })?)
}

pub async fn get_mining_nft_cost_in_bfc(
    config: IndexerConfig,
    http_client: HttpClient,
) -> Result<u64, IndexerError> {
    let usd_cost = get_mining_nft_cost_in_usd(config, http_client.clone()).await?;
    let price = get_bfc_price_in_usd(http_client).await?;
    Ok((usd_cost / price * MIST_PER_SUI as f64) as u64)
}

pub fn timestamp_to_dt(timestamp_ms: i64) -> i64 {
    let date = timestamp_to_dt_string(timestamp_ms);
    let naive = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
    naive.and_time(NaiveTime::MIN).timestamp_millis()
}

fn timestamp_to_dt_string(timestamp_ms: i64) -> String {
    let naive = NaiveDateTime::from_timestamp_millis(timestamp_ms).unwrap_or_default();
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
    datetime.format("%Y-%m-%d").to_string()
}

pub fn get_yesterday_started_at() -> i64 {
    timestamp_to_dt(Utc::now().timestamp_millis() - 86_400_000)
}

fn get_shared_global_object_id(config: &IndexerConfig) -> ObjectID {
    let account_address = SuiAddress::from_str(&config.mining_nft_global).unwrap();
    ObjectID::from_address(account_address.into())
}

pub async fn get_nft_staking_overview(
    http_client: HttpClient,
    config: IndexerConfig,
    mut overall_profits: Vec<SuiOwnedMiningNFTProfit>,
    timestamp: u64,
) -> Result<NFTStakingOverview, IndexerError> {
    let latest_profit = if overall_profits.len() > 0 {
        overall_profits[overall_profits.len() - 1].clone()
    } else {
        SuiOwnedMiningNFTProfit {
            mint_bfc: 0,
            mint_usd: 0,
            cost_bfc: 0,
            dt_timestamp_ms: 0,
        }
    };

    let nft_staking = get_global_nft_staking(http_client.clone(), config.clone()).await?;
    let di = nft_staking.decrease_interval;
    let rps = nft_staking.reward_per_second;
    let diff: u64 = if timestamp > nft_staking.begin_at {timestamp - nft_staking.begin_at} else {0};
    let p: u64 = diff / di + 1; // nft_staking.period;
    let prev_total_reward: f64 = (1..p)
        .into_iter()
        .map(|n| di as f64 * current_period_rps(rps, n))
        .sum();
    let past_period_secs = di * (p - 1);
    let current_period_past_secs = if diff > past_period_secs {diff - past_period_secs} else {0};
    let current_total_reward = current_period_past_secs as f64 * current_period_rps(rps, p);
    let nft_config = get_global_nft_config(http_client, config).await?;
    let mut nft_future_rewards = vec![];
    let mut nft_future_profit_rates = vec![];
    let mut rewarded: u64 = 0;
    let mut overall_reward: u64 = 0;
    let max_count_per_day: u64 = 1000;
    for d in 0..180 {
        let new_suply = (d + 1) * max_count_per_day;
        let dp = timestamp + d * 86_400;
        let diff: u64 = if dp > nft_staking.begin_at {dp - nft_staking.begin_at} else {0};
        let p: u64 = diff / di + 1;
        let crps = current_period_rps(rps, p);
        overall_reward += (crps * 86_400f64) as u64;
        let current_supply = nft_config.total_supply + new_suply;
        rewarded += (crps * 86_400f64 / current_supply as f64) as u64;
        nft_future_rewards.push(SuiMiningNFTFutureReward {
            reward: rewarded,
            dt_timestamp_ms: dp * 1_000,
        });
        let nft_cost = calculate_nft_cost(nth_day(nft_staking.begin_at, dp));
        nft_future_profit_rates.push(SuiMiningNFTProfitRate {
            rate: rewarded as f64 / nft_cost as f64,
            dt_timestamp_ms: dp * 1_000,
        });
        if d < 30 {
            overall_profits.push(SuiOwnedMiningNFTProfit {
                mint_bfc: latest_profit.mint_bfc + overall_reward,
                mint_usd: 0,
                cost_bfc: latest_profit.cost_bfc + nft_cost * ((d + 1) * (max_count_per_day - 500)),
                dt_timestamp_ms: dp * 1_000,
            })
        }
    }

    Ok(NFTStakingOverview {
        total_power: nft_staking.total_power,
        reward_per_day: (86_400f64 * current_period_rps(rps, p)) as u64,
        total_rewarded: (prev_total_reward + current_total_reward) as u64,
        bfc_usd_price: 0f64,
        bfc_24h_rate: 0f64,
        nft_future_rewards,
        nft_future_profit_rates,
        overall_profit_rates: overall_profits
            .iter()
            .map(|x| SuiMiningNFTProfitRate {
                rate: if x.cost_bfc > 0 {
                    x.mint_bfc as f64 / x.cost_bfc as f64
                } else {
                    0f64
                },
                dt_timestamp_ms: x.dt_timestamp_ms,
            })
            .collect(),
        total_addresses: 0,
        total_long: 0,
    })
}

fn current_period_rps(rps: u64, p: u64) -> f64 {
    rps as f64 / (2f64.powi(p as i32 - 1) as f64)
}

fn nth_day(begin_at: u64, now: u64) -> u64 {
    let diff = if now > begin_at {now - begin_at} else {0};
    diff / 86_400 + 1
}

fn calculate_nft_cost(n: u64) -> u64 {
    let m: u64 = 0;
    let l = 259_200f64 / 1_000f64;
    let t = if n < 180 { 60f64 } else { 180f64 };
    ((1f64 + t / (n as f64 + m as f64)).ln() * l) as u64 * MIST_PER_SUI
}

const NFT_SHARED_GLOBAL_STAKING_FIELD: &'static str = "4";
const NFT_SHARED_GLOBAL_NFT_FIELD: &'static str = "2";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct NFTConfig {
    pub token_id: u64,
    pub total_supply: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct NFTStaking {
    id: UID,
    pub paused: bool, // 质押是否暂停
    bfc_balance: u64, // 矿池总额
    begin_at: u64,    // 开始质押时间

    // reward information
    period: u64,            // 当前周期数
    reward_per_second: u64, // 每秒产出的 bfc
    decrease_interval: u64, // 减半周期
    reward_per_power: u64,  // 每一份算力的奖励
    total_power: u64,       // 全网总算力
    last_reward_ts: u64,    // 上次更新奖励的时间

    // tickets
    items: LinkedTable<ID>, // 质押的 NFT
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StakeItem {
    miner_id: UID,
    owner: AccountAddress,
    debt: u64,
}

pub async fn get_global_nft_staking(
    http_client: HttpClient,
    config: IndexerConfig,
) -> Result<NFTStaking, IndexerError> {
    let object = get_dynamic_field_object(
        http_client,
        get_shared_global_object_id(&config),
        TypeTag::U64,
        NFT_SHARED_GLOBAL_STAKING_FIELD,
    )
    .await
    .map_err(|err| {
        IndexerError::FullNodeReadingError(format!(
            "Failed to load Staking object from its parent Global object {:?} with error {:?}",
            get_shared_global_object_id(&config),
            err
        ))
    })?;
    let config: NFTStaking = dynamic_field_from_object::<u64, NFTStaking>(&object)?;
    Ok(config)
}

async fn get_global_nft_config(
    http_client: HttpClient,
    config: IndexerConfig,
) -> Result<NFTConfig, IndexerError> {
    let object = get_dynamic_field_object(
        http_client,
        get_shared_global_object_id(&config),
        TypeTag::U64,
        NFT_SHARED_GLOBAL_NFT_FIELD,
    )
    .await?;
    let config: NFTConfig = dynamic_field_from_object::<u64, NFTConfig>(&object)?;
    Ok(config)
}

pub async fn get_dynamic_field_object<'a, N>(
    http_client: HttpClient,
    owner: ObjectID,
    field_type: TypeTag,
    field_name: N,
) -> Result<Object, IndexerError>
where
    N: Serialize + Deserialize<'a>,
{
    let response: SuiObjectResponse = http_client
        .get_dynamic_field_object(
            owner,
            DynamicFieldName {
                type_: field_type,
                value: serde_json::to_value(field_name).map_err(|err| {
                    IndexerError::SerdeError(format!(
                        "Failed encode field_name to json with error {:?}",
                        err
                    ))
                })?,
            },
        )
        .await
        .map_err(|err| {
            IndexerError::FullNodeReadingError(format!(
                "Failed to read stable pool dynamic field with error {:?}",
                err,
            ))
        })?;

    let object_id = response.object_id()?;
    let response: SuiObjectResponse = http_client
        .get_object(
            object_id.clone(),
            Some(SuiObjectDataOptions::bcs_lossless()),
        )
        .await
        .map_err(|err| {
            IndexerError::FullNodeReadingError(format!(
                "Failed to read stable pool dynamic field object {:?}",
                err
            ))
        })?;
    let object: Object = response.object()?.to_owned().try_into()?;
    Ok(object)
}

pub fn dynamic_field_from_object<'a, N, V>(object: &'a Object) -> Result<V, IndexerError>
where
    N: Deserialize<'a>,
    V: Deserialize<'a>,
{
    let move_object = object.data.try_as_move().ok_or_else(|| {
        SuiError::DynamicFieldReadError(format!(
            "Dynamic field {:?} is not a Move object",
            object.id()
        ))
    })?;
    let result: V = bcs::from_bytes::<Field<N, V>>(move_object.contents())
        .map_err(|err| SuiError::DynamicFieldReadError(err.to_string()))?
        .value;

    Ok(result)
}

pub async fn get_mining_nft_pending_reward(
    http_client: HttpClient,
    contract: &str,
    global: &str,
    ticket_ids: Vec<String>,
) -> Result<u64, IndexerError> {
    let contract = ObjectID::from_address(SuiAddress::from_str(contract)?.into());
    let global = ObjectID::from_address(SuiAddress::from_str(global)?.into());
    let mut ids = vec![];
    for ticket_id in ticket_ids.iter() {
        let oid =
            ObjectID::from_address(AccountAddress::from_hex_literal(ticket_id).map_err(|err| {
                anyhow!(
                    "Failed to parse ticket_id: {} with error: {:?}",
                    ticket_id,
                    err
                )
            })?);
        ids.push(ID::new(oid));
    }
    let tx = build_nft_stake_pending_reward_tx(contract, global, ids)?;
    let val: u64 = dev_inspect_tx(http_client, &tx).await.map_err(|err| {
        IndexerError::FullNodeReadingError(format!(
            "Failed to get ticket pending reward {:?} with error {:?}",
            ticket_ids, err,
        ))
    })?;
    Ok(val)
}

fn build_nft_stake_pending_reward_tx(
    contract: ObjectID,
    global: ObjectID,
    ticket_ids: Vec<ID>,
) -> Result<TransactionKind, IndexerError> {
    let tx = ProgrammableTransaction {
        inputs: vec![
            CallArg::Object(ObjectArg::SharedObject {
                id: global,
                initial_shared_version: 1u64.into(),
                mutable: false,
            }),
            CallArg::Pure(bcs::to_bytes(&ticket_ids)?),
            CallArg::CLOCK_IMM,
        ],
        commands: vec![Command::move_call(
            contract,
            Identifier::new("staking")?,
            Identifier::new("batch_pending")?,
            vec![],
            vec![Argument::Input(0), Argument::Input(1), Argument::Input(2)],
        )],
    };
    Ok(TransactionKind::ProgrammableTransaction(tx))
}

#[derive(Debug, Clone)]
pub struct NFTDisplay {
    pub image_url: String,
    pub name: String,
}

pub async fn get_nft_display(
    http_client: HttpClient,
    object_id: ObjectID,
) -> Result<NFTDisplay, IndexerError> {
    let response: SuiObjectResponse = http_client
        .get_object(
            object_id.clone(),
            Some(SuiObjectDataOptions::bcs_lossless().with_display()),
        )
        .await
        .map_err(|err| {
            IndexerError::FullNodeReadingError(format!(
                "Failed to read object {:?} with error {:?}",
                object_id, err
            ))
        })?;
    let object = response.object()?;
    if let Some(display) = &object.display {
        if let Some(data) = &display.data {
            let image_url = data.get("image_url");
            let name = data.get("name");
            return Ok(NFTDisplay {
                image_url: image_url.unwrap_or(&"".to_string()).to_owned(),
                name: name.unwrap_or(&"".to_string()).to_owned(),
            });
        }
    }
    Err(IndexerError::FullNodeReadingError(format!(
        "Failed to extract NFT display from object: {:?}",
        object_id
    )))
}

pub async fn get_price_at_tick(
    http_client: HttpClient,
    contract: &str,
    tick: u32,
) -> Result<f64, IndexerError> {
    let contract = ObjectID::from_address(SuiAddress::from_str(contract)?.into());
    let tx = ProgrammableTransaction {
        inputs: vec![CallArg::Pure(bcs::to_bytes(&tick)?)],
        commands: vec![Command::move_call(
            contract,
            Identifier::new("tick_math")?,
            Identifier::new("get_sqrt_price_at_tick")?,
            vec![],
            vec![Argument::Input(0)],
        )],
    };
    let tx = TransactionKind::ProgrammableTransaction(tx);
    let val: u128 = dev_inspect_tx(http_client, &tx).await?;
    let coin_b = get_price_from_sqrt_price(&format!("{}", val))?;
    if coin_b == 0f64 {
        return Ok(coin_b);
    }
    Ok(1f64 / coin_b)
}

#[cfg(test)]
mod test_benfen {
    use std::str::FromStr;
    // use fastcrypto::encoding::{Base64, Encoding};
    // use jsonrpsee::core::RpcResult;
    use crate::{
        benfen::{
            get_global_nft_config, get_global_nft_staking, get_nft_display,
            get_nft_staking_overview,
        },
        IndexerConfig,
    };

    use super::{
        get_bfc_price_in_usd, get_mining_nft_pending_reward, timestamp_to_dt,
        timestamp_to_dt_string,
    };
    use jsonrpsee::http_client::{HeaderMap, HeaderValue, HttpClient, HttpClientBuilder};
    // use sui_json_rpc_api::{IndexerApiClient, WriteApiClient};
    use sui_json_rpc_api::CLIENT_SDK_TYPE_HEADER;
    // use sui_json_rpc_types::{SuiTransactionBlockResponseOptions, SuiTransactionBlockResponseQuery};
    // use sui_json_rpc_types::TransactionFilter::Checkpoint;
    use sui_types::base_types::{ObjectID, SuiAddress};
    // use sui_types::transaction::SenderSignedData;
    // use sui_json_rpc_types::sui_transaction::SuiTransactionBlockData::V1;

    fn create_http_client() -> HttpClient {
        let rpc_client_url = "https://rpc-mainnet.benfen.org:443/";
        let mut headers = HeaderMap::new();
        headers.insert(CLIENT_SDK_TYPE_HEADER, HeaderValue::from_static("indexer"));
        HttpClientBuilder::default()
            .max_request_body_size(2 << 30)
            .max_concurrent_requests(usize::MAX)
            .set_headers(headers.clone())
            .build(rpc_client_url)
            .unwrap()
    }

    #[ignore]
    #[tokio::test]
    async fn test_stable_coin_bfc_price() {
        let http_client = create_http_client();
        let price = get_bfc_price_in_usd(http_client).await.unwrap();
        println!("price {:.6}", price);
    }

    #[ignore]
    #[tokio::test]
    async fn test_global_nft_config() {
        let config = get_global_nft_config(create_http_client(), IndexerConfig::default())
            .await
            .unwrap();
        println!("Config: {:?}", config);
    }

    #[ignore]
    #[tokio::test]
    async fn test_global_staking() {
        let config = get_global_nft_staking(create_http_client(), IndexerConfig::default())
            .await
            .unwrap();
        println!("Config: {:?}", config);
    }

    #[ignore]
    #[tokio::test]
    async fn test_staking_overview() {
        let overview = get_nft_staking_overview(
            create_http_client(),
            IndexerConfig::default(),
            vec![],
            1_708_326_921,
        )
        .await
        .unwrap();
        println!("Overview: {:?}", overview);

        let overview = get_nft_staking_overview(
            create_http_client(),
            IndexerConfig::default(),
            vec![],
            1_707_100_000,
        )
        .await
        .unwrap();
        println!("Overview: {:?}", overview);
        for i in 1..2 {
            println!("{}", i);
        }
    }

    #[ignore]
    #[tokio::test]
    async fn test_nft_display() {
        let overview = get_nft_display(
            create_http_client(),
            ObjectID::from_address(
                SuiAddress::from_str(
                    "BFC403b76fced492d29215715dcb0a2c2311817212b0dd6f3ae6824fe6705d0546096e2",
                )
                .unwrap()
                .into(),
            ),
        )
        .await
        .unwrap();
        println!("NFT Display: {:?}", overview);
    }

    #[tokio::test]
    async fn test_mining_nft_pending_award() {
        let config = IndexerConfig::default();
        let pending = get_mining_nft_pending_reward(
            create_http_client(),
            &config.mining_nft_contract,
            &config.mining_nft_global,
            vec![
                "0x0003715fd1704173c050f4eb5c40c0b4cee3bd8a794a030e34c5380543f6c782".to_owned(),
                "0x00296924cf934bc7a21fd5b710754fd81f4397a4901a13224c0bf32a0b0d8aba".to_owned(),
                "0x00c56e7c903926ff20c01dbe428fac379e8098353c89ee1710f4587457232560".to_owned(),
            ],
            // "0xd146427073345317d1f5e82aeff9fd8d18448e49dc9af9a04337f8b990ee41e",
            // "BFC7232d3d3973b9f27eae8edbc2d379abfae5098c4eb8ebe943b2dba5d0dd48c9c8a8e",
        )
        .await
        .unwrap();
        println!("{:?}", pending);
    }

    #[tokio::test]
    async fn test_replay_transaction() {
        // let client = HttpClientBuilder::default().build("http://127.0.0.1:9000/").unwrap();
        // let mut from: u64 = 162;
        // loop {
        //     let filter = Some(Checkpoint(from));
            // let options = Some(SuiTransactionBlockResponseOptions{
            //     show_input: true,
            //     show_raw_input: true,
            //     show_effects: true,
            //     show_events: false,
            //     show_object_changes: false,
            //     show_balance_changes: false,
            // });
            // let page = client.query_transaction_blocks(SuiTransactionBlockResponseQuery::new(filter, options), None, Some(20), Some(true)).await.unwrap();
            // for tx in page.data {
            //     let tx_data = tx.transaction.unwrap();
            //     match tx_data.data {
            //         V1(v) => {
            //             if v.gas_data.budget != 0 {
            //                 let orig_tx: SenderSignedData = bcs::from_bytes(&tx.raw_transaction).unwrap();
            //                 let data = &orig_tx.inner().intent_message.value;
            //                 let data_bytes = &bcs::to_bytes(data).unwrap();
            //                 let tx_byte = Base64::encode(bcs::to_bytes(data).unwrap());
            //                 let s_json = serde_json::to_string(&orig_tx).unwrap();
            //                 let from = s_json.find("tx_signatures").unwrap() + 17;
            //                 let to = s_json.find("\"]}]").unwrap();
            //                 let signatures = s_json[from..to].to_string();
            //                 let r = client.execute_transaction_block(Base64::try_from(tx_byte).unwrap(),
            //                                                          vec![Base64::try_from(signatures).unwrap()]
            //                                                          , None, None).await;
            //                 match r {
            //                     Ok(rV) => {
            //                         let s = rV.digest.to_string();
            //                         println!("{}", s);
            //                     }
            //                     Err(_) => {}
            //                 }
            //             }
            //         }
            //     }
            // }
            // from = from + 1;
        // }
    }

    #[ignore]
    #[tokio::test]
    async fn test_long_coin_price_in_usd() {
        let usd = super::get_mining_nft_cost_in_usd(IndexerConfig::default(), create_http_client())
            .await
            .unwrap();
        println!("usd: {}", usd);

        let bfc = super::get_mining_nft_cost_in_bfc(IndexerConfig::default(), create_http_client())
            .await
            .unwrap();
        println!("bfc: {}", bfc);
    }

    #[ignore]
    #[tokio::test]
    async fn test_tick_price() {
        let usd = super::get_price_at_tick(
            create_http_client(),
            &IndexerConfig::default().mining_nft_dex_contract,
            4294950696u32,
        )
        .await
        .unwrap();
        println!("lower: {}", usd); // 0.190154

        let usd = super::get_price_at_tick(
            create_http_client(),
            &IndexerConfig::default().mining_nft_dex_contract,
            4294964496u32,
        )
        .await
        .unwrap();
        println!("upper: {}", usd); // 0.755794
    }

    #[test]
    fn test_timestamp_ms_to_dt_string() {
        assert_eq!(timestamp_to_dt_string(1706640071000), "2024-01-30");
    }

    #[test]
    fn test_timestamp_ms_to_dt() {
        assert_eq!(timestamp_to_dt(1706640071000), 1706572800000)
    }

    #[test]
    fn test_calculate_nft_cost() {
        assert_eq!(super::calculate_nft_cost(1), 1_065_000_000_000);
        assert_eq!(super::calculate_nft_cost(10), 504_000_000_000);
        assert_eq!(super::calculate_nft_cost(30), 284_000_000_000);
        assert_eq!(super::calculate_nft_cost(60), 179_000_000_000);
        assert_eq!(super::calculate_nft_cost(90), 132_000_000_000);
        assert_eq!(super::calculate_nft_cost(179), 74_000_000_000);

        assert_eq!(super::calculate_nft_cost(180), 179_000_000_000);
        assert_eq!(super::calculate_nft_cost(270), 132_000_000_000);
        assert_eq!(super::calculate_nft_cost(360), 105_000_000_000);
    }
}
