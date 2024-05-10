use anyhow::anyhow;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::Utc;
use fastcrypto::encoding::Base64;
use jsonrpsee::http_client::HttpClient;
use move_core_types::account_address::AccountAddress;
use move_core_types::parser::parse_type_tag;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::Div;
use sui_json_rpc::api::{IndexerApiClient, ReadApiClient, WriteApiClient};
use sui_json_rpc_types::DevInspectResults;
use sui_json_rpc_types::NFTStakingOverview;
use sui_json_rpc_types::SuiMiningNFTFutureReward;
use sui_json_rpc_types::SuiMiningNFTProfitRate;
use sui_json_rpc_types::SuiObjectDataOptions;
use sui_json_rpc_types::SuiObjectResponse;
use sui_types::base_types::ObjectID;
use sui_types::base_types::SuiAddress;
use sui_types::collection_types::VecMap;
use sui_types::dynamic_field::DynamicFieldName;
use sui_types::dynamic_field::Field;
use sui_types::error::SuiError;
use sui_types::id::ID;
use sui_types::id::UID;
use sui_types::object::Object;
use sui_types::transaction::ObjectArg;
use sui_types::{
    transaction::{Argument, CallArg, Command, ProgrammableTransaction, TransactionKind},
    Identifier, TypeTag, BFC_SYSTEM_PACKAGE_ID, BFC_SYSTEM_STATE_OBJECT_ID,
};
use tracing::info;

use crate::errors::IndexerError;
use crate::IndexerConfig;

// Rust types of VaultInfo in crates/sui-framework/packages/bfc-system/sources/treasury/vault.move
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultInfo {
    pub vault_id: ID,
    pub position_number: u32,
    pub state: u8,
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
    pub bfc_accrued_consume: u64,
    pub last_bfc_rebalance_amount: u64,
}

pub async fn get_bfc_price_in_usd(http_client: HttpClient) -> Result<f64, IndexerError> {
    let price =
        get_bfc_price_in_stable_coin(parse_type_tag("0xc8::busd::BUSD").unwrap(), http_client)
            .await?;
    Ok(price)
}

#[allow(dead_code)]
pub async fn get_bfc_value_in_stable_coin(
    coin: TypeTag,
    amount: u64,
    http_client: HttpClient,
) -> Result<u64, IndexerError> {
    let price = get_bfc_price_in_stable_coin(coin, http_client).await?;
    Ok((amount as f64 * price) as u64)
}

