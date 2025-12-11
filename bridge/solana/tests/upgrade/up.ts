// import * as anchor from "@coral-xyz/anchor";
// import { Program, BN } from "@coral-xyz/anchor";
// import { SEEDS, CHAIN_IDS, MESSAGE_TYPES } from "../../utils/const";
// import { expect } from "chai";
// import { createUpgradePayload,createMessage,computeMessageHash } from "../../utils/message";
// import { committeeInfo } from "../../utils/committee";
// import { getSignature } from "../../utils/signatures";
// import { BenfenBridge } from "../../target/types/benfen_bridge"; 

// import {
//   Connection,
//   PublicKey,
//   Signer,
//   Transaction,
//   TransactionInstruction,
//   sendAndConfirmTransaction,
//   TransactionSignature,
//   clusterApiUrl,
// } from "@solana/web3.js"

// import * as borsh from "borsh";
// // import BN from "bn.js";



// describe("upgrade program", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.BenfenBridge as Program;

//   const targetProgramId = new anchor.web3.PublicKey("BenfeniwCqsDhGKB4snUSiz34DPMvGaz1hP2y4UMMWMo");
// const transaction = new Transaction()

// class UpgradeProgramArgs {
//   message_type: number;
//   version: number;
//   nonce: BN;
//   chain_id: number;
//   payload: Buffer;
//   signatures: Buffer[];

//   constructor(fields: {
//     message_type: number;
//     version: number;
//     nonce: BN;
//     chain_id: number;
//     payload: Buffer;
//     signatures: Buffer[];
//   }) {
//     this.message_type = fields.message_type;
//     this.version = fields.version;
//     this.nonce = fields.nonce;
//     this.chain_id = fields.chain_id;
//     this.payload = fields.payload;
//     this.signatures = fields.signatures;
//   }
// }

// const schema = new Map([
//   [UpgradeProgramArgs, {
//     kind: "struct",
//     fields: [
//       ["message_type", "u8"],
//       ["version", "u8"],
//       ["nonce", "u64"],
//       ["chain_id", "u8"],
//       ["payload", ["u8"]],          // 根据长度可改
//       ["signatures", [["u8"]]]      // 多签数组
//     ]
//   }]
// ]);




//     // const instruction = new TransactionInstruction({
//     //   keys: [],
//     //   programId: targetProgramId,
//     // })

//     // transaction.add(instruction);


// //   const program111 = new anchor.Program(anchor.workspace.BenfenBridge.idl, program.programId, provider);

// // const targetProgram = new Program<BenfenBridge>(
// //   anchor.workspace.BenfenBridge.idl,
// //   targetProgramId,
// //   provider
// // );
// // const targetProgram = new Program(IDL, targetProgramId, provider);


//   it("upgrade program", async () => {
//     let message_type=MESSAGE_TYPES.UPGRADE;
//     let nonce = 0; 
//     let version=1;
//     let chain_id=CHAIN_IDS.SOLANA_TESTNET;

//       // 最新编译的代码写到buffer中
//       const buffer = new anchor.web3.PublicKey("8qbJyzM8F1rGRkuBQaQeeESsuL3xJU88hVq5MNWaLVjF");

//       //管理系统升级合约的程序ID
//       const BPF_LOADER_UPGRADEABLE_ID = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");

//      //program id
//      const targetProgram = new anchor.web3.PublicKey("BenfeniwCqsDhGKB4snUSiz34DPMvGaz1hP2y4UMMWMo");


//     const [programData] = anchor.web3.PublicKey.findProgramAddressSync(
//       [program.programId.toBuffer()],
//       BPF_LOADER_UPGRADEABLE_ID
//     );


//     const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from(SEEDS.BRIDGE_CONFIG)],
//         targetProgram
//     );

//       const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
//           [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
//           targetProgram
//        );
//        console.log("Committee PDA:", committeePDA.toString());


//     //   const [upgradeAuthorityPDA] = anchor.web3.PublicKey.findProgramAddressSync(
//     //         [Buffer.from(SEEDS.UPGRADE_AUTHORITY),committeePDA.toBuffer()],
//     //         program.programId
//     //  );


