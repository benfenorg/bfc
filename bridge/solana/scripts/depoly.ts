// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { BenfenBridge } from "../target/types/benfen_bridge";
// import { SEEDS, CHAIN_IDS, DEFAULTS } from "./utils/const";
// import { expect } from "chai";
// import { committeeInfo } from "./utils/committee";

// async function main() {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;


//   await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
//       program.provider.wallet.publicKey,
//       DEFAULTS.AIRDROP_AMOUNT
//   );

//    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from(SEEDS.BRIDGE_CONFIG)],
//       program.programId
//     );


//    const tx = await program.methods.initializeBridgeConfig(CHAIN_IDS.SOLANA_TESTNET)
//     .accounts(
//     {
//         payer: program.provider.wallet.publicKey,
//         bridgeConfig: bridgeConfigPDA,
//         systemProgram: anchor.web3.SystemProgram.programId,
//      } as any
//     ).rpc();


//     // 初始化committee
//     const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
//       program.programId
//     );

//     let addresses = committeeInfo.map((item) => {
//       // Convert hex string to byte array [u8; 20]
//       const hex = item.address.startsWith('0x') ? item.address.slice(2) : item.address;
//       const bytes = [];
//       for (let i = 0; i < hex.length; i += 2) {
//         bytes.push(parseInt(hex.substr(i, 2), 16));
//       }
//       return bytes;
//     });
//     let stakes = committeeInfo.map((item) => item.stakes);
//     let minStakeRequired = 7500;
    
    
//     let committeeTx = await program.methods.initializeCommittee(
//       addresses,
//       stakes,
//       minStakeRequired  
//     )
//     .accounts({
//       payer: program.provider.wallet.publicKey,
//       committee: committeePDA,
//       bridgeConfig: bridgeConfigPDA,
//       systemProgram: anchor.web3.SystemProgram.programId,
//     } as any)
//     .rpc();

//     console.log("Your transaction signature", committeeTx);

//  const [messageVerifyPDA] = anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
//       program.programId
//   );

//   let messageVerifyTx = await program.methods.initializeMessageVerifier()
//   .accounts({
//     payer: program.provider.wallet.publicKey,
//     messageVerify: messageVerifyPDA,
//     committee: committeePDA,
//     systemProgram: anchor.web3.SystemProgram.programId,
//   } as any)
//   .rpc();

//   console.log("Your transaction signature", messageVerifyTx);

//   const [limitChainPDA] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from(SEEDS.CHAIN_LIMIT), Buffer.from([CHAIN_IDS.BENFEN_TESTNET]), committeePDA.toBuffer()],
//     program.programId
//   );


//   let limitChainTx = await program.methods.initializeBridgeLimiter(
//     bridgeConfigPDA,
//     CHAIN_IDS.BENFEN_TESTNET,
//     new anchor.BN(1000000000),
//     new anchor.BN(1000000000)
//   )
//   .accounts({
//     payer: program.provider.wallet.publicKey,
//     chainLimit: limitChainPDA,
//     committee: committeePDA,
//     systemProgram: anchor.web3.SystemProgram.programId,
//   } as any)
//   .rpc();

//   console.log("Your transaction signature", limitChainTx);

//   let benfenBridgeTx = await program.methods
//     .initializeBenfenBridge()
//     .accounts({
//       payer: program.provider.wallet.publicKey,
//       bridge: bridgeConfigPDA,
//       committee: committeePDA,
//       systemProgram: anchor.web3.SystemProgram.programId,
//     } as any)
//     .rpc();

//   console.log("Your transaction signature", benfenBridgeTx);














  






// }

// main().catch((error) => {
//   console.error("Error deploying program:", error);
// });