// https://github.com/hellokittyboy-code/obc/blob/1d3e3f066b59c4dd5a7395adf728a4878b8dc48a/apps/explorer/src/hooks/useTokenPrice.ts#L30-L68
pub async fn get_bfc_price_in_stable_coin(
    coin: TypeTag,
    http_client: HttpClient,
) -> Result<f64, IndexerError> {
    let tx = Base64::from_bytes(&bcs::to_bytes(&build_vault_info_tx(coin.clone())?)?);
    let results: DevInspectResults = http_client
        .dev_inspect_transaction_block(AccountAddress::ZERO.into(), tx, None, None)
        .await
        .map_err(|err| {
            IndexerError::FullNodeReadingError(format!(
                "Failed to get stable coin exchange rate {:?} with error {:?}",
                coin, err,
            ))
        })?;
    if let Some(err) = results.error {
        return Err(IndexerError::FullNodeReadingError(format!(
            "Failed to get stable coin exchange rate {:?} with error {:?}",
            coin, err,
        )));
    }

    if let Some(results) = results.results {
        if results.len() > 0 && results[0].return_values.len() > 0 {
            let val = bcs::from_bytes::<VaultInfo>(&results[0].return_values[0].0)?;
            return calculate_price(val.current_sqrt_price);
        }
    }
    Err(IndexerError::FullNodeReadingError(format!(
        "Failed to get stable coin exchange rate {:?} with no results",
        coin,
    )))
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

// {"data":{"amount":"42288.58","base":"BTC","currency":"USD"}}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CoinbasePriceResponse {
    pub data: CoinbasePriceData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CoinbasePriceData {
    pub amount: String,
    pub base: String,
    pub currency: String,
}

pub async fn get_btc_price_in_usd(dt_timestamp_ms: Option<i64>) -> Result<f64, IndexerError> {
    let url = match dt_timestamp_ms {
        Some(ts) => format!(
            "https://api.coinbase.com/v2/prices/BTC-USD/spot?date={}",
            timestamp_to_dt_string(ts)
        ),
        None => "https://api.coinbase.com/v2/prices/BTC-USD/spot".to_owned(),
    };
    let response = reqwest::get(&url)
        .await
        .map_err(|x| {
            IndexerError::UncategorizedError(anyhow::anyhow!(
                "Failed to get BTC price from {}, err: {:?}",
                url,
                x
            ))
        })?
        .json::<CoinbasePriceResponse>()
        .await
        .map_err(|x| -> IndexerError {
            IndexerError::UncategorizedError(anyhow::anyhow!(
                "Failed to parse BTC price from {}, err: {:?}",
                url,
                x
            ))
        })?;
    Ok(response.data.amount.parse::<f64>().map_err(|x| {
        IndexerError::UncategorizedError(anyhow::anyhow!(
            "Failed to parse amount {} to f64 with error : {:?}",
            response.data.amount,
            x,
        ))
    })?)
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
    timestamp: u64,
) -> Result<NFTStakingOverview, IndexerError> {
    let nft_staking = get_global_nft_staking(http_client.clone(), config.clone()).await?;
    let di = nft_staking.decrease_interval;
    let rps = nft_staking.reward_per_second;
    let p: u64 = (timestamp - nft_staking.begin_at) / di + 1; // nft_staking.period;
    let prev_total_reward: f64 = (1..p)
        .into_iter()
        .map(|n| di as f64 * current_period_rps(rps, n))
        .sum();
    let past_period_secs = di * (p - 1);
    let current_period_past_secs = timestamp - nft_staking.begin_at - past_period_secs;
    let current_total_reward = current_period_past_secs as f64 * current_period_rps(rps, p);

    let nft_config = get_global_nft_config(http_client, config).await?;
    let mut nft_future_rewards = vec![];
    let mut initial_rewarded: u64 = 0;
    let mut nft_future_profit_rates = vec![];
    let mut rewarded: u64 = 0;
    let max_count_per_day: u64 = 20;
    for d in 0..180 {
        let dp = timestamp + d * 86_400;
        let p: u64 = (dp - nft_staking.begin_at) / di + 1;
        let crps = current_period_rps(rps, p);
        let current_supply = nft_config.total_supply + d * max_count_per_day + 1;
        rewarded += (crps * 86_400f64 / current_supply as f64) as u64;
        nft_future_rewards.push(SuiMiningNFTFutureReward {
            reward: rewarded,
            dt_timestamp_ms: dp * 1_000,
        });
        if d == 0 {
            initial_rewarded = rewarded;
        }
        nft_future_profit_rates.push(SuiMiningNFTProfitRate {
            rate: (rewarded - initial_rewarded) as f64 / initial_rewarded as f64,
            dt_timestamp_ms: dp * 1_1000,
        });
    }

    Ok(NFTStakingOverview {
        total_power: nft_staking.total_power,
        reward_per_day: (86_400f64 * current_period_rps(rps, p)) as u64,
        total_rewarded: (prev_total_reward + current_total_reward) as u64,
        bfc_usd_price: 0f64,
        bfc_24h_rate: 0f64,
        nft_future_rewards,
        nft_future_profit_rates,
        btc_past_profit_rates: vec![],
    })
}

fn current_period_rps(rps: u64, p: u64) -> f64 {
    rps as f64 / (2f64.powi(p as i32 - 1) as f64)
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
    items: VecMap<ID, StakeItem>, // 质押的 NFT
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

async fn get_dynamic_field_object<'a, N>(
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
    info!(
        "Got dynamic fields object {:?} from its owner {:?}",
        object_id, owner
    );
    Ok(object)
}

fn dynamic_field_from_object<'a, N, V>(object: &'a Object) -> Result<V, IndexerError>
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
    ticket_id: &str,
) -> Result<u64, IndexerError> {
    let contract = ObjectID::from_address(SuiAddress::from_str(contract)?.into());
    let global = ObjectID::from_address(SuiAddress::from_str(global)?.into());
    let ticket_id =
        ObjectID::from_address(AccountAddress::from_hex_literal(ticket_id).map_err(|err| {
            anyhow!(
                "Failed to parse ticket_id: {} with error: {:?}",
                ticket_id,
                err
            )
        })?);
    let tx = Base64::from_bytes(&bcs::to_bytes(&build_nft_stake_pending_reward_tx(
        contract,
        global,
        ID::new(ticket_id.clone()),
    )?)?);
    let results: DevInspectResults = http_client
        .dev_inspect_transaction_block(AccountAddress::ZERO.into(), tx, None, None)
        .await
        .map_err(|err| {
            IndexerError::FullNodeReadingError(format!(
                "Failed to get ticket pending reward {:?} with error {:?}",
                ticket_id, err,
            ))
        })?;
    if let Some(err) = results.error {
        return Err(IndexerError::FullNodeReadingError(format!(
            "Failed to get ticket pending reward {:?} with error {:?}",
            ticket_id, err,
        )));
    }

    if let Some(results) = results.results {
        if results.len() > 0 && results[0].return_values.len() > 0 {
            let val = bcs::from_bytes::<u64>(&results[0].return_values[0].0)?;
            return Ok(val);
        }
    }
    Err(IndexerError::FullNodeReadingError(format!(
        "Failed to get ticket pending reward {:?} with no results",
        ticket_id,
    )))
}

