import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, DEFAULTS } from "../../utils/const";
import { expect } from "chai";

describe("BenfenBridge -  init benfen bridge", () => {

    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;


    it("用非管理员账户初始化,应该失败",async()=>{

     const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );

    //检查config 账户存在
    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA);
    expect(accountInfo).to.be.not.null;

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );
     //检查committee 账户存在
    const accountCommitteeInfo = await program.provider.connection.getAccountInfo(committeePDA);
    expect(accountCommitteeInfo).to.be.not.null;

    // 创建一个新的非管理员账户
    const nonAdminKeypair = anchor.web3.Keypair.generate();
    
    // 为非管理员账户空投一些SOL用于交易费用
    const airdropSignature = await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      nonAdminKeypair.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );
    
    // 等待空投交易确认
    await program.provider.connection.confirmTransaction(airdropSignature);

    const nonAdminAccountInfo = await program.provider.connection.getAccountInfo(nonAdminKeypair.publicKey);
    expect(nonAdminAccountInfo).to.be.not.null;

    const [benfenBridgePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BENFEN_BRIDGE), committeePDA.toBuffer()],
      program.programId
    );



     try {

        await program.methods.initializeBenfenBridge()
        .accounts({
          authority: nonAdminKeypair.publicKey,
          bridgeConfig: bridgeConfigPDA,
          committee: committeePDA,
          bridge: benfenBridgePDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        }as any)
        .signers([nonAdminKeypair])
        .rpc();
         expect.fail("Expected initialize benfen bridge to fail for non-admin account");
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
    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA);
    expect(accountInfo).to.be.not.null;

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );
     //检查committee 账户存在
    const accountCommitteeInfo = await program.provider.connection.getAccountInfo(committeePDA);
    expect(accountCommitteeInfo).to.be.not.null;


    // 为管理员账户空投一些SOL用于交易费用
    const airdropSignature = await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );

    let [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
      program.programId
    );

    // 等待空投交易确认
    await program.provider.connection.confirmTransaction(airdropSignature);

    // 初始化桥接器
    await program.methods.initializeBenfenBridge()
    .accounts({
      authority: program.provider.wallet.publicKey,
      bridgeConfig: bridgeConfigPDA,
      bridge:benfenBridgePDA,
      committee: committeePDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    }as any)
    .rpc();

    // 检查桥接器账户是否存在
    const bridgeAccountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA,{ commitment: "confirmed" });
    expect(bridgeAccountInfo).to.be.not.null;


    })

    it("用管理员账户第二次初始化,应该失败",async()=>{   
        
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


    let [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
      program.programId
    );



    // 为管理员账户空投一些SOL用于交易费用
    const airdropSignature = await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );
    
    // 等待空投交易确认
    await program.provider.connection.confirmTransaction(airdropSignature);


    let bridgeAccountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA,{ commitment: "confirmed" });
    expect(bridgeAccountInfo).to.be.not.null;

    // 初始化桥接器   

      try {
        await program.methods.initializeBenfenBridge()
        .accounts({
        authority: program.provider.wallet.publicKey,
        bridgeConfig: bridgeConfigPDA,
        committee: committeePDA,
        bridge:benfenBridgePDA,
        systemProgram: anchor.web3.SystemProgram.programId,
        }as any)
        .rpc();
        
        // 如果到达这里，说明没有抛出错误，测试应该失败
        expect.fail("Expected initialization to fail for existing account");
      } catch (error) {
        // 预期的错误：账户已存在
        console.log("Expected error caught:", error.message);
        expect(error.message).to.include("already in use");
      }


    })


})