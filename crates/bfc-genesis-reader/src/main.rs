// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use std::path::PathBuf;
use sui_config::{Config, NodeConfig};
use sui_types::multiaddr::Multiaddr;
use sui_types::effects::TransactionEffects::V1;
use sui_types::object::Data::Move;

const GIT_REVISION: &str = {
    if let Some(revision) = option_env!("GIT_REVISION") {
        revision
    } else {
        let version = git_version::git_version!(
            args = ["--always", "--dirty", "--exclude", "*"],
            fallback = ""
        );

        if version.is_empty() {
            panic!("unable to query git revision");
        }
        version
    }
};
const VERSION: &str = const_str::concat!(env!("CARGO_PKG_VERSION"), "-", GIT_REVISION);

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(name = env!("CARGO_BIN_NAME"))]
#[clap(version = VERSION)]
struct Args {
    #[clap(long)]
    pub config_path: PathBuf,

    #[clap(long, help = "Specify address to listen on")]
    listen_address: Option<Multiaddr>,
}

//
//./target/debug/bfc-genesis-reader --config-path /Users/xxxx/.bfc/bfc_config/fullnode.yaml
fn main() {
    // Ensure that a validator never calls get_for_min_version/get_for_max_version.
    // TODO: re-enable after we figure out how to eliminate crashes in prod because of this.
    // ProtocolConfig::poison_get_for_min_version();

    let args = Args::parse();
    let config = NodeConfig::load(&args.config_path).unwrap();

    let effects = config.genesis().unwrap().effects();
    let genesis = config.genesis().unwrap();

    match effects {
        V1(e)=>{
            //println!("created is {:?}",e.created);
            let mut gas_coin_count = 0;
            let mut staked_coin_count = 0;

            let mut sum:u64 = 0;
            let mut staked_sum:u64=0;
            let mut total:u64=0;
            let mut dynamic_count = 0;
            for ((obj_id,_,_),_) in &e.created{
                //println!("key obj id is {:?}",obj_id);
                total = total + 1;
                let data = &genesis.object(obj_id.clone()).unwrap().data;

                if let Move(d) = data {
                    //if d.has_public_transfer() {
                    if d.type_().is_gas_coin(){
                        gas_coin_count = gas_coin_count + 1;
                        //println!("data {:?}", d.get_coin_value_unsafe()/1_000_000_000);
                        sum+= d.get_coin_value_unsafe()/1_000_000_000;
                    }
                    if d.type_().is_staked_sui(){
                        staked_coin_count = staked_coin_count + 1;
                        //println!("data staked sui{:?}", d.get_scoin_value_unsafe()/1_000_000_000);
                        staked_sum+= d.get_scoin_value_unsafe()/1_000_000_000;
                    }
                    //}
                    if d.type_().is_dynamic_field() {
                        dynamic_count = dynamic_count + 1;
                        println!("{:?}",obj_id);
                    }
                }
            }

            println!("total is {:?}", total);
            println!("gas_coin_sum count {:?} total is {:?}", gas_coin_count,sum);
            println!("staked_sum count {:?} total is {:?}", staked_coin_count,staked_sum);
            println!("dynamic_count is {:?}", dynamic_count);
        }
    }
}
