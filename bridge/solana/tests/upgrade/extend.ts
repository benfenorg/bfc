import * as anchor from "@coral-xyz/anchor";
import { Program,BN } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { expect } from "chai";
import { SEEDS, CHAIN_IDS, MESSAGE_TYPES } from "../../utils/const";
import { createExtendPayload,createMessage,computeMessageHash } from "../../utils/message";
import { getSignature } from "../../utils/signatures";
import { committeeInfo } from "../../utils/committee";





describe("Extend Program size", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BenfenBridge as Program<BenfenBridge>;

  it("Transfer program upgrade authority to PDA", async () => {
    const bpfLoaderUpgradeable = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");
    
    // 计算committee PDA
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BRIDGE_CONFIG)],
        program.programId
    );

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
          program.programId
    );

    const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
          program.programId
    );


     const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([MESSAGE_TYPES.EXTEND_PROGRAM]),messageVerifierPDA.toBuffer()],
          program.programId
    );

    // 计算升级权限PDA
    const [upgradeAuthorityPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.UPGRADE_AUTHORITY),committeePDA.toBuffer()],
      program.programId
    );

    const upgradeAuthorityPDA1 = await program.provider.connection.getAccountInfo(upgradeAuthorityPDA);
    expect(upgradeAuthorityPDA1).to.be.not.null;

    // const programDataAddress = new anchor.web3.PublicKey("7cVwth8zehVouPkHvpXoavKKCT7Xqk15F34bms269SJq");

    const [programDataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [program.programId.toBuffer()],
      bpfLoaderUpgradeable
    );

    console.log("programDataAddress",programDataAddress.toString());
    console.log("upgradeAuthorityPDA",upgradeAuthorityPDA.toString());
    
    const info = await anchor.getProvider().connection.getAccountInfo(programDataAddress);
  console.log('ProgramData Address:', programDataAddress.toBase58());
  console.log('Data Length:', info?.data.length);

    let message_type=MESSAGE_TYPES.EXTEND_PROGRAM;
    let nonce = 0; 
    let version=1;
    let chain_id=CHAIN_IDS.SOLANA_TESTNET;

    //单次最多扩展10KB
    let payload = createExtendPayload(program.programId, 10000);
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
    // 调用指令
    await program.methods
      .extendProgram(
        message_type,
        version,
        new BN(nonce),
        chain_id,
        Buffer.from(payload),
        signatures.map(sig => Buffer.from(sig))
      )
      .accounts({
        payer: provider.wallet.publicKey,
        bridgeConfig: bridgeConfigPDA,
        committee: committeePDA,
        verifier: messageVerifierPDA,
        messageConfig: messageConfigPDA,
        upgradeAuthority: upgradeAuthorityPDA,
        program: program.programId,
        programData: programDataAddress,
        systemProgram: anchor.web3.SystemProgram.programId,
        bpfLoader: new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111"),
      } as any)
      .rpc()
    })


    

});