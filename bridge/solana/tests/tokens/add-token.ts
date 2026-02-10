import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, DEFAULTS, TOKEN_IDS, MESSAGE_TYPES, TokenAddrInMainnet } from "../../utils/const";
import { expect } from "chai";
import { createMessage,createAddSlpTokenPayload, computeMessageHash } from "../../utils/message";
import { committeeInfo } from "../../utils/committee";
import { getSignature } from "../../utils/signatures";
import { deployUSDC, USDCDeploymentResult} from "../../utils/tokens/deploy_usdc";
import fs from 'fs';
import { saveTokenToJsonArray,resetTokensFile } from "../../utils/tokens/token-manager";

import {
  NATIVE_MINT,
} from "@solana/spl-token";

describe("BenfenBridge - Add Token", () => {
    const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

    let deployToken : USDCDeploymentResult;

    before(()=>{
        resetTokensFile();
    })

    beforeEach(async()=>{
        // const connection = new Connection("http://localhost:8899", "confirmed");
        // const payer = program.provider.wallet.publicKey
        deployToken=await deployUSDC(program.provider.connection,program.provider.wallet.payer,1_000_000,6);
    })


     it("add token  committee stake is 5000",async()=>{
        let tokenId=TOKEN_IDS.USDT;
        const encodeTokenId = BigInt(tokenId); // Replace with actual token ID
        // 把 token_id 转换成 big-endian Uint8Array (8 字节)
        const tokenIdBytes = Buffer.alloc(8);
        tokenIdBytes.writeBigUInt64BE(encodeTokenId); 
        let benfenDecimal=9;
        const usdMint = deployToken.mintAddress;
        let version=1;
        let nonce=0;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;


         const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );
        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        expect(chainLimitPDA).to.be.not.null;

        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );
        
        //token_config
        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );
        //message_config
        const message_type = MESSAGE_TYPES.ADD_SVM_TOKEN;
        console.log(message_type)
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
      

        const [vaultPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
        program.programId
        );
      

        let addTokenMessage= createAddSlpTokenPayload(
            false,
            BigInt(tokenId),
            usdMint,
            benfenDecimal,
            BigInt(100000000)
        );

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            addTokenMessage
        );

        let messageHash=computeMessageHash(message);
        let signatures=[];
        // committeeInfo 2 stake is 5000
        for (let i = 0; i < 2; i++) {
            const element = committeeInfo[i];
            let signature=getSignature(messageHash,element.privateKey);
            signatures.push(signature);
        }
        expect(signatures.length).to.be.equal(2);
        //token_mint

         try {
            await program.methods.addTokenToBridge(
                new BN(tokenId),
                message_type,
                version,
                new BN(nonce),
                chain_id,
                Buffer.from(addTokenMessage),
                signatures.map(sig => Buffer.from(sig))
            )
            .accounts({
                payer: program.provider.wallet.publicKey,
                messageConfig: messageConfigPDA,
                verifier: messageVerifierPDA,
                tokenVault: vaultPDA,
                tokenConfig:tokenConfigPDA,
                chainLimit: chainLimitPDA,
                bridgeConfig:bridgeConfigPDA,
                benfenBridge: benfenBridgePDA,
                tokenMint: usdMint,
                committee: committeePDA,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            }as any)
            .rpc();
            expect.fail("Expected add token to fail for stake 5000");
         } catch (error) {
            console.log(error.message)
            expect(error.message).to.include("Total stake is less than minimum required");
            console.log (error)
         }
    })

    it("add token is verify",async ()=>{
        let tokenId=TOKEN_IDS.USDT;
        const encodeTokenId = BigInt(tokenId); // Replace with actual token ID

        // 把 token_id 转换成 big-endian Uint8Array (8 字节)
        const tokenIdBytes = Buffer.alloc(8);
        tokenIdBytes.writeBigUInt64BE(encodeTokenId); 
        let benfenDecimal=9;
        const usdMint = deployToken.mintAddress;
        let version=1;
        let nonce=0;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;

         const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );
        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        expect(chainLimitPDA).to.be.not.null;

        const limitInfo = await program.account.chainLimit.fetch(chainLimitPDA);
        console.log(limitInfo)

        
        // expect(limitInfo).to.be.not.null;

        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );
        
        //token_config
        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );
        //message_config
        const message_type = MESSAGE_TYPES.ADD_SVM_TOKEN;
        console.log(message_type)
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
      

        const [vaultPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
        program.programId
        );
        // const usdtMint = deployToken.mintAddress;

        let addTokenMessage= createAddSlpTokenPayload(
            false,
            BigInt(tokenId),
            usdMint,
            benfenDecimal,
            BigInt(100000000)
        );

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            addTokenMessage
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
        //token_mint

        let tx=await program.methods.addTokenToBridge(
                new BN(tokenId),
                message_type,
                version,
                new BN(nonce),
                chain_id,
                Buffer.from(addTokenMessage),
                signatures.map(sig => Buffer.from(sig))
            )
            .accounts({
                payer: program.provider.wallet.publicKey,
                messageConfig: messageConfigPDA,
                verifier: messageVerifierPDA,
                tokenVault: vaultPDA,
                tokenConfig:tokenConfigPDA,
                chainLimit: chainLimitPDA,
                bridgeConfig:bridgeConfigPDA,
                benfenBridge: benfenBridgePDA,
                tokenMint: usdMint,
                committee: committeePDA,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            }as any)
            .rpc();
        //将usdMint 写到文件里

        saveTokenToJsonArray({
                tokenType: "USDT",
                tokenId: tokenId,
                mintAddress: deployToken.mintAddress.toString(),
                tokenAccountAddress: deployToken.tokenAccountAddress.toString(),
        });

        // fs.writeFileSync('./deployToken.json', JSON.stringify("USDT:"+deployToken));

        await program.provider.connection.confirmTransaction(tx);
        const tokenInfo = await program.account.tokenConfigAccount.fetch(tokenConfigPDA);
        expect(tokenInfo.tokenId.toNumber()).to.be.equal(tokenId);

        const messageConfig = await program.account.messageConfig.fetch(messageConfigPDA);
        expect(messageConfig.messageType).to.be.equal(message_type);
        expect(messageConfig.nonce.toNumber()).to.be.equal(nonce+1);

        const bridgeConfig = await program.account.bridgeConfig.fetch(bridgeConfigPDA);
        expect(bridgeConfig.tokenCount.toNumber()).to.be.equal(1);
    })


    it("should fail when trying to add token already existing token ",async ()=>{
         let tokenId=TOKEN_IDS.USDT;
        const encodeTokenId = BigInt(tokenId); // Replace with actual token ID

        // 把 token_id 转换成 big-endian Uint8Array (8 字节)
        const tokenIdBytes = Buffer.alloc(8);
        tokenIdBytes.writeBigUInt64BE(encodeTokenId); 

         const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );
        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        expect(chainLimitPDA).to.be.not.null;

        const limitInfo = await program.account.chainLimit.fetch(chainLimitPDA);
        console.log(limitInfo)

        
        // expect(limitInfo).to.be.not.null;

        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );
        
        //token_config
        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );
        //message_config
        const message_type = MESSAGE_TYPES.ADD_SVM_TOKEN;
        console.log(message_type)
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
      

        const [vaultPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
        program.programId
        );
        const usdtMint = deployToken.mintAddress;

        let addTokenMessage= createAddSlpTokenPayload(
            false,
            BigInt(tokenId),
            usdtMint,
            6,
            BigInt(100000000)
        );
        let version=1;
        let nonce=0;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;


        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            addTokenMessage
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
        //token_mint

        try {

            await program.methods.addTokenToBridge(
                new BN(tokenId),
                message_type,
                version,
                new BN(nonce),
                chain_id,
                Buffer.from(addTokenMessage),
                signatures.map(sig => Buffer.from(sig))
            )
            .accounts({
                payer: program.provider.wallet.publicKey,
                messageConfig: messageConfigPDA,
                verifier: messageVerifierPDA,
                tokenVault: vaultPDA,
                tokenConfig:tokenConfigPDA,
                chainLimit: chainLimitPDA,
                bridgeConfig:bridgeConfigPDA,
                benfenBridge: benfenBridgePDA,
                tokenMint: usdtMint,
                committee: committeePDA,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            }as any)
            .rpc();
             expect.fail("Expected add token to fail for existing account");
        }catch(error) {
             console.log("Expected error caught:", error.message);
             expect(error.message).to.include("already in use");
        }
    })

    it("add other token is success",async ()=>{
        let tokenId=TOKEN_IDS.USDC;
        const encodeTokenId = BigInt(tokenId); // Replace with actual token ID
        let message_type=MESSAGE_TYPES.ADD_SVM_TOKEN;
        let nonce = 1;// already use 0
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let targetChainId=CHAIN_IDS.BENFEN_TESTNET;


        // 把 token_id 转换成 big-endian Uint8Array (8 字节)
        const tokenIdBytes = Buffer.alloc(8);
        tokenIdBytes.writeBigUInt64BE(encodeTokenId); 
        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );
        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([targetChainId]),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        expect(chainLimitPDA).to.be.not.null;

        const limitInfo = await program.account.chainLimit.fetch(chainLimitPDA);
        console.log(limitInfo)

        
        // expect(limitInfo).to.be.not.null;

        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );


        
        //token_config
        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );
      
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );


        const messageConfig = await program.account.messageConfig.fetch(messageConfigPDA);
        expect(messageConfig.messageType).to.be.equal(30);

        expect(messageConfig.nonce.toNumber()).to.be.equal(1);

        console.log("success")


      

        const [vaultPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
        program.programId
        );
        const usdMint = deployToken.mintAddress;

         saveTokenToJsonArray({
                tokenType: "USDC",
                tokenId: tokenId,
                mintAddress: deployToken.mintAddress.toString(),
                tokenAccountAddress: deployToken.tokenAccountAddress.toString(),
        });

        let addTokenMessage= createAddSlpTokenPayload(
            false,
            BigInt(tokenId),
            usdMint,
            6,
            BigInt(100000000)
        );
       

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            addTokenMessage
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


        let tx=await program.methods.addTokenToBridge(
                new BN(tokenId),
                message_type,
                version,
                new BN(nonce),
                chain_id,
                Buffer.from(addTokenMessage),
                signatures.map(sig => Buffer.from(sig))
            )
            .accounts({
                payer: program.provider.wallet.publicKey,
                messageConfig: messageConfigPDA,
                verifier: messageVerifierPDA,
                tokenVault: vaultPDA,
                tokenConfig:tokenConfigPDA,
                chainLimit: chainLimitPDA,
                bridgeConfig:bridgeConfigPDA,
                benfenBridge: benfenBridgePDA,
                tokenMint: usdMint,
                committee: committeePDA,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            }as any)
            .rpc();
        await program.provider.connection.confirmTransaction(tx);

        const tokenInfo = await program.account.tokenConfigAccount.fetch(tokenConfigPDA);
        expect(tokenInfo.tokenId.toNumber()).to.be.equal(tokenId);

        const bridgeConfig = await program.account.bridgeConfig.fetch(bridgeConfigPDA);
        expect(bridgeConfig.tokenCount.toNumber()).to.be.equal(2);

    })

    it("add solana token to bridge",async ()=>{
        let tokenId=TOKEN_IDS.SOL;
        const encodeTokenId = BigInt(tokenId); // Replace with actual token ID
        let message_type=MESSAGE_TYPES.ADD_SVM_TOKEN;
        let nonce = 2;// already use 0,1
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let targetChainId=CHAIN_IDS.BENFEN_TESTNET;
        let price = BigInt(200*100000000);
        let decimal=9;


        // 把 token_id 转换成 big-endian Uint8Array (8 字节)
        const tokenIdBytes = Buffer.alloc(8);
        tokenIdBytes.writeBigUInt64BE(encodeTokenId); 
        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );
        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([targetChainId]),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        expect(chainLimitPDA).to.be.not.null;

        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );        
        //token_config
        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );
      
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );


        const messageConfig = await program.account.messageConfig.fetch(messageConfigPDA);
        expect(messageConfig.messageType).to.be.equal(30);
        expect(messageConfig.nonce.toNumber()).to.be.equal(2);
        const [vaultPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),tokenIdBytes],
            program.programId
        );

        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
        program.programId
        );

        const solMint = NATIVE_MINT;

         saveTokenToJsonArray({
            tokenType: "SOL",
            tokenId: tokenId,
            mintAddress: NATIVE_MINT.toString(),
            tokenAccountAddress: "",
        });

        let addTokenMessage= createAddSlpTokenPayload(
            false,
            BigInt(tokenId),
            solMint,
            decimal,
            price
        );
       

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            addTokenMessage
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


        let tx=await program.methods.addTokenToBridge(
                new BN(tokenId),
                message_type,
                version,
                new BN(nonce),
                chain_id,
                Buffer.from(addTokenMessage),
                signatures.map(sig => Buffer.from(sig))
            )
            .accounts({
                payer: program.provider.wallet.publicKey,
                messageConfig: messageConfigPDA,
                verifier: messageVerifierPDA,
                tokenVault: vaultPDA,
                tokenConfig:tokenConfigPDA,
                chainLimit: chainLimitPDA,
                bridgeConfig:bridgeConfigPDA,
                benfenBridge: benfenBridgePDA,
                tokenMint: solMint,
                committee: committeePDA,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            }as any)
            .rpc();
        await program.provider.connection.confirmTransaction(tx);

        const tokenInfo = await program.account.tokenConfigAccount.fetch(tokenConfigPDA);
        expect(tokenInfo.tokenId.toNumber()).to.be.equal(tokenId);

        const bridgeConfig = await program.account.bridgeConfig.fetch(bridgeConfigPDA);
        expect(bridgeConfig.tokenCount.toNumber()).to.be.equal(3);
    })
    return
    it("add token is verify",async()=>{
      
        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );
        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        const limitInfo = await program.account.chainLimit.fetch(chainLimitPDA);
        console.log(limitInfo)

        
        // expect(limitInfo).to.be.not.null;

        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );
        
        //token_config
        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),Buffer.from([TOKEN_IDS.USDC])],
            program.programId
        );
        //message_config
        const message_type = MESSAGE_TYPES.ADD_SVM_TOKEN;
        console.log(message_type)
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );

        const [vaultPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.VAULT),Buffer.from([TOKEN_IDS.USDC])],
            program.programId
        );


    const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
      program.programId
    );

        const usdcMint = deployToken.mintAddress;

        let addTokenMessage= createAddSlpTokenPayload(
            false,
            BigInt(TOKEN_IDS.USDC),
            deployToken.mintAddress,
            6,
            BigInt(100000000)
        );
        let version=1;
        let nonce=0;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;


        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            addTokenMessage
        );

        let messageHash=computeMessageHash(message);
        let signatures=[];
        for (let i = 0; i < committeeInfo.length; i++) {
            const element = committeeInfo[i];
            let signature=getSignature(messageHash,element.privateKey);
            signatures.push(signature);
        }
        //token_mint

        let tx=await program.methods.addTokenToBridge(
            new BN(TOKEN_IDS.USDC),
            message_type,
            version,
            new BN(nonce),
            chain_id,
            Buffer.from(addTokenMessage),
            signatures.map(sig => Buffer.from(sig))
        )
        .accounts({
            payer: program.provider.wallet.publicKey,
            messageConfig: messageConfigPDA,
            verifier: messageVerifierPDA,
            tokenVault: vaultPDA,
            tokenConfig:tokenConfigPDA,
            chainLimit: chainLimitPDA,
            bridgeConfig:bridgeConfigPDA,
            benfenBridge: benfenBridgePDA,
            tokenMint: usdcMint,
            committee: committeePDA,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        }as any)
            .rpc();

        await program.provider.connection.confirmTransaction(tx);


        const accountInfo = await program.provider.connection.getAccountInfo(tokenConfigPDA,{ commitment: "confirmed" });
        console.log(accountInfo)
        expect(accountInfo).to.be.not.null;
    })


   

})