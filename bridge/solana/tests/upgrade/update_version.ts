// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { BenfenBridge } from "../../target/types/benfen_bridge";
// import { SEEDS,TOKEN_IDS } from "../../utils/const";
// import { expect } from "chai";


// describe("Transfer Authority", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

//   it("Transfer program upgrade authority to PDA", async () => {


//     const bpfLoaderUpgradeable = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");
//     // 计算committee PDA
//     const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
//         [Buffer.from(SEEDS.BRIDGE_CONFIG)],
//         program.programId
//     );

//     const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
//           [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
//           program.programId
//     );

//     let tokenId=TOKEN_IDS.USDT;
//     const encodeTokenId = BigInt(tokenId); // Replace with actual token ID

//     // 把 token_id 转换成 big-endian Uint8Array (8 字节)
//     const tokenIdBytes = Buffer.alloc(8);
//     tokenIdBytes.writeBigUInt64BE(encodeTokenId); 

//     const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
//             [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
//             program.programId
//     );

//   //update_token_config_version
  
   
//     // 调用指令
//     await program.methods.updateTokenConfigVersion(
//       2,
//     ).accounts({
//       tokenConfig: tokenConfigPDA,
//       systemProgram: anchor.web3.SystemProgram.programId,
//       } as any)
//       .rpc()
    

    
//     const tokenInfo = await program.account.tokenConfigAccount.fetch(tokenConfigPDA);
//     expect(tokenInfo.version).to.equal(2);
//     // let tokenConfigPDAA=await get

//     // 检查版本是否更新
    


//     })

// });