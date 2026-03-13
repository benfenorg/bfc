import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { BenfenBridge } from "../target/types/benfen_bridge.ts";
import BN from "bn.js";
import { SEEDS, CHAIN_IDS } from "../utils/const.ts";
import { tokenIdToBytes, nonceToBytes } from "../utils/util.ts";
import { getTokenByTokenId } from "../utils/tokens/token-manager.ts";
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";

async function main() {
    // 1. Setup Provider and Program
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

    console.log("Program ID:", program.programId.toBase58());
    console.log("Wallet:", provider.wallet.publicKey.toBase58());
    console.log("RPC Endpoint:", provider.connection.rpcEndpoint);

    try {
        const version = await provider.connection.getVersion();
        console.log("Connected to cluster version:", version["solana-core"]);
    } catch (e) {
        console.error("Failed to connect to cluster. Please check your ANCHOR_PROVIDER_URL or ensure local validator is running.");
        console.error("Error:", e.message);
        return;
    }

    // 2. User Provided Data
    const input = {
        "chain_id": 2,
        "nonce": 8,
        "message_type": 0,
        "version": 3,
        "payload": "IGFpKaMbpq0CRlp5d8tmFFQwTyfbSul49rok8OOVe27WMyBBali1icJCW02UeSPXUmcfNDwtZ6KPUwBUgvRxqvO0WAAAAAAAAAADAAAAAlO/mMAAAAA=",
        "signatures": [
          "vaSaOCKOChCLy913+sSE1KqaIMgn7Y20bKOKzIf5OQ4SWBrL6lk+n3ol/WCfyChUxGGpy4hH1uO9UJwIwOK0hAE=",
          "+QpizWmnAdBnrgaNNiksqO3LokiE9l9Dta3hZErzz5xJzI61Bs8oIV01oVDMQZv/iPbSbCM974dY+3q2zE0OnQA="
        ]
    };

    console.log("Input Data:", JSON.stringify(input, null, 2));

    const payloadBuffer = Buffer.from(input.payload, 'base64');
    const signaturesBuffers = input.signatures.map(s => Buffer.from(s, 'base64'));

    // 3. Parse Payload to get Token ID and Recipient
    // Payload layout:
    // 0: sender_len (1)
    // 1-32: sender
    // 33: target_chain
    // 34: recipient_len (1)
    // 35-66: recipient (32)
    // 67-74: token_id (8, BE)
    // 75-82: amount (8, BE)

    const recipientAddressBuffer = payloadBuffer.slice(35, 67);
    const recipientAddress = new anchor.web3.PublicKey(recipientAddressBuffer);
    console.log("Recipient Address from payload:", recipientAddress.toBase58());

    const tokenIdBuffer = payloadBuffer.slice(67, 75);
    const tokenId = new BN(tokenIdBuffer, 'be').toNumber();
    console.log("Token ID from payload:", tokenId);

    // Verify recipient
    if (!recipientAddress.equals(provider.wallet.publicKey)) {
        console.warn("Warning: Recipient address in payload does not match wallet public key.");
        console.warn("Wallet:", provider.wallet.publicKey.toBase58());
        console.warn("This transaction might fail if the signer must be the recipient.");
    }

    // 4. Get Token Info
    // const tokenInfo = getTokenByTokenId(tokenId);
    // if (!tokenInfo) {
    //     throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
    // }
    // const mint = new anchor.web3.PublicKey(tokenInfo.mintAddress);
    // console.log("Token Mint:", mint.toBase58());
    const mint=new anchor.web3.PublicKey("8h2Lm9gcGEHdCv2EKGwaDxF7cMWC2H9Xyqa91jxYptM9");

    // 5. Derive PDAs
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

    const [messageConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.MESSAGE_CONFIG), Buffer.from([input.message_type]), messageVerifierPDA.toBuffer()],
        program.programId
    );

    const tokenIdBytes = tokenIdToBytes(tokenId);
    const [tokenConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.TOKEN_CONFIG), tokenIdBytes],
        program.programId
    );

    const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.CHAIN_LIMIT), Buffer.from([input.chain_id]), bridgeConfigPDA.toBuffer()],
        program.programId
    );

    const [benfenBridgePDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BENFEN_BRIDGE), committeePDA.toBuffer()],
        program.programId
    );

    const [tokenVaultPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.VAULT), tokenIdBytes],
        program.programId
    );

    const nonceBytes = nonceToBytes(input.nonce);
    const [processedTransferPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from(SEEDS.PROCESSED_TRANSFER),
            Buffer.from([input.message_type]),
            Buffer.from([input.chain_id]),
            nonceBytes
        ],
        program.programId
    );

    // 6. Get/Create User Token Account
    // In crossOut, user receives tokens, so account should ideally exist, but ATA can be created if not?
    // Usually crossOut logic might require existing account or init if needed.
    // getAssociatedTokenAddress just derives address.
    const userTokenAccount = await getAssociatedTokenAddress(
        mint,
        new anchor.web3.PublicKey("5QMZ7qRioim21KqgS9qvxehH6kaUCGqEGi23f9xcS4Zu")
    );
    console.log("User Token Account:", userTokenAccount.toBase58());

    // 7. Execute Cross Out
    console.log("Sending crossOut transaction...");
    
    try {
        const tx = await program.methods.crossOut(
            input.chain_id,
            new BN(input.nonce),
            input.message_type,
            input.version,
            payloadBuffer,
            signaturesBuffers
        )
        .accounts({
            signer: provider.wallet.publicKey,
            bridge: benfenBridgePDA,
            processTransfer: processedTransferPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
            messageConfig: messageConfigPDA,
            tokenConfig: tokenConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            chainLimit: chainLimitPDA,
            tokenMint: mint,
            tokenVault: tokenVaultPDA,
            tokenAccount: userTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

        console.log("Transaction successful!");
        console.log("Signature:", tx);
    } catch (error: any) {
        console.error("Error executing cross out:");
        console.error(error);
        if (error.logs) {
            console.error("Logs:", error.logs);
        }
    }
}

main().then(() => process.exit(0)).catch(err => {
    console.error(err);
    process.exit(1);
});
