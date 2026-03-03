import * as anchor from "@coral-xyz/anchor";
import type { BenfenBridge } from "../target/types/benfen_bridge.ts";
import { Program } from "@coral-xyz/anchor";
import BN from "bn.js";
import { SEEDS, DEFAULTS } from "../utils/const.ts";
import fs from "fs";
import path from "path";
import { benfenAddressToBytes, getDefaultBenfenAddressBytes, tokenIdToBytes } from "../utils/util.ts";

type DeployConfig = {
  committeeMemberStake: number[];
  committeeMembers: string[];
  minCommitteeStakeRequired: number;
  sourceChainId: number;
  destinationChainId: number;
  supportedChainLimitsInDollars: number;
  maxSingleTransferLimitInDollars: number;
};

function hex20ToBytes(hex: string): number[] {
  const h = hex.startsWith("0x") ? hex.slice(2) : hex;
  const out: number[] = [];
  for (let i = 0; i < h.length; i += 2) {
    out.push(parseInt(h.substr(i, 2), 16));
  }
  return out;
}

async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;
  
  // Fix: Use process.cwd() instead of __dirname for ESM compatibility
  const cfgPath = path.resolve(process.cwd(), "scripts/deploy-configs/devnet.json");
  const cfg: DeployConfig = JSON.parse(fs.readFileSync(cfgPath, "utf-8"));

  // await program.provider.connection.requestAirdrop(
  //   program.provider.wallet.publicKey,
  //   DEFAULTS.AIRDROP_AMOUNT
  // );

  const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(SEEDS.BRIDGE_CONFIG)],
    program.programId
  );

  console.log("Initializing Bridge Config...");
  // try {
  //     await program.methods
  //       .initializeBridgeConfig(cfg.sourceChainId)
  //       .accounts({
  //         payer: program.provider.wallet.publicKey,
  //         bridgeConfig: bridgeConfigPDA,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       } as any)
  //       .rpc();
  //     console.log("Bridge Config Initialized");
  // } catch(e) {
  //     console.log("Bridge Config might be already initialized or failed:", e);
  // }

  const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(SEEDS.COMMITTEE_SEED), bridgeConfigPDA.toBuffer()],
    program.programId
  );

  const addresses = cfg.committeeMembers.map(hex20ToBytes);
  const stakes = cfg.committeeMemberStake;
  const minStakeRequired = cfg.minCommitteeStakeRequired;

  console.log("Initializing Committee...");
  // try {
  //     await program.methods
  //       .initializeCommittee(addresses, stakes, minStakeRequired)
  //       .accounts({
  //         payer: program.provider.wallet.publicKey,
  //         committee: committeePDA,
  //         bridgeConfig: bridgeConfigPDA,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       } as any)
  //       .rpc();
  //     console.log("Committee Initialized");
  // } catch(e) {
  //      console.log("Committee might be already initialized or failed:", e);
  // }

  const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(SEEDS.MESSAGE_VERIFIER), committeePDA.toBuffer()],
    program.programId
  );

  console.log("Initializing Message Verifier...");
  // try {
  //     await program.methods
  //       .initializeMessageVerifier()
  //       .accounts({
  //         payer: program.provider.wallet.publicKey,
  //         messageVerifier: messageVerifierPDA,
  //         committee: committeePDA,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       } as any)
  //       .rpc();
  //     console.log("Message Verifier Initialized");
  // } catch(e) {
  //     console.log("Message Verifier might be already initialized or failed:", e);
  // }

  const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(SEEDS.CHAIN_LIMIT),
      Buffer.from([cfg.destinationChainId]),
      bridgeConfigPDA.toBuffer(),
    ],
    program.programId
  );

  console.log("Initializing Bridge Limiter...");
  try {
      await program.methods
        .initializeBridgeLimiter(
          cfg.destinationChainId,
          new BN(cfg.supportedChainLimitsInDollars),
          new BN(cfg.maxSingleTransferLimitInDollars)
        )
        .accounts({
          payer: program.provider.wallet.publicKey,
          chainLimit: chainLimitPDA,
          bridgeConfig: bridgeConfigPDA,
          committee: committeePDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      console.log("Bridge Limiter Initialized");
  } catch(e) {
      console.log("Bridge Limiter might be already initialized or failed:", e);
  }

  const [benfenBridgePDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(SEEDS.BENFEN_BRIDGE), committeePDA.toBuffer()],
    program.programId
  );

  console.log("Initializing Benfen Bridge...");
  try {
      await program.methods
        .initializeBenfenBridge()
        .accounts({
          authority: program.provider.wallet.publicKey,
          bridgeConfig: bridgeConfigPDA,
          committee: committeePDA,
          bridge: benfenBridgePDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      console.log("Benfen Bridge Initialized");
  } catch(e) {
      console.log("Benfen Bridge might be already initialized or failed:", e);
  }

  const [upgradeAuthorityPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(SEEDS.UPGRADE_AUTHORITY), committeePDA.toBuffer()],
    program.programId
  );

  console.log("Initializing Upgrade Authority...");
  try {
      await program.methods
        .initializeUpgradeAuthority(true)
        .accounts({
          payer: program.provider.wallet.publicKey,
          upgradeAuthority: upgradeAuthorityPDA,
          committee: committeePDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      console.log("Upgrade Authority Initialized");
  } catch(e) {
      console.log("Upgrade Authority might be already initialized or failed:", e);
  }
  const bpfLoaderUpgradeable = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");
     const [programDataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
        [program.programId.toBuffer()],
        bpfLoaderUpgradeable
      );

  //transfer_upgrade_authority to benfenBridgePDA
  console.log("Transferring Upgrade Authority to Benfen Bridge...");
  try {
      await program.methods
        .transferUpgradeAuthority()
        .accounts({
          program: program.programId,
          oldUpgradeAuthority: program.provider.wallet.publicKey,
          newUpgradeAuthority: upgradeAuthorityPDA,
          programData: programDataAddress,
          bpfLoaderUpgradeable: bpfLoaderUpgradeable,
        } as any)
        .rpc();
      console.log("Upgrade Authority Transferred to Benfen Bridge");
  } catch(e) {
      console.log("Upgrade Authority Transfer to Benfen Bridge failed:", e);
  }

}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