//     const [upgradeAuthorityPDA] = anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from(SEEDS.UPGRADE_AUTHORITY),committeePDA.toBuffer()],
//      targetProgram
//     );

//     console.log("Target Program:", targetProgram.toString());
//     console.log("program", program.programId);
//     console.log("Program Data:", programData.toString());
//     console.log("Upgrade Authority:", upgradeAuthorityPDA.toString());
//     console.log("Buffer:", buffer.toString());
//     console.log("Correct Upgrade Authority PDA:", upgradeAuthorityPDA.toString());
//     console.log("Current Upgrade Authority PDA:", "7nqbKtMwEBAZ4taNWrEcyHnLNaGxvoNKSrPF7czpuH6D");
//     console.log("Match:", upgradeAuthorityPDA.toString() === "7nqbKtMwEBAZ4taNWrEcyHnLNaGxvoNKSrPF7czpuH6D");

        


// try {
//     const programDataAccount = await provider.connection.getAccountInfo(programData);
//     if (!programDataAccount) {
//         throw new Error("Program data account not found");
//     }
//     console.log("Program data account found");
// } catch (error) {
//     console.error("Error fetching program data:", error);
// }


//        let payload = createUpgradePayload(targetProgram, buffer, 2);
//        console.log(payload.length)

//        let message=createMessage(
//             message_type,
//             version,
//             BigInt(nonce),
//             chain_id,
//             payload
//         );


//         let messageHash=computeMessageHash(message);
//         let signatures=[];
//         // committeeInfo 2 stake is 5000
//         for (let i = 0; i < 3; i++) {
//             const element = committeeInfo[i];
//             let signature=getSignature(messageHash,element.privateKey);
//             signatures.push(signature);
//         }
//         expect(signatures.length).to.be.equal(3);

//         //生成一个新账号
//         const nonAdminKeypair = anchor.web3.Keypair.generate();
//         await provider.connection.confirmTransaction(
//             await provider.connection.requestAirdrop(nonAdminKeypair.publicKey, 1000000000),
//             "confirmed"
//         );

//         await new Promise(resolve => setTimeout(resolve, 1000));

//         const data = borsh.serialize(schema, new UpgradeProgramArgs({
//             message_type,
//             version,
//             nonce:  new BN(nonce),
//             chain_id,
//             payload: Buffer.from(payload),
//             signatures: signatures.map(sig => Buffer.from(sig))
//         }));


//         const keys = [
//             { pubkey: provider.wallet.publicKey, isSigner: true, isWritable: true },  // payer
//             { pubkey: provider.wallet.publicKey, isSigner: false, isWritable: true }, // spill
//             { pubkey: upgradeAuthorityPDA, isSigner: false, isWritable: false },
//             { pubkey: buffer, isSigner: false, isWritable: true },
//             { pubkey: targetProgram, isSigner: false, isWritable: true },
//             { pubkey: programData, isSigner: false, isWritable: true },
//             { pubkey: committeePDA, isSigner: false, isWritable: false },
//             { pubkey: BPF_LOADER_UPGRADEABLE_ID, isSigner: false, isWritable: false },
//             { pubkey: anchor.web3.SystemProgram.programId, isSigner: false, isWritable: false },
//             { pubkey: anchor.web3.SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false },
//             { pubkey: anchor.web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
//             // { pubkey: nonAdminKeypair.publicKey, isSigner: false, isWritable: true },
//         ];

//         const data1 = Buffer.from(data);


//         const instruction = new TransactionInstruction({
//             programId: targetProgramId, // 你的 upgradeProgram 所在 programId
//             keys,
//             data: data1
//         });

//         const transaction = new Transaction().add(instruction);

//         // 发送交易
//         const txSignature = await sendAndConfirmTransaction(
//             provider.connection,
//             transaction,
//             [provider.wallet.payer] // 签名者
//         );

//         console.log("Transaction Signature:", txSignature);
//   })
// });
