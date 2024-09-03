// use std::time::Duration;
use anyhow::Result;
// use clap::Parser;
// use jsonrpsee::http_client::{HttpClientBuilder};
// use sui_json_rpc::api::{IndexerApiClient, WriteApiClient};
// use sui_json_rpc_types::{SuiTransactionBlockResponseOptions, SuiTransactionBlockResponseQuery};
// use sui_json_rpc_types::TransactionFilter::Checkpoint;
// use sui_types::transaction::SenderSignedData;
// use sui_json_rpc_types::sui_transaction::SuiTransactionBlockData::V1;
// use fastcrypto::encoding::{Base64, Encoding};
//
#[tokio::main]
async fn main() -> Result<()> {
//     let config = TestReplayConfig::parse();
//     let from_client = HttpClientBuilder::default().build(config.from_url).unwrap();
//     let to_client = HttpClientBuilder::default().build(config.to_url).unwrap();
//     let mut from: u64 = config.from_checkpoint;
//     let options = Some(SuiTransactionBlockResponseOptions{
//         show_input: true,
//         show_raw_input: true,
//         show_effects: true,
//         show_events: false,
//         show_object_changes: false,
//         show_balance_changes: false,
//     });
//     loop {
//         let filter = Some(Checkpoint(from));
//         let page_result = from_client.query_transaction_blocks(SuiTransactionBlockResponseQuery::new(filter, options.clone()), None, Some(20), Some(true)).await;
//         if page_result.is_err() {
//             println!("waiting for 1sec {:?}", page_result.err());
//             tokio::time::sleep(Duration::from_millis(1000)).await;
//             continue;
//         }
//         let page = page_result.unwrap();
//         println!("checkpoint {:?} size {:?}", from, page.data.len());
//         for tx in page.data.into_iter() {
//             let tx_data = tx.transaction.unwrap();
//             let V1(v) = tx_data.data;
//             if v.gas_data.budget != 0u64 {
//                 let orig_tx: SenderSignedData = bcs::from_bytes(&tx.raw_transaction).unwrap();
//                 let data = &orig_tx.inner().intent_message.value;
//                 let tx_byte = Base64::encode(bcs::to_bytes(data).unwrap());
//                 let s_json = serde_json::to_string(&orig_tx).unwrap();
//                 let from = s_json.find("tx_signatures").unwrap() + 17;
//                 let to = s_json.find("\"]}]").unwrap();
//                 let signatures = s_json[from..to].to_string();
//                 println!("tx {:?} sign {:?}", tx_byte, signatures);
//                 let r = to_client.execute_transaction_block(Base64::try_from(tx_byte).unwrap(),
//                                                          vec![Base64::try_from(signatures).unwrap()]
//                                                          , None, None).await;
//                 match r {
//                     Ok(r_v) => {
//                         let s = r_v.digest.to_string();
//                         println!("{}", s);
//                     }
//                     Err(e) => {
//                         println!("e {:?}", e);
                        return Ok(());
//                     }
//                 }
//             }
//         }
//         from = from + 1;
//     }
}
//
// #[derive(Parser)]
// #[clap(name = "Transactions Replay Test")]
// pub struct TestReplayConfig {
//     #[clap(long)]
//     pub from_url: String,
//     #[clap(long)]
//     pub to_url: String,
//     #[clap(long)]
//     pub from_checkpoint: u64,
// }