fn build_nft_stake_pending_reward_tx(
    contract: ObjectID,
    global: ObjectID,
    ticket_id: ID,
) -> Result<TransactionKind, IndexerError> {
    let tx = ProgrammableTransaction {
        inputs: vec![
            CallArg::Object(ObjectArg::SharedObject {
                id: global,
                initial_shared_version: 1u64.into(),
                mutable: false,
            }),
            CallArg::Pure(bcs::to_bytes(&ticket_id)?),
            CallArg::CLOCK_IMM,
        ],
        commands: vec![Command::move_call(
            contract,
            Identifier::new("staking")?,
            Identifier::new("pending")?,
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

#[cfg(test)]
mod test_benfen {
    use std::str::FromStr;

    use crate::{
        benfen::{
            get_global_nft_config, get_global_nft_staking, get_nft_display,
            get_nft_staking_overview,
        },
        IndexerConfig,
    };

    use super::{
        get_bfc_price_in_usd, get_btc_price_in_usd, get_mining_nft_pending_reward, timestamp_to_dt,
        timestamp_to_dt_string,
    };
    use jsonrpsee::http_client::{HeaderMap, HeaderValue, HttpClient, HttpClientBuilder};
    use sui_json_rpc::CLIENT_SDK_TYPE_HEADER;
    use sui_types::base_types::{ObjectID, SuiAddress};

    fn create_http_client() -> HttpClient {
        let rpc_client_url = "https://testrpc.benfen.org:443/";
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
    async fn test_btc_price() {
        let price = get_btc_price_in_usd(None).await.unwrap();
        println!("Current price: {:.2}", price);
        let prev_price = get_btc_price_in_usd(Some(1706640071000)).await.unwrap();
        assert_eq!(prev_price, 43303.3);
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
            1_708_326_921,
        )
        .await
        .unwrap();
        println!("Overview: {:?}", overview);

        let overview = get_nft_staking_overview(
            create_http_client(),
            IndexerConfig::default(),
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

    #[ignore]
    #[tokio::test]
    async fn test_mining_nft_pending_award() {
        let config = IndexerConfig::default();
        let pending = get_mining_nft_pending_reward(
            create_http_client(),
            &config.mining_nft_contract,
            &config.mining_nft_global,
            "0xd146427073345317d1f5e82aeff9fd8d18448e49dc9af9a04337f8b990ee41e",
            // "BFC7232d3d3973b9f27eae8edbc2d379abfae5098c4eb8ebe943b2dba5d0dd48c9c8a8e",
        )
        .await
        .unwrap();
        println!("{:?}", pending);
    }

    #[test]
    fn test_timestamp_ms_to_dt_string() {
        assert_eq!(timestamp_to_dt_string(1706640071000), "2024-01-30");
    }

    #[test]
    fn test_timestamp_ms_to_dt() {
        assert_eq!(timestamp_to_dt(1706640071000), 1706572800000)
    }
}
