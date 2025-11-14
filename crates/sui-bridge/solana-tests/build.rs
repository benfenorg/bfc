use anchor_client_codegen::{
    accounts::{self, Account},
    events, idl, instructions, programs, state,
};
use anyhow::Result;
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() -> Result<()> {
    let idl_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../idls/benfen_bridge.json");
    let idl: idl::Idl = serde_json::from_reader(File::open(&idl_path)?)?;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("benfen_bridge.rs");

    let mut file = File::create(&dest_path)?;
    let accounts_ts = idl
        .accounts
        .iter()
        .map(accounts::generate)
        .collect::<Vec<_>>();
    let instructions_ts = idl
        .instructions
        .iter()
        .map(instructions::generate)
        .collect::<Vec<_>>();

    let events_ts = idl.events.as_ref().map(|events| {
        events
            .iter()
            .map(events::generate)
            .collect::<Vec<TokenStream>>()
    });

    let state_ts = idl.state.as_ref().map(state::generate);

    let program_name = idl.name.to_snake_case();
    let program_mod = programs::generate(program_name, &idl.address, &idl.metadata);

    let code = quote! {
        #program_mod

        pub mod instructions {
            use super::*;
            #(#instructions_ts)*
        }

        pub mod accounts {
            use super::*;
            #(#accounts_ts)*
        }

        #state_ts

        #events_ts
    };

    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}
