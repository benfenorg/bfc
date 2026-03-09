import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";

import { SEEDS, CHAIN_IDS, MESSAGE_TYPES } from "../utils/const.ts";
import { createUpgradePayload, createMessage, computeMessageHash } from "../utils/message.ts";
import { committeeInfo } from "../utils/committee.ts";
import { getSignature } from "../utils/signatures.ts";
import type { BenfenBridge } from "../target/types/benfen_bridge.ts";

async function main() {
    // 1. Setup Provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    console.log("Wallet:", provider.wallet.publicKey.toBase58());

    // 2. Setup Program
    // Using the Devnet Program ID as seen in Anchor.toml and previous context
    // You can change this to match your target environment
    const programId = new anchor.web3.PublicKey("BZyn9uqtYXsWYa6ksAZ27Jeh7zVVG7bbULAgFGXqzfjN");
    
    // Load IDL
    const idl = await anchor.Program.fetchIdl(programId, provider);
    if (!idl) {
        throw new Error("IDL not found for program: " + programId.toBase58());
    }
    const program = new anchor.Program<BenfenBridge>(idl as any, provider);
    console.log("Program ID:", program.programId.toBase58());

    // 3. Configuration
    const message_type = MESSAGE_TYPES.UPGRADE;
    const nonce = 3; // TODO: Increment this or fetch from chain
    const version = 1;
    const chain_id = CHAIN_IDS.SOLANA_TESTNET; // Or CHAIN_IDS.BENFEN_TESTNET depending on direction? 
                                               // Test uses SOLANA_TESTNET (2).

    // Buffer account containing the new program data
    // TODO: Update this with your actual buffer account from `solana program write-buffer`
    const buffer = new anchor.web3.PublicKey("44R5jmbWHEodGHH6qUK4gu1wCD3bvfktFQpHvaBzeu7F");

    const BPF_LOADER_UPGRADEABLE_ID = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");
    const targetProgram = program.programId;

    // 4. Derive PDAs
    const [programData] = anchor.web3.PublicKey.findProgramAddressSync(
        [targetProgram.toBuffer()],
        BPF_LOADER_UPGRADEABLE_ID
    );

    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BRIDGE_CONFIG)],
        targetProgram
    );

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.COMMITTEE_SEED), bridgeConfigPDA.toBuffer()],
        targetProgram
    );

    const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.MESSAGE_VERIFIER), committeePDA.toBuffer()],
        targetProgram
    );

    const [messageConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.MESSAGE_CONFIG), Buffer.from([message_type]), messageVerifierPDA.toBuffer()],
        targetProgram
    );

    const [upgradeAuthorityPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.UPGRADE_AUTHORITY), committeePDA.toBuffer()],
        targetProgram
    );

    console.log("Target Program:", targetProgram.toString());
    console.log("Program Data:", programData.toString());
    console.log("Upgrade Authority:", upgradeAuthorityPDA.toString());
    console.log("Buffer:", buffer.toString());

    // 5. Check Program Data
    try {
        const programDataAccount = await provider.connection.getAccountInfo(programData);
        if (!programDataAccount) {
            console.warn("Warning: Program data account not found. Is the program deployed?");
        } else {
            console.log("Program data account found.");
        }
    } catch (error) {
        console.error("Error fetching program data:", error);
    }

    // 6. Create Message and Signatures
    // Authority Type 2 (Buffer Authority?) - from test
    const payload = createUpgradePayload(targetProgram, buffer, 4);
    console.log("Payload length:", payload.length);

    const message = createMessage(
        message_type,
        version,
        BigInt(nonce),
        chain_id,
        payload
    );

    const messageHash = computeMessageHash(message);
    const signatures: Uint8Array[] = [];

    // Sign with committee keys
    // NOTE: Using hardcoded committee keys from utils/committee.ts
    for (let i = 0; i < 3; i++) {
        const element = committeeInfo[i];
        const signature = getSignature(messageHash, element.privateKey);
        signatures.push(signature);
    }
    console.log(`Generated ${signatures.length} signatures.`);

    // 7. Execute Upgrade
    try {
        const tx = await program.methods
            .upgradeProgram(
                message_type,
                version,
                new BN(nonce),
                chain_id,
                Buffer.from(payload),
                signatures.map(sig => Buffer.from(sig))
            )
            .accounts({
                payer: provider.wallet.publicKey,
                spill: provider.wallet.publicKey,
                upgradeAuthority: upgradeAuthorityPDA,
                buffer,
                program: targetProgram,
                bridgeConfig: bridgeConfigPDA,
                verifier: messageVerifierPDA,
                messageConfig: messageConfigPDA,
                programData,
                committee: committeePDA,
                bpfLoader: BPF_LOADER_UPGRADEABLE_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            } as any)
            .rpc();

        console.log("Upgrade transaction successful!");
        console.log("Signature:", tx);
    } catch (error: any) {
        console.error("Error upgrading program:");
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
