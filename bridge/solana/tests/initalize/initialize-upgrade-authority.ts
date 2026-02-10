import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, DEFAULTS } from "../../utils/const";
import { committeeInfo } from "../../utils/committee";

import { expect } from "chai";
import { assert } from "console";


describe("BenfenBridge - Upgrade Authority", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

 it("初始化升级权限账户",async()=>{



   const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );

    const [upgradeAuthorityPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.UPGRADE_AUTHORITY),committeePDA.toBuffer()],
      program.programId
    );

    let tx=await program.methods.initializeUpgradeAuthority(true)
    .accounts({
      payer: program.provider.wallet.publicKey,
      upgradeAuthority: upgradeAuthorityPDA,
      committee: committeePDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    }as any)
    .rpc();

    await program.provider.connection.confirmTransaction(tx);

    await new Promise(resolve => setTimeout(resolve, 1000));



    // 检查升级权限账户是否存在
    const accountInfo = await program.provider.connection.getAccountInfo(upgradeAuthorityPDA,{ commitment: "confirmed" });
    expect(accountInfo).to.be.not.null;
    console.log("Upgrade Authority PDA:", upgradeAuthorityPDA.toString());

 })

});