import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, DEFAULTS } from "../../utils/const";
import { committeeInfo } from "../../utils/committee";
import { expect } from "chai";


describe("BenfenBridge - Committee", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;



  it("用非管理员账户初始化,应该失败",async()=>{

    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );
    //准备前置条件 
    //先检查config 账户存在
    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA,{ commitment: "confirmed" });
    expect(accountInfo).to.be.not.null;

    // 创建一个新的非管理员账户
    const nonAdminKeypair = anchor.web3.Keypair.generate();
    
    // 为非管理员账户空投一些SOL用于交易费用
    await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      nonAdminKeypair.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );

     // 初始化committee
    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );

    let addresses = committeeInfo.map((item) => {
      // Convert hex string to byte array [u8; 20]
      const hex = item.address.startsWith('0x') ? item.address.slice(2) : item.address;
      const bytes = [];
      for (let i = 0; i < hex.length; i += 2) {
        bytes.push(parseInt(hex.substr(i, 2), 16));
      }
      return bytes;
    });
    let stakes = committeeInfo.map((item) => item.stakes);
    let minStakeRequired = 7500;



    
    // 等待空投确认
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    try {
      // 尝试用非管理员账户初始化桥接配置
      await program.methods.initializeCommittee(
        addresses,
        stakes,
        minStakeRequired  
      )
      .accounts({
        payer: nonAdminKeypair.publicKey,
        committee: committeePDA,
        bridgeConfig: bridgeConfigPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([nonAdminKeypair])
      .rpc();
      
      // 如果到达这里，说明没有抛出错误，测试应该失败
      expect.fail("Expected committee initialization to fail for non-admin account");
    } catch (error) {
      // 预期的错误：权限不足或其他访问控制错误
      console.log("Expected error for non-admin account:", error.message);
      // 根据具体的错误类型进行验证
      expect(error.message).to.match(/(not approved)/i);

    }


  })

  it("用重复地址初始化,应该失败",async()=>{

    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );

    await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );   
    // 等待空投确认
    await new Promise(resolve => setTimeout(resolve, 1000));

    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA,{ commitment: "confirmed" });
    expect(accountInfo).to.be.not.null;

  
    // 初始化committee
    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );

    console.log("Committee PDA:", committeePDA.toString());

    let addresses = committeeInfo.map((item) => {
      // Convert hex string to byte array [u8; 20]
      const hex = item.address.startsWith('0x') ? item.address.slice(2) : item.address;
      const bytes = [];
      for (let i = 0; i < hex.length; i += 2) {
        bytes.push(parseInt(hex.substr(i, 2), 16));
      }
      return bytes;
    });
    let stakes = committeeInfo.map((item) => item.stakes);
    let minStakeRequired = 7500;

    // 添加重复地址
    addresses.push(addresses[0]);
    stakes.push(stakes[0]);
    
    
  
      try {
          let committeeTx = await program.methods.initializeCommittee(
          addresses,
          stakes,
          minStakeRequired  
        )
        .accounts({
          payer: program.provider.wallet.publicKey,
          committee: committeePDA,
          bridgeConfig: bridgeConfigPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

        expect.fail("Expected committee initialization to fail for add duplicate address");

      }catch(error){
        console.log(error.message);
        expect(error.message).to.match(/(duplicate)/i);
        console.log(error);
      }



  })


  it("should initialize committee ", async () => {
    //重置网络
    //部署
    // 初始化桥接配置
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );

    await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );   

    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA,{ commitment: "confirmed" });
    expect(accountInfo).to.be.not.null;

    
    const account = await program.account.bridgeConfig.fetch(bridgeConfigPDA);
    console.log("second")
    expect(account.chainId).equal(CHAIN_IDS.SOLANA_TESTNET);
    expect(account.chainCount).equal(0);
    expect(account.supportedChains).deep.equal(new Array(256).fill(255));
    expect(account.tokenCount.toNumber()).equal(0);

    // 初始化committee
    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );

    let addresses = committeeInfo.map((item) => {
      // Convert hex string to byte array [u8; 20]
      const hex = item.address.startsWith('0x') ? item.address.slice(2) : item.address;
      const bytes = [];
      for (let i = 0; i < hex.length; i += 2) {
        bytes.push(parseInt(hex.substr(i, 2), 16));
      }
      return bytes;
    });
    let stakes = committeeInfo.map((item) => item.stakes);
    let minStakeRequired = 7500;
    
    
    let committeeTx = await program.methods.initializeCommittee(
      addresses,
      stakes,
      minStakeRequired  
    )
    .accounts({
      payer: program.provider.wallet.publicKey,
      committee: committeePDA,
      bridgeConfig: bridgeConfigPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    } as any)
    .rpc();

    console.log("Your transaction signature", committeeTx);

    const committeeAccount = await program.account.committee.fetch(committeePDA);
    expect(committeeAccount.memberCount).equal(4);
    expect(committeeAccount.minStakeRequired).equal(minStakeRequired);
    expect(committeeAccount.members.slice(0, 4).map((item) => item.address)).deep.equal(addresses);
    expect(committeeAccount.members.slice(0, 4).map((item) => item.stake)).deep.equal(stakes);

  });

  it("should fail when trying to initialize existing bridge committee",async ()=>{
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );

    await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
      program.provider.wallet.publicKey,
      DEFAULTS.AIRDROP_AMOUNT
    );   
    //检查config 账户存在
    const accountInfo = await program.provider.connection.getAccountInfo(bridgeConfigPDA,{ commitment: "confirmed" });
    expect(accountInfo).to.be.not.null;

    // 初始化committee
    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
      program.programId
    );

    let addresses = committeeInfo.map((item) => {
      // Convert hex string to byte array [u8; 20]
      const hex = item.address.startsWith('0x') ? item.address.slice(2) : item.address;
      const bytes = [];
      for (let i = 0; i < hex.length; i += 2) {
        bytes.push(parseInt(hex.substr(i, 2), 16));
      }
      return bytes;
    });
    let stakes = committeeInfo.map((item) => item.stakes);
    let minStakeRequired = 7500;

    


    
    // 首先确保账户存在
    let accountExists = false;
    try {
      await program.account.committee.fetch(committeePDA);
      accountExists = true;
    } catch (error) {
      // 如果账户不存在，先创建一个
      await anchor.workspace.BenfenBridge.provider.connection.requestAirdrop(
        program.provider.wallet.publicKey,
        DEFAULTS.AIRDROP_AMOUNT
      );
      
      await program.methods.initializeCommittee(
        addresses,
        stakes,
        minStakeRequired  
      )
      .accounts({
        payer: program.provider.wallet.publicKey,
        committee: committeePDA,
        bridgeConfig: bridgeConfigPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();
      accountExists = true;
    }
    
    // 现在尝试重复初始化，应该失败
    if (accountExists) {
      try {
        await program.methods.initializeCommittee(
          addresses,
          stakes,
          minStakeRequired  
        )
        .accounts({
          payer: program.provider.wallet.publicKey,
          committee: committeePDA,
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
    


  })





});
