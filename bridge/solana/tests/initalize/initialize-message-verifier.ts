import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, DEFAULTS } from "../../utils/const";
import { committeeInfo } from "../../utils/committee";

import { expect } from "chai";
import { assert } from "console";

describe("BenfenBridge - Message Verifier", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

  it("用非管理员账户初始化,应该失败",async()=>{
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );

    //检查config 账户存在
    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA,{ commitment: "confirmed" });
    expect(accountInfo).to.be.not.null;

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );
     //检查committee 账户存在
    const accountCommitteeInfo = await program.provider.connection.getAccountInfo(committeePDA,{ commitment: "confirmed" });
    expect(accountCommitteeInfo).to.be.not.null;


  // 创建一个新的非管理员账户
    const nonAdminKeypair = anchor.web3.Keypair.generate();
    
    // 为非管理员账户空投一些SOL用于交易费用
    const airdropSignature = await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      nonAdminKeypair.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );
    
    // 等待空投交易确认
    await anchor.workspace.BenfenBridge.provider.connection.confirmTransaction(airdropSignature);

    const nonAdminAccountInfo = await program.provider.connection.getAccountInfo(nonAdminKeypair.publicKey,{ commitment: "confirmed" });

    console.log(nonAdminAccountInfo)


    // 初始化messageVerifier
    const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
      program.programId
    );

       try {

        await program.methods.initializeMessageVerifier()
        .accounts({
          payer: nonAdminKeypair.publicKey,
          messageVerifier: messageVerifierPDA,
          committee: committeePDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        }as any)
        .signers([nonAdminKeypair])
        .rpc();
         expect.fail("Expected committee initialization to fail for non-admin account");
       }catch (error){
         // 预期的错误：权限不足或其他访问控制错误
        console.log("Expected error for non-admin account:", error.message);
        // 根据具体的错误类型进行验证
        expect(error.message).to.match(/(not approved)/i);

       }

  })

  it("用管理员账户初始化,应该成功",async()=>{

     const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );

    //检查config 账户存在
    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA,{ commitment: "confirmed" });
    expect(accountInfo).to.be.not.null;

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );
     //检查committee 账户存在
    const accountCommitteeInfo = await program.provider.connection.getAccountInfo(committeePDA,{ commitment: "confirmed" });
    expect(accountCommitteeInfo).to.be.not.null;


     let airdropTx=await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );   

    await anchor.workspace.BenfenBridge.provider.connection.confirmTransaction(airdropTx);



    // 初始化messageVerifier
    const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
      program.programId
    );


    let tx=await program.methods.initializeMessageVerifier()
    .accounts({
      payer: program.provider.wallet.publicKey,
      messageVerifier: messageVerifierPDA,
      committee: committeePDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    }as any)
    .rpc();


    await program.provider.connection.confirmTransaction(tx);


    await new Promise(resolve => setTimeout(resolve, 1000));




    //检查messageVerifier 账户存在
    const accountMessageVerifierInfo = await program.provider.connection.getAccountInfo(messageVerifierPDA,{ commitment: "confirmed" });
    expect(accountMessageVerifierInfo).to.be.not.null;

    const messageVerifierAccount = await program.account.messageVerifier.fetch(messageVerifierPDA);
    expect(messageVerifierAccount.committee.equals(committeePDA)).to.be.true;

  })

  it("should fail when trying to initialize existing message verifier",async()=>{
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );

    //检查config 账户存在
    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA,{ commitment: "confirmed" });
    expect(accountInfo).to.be.not.null;

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );
     //检查committee 账户存在
    const accountCommitteeInfo = await program.provider.connection.getAccountInfo(committeePDA,{ commitment: "confirmed" });
    expect(accountCommitteeInfo).to.be.not.null;


     let airdropTx=await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );   

    await program.provider.connection.confirmTransaction(airdropTx);

    const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
      program.programId
    );

    //检查messageVerifier 账户存在
    const accountMessageVerifierInfo = await program.provider.connection.getAccountInfo(messageVerifierPDA,{ commitment: "confirmed" });
    expect(accountMessageVerifierInfo).to.be.not.null;


      try {
        await program.methods.initializeMessageVerifier()
        .accounts({
          payer: program.provider.wallet.publicKey,
          messageVerifier: messageVerifierPDA,
          committee: committeePDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
        
        // 如果到达这里，说明没有抛出错误，测试应该失败
        expect.fail("Expected initialization to fail for existing account");
      } catch (error) {
        // 预期的错误：账户已存在
        console.log("Expected error caught:", error.message);
        expect(error.message).to.include("already in use");
      }


  });



});
