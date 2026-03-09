import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, DEFAULTS } from "../../utils/const";
import { expect } from "chai";

describe("BenfenBridge - Bridge Config", () => {
  
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

  it("用非管理员账户初始化,应该失败",async ()=>{
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );
    
    // 创建一个新的非管理员账户
    const nonAdminKeypair = anchor.web3.Keypair.generate();
    
    // 为非管理员账户空投一些SOL用于交易费用
    await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      nonAdminKeypair.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );
    
    // 等待空投确认
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    try {
      // 尝试用非管理员账户初始化桥接配置
      await program.methods.initializeBridgeConfig(CHAIN_IDS.SOLANA_TESTNET)
      .accounts({
        payer: nonAdminKeypair.publicKey,
        bridgeConfig: bridgeConfigPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([nonAdminKeypair])
      .rpc();
      
      // 如果到达这里，说明没有抛出错误，测试应该失败
      expect.fail("Expected initialization to fail for non-admin account");
    } catch (error) {
  
      // 预期的错误：权限不足或其他访问控制错误
      console.log("Expected error for non-admin account:", error.message);
      // 根据具体的错误类型进行验证
      expect(error.message).to.match(/(not approved|already in use)/i);
    }
  })


  it("should initialize bridge config ", async () => {
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );
    
    await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );   
    
    let tx: string | null = null;
    try {
      tx = await program.methods.initializeBridgeConfig(CHAIN_IDS.SOLANA_TESTNET)
      .accounts({
        payer: program.provider.wallet.publicKey,
        bridgeConfig: bridgeConfigPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();
      console.log("Your transaction signature", tx);
      await program.provider.connection.confirmTransaction(tx);
    } catch (error) {
      console.log("Initialize bridge config error:", error.message);
      expect(error.message).to.match(/(already in use)/i);
    }
    
    const account = await program.account.bridgeConfig.fetch(bridgeConfigPDA);
    expect(account.chainId).equal(CHAIN_IDS.SOLANA_TESTNET);
    expect(account.chainCount).equal(0);
    expect(account.supportedChains).deep.equal(new Array(256).fill(255));
    expect(account.tokenCount.toNumber()).equal(0);
    
    // 延迟等待交易确认（包含“already in use”情况下的读一致性）
    await new Promise(resolve => setTimeout(resolve, 1000));
    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA, { commitment: "confirmed" });
    expect(accountInfo).to.be.not.null;
  });

  it("should fail when trying to initialize existing bridge config", async () => {
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );
    
    // 首先确保账户存在
    let accountExists = false;
    try {
      await program.account.bridgeConfig.fetch(bridgeConfigPDA);
      accountExists = true;
    } catch (error) {
      // 如果账户不存在，先创建一个
      await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
        program.provider.wallet.publicKey,
        DEFAULTS.AIRDROP_AMOUNT
      );
      
      await program.methods.initializeBridgeConfig(CHAIN_IDS.SOLANA_TESTNET)
      .accounts({
        payer: program.provider.wallet.publicKey,
        bridgeConfig: bridgeConfigPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();
      accountExists = true;
    }
    
    // 现在尝试重复初始化，应该失败
    if (accountExists) {
      try {
        await program.methods.initializeBridgeConfig(CHAIN_IDS.SOLANA_TESTNET)
        .accounts({
          payer: program.provider.wallet.publicKey,
          bridgeConfig: bridgeConfigPDA,
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
    }
  });

  
  it("should verify bridge config state", async () => {
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );
    
    // 验证账户状态
    const account = await program.account.bridgeConfig.fetch(bridgeConfigPDA);
    expect(account.chainId).equal(CHAIN_IDS.SOLANA_TESTNET);
    expect(account.chainCount).equal(0);
    expect(account.supportedChains).deep.equal(new Array(256).fill(255));
    expect(account.tokenCount.toNumber()).equal(0);
  });

});
