import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { SEEDS, CHAIN_IDS, MESSAGE_TYPES } from "../../utils/const";
import { expect } from "chai";
import { createUpgradePayload,createMessage,computeMessageHash } from "../../utils/message";
import { committeeInfo } from "../../utils/committee";
import { getSignature } from "../../utils/signatures";
import { BenfenBridge } from "../../target/types/benfen_bridge"; 



describe("upgrade program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BenfenBridge as Program;

  it("upgrade program", async () => {
    let message_type=MESSAGE_TYPES.UPGRADE;
    let nonce = 0; 
    let version=1;
    let chain_id=CHAIN_IDS.SOLANA_TESTNET;

      // 最新编译的代码写到buffer中
      const buffer = new anchor.web3.PublicKey("6PXJzWhbK1fF5uczh5rLERaSxZWJJ5it8mQz3NniT8Vq");

      //管理系统升级合约的程序ID
      const BPF_LOADER_UPGRADEABLE_ID = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");

     //program id
     const targetProgram = new anchor.web3.PublicKey("BenfeniwCqsDhGKB4snUSiz34DPMvGaz1hP2y4UMMWMo");


    const [programData] = anchor.web3.PublicKey.findProgramAddressSync(
      [program.programId.toBuffer()],
      BPF_LOADER_UPGRADEABLE_ID
    );


    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BRIDGE_CONFIG)],
        targetProgram
    );

      const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
          targetProgram
       );
       console.log("Committee PDA:", committeePDA.toString());


    //   const [upgradeAuthorityPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.UPGRADE_AUTHORITY),committeePDA.toBuffer()],
    //         program.programId
    //  );


    const [upgradeAuthorityPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.UPGRADE_AUTHORITY),committeePDA.toBuffer()],
     targetProgram
    );

    console.log("Target Program:", targetProgram.toString());
    console.log("program", program.programId);
    console.log("Program Data:", programData.toString());
    console.log("Upgrade Authority:", upgradeAuthorityPDA.toString());
    console.log("Buffer:", buffer.toString());
    console.log("Correct Upgrade Authority PDA:", upgradeAuthorityPDA.toString());
    console.log("Current Upgrade Authority PDA:", "7nqbKtMwEBAZ4taNWrEcyHnLNaGxvoNKSrPF7czpuH6D");
    console.log("Match:", upgradeAuthorityPDA.toString() === "7nqbKtMwEBAZ4taNWrEcyHnLNaGxvoNKSrPF7czpuH6D");

        


try {
    const programDataAccount = await provider.connection.getAccountInfo(programData);
    if (!programDataAccount) {
        throw new Error("Program data account not found");
    }
    console.log("Program data account found");
} catch (error) {
    console.error("Error fetching program data:", error);
}


       let payload = createUpgradePayload(targetProgram, buffer, 2);
       console.log(payload.length)

       let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            payload
        );


        let messageHash=computeMessageHash(message);
        let signatures=[];
        // committeeInfo 2 stake is 5000
        for (let i = 0; i < 3; i++) {
            const element = committeeInfo[i];
            let signature=getSignature(messageHash,element.privateKey);
            signatures.push(signature);
        }
        expect(signatures.length).to.be.equal(3);

        //生成一个新账号
        const nonAdminKeypair = anchor.web3.Keypair.generate();
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(nonAdminKeypair.publicKey, 1000000000),
            "confirmed"
        );

        await new Promise(resolve => setTimeout(resolve, 1000));


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
                programData,
                committee: committeePDA,
                bpfLoader: BPF_LOADER_UPGRADEABLE_ID,   
                systemProgram: anchor.web3.SystemProgram.programId,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                // bpfLoaderUpgradeable: BPF_LOADER_UPGRADEABLE_ID,
            } as any)
            .rpc();








       

    



   





        

  })
});
