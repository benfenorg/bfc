import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BenfenBridge } from "../target/types/benfen_bridge";

describe("solana-bridge", () => {
  // // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.AnchorProvider.env());

  // const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

  // it("Is initialized!", async () => {
  //   const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("bridge_config")],
  //     program.programId
  //   );

  //   // 重新构建并部署程序
  //   await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
  //     program.provider.wallet.publicKey,
  //     1000000000
  //   );

   
  // const tx = await program.methods
  // .initializeBridgeConfig(1, Buffer.from([2, 3]))
  // .accounts({
  //   payer: program.provider.wallet.publicKey,
  //   bridgeConfig: bridgeConfigPDA,
  //   systemProgram: anchor.web3.SystemProgram.programId,
  // } as any)
  // .rpc();
  //   console.log("Your transaction signature", tx);

  //   // 验证初始化结果
  //   const account = await program.account.bridgeConfig.fetch(bridgeConfigPDA);
  //   console.log("Bridge config:", account);

  //   const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA);
  //    console.log("Bridge config:", accountInfo);
  // });



  // it("initailiazed bridge config",async ()=>{
  //   const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("bridge_config")],
  //     program.programId
  //   );



  // })

  // it("Add token to bridge", async () => {
  //   const tx = await program.methods.addTokenToBridge(1, [2]).rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
