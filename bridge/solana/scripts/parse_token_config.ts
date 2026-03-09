import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import BN from "bn.js";
import { SEEDS, TOKEN_IDS } from "../utils/const.ts";
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

    // 3. Iterate over known Token IDs to find initialized TokenConfig accounts
    console.log("\n--- Checking Token Configs ---");
    
    // Get all numeric values from TOKEN_IDS
    const tokenIds = Object.entries(TOKEN_IDS);

    //for (const [tokenName, tokenId] of tokenIds) {
    let tokenId=3
        // Derive Token Config PDA
        
        // seeds = [TOKEN_CONFIG_SEED.as_bytes(), &token_id.to_le_bytes()]
        const tokenIdBuffer = new BN(tokenId).toArrayLike(Buffer, "le", 8);
       const tokenConfigPDA=new anchor.web3.PublicKey("49ztZP1nf4B1P5cQ3eArDbRUYb1fudScpugFwVcZB5mA")

        try {
            // Fetch the account
            // Note: Since it's zero_copy, Anchor handles deserialization automatically
            const account = await program.account.tokenConfigAccount.fetch(tokenConfigPDA);
            
            console.log(`\n[Token: (ID: ${tokenId})] Found TokenConfig Account`);
            console.log("  PDA Address:", tokenConfigPDA.toBase58());
            console.log("  Bridge Config:", account.config.toBase58());
            console.log("  Chain Limit Config:", account.chain.toBase58()); // 'chain' field seems to point to chain limit or chain config?
            console.log("  Mint Address:", account.mint.toBase58());
            console.log("  Token ID:", account.tokenId.toString());
            console.log("  Price (8 decimals):", account.price.toString());
            console.log("  Solana Decimals:", account.decimal);
            console.log("  Benfen Decimals:", account.benfenDecimal);
            console.log("  Is Native:", account.isNative === 1 ? "True" : "False");

        } catch (error: any) {
            // Account not found is expected for unused token IDs
            if (!error.message.includes("Account does not exist")) {
                 console.log(`\n[Token:  (ID: ${tokenId})] Error fetching: ${error.message}`);
            } else {
                 console.log(`\n[Token: (ID: ${tokenId})] Not initialized.`);
            }
        }
    }

main().then(() => process.exit(0)).catch(err => {
    console.error(err);
    process.exit(1);
});
