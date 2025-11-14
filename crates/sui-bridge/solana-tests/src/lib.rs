// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use anchor_client::{Client, Program};
    use solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    };
    use solana_test_validator::TestValidator;
    use std::{path::PathBuf, str::FromStr, sync::Arc};

    // Include the generated code.
    pub mod benfen_bridge {
        use anchor_client::solana_sdk::pubkey;
        include!(concat!(env!("OUT_DIR"), "/benfen_bridge.rs"));
    }

    #[tokio::test]
    async fn test_solana_bridge_initialize() {
        let program_id = benfen_bridge::ID;

        // 1. Start the test validator
        let program_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../bridge/solana/target/deploy/benfen_bridge.so");
        if !program_path.exists() {
            // Provide a helpful message if the program is not built.
            // The user can build it by running `anchor build` in `bridge/solana`.
            // Note: This test will still fail to compile if the program is not built,
            // because the generated code depends on it.
            println!(
                "Program binary not found at {:?}. Run `anchor build` in `bridge/solana` first.",
                program_path
            );
            return;
        }

        let (validator, payer) = TestValidator::with_custom_bpf(
            program_path.to_str().unwrap(),
            &program_id,
        )
        .start();
        let rpc_url = validator.rpc_url();
        let payer = Arc::new(payer);

        // 2. Set up the client
        let connection = anchor_client::rpc::RpcClient::new_with_commitment(
            rpc_url,
            CommitmentConfig::processed(),
        );
        let client = Client::new_with_options(
            connection,
            payer.clone(),
            CommitmentConfig::processed(),
        );
        let program = client.program(program_id).unwrap();

        // 3. Run the `initialize_bridge_config` instruction
        let (bridge_config_pda, _bump) =
            Pubkey::find_program_address(&[b"bridge_config"], &program_id);

        let request = program
            .request()
            .accounts(benfen_bridge::accounts::InitializeBridgeConfig {
                payer: payer.pubkey(),
                bridge_config: bridge_config_pda,
                system_program: solana_sdk::system_program::ID,
            })
            .args(benfen_bridge::instruction::InitializeBridgeConfig {
                chain_id: 5,
            });
        
        let tx_sig = request.send().await.unwrap();

        println!("Transaction signature: {}", tx_sig);

        // 4. Verify the account was created and initialized
        let bridge_config_account: benfen_bridge::accounts::BridgeConfig =
            program.account(bridge_config_pda).await.unwrap();

        assert_eq!(bridge_config_account.chain_id, 5);
        assert_eq!(bridge_config_account.token_count, 0);
    }
}
