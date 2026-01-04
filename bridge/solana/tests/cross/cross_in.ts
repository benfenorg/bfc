import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, MESSAGE_TYPES,TOKEN_IDS } from "../../utils/const";
import { expect } from "chai";
import { createMessage, computeMessageHash,createBlocklistPayload } from "../../utils/message";
import { committeeInfo } from "../../utils/committee";
import { getSignature } from "../../utils/signatures";
import { deployUSDC, USDCDeploymentResult} from "../../utils/tokens/deploy_usdc";
import { getAssociatedTokenAddress, createAssociatedTokenAccountInstruction,getAccount,NATIVE_MINT } from '@solana/spl-token';
import { getTokenByTokenId } from "../../utils/tokens/token-manager";

import { benfenAddressToBytes, getDefaultBenfenAddressBytes,tokenIdToBytes } from "../../utils/util";

import {
  LAMPORTS_PER_SOL, 
} from "@solana/web3.js";
import { wrapSol } from "../../utils/tokens/wsol";


describe("BenfenBridge - Cross Chain", () => {
    const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;



    // beforeEach(async()=>{
    //     // const connection = new Connection("http://localhost:8899", "confirmed");
    //     // const payer = program.provider.wallet.publicKey
    //     deployToken=await deployUSDC(program.provider.connection,program.provider.wallet.payer,1_000_000,6);
    // })


    it("should cross chain sol(native token)",async ()=>{
        let tokenId=TOKEN_IDS.SOL;
        let tokenInfo= getTokenByTokenId(tokenId)
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }

        let solMint=new anchor.web3.PublicKey(tokenInfo.mintAddress);

        const tokenIdBytes=tokenIdToBytes(tokenId);

        const  benfenAddressBytes=getDefaultBenfenAddressBytes();


         const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );

        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        //verifier
        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );

        //message_config
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );

         const messageInfo = await program.provider.connection.getAccountInfo(messageConfigPDA,{ commitment: "confirmed" });

         let messageBeforeNonce=new BN(0);
         if (messageInfo!=null){
            let messageConfig=await program.account.messageConfig.fetch(messageConfigPDA);
            if (messageConfig!=null){
                messageBeforeNonce=messageConfig.nonce;
            }
         }



        //判断messageConfigPDA 是否存在
       

        //token_config
          const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );

        //chain_limit
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        //bridge
        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
            program.programId
        );

        //token valut 已经在add token 创建
        const [tokenValutPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        expect(tokenValutPDA).to.be.not.null;


        const mintInfo = await program.provider.connection.getAccountInfo(solMint);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            solMint,
            program.provider.wallet.publicKey
        );
        
        await wrapSol(program.provider.connection,program.provider.wallet.payer);
        await new Promise(resolve => setTimeout(resolve, 1000));

        let beforeTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(beforeTokenValut.amount)

        expect(beforeTokenValut.amount).to.be.equal(BigInt(0));

        


        // 检查关联token账户是否存在
        const tokenAccountInfo = await program.provider.connection.getAccountInfo(userTokenAccount);
        expect(tokenAccountInfo).to.be.not.null;

        let crossInTx=await program.methods.crossTokenToBridge(
            //new anchor.BN(tokenId),
            new anchor.BN(LAMPORTS_PER_SOL),
            benfenAddressBytes,
        )
        .accounts({
          payer: program.provider.wallet.publicKey,
          tokenAccount: userTokenAccount,
          tokenVault: tokenValutPDA,
          messageConfig: messageConfigPDA,
          tokenConfig: tokenConfigPDA,
          chainLimit: chainLimitPDA,
          bridgeConfig: bridgeConfigPDA,
          bridge: benfenBridgePDA,
          verifier: messageVerifierPDA,
          tokenMint: solMint,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

        await program.provider.connection.confirmTransaction(crossInTx);

        await new Promise(resolve => setTimeout(resolve, 1000));

          // 检查关联tokenValut账户的资金
        const tokenValutAccountInfo = await program.provider.connection.getAccountInfo(tokenValutPDA);
        expect(tokenValutAccountInfo).to.be.not.null;
    
        // const tokenValutAccount = userTokenAccount.fromAccountData(tokenValutAccountInfo.data);
        // expect(tokenValutAccount.amount).to.be.equal(1000000);

        const afterTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(afterTokenValut.amount)

        expect(afterTokenValut.amount).to.be.equal(BigInt(LAMPORTS_PER_SOL));

        let afterMessageConfig=await program.account.messageConfig.fetch(messageConfigPDA);
        console.log(afterMessageConfig.nonce.toNumber())
        expect(afterMessageConfig.nonce.toNumber()).to.be.equal(messageBeforeNonce.add(new BN(1)).toNumber());
    })


    it("should cross chain USDT", async () => {
        let tokenId=TOKEN_IDS.USDT;
        let tokenInfo= getTokenByTokenId(tokenId)
        console.log(tokenInfo)

        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        const deployToken: USDCDeploymentResult = {
            mintAddress: new anchor.web3.PublicKey(tokenInfo.mintAddress),
            tokenAccountAddress: new anchor.web3.PublicKey(tokenInfo.tokenAccountAddress)
        };
        

        const usdMint = deployToken.mintAddress;

        const tokenIdBytes=tokenIdToBytes(tokenId);

        const  benfenAddressBytes=getDefaultBenfenAddressBytes();

        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );

        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        //verifier
        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );

        //message_config
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
         const messageInfo = await program.provider.connection.getAccountInfo(messageConfigPDA,{ commitment: "confirmed" });

        let messageBeforeNonce=new BN(0);
        if (messageInfo!=null){
            let messageConfig=await program.account.messageConfig.fetch(messageConfigPDA);
            if (messageConfig!=null){
                messageBeforeNonce=messageConfig.nonce;
            }
         }

        
        //token_config
          const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );

        //chain_limit
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        //bridge
        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
            program.programId
        );

        //token valut 已经在add token 创建
        const [tokenValutPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        expect(tokenValutPDA).to.be.not.null;


        const mintInfo = await program.provider.connection.getAccountInfo(deployToken.mintAddress);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            usdMint,
            program.provider.wallet.publicKey
        );

        await new Promise(resolve => setTimeout(resolve, 1000));

        let beforeTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(beforeTokenValut.amount)

        expect(beforeTokenValut.amount).to.be.equal(BigInt(0));

        


        // 检查关联token账户是否存在
        const tokenAccountInfo = await program.provider.connection.getAccountInfo(userTokenAccount);
        expect(tokenAccountInfo).to.be.not.null;

        let crossInTx=await program.methods.crossTokenToBridge(
            //new anchor.BN(tokenId),
            new anchor.BN(1000000),
            benfenAddressBytes,
        )
        .accounts({
          payer: program.provider.wallet.publicKey,
          tokenAccount: userTokenAccount,
          tokenVault: tokenValutPDA,
          messageConfig: messageConfigPDA,
          tokenConfig: tokenConfigPDA,
          chainLimit: chainLimitPDA,
          bridgeConfig: bridgeConfigPDA,
          bridge: benfenBridgePDA,
          verifier: messageVerifierPDA,
          tokenMint: usdMint,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

        await program.provider.connection.confirmTransaction(crossInTx);

        console.log("USDCUSDC 跨到Benfen",crossInTx)

        await new Promise(resolve => setTimeout(resolve, 1000));

        // 检查关联tokenValut账户的资金
        const tokenValutAccountInfo = await program.provider.connection.getAccountInfo(tokenValutPDA);
        expect(tokenValutAccountInfo).to.be.not.null;
    
        const afterTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(afterTokenValut.amount)

        expect(afterTokenValut.amount).to.be.equal(BigInt(1000000));

           let afterMessageConfig=await program.account.messageConfig.fetch(messageConfigPDA);
        console.log(afterMessageConfig.nonce.toNumber())
        expect(afterMessageConfig.nonce.toNumber()).to.be.equal(messageBeforeNonce.add(new BN(1)).toNumber());
        
    })

    it("cross in usdc not match token",async ()=>{
        let tokenId=TOKEN_IDS.USDC;
        let mockErrorTokenId=TOKEN_IDS.USDT;
        let tokenInfo= getTokenByTokenId(tokenId)
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        const deployToken: USDCDeploymentResult = {
            mintAddress: new anchor.web3.PublicKey(tokenInfo.mintAddress),
            tokenAccountAddress: new anchor.web3.PublicKey(tokenInfo.tokenAccountAddress)
        };

         const usdMint = deployToken.mintAddress;
        //token id 转换成字节数组用于生成 派生PDA账户
        const tokenIdBytes=tokenIdToBytes(tokenId);

        const mockErrorTokenIdBytes=tokenIdToBytes(mockErrorTokenId);
        const  benfenAddressBytes=getDefaultBenfenAddressBytes();

        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );

        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        //verifier
        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );

        //message_config
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
        //token_config
        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );

        const [mockErrorTokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),mockErrorTokenIdBytes],
            program.programId
        );

        //chain_limit
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        //bridge
        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
            program.programId
        );

        //token valut 已经在add token 创建
        const [tokenValutPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        expect(tokenValutPDA).to.be.not.null;


        const mintInfo = await program.provider.connection.getAccountInfo(deployToken.mintAddress);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            usdMint,
            program.provider.wallet.publicKey
        );

        //查询账户里有多少usdc
         let beforeAccountValut = await getAccount(
            program.provider.connection,
            userTokenAccount
        );

        let beforeAccountAmount=beforeAccountValut.amount;




        await new Promise(resolve => setTimeout(resolve, 1000));

        let beforeTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(beforeTokenValut.amount)
        expect(beforeTokenValut.amount).to.be.equal(BigInt(0));

        // 检查关联token账户是否存在
        const tokenAccountInfo = await program.provider.connection.getAccountInfo(userTokenAccount);
        expect(tokenAccountInfo).to.be.not.null;

        try {
            await program.methods.crossTokenToBridge(
                //超过用户余额
                new anchor.BN(beforeAccountAmount+BigInt(1)),
                benfenAddressBytes,
            )
            .accounts({
            payer: program.provider.wallet.publicKey,
            tokenAccount: userTokenAccount,
            tokenVault: tokenValutPDA,
            messageConfig: messageConfigPDA,
            tokenConfig: mockErrorTokenConfigPDA,
            chainLimit: chainLimitPDA,
            bridgeConfig: bridgeConfigPDA,
            bridge: benfenBridgePDA,
            verifier: messageVerifierPDA,
            tokenMint: usdMint,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
            } as any)
            .rpc();
            expect.fail("Expected crossTokenToBridge to fail for token not match");
            
        }catch (error){
            console.log(error)
            expect(error.message).to.include("Invalid token mint");
        }
    })

    it("cross in usdc insufficient allowance",async ()=>{
         //let deployToken : USDCDeploymentResult;
        let tokenId=TOKEN_IDS.USDC;
        let tokenInfo= getTokenByTokenId(tokenId)
        let amount=1000000;
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        const deployToken: USDCDeploymentResult = {
            mintAddress: new anchor.web3.PublicKey(tokenInfo.mintAddress),
            tokenAccountAddress: new anchor.web3.PublicKey(tokenInfo.tokenAccountAddress)
        };
        
        const usdMint = deployToken.mintAddress;
        //token id 转换成字节数组用于生成 派生PDA账户
        const tokenIdBytes=tokenIdToBytes(tokenId);
        const  benfenAddressBytes=getDefaultBenfenAddressBytes();

        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );

        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        //verifier
        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );

        //message_config
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
        //token_config
          const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );

        //chain_limit
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        //bridge
        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
            program.programId
        );

        //token valut 已经在add token 创建
        const [tokenValutPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        expect(tokenValutPDA).to.be.not.null;


        const mintInfo = await program.provider.connection.getAccountInfo(deployToken.mintAddress);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            usdMint,
            program.provider.wallet.publicKey
        );

        //查询账户里有多少usdc
         let beforeAccountValut = await getAccount(
            program.provider.connection,
            userTokenAccount
        );

        let beforeAccountAmount=beforeAccountValut.amount;




        await new Promise(resolve => setTimeout(resolve, 1000));

        let beforeTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(beforeTokenValut.amount)
        expect(beforeTokenValut.amount).to.be.equal(BigInt(0));

        // 检查关联token账户是否存在
        const tokenAccountInfo = await program.provider.connection.getAccountInfo(userTokenAccount);
        expect(tokenAccountInfo).to.be.not.null;

        try {
            await program.methods.crossTokenToBridge(
                //超过用户余额
                new anchor.BN(beforeAccountAmount+BigInt(1)),
                benfenAddressBytes,
            )
            .accounts({
            payer: program.provider.wallet.publicKey,
            tokenAccount: userTokenAccount,
            tokenVault: tokenValutPDA,
            messageConfig: messageConfigPDA,
            tokenConfig: tokenConfigPDA,
            chainLimit: chainLimitPDA,
            bridgeConfig: bridgeConfigPDA,
            bridge: benfenBridgePDA,
            verifier: messageVerifierPDA,
            tokenMint: usdMint,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
            } as any)
            .rpc();
            expect.fail("Expected crossTokenToBridge to fail for insufficient allowance");
            
        }catch (error){
            console.log(error)
            expect(error.message).to.include("Insufficient balance");
        }

    })

    it("cross in usdc invalid recipient address",async ()=>{

        let tokenId=TOKEN_IDS.USDC;
        let tokenInfo= getTokenByTokenId(tokenId)
       
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        const deployToken: USDCDeploymentResult = {
            mintAddress: new anchor.web3.PublicKey(tokenInfo.mintAddress),
            tokenAccountAddress: new anchor.web3.PublicKey(tokenInfo.tokenAccountAddress)
        };
        
        const usdMint = deployToken.mintAddress;
        //token id 转换成字节数组用于生成 派生PDA账户
        const tokenIdBytes=tokenIdToBytes(tokenId);
        const  benfenAddressBytes=benfenAddressToBytes("0x1234567890123456789012345678901234567890",false);

        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );

        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        //verifier
        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );

        //message_config
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
        //token_config
          const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );

        //chain_limit
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        //bridge
        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
            program.programId
        );

        //token valut 已经在add token 创建
        const [tokenValutPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        expect(tokenValutPDA).to.be.not.null;


        const mintInfo = await program.provider.connection.getAccountInfo(deployToken.mintAddress);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            usdMint,
            program.provider.wallet.publicKey
        );

        //查询账户里有多少usdc
         let beforeAccountValut = await getAccount(
            program.provider.connection,
            userTokenAccount
        );


        let beforeAccountAmount=beforeAccountValut.amount;
        console.log("beforeAccountAmount",beforeAccountAmount)
        expect(Number(beforeAccountAmount)).to.be.greaterThanOrEqual(5000*100000000);




        await new Promise(resolve => setTimeout(resolve, 1000));

        let beforeTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(beforeTokenValut.amount)
        expect(beforeTokenValut.amount).to.be.equal(BigInt(0));

        // 检查关联token账户是否存在
        const tokenAccountInfo = await program.provider.connection.getAccountInfo(userTokenAccount);
        expect(tokenAccountInfo).to.be.not.null;

        try {
            await program.methods.crossTokenToBridge(
                //new anchor.BN(tokenId),
                //小于用户余额
                new anchor.BN(beforeAccountAmount-BigInt(1)),
                benfenAddressBytes,
            )
            .accounts({
            payer: program.provider.wallet.publicKey,
            tokenAccount: userTokenAccount,
            tokenVault: tokenValutPDA,
            messageConfig: messageConfigPDA,
            tokenConfig: tokenConfigPDA,
            chainLimit: chainLimitPDA,
            bridgeConfig: bridgeConfigPDA,
            bridge: benfenBridgePDA,
            verifier: messageVerifierPDA,
            tokenMint: usdMint,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
            } as any)
            .rpc();
            expect.fail("Expected crossTokenToBridge to fail for invalid recipient address");
            
        }catch (error){
            console.log(error)
            expect(error.message).to.include("Invalid benfen address");
        }
    })

    it("cross in usdc exceed single transfer limit",async ()=>{    

        let tokenId=TOKEN_IDS.USDC;
        let tokenInfo= getTokenByTokenId(tokenId)
       
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        const deployToken: USDCDeploymentResult = {
            mintAddress: new anchor.web3.PublicKey(tokenInfo.mintAddress),
            tokenAccountAddress: new anchor.web3.PublicKey(tokenInfo.tokenAccountAddress)
        };
        
        const usdMint = deployToken.mintAddress;
        //token id 转换成字节数组用于生成 派生PDA账户
        const tokenIdBytes=tokenIdToBytes(tokenId);
        const benfenAddressBytes=getDefaultBenfenAddressBytes();

        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );

        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        //verifier
        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );

        //message_config
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
        //token_config
          const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );

        //chain_limit
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        //bridge
        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
            program.programId
        );

        //token valut 已经在add token 创建
        const [tokenValutPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        expect(tokenValutPDA).to.be.not.null;


        const mintInfo = await program.provider.connection.getAccountInfo(deployToken.mintAddress);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            usdMint,
            program.provider.wallet.publicKey
        );

        //查询账户里有多少usdc
         let beforeAccountValut = await getAccount(
            program.provider.connection,
            userTokenAccount
        );


        let beforeAccountAmount=beforeAccountValut.amount;
        console.log("beforeAccountAmount",beforeAccountAmount)
        expect(Number(beforeAccountAmount)).to.be.greaterThanOrEqual(5000*100000000);




        await new Promise(resolve => setTimeout(resolve, 1000));

        let beforeTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(beforeTokenValut.amount)
        expect(beforeTokenValut.amount).to.be.equal(BigInt(0));

        // 检查关联token账户是否存在
        const tokenAccountInfo = await program.provider.connection.getAccountInfo(userTokenAccount);
        expect(tokenAccountInfo).to.be.not.null;

        try {
            await program.methods.crossTokenToBridge(
               // new anchor.BN(tokenId),
                //小于用户余额
                new anchor.BN(beforeAccountAmount-BigInt(1)),
                benfenAddressBytes,
            )
            .accounts({
            payer: program.provider.wallet.publicKey,
            tokenAccount: userTokenAccount,
            tokenVault: tokenValutPDA,
            messageConfig: messageConfigPDA,
            tokenConfig: tokenConfigPDA,
            chainLimit: chainLimitPDA,
            bridgeConfig: bridgeConfigPDA,
            bridge: benfenBridgePDA,
            verifier: messageVerifierPDA,
            tokenMint: usdMint,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
            } as any)
            .rpc();
            expect.fail("Expected crossTokenToBridge to fail for single transfer limit");
            
        }catch (error){
            console.log(error)
            expect(error.message).to.include("SingleTransferAmountExceedsLimit");
        }
           
       
    })

    it("cross in usdc",async ()=>{
        let tokenId=TOKEN_IDS.USDC;
        let tokenInfo= getTokenByTokenId(tokenId)
        let amount=1000000;

        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        const deployToken: USDCDeploymentResult = {
            mintAddress: new anchor.web3.PublicKey(tokenInfo.mintAddress),
            tokenAccountAddress: new anchor.web3.PublicKey(tokenInfo.tokenAccountAddress)
        };
        

        const usdMint = deployToken.mintAddress;

        const tokenIdBytes=tokenIdToBytes(tokenId);

        const  benfenAddressBytes=getDefaultBenfenAddressBytes();

        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );

        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        //verifier
        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );

        //message_config
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );

         const messageInfo = await program.provider.connection.getAccountInfo(messageConfigPDA,{ commitment: "confirmed" });

         let messageBeforeNonce=new BN(0);
         if (messageInfo!=null){
            let messageConfig=await program.account.messageConfig.fetch(messageConfigPDA);
            if (messageConfig!=null){
                messageBeforeNonce=messageConfig.nonce;
            }
         }


        //token_config
        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );

        //chain_limit
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        //bridge
        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
            program.programId
        );

        //token valut 已经在add token 创建
        const [tokenValutPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        expect(tokenValutPDA).to.be.not.null;


        const mintInfo = await program.provider.connection.getAccountInfo(deployToken.mintAddress);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            usdMint,
            program.provider.wallet.publicKey
        );

        //查询账户里有多少usdc
        let beforeAccountValut = await getAccount(
            program.provider.connection,
            userTokenAccount
        );

        expect(Number(beforeAccountValut.amount)).to.be.greaterThanOrEqual(amount);

        await new Promise(resolve => setTimeout(resolve, 1000));

        let beforeTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(beforeTokenValut.amount)

        expect(beforeTokenValut.amount).to.be.equal(BigInt(0));

    
        // 检查关联token账户是否存在
        const tokenAccountInfo = await program.provider.connection.getAccountInfo(userTokenAccount);
        expect(tokenAccountInfo).to.be.not.null;

        let crossInTx=await program.methods.crossTokenToBridge(
           // new anchor.BN(tokenId),
            new anchor.BN(amount),
            benfenAddressBytes,
        )
        .accounts({
          payer: program.provider.wallet.publicKey,
          tokenAccount: userTokenAccount,
          tokenVault: tokenValutPDA,
          messageConfig: messageConfigPDA,
          tokenConfig: tokenConfigPDA,
          chainLimit: chainLimitPDA,
          bridgeConfig: bridgeConfigPDA,
          bridge: benfenBridgePDA,
          verifier: messageVerifierPDA,
          tokenMint: usdMint,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

        await program.provider.connection.confirmTransaction(crossInTx);

        await new Promise(resolve => setTimeout(resolve, 1000));

        // 检查关联tokenValut账户的资金
        const tokenValutAccountInfo = await program.provider.connection.getAccountInfo(tokenValutPDA);
        expect(tokenValutAccountInfo).to.be.not.null;
    
        const afterTokenValut = await getAccount(
            program.provider.connection,
            tokenValutPDA
        );
        console.log(afterTokenValut.amount)

        expect(afterTokenValut.amount).to.be.equal(BigInt(amount));

        let afterMessageConfig=await program.account.messageConfig.fetch(messageConfigPDA);
        console.log(afterMessageConfig.nonce.toNumber())
        expect(afterMessageConfig.nonce.toNumber()).to.be.equal(messageBeforeNonce.add(new BN(1)).toNumber());
    })

})