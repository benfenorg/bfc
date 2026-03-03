import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SEEDS, CHAIN_IDS } from "../utils/const.ts";
import BN from "bn.js";

import type { BenfenBridge } from "../target/types/benfen_bridge.ts";

async function main() {
    // 1. Setup Provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    console.log("Wallet:", provider.wallet.publicKey.toBase58());

    // 2. Setup Program
    const programId = new anchor.web3.PublicKey("BZyn9uqtYXsWYa6ksAZ27Jeh7zVVG7bbULAgFGXqzfjN");

    
    // Load IDL
    const idl = await anchor.Program.fetchIdl(programId, provider);
    if (!idl) {
        throw new Error("IDL not found for program: " + programId.toBase58());
    }
    const program = new anchor.Program<BenfenBridge>(idl as any, provider);
    console.log("Program ID:", program.programId.toBase58());

    // 3. Derive Bridge Config PDA
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BRIDGE_CONFIG)],
        program.programId
    );
    console.log("Bridge Config PDA:", bridgeConfigPDA.toBase58());

    // 4. Iterate over known Chain IDs to find initialized ChainLimit accounts
    console.log("\n--- Checking Chain Limits ---");
    
    // Get all numeric values from CHAIN_IDS
    const chainIds = Object.values(CHAIN_IDS).filter((v) => typeof v === 'number') as number[];



    // for (const chainId of chainIds) {
    const chainId=2
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from(SEEDS.CHAIN_LIMIT),
                Buffer.from([chainId]),
                bridgeConfigPDA.toBuffer()
            ],
            program.programId
        );

        try {
            // Fetch the account
            // Note: Since it's zero_copy, Anchor handles deserialization automatically
            const account = await program.account.chainLimit.fetch(chainLimitPDA);
            
            console.log(`\n[Chain ID: ${chainId}] Found ChainLimit Account`);
            console.log("  PDA Address:", chainLimitPDA.toBase58());
            console.log("  Config PDA:", account.config.toBase58());
            // account.chainId is [u8; 1], so we access the first element
            console.log("  Stored Chain ID:", account.chainId[0]); 
            console.log("  Max USD Limit:", account.maxUsdLimit.toString());
            console.log("  Total Limit (24h):", account.totalLimit.toString());
            console.log("  Current Hourly Index:", account.hourlyTransferIndex);
            
            // Check hourly transfers
            const hourlyAmounts = account.hourlyTransferAmount as any[];
            const activeTransfers = hourlyAmounts
                .map((item, index) => ({ index, ...item }))
                .filter(item => item.amount.gt(new BN(0)));

            if (activeTransfers.length > 0) {
                console.log("  Active Hourly Transfers:");
                activeTransfers.forEach(t => {
                    console.log(`    Index ${t.index}: Hour ${t.hour.toString()} -> Amount ${t.amount.toString()}`);
                });
            } else {
                console.log("  No active hourly transfers recorded.");
            }

        } catch (error: any) {
            // Account not found is expected for unused chain IDs
            if (!error.message.includes("Account does not exist")) {
                 console.log(`\n[Chain ID: ${chainId}] Error fetching: ${error.message}`);
            }
        }
    }


main().then(() => process.exit(0)).catch(err => {
    console.error(err);
    process.exit(1);
});
