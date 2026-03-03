import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, DEFAULTS } from "../../utils/const";
import { expect } from "chai";

describe("BenfenBridge - Bridge Config", () => {
  
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
    await program.provider.connection.confirmTransaction(airdropSignature);

     await new Promise(resolve => setTimeout(resolve, 1000));


    const nonAdminAccountInfo = await program.provider.connection.getAccountInfo(nonAdminKeypair.publicKey,{ commitment: "confirmed" });
    expect(nonAdminAccountInfo).to.be.not.null;


     // 初始化chainLimit
    const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
      program.programId
    );

    console.log("TS derived configs:", bridgeConfigPDA.toBuffer());

    console.log("TS derived PDA:", chainLimitPDA.toBase58());

       try {

        console.log("Using chainLimit PDA in accounts:", chainLimitPDA.toBase58());

        await program.methods.initializeBridgeLimiter(
            CHAIN_IDS.BENFEN_TESTNET,
            new anchor.BN(1000000),
            new anchor.BN(1000000),
            new anchor.BN(1000000),
        )
        .accounts({
          payer: nonAdminKeypair.publicKey,
          chainLimit: chainLimitPDA,
          bridgeConfig: bridgeConfigPDA,
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

    // 创建一个新的非管理员账户
    
    // 为非管理员账户空投一些SOL用于交易费用
    const airdropSignature = await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );
    
    // 等待空投交易确认
    await program.provider.connection.confirmTransaction(airdropSignature);

    const adminAccountInfo = await program.provider.connection.getAccountInfo(program.provider.wallet.publicKey,{ commitment: "confirmed" });
    expect(adminAccountInfo).to.be.not.null;

    // 初始化chainLimit
    const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
      program.programId
    );


    let initTx = await program.methods.initializeBridgeLimiter(
            CHAIN_IDS.BENFEN_TESTNET,
            new anchor.BN(1000000),
            new anchor.BN(1000000),
            new anchor.BN(1000000),
    )
    .accounts({
        payer: program.provider.wallet.publicKey,
        chainLimit: chainLimitPDA,
        bridgeConfig: bridgeConfigPDA,
        committee: committeePDA,
        systemProgram: anchor.web3.SystemProgram.programId,
    }as any)
        .rpc();

    await program.provider.connection.confirmTransaction(initTx);
    await new Promise(resolve => setTimeout(resolve, 1000));

    console.log("s123456")

    //检查chainLimit 账户存在
    const accountChainLimitInfo = await program.provider.connection.getAccountInfo(chainLimitPDA,{ commitment: "confirmed" });
    expect(accountChainLimitInfo).to.be.not.null;
  })

  it("should fail when trying to initialize existing bridge limit",async()=>{

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


    // 为管理员账户空投一些SOL用于交易费用
    const airdropSignature = await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );
    
    // 等待空投交易确认
    await program.provider.connection.confirmTransaction(airdropSignature);


    const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
      program.programId
    );


      //检查chainLimit 账户存在
    const accountChainLimitInfo = await program.provider.connection.getAccountInfo(chainLimitPDA,{ commitment: "confirmed" });
    expect(accountChainLimitInfo).to.be.not.null;


      try {
        await program.methods.initializeBridgeLimiter(
            CHAIN_IDS.BENFEN_TESTNET,
            new anchor.BN(1000000),
            new anchor.BN(1000000),
            new anchor.BN(1000000),
        )
        .accounts({
          payer: program.provider.wallet.publicKey,
          chainLimit: chainLimitPDA,
          bridgeConfig: bridgeConfigPDA,
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




  })
});