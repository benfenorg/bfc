import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { BenfenBridge } from "../target/types/benfen_bridge.ts";
import BN from "bn.js";
import { SEEDS, TOKEN_IDS, MESSAGE_TYPES, CHAIN_IDS } from "../utils/const.ts";
import { tokenIdToBytes, benfenAddressToBytes, getDefaultBenfenAddressBytes } from "../utils/util.ts";
import { getTokenByTokenId } from "../utils/tokens/token-manager.ts";
import { getAssociatedTokenAddress, getAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import path from "path";

async function main() {
    // 1. Setup Provider and Program
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // const accountInfo = await getAccount(provider.connection,new anchor.web3.PublicKey("GuYciToJmnjV1c6gCaEBvfNRxAwcRqeAZMtKv4eLSqmR"));
    // console.log("Account Info:", accountInfo);


    // return



    
    // new Program()
    // Use program ID from IDL or environment
    const programId = new anchor.web3.PublicKey("BZyn9uqtYXsWYa6ksAZ27Jeh7zVVG7bbULAgFGXqzfjN");
    
    // Load IDL
    // Note: You might need to adjust the path to where your IDL is located
    const idl = await anchor.Program.fetchIdl(programId, provider);
    if (!idl) {
        throw new Error("IDL not found");
    }

    const program = new anchor.Program<BenfenBridge>(idl as any, provider);

    console.log("Program ID:", program.programId.toBase58());
    console.log("Wallet:", provider.wallet.publicKey.toBase58());

    // 2. Configuration
    const tokenId = TOKEN_IDS.USDC;
    
    // Note: ensure deployTokens.json exists in the root directory
    // const tokenInfo = getTokenByTokenId(tokenId);
    // if (!tokenInfo) {
    //     throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
    // } 

    const owner = provider.wallet.publicKey;
    const usdMint = new anchor.web3.PublicKey("8h2Lm9gcGEHdCv2EKGwaDxF7cMWC2H9Xyqa91jxYptM9");
    const tokenIdBytes = tokenIdToBytes(tokenId);
    
    // Use default valid Benfen address
    const benfenAddressBytes = getDefaultBenfenAddressBytes();
    // To test invalid address (as per reference test):
    // const benfenAddressBytes = benfenAddressToBytes("0x1234567890123456789012345678901234567890", false);

    console.log("Mint:", usdMint.toBase58());

    // 3. Derive PDAs
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BRIDGE_CONFIG)],
        program.programId
    );

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.COMMITTEE_SEED), bridgeConfigPDA.toBuffer()],
        program.programId
    );

    const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.MESSAGE_VERIFIER), committeePDA.toBuffer()],
        program.programId
    );

    const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
    const [messageConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.MESSAGE_CONFIG), Buffer.from([message_type]), messageVerifierPDA.toBuffer()],
        program.programId
    );

    const [tokenConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.TOKEN_CONFIG), tokenIdBytes],
        program.programId
    );

    const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.CHAIN_LIMIT), Buffer.from([CHAIN_IDS.BENFEN_TESTNET]), bridgeConfigPDA.toBuffer()],
        program.programId
    );

    const [benfenBridgePDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BENFEN_BRIDGE), committeePDA.toBuffer()],
        program.programId
    );

    const [tokenValutPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.VAULT), tokenIdBytes],
        program.programId
    );

    // 4. Get User Token Account
    const userTokenAccount = await getAssociatedTokenAddress(
        usdMint,
        owner
    );

    console.log("User Token Account:", userTokenAccount.toBase58());

    // Check balance
    try {
        const beforeAccountValut = await getAccount(
            provider.connection,
            userTokenAccount
        );
        const beforeAccountAmount = beforeAccountValut.amount;
        console.log("Current Balance:", beforeAccountAmount.toString());

        if (beforeAccountAmount <= BigInt(0)) {
            console.error("Insufficient balance to cross in.");
            return;
        }

        // 5. Execute Cross In
        // Use a small amount or based on balance
        // Example: Cross 1 unit (considering decimals)
        const amountToCross = new BN(10000000); // Adjust based on decimals (e.g. 1 USDC if 6 decimals)
        
        console.log(`Crossing in ${amountToCross.toString()} units...`);

        const tx = await program.methods.crossTokenToBridge(
            amountToCross,
            benfenAddressBytes,
            new BN(tokenId)
        )
        .accounts({
            payer: owner,
            tokenAccount: userTokenAccount,
            tokenVault: tokenValutPDA,
            messageConfig: messageConfigPDA,
            tokenConfig: tokenConfigPDA,
            chainLimit: chainLimitPDA,
            bridgeConfig: bridgeConfigPDA,
            bridge: benfenBridgePDA,
            verifier: messageVerifierPDA,
            tokenMint: usdMint,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        } as any).rpc();

        console.log("Transaction successful!");
        console.log("Signature:", tx);

    } catch (error: any) {
        console.error("Error executing cross in:");
        console.error(error);
    }
}

main().then(() => process.exit(0)).catch(err => {
    console.error(err);
    process.exit(1);
});
