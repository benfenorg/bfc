import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, MESSAGE_TYPES,TOKEN_IDS } from "../../utils/const";
import { expect } from "chai";
import { createMessage,createTokenTransferPayload ,encodeTokenTransferPayload,computeMessageHash} from "../../utils/message";
import { committeeInfo } from "../../utils/committee";
import { getSignature } from "../../utils/signatures";
import { deployUSDC,mintTo, USDCDeploymentResult} from "../../utils/tokens/deploy_usdc";
import { getAssociatedTokenAddress, createAssociatedTokenAccountInstruction,getAccount,NATIVE_MINT } from '@solana/spl-token';
import { getTokenByTokenId } from "../../utils/tokens/token-manager";
import { nonceToBytes } from "../../utils/util";


import {

  LAMPORTS_PER_SOL,

} from "@solana/web3.js";
import { unwrapSol, wrapSol } from "../../utils/tokens/wsol";
import { tokenIdToBytes } from "../../utils/util";
import { convertBenfenToSlpDecimal } from "../../utils/convert";


describe("BenfenBridge - Cross Chain", () => {
    const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

    // it("测试领取usdt 不合法的target chain",async ()=>{
    //     let tokenId=TOKEN_IDS.USDT;
    //     let nonce=100; 
    //     let amount=5000000;
    //     const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
    //     let version=1;
    //     let chain_id=CHAIN_IDS.BENFEN_TESTNET;
    //     let tokenInfo= getTokenByTokenId(tokenId)
    //     if (!tokenInfo) {
    //         throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
    //     }
    //     let solMint=new anchor.web3.PublicKey(tokenInfo.mintAddress);
    //     let tokenIdBytes=tokenIdToBytes(tokenId);
    //     let encodeNonceBytes = nonceToBytes(nonce);

    //     const benfenAddress='0xaea8ea4ce7c82b9f32835f5cee1057a19dc673cf71b313db8f2bf01f1cc7a91e'
    //     const benfenAddressBytes = Buffer.from(benfenAddress.slice(2), 'hex');

    //     //processed_transfer
    //     const [processedTransferPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.PROCESSED_TRANSFER),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),encodeNonceBytes],
    //         program.programId
    //     );
    //     const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.BRIDGE_CONFIG)],
    //         program.programId
    //     );
    //     const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
    //         program.programId
    //     );
    //     //verifier
    //     const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
    //         program.programId
    //     );
    //     //message_config
    //     const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
    //         program.programId
    //     );
    //     //token_config
    //       const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
    //         program.programId
    //     );

    //     //chain_limit
    //     const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),bridgeConfigPDA.toBuffer()],
    //         program.programId
    //     );
    //     //bridge
    //     const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
    //         program.programId
    //     );

    //     //token valut 已经在add token 创建
    //     const [tokenValutPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [Buffer.from(SEEDS.VAULT),tokenIdBytes],
    //         program.programId
    //     );

    //     expect(tokenValutPDA).to.be.not.null;


    //     await new Promise(resolve => setTimeout(resolve, 1000));



    //     const mintInfo = await program.provider.connection.getAccountInfo(solMint);
    //     console.log("Mint owner program ID:", mintInfo?.owner.toBase58());
    //     console.log("program.provider.wallet.publicKey:", program.provider.wallet.publicKey.toBase58());

    //     //token_account 还没有创建创建
    //     const userTokenAccount = await getAssociatedTokenAddress(
    //         solMint,
    //         program.provider.wallet.publicKey
    //     );

    //     let tokenAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
    //     console.log("userTokenAccount:",tokenAccountInfo.owner.toBase58());

 

    //     let tokenTransferPayload=createTokenTransferPayload(
    //         benfenAddressBytes,
    //         CHAIN_IDS.SOLANA_TESTNET,
    //         program.provider.wallet.publicKey,
    //         BigInt(tokenId),
    //         BigInt(amount),
    //         Buffer.from('0x1234567890123456789012345678901234567890123456789012345678901234'),
    //         0
    //     )
    //     let encodedTokenTransferPayload=encodeTokenTransferPayload(tokenTransferPayload);

    //     let message=createMessage(
    //         message_type,
    //         version,
    //         BigInt(nonce),
    //         CHAIN_IDS.BENFEN_MAINNET, //mock
    //         encodedTokenTransferPayload
    //     );

    //     let messageHash=computeMessageHash(message);


    //     let signatures=[];
        
    //     for (let i = 0; i < 3; i++) {
    //         const element = committeeInfo[i];
    //         let signature=getSignature(messageHash,element.privateKey);
    //         signatures.push(signature);
    //     }
    //     expect(signatures.length).to.be.equal(3);

    //     let bridgeConfigData=await program.account.bridgeConfig.fetch(bridgeConfigPDA);
    //     console.log(bridgeConfigData.supportedChains)


    //     try {
    //         await program.methods.crossOut(
    //             new anchor.BN(nonce),
    //             message_type,
    //             version,
    //             CHAIN_IDS.BENFEN_MAINNET, //mock
    //             Buffer.from(encodedTokenTransferPayload),
    //             signatures.map(sig => Buffer.from(sig))
    //         ).accounts({
    //             signer: program.provider.wallet.publicKey,
    //             bridge: benfenBridgePDA,
    //             processTransfer: processedTransferPDA,
    //             verifier: messageVerifierPDA,
    //             committee: committeePDA,
    //             messageConfig: messageConfigPDA,
    //             tokenConfig: tokenConfigPDA,
    //             bridgeConfig: bridgeConfigPDA,
    //             chainLimit: chainLimitPDA,
    //             tokenMint: solMint,
    //             tokenVault: tokenValutPDA,
    //             tokenAccount: userTokenAccount,
    //             tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    //             systemProgram: anchor.web3.SystemProgram.programId,
    //         } as any).rpc();
    //         expect.fail("Expected crossOut to fail for Unsupported cross to chain id");    
    //     }catch (error){
    //         console.log(error)
    //         expect(error.message).to.contain("Unsupported cross to chain id");
    //     }
    // })




    it("测试领取usdt sigature stake 不足",async ()=>{
        let tokenId=TOKEN_IDS.USDT;
        let nonce=100; 
        let amount=5000000+1;
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        let version=1;
        let chain_id=CHAIN_IDS.BENFEN_TESTNET;
        let tokenInfo= getTokenByTokenId(tokenId)
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        let solMint=new anchor.web3.PublicKey(tokenInfo.mintAddress);
        let tokenIdBytes=tokenIdToBytes(tokenId);
        let encodeNonceBytes = nonceToBytes(nonce);

        const benfenAddress='0xaea8ea4ce7c82b9f32835f5cee1057a19dc673cf71b313db8f2bf01f1cc7a91e'
        const benfenAddressBytes = Buffer.from(benfenAddress.slice(2), 'hex');

        //processed_transfer
        const [processedTransferPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.PROCESSED_TRANSFER), Buffer.from([message_type]),Buffer.from([CHAIN_IDS.BENFEN_TESTNET]),encodeNonceBytes],
            program.programId
        );
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


        await mintTo(
            program.provider.connection,
            program.provider.wallet.payer,
            solMint,
            tokenValutPDA,
            amount,
        );

        await new Promise(resolve => setTimeout(resolve, 1000));



        const mintInfo = await program.provider.connection.getAccountInfo(solMint);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());
        console.log("program.provider.wallet.publicKey:", program.provider.wallet.publicKey.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            solMint,
            program.provider.wallet.publicKey
        );

        let tokenAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
        console.log("userTokenAccount:",tokenAccountInfo.owner.toBase58());

 

        let tokenTransferPayload=createTokenTransferPayload(
            benfenAddressBytes,
            CHAIN_IDS.SOLANA_TESTNET,
            program.provider.wallet.publicKey,
            BigInt(tokenId),
            BigInt(amount),
            Buffer.from('0x1234567890123456789012345678901234567890123456789012345678901234'),
            0
        )
        let encodedTokenTransferPayload=encodeTokenTransferPayload(tokenTransferPayload);

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            encodedTokenTransferPayload
        );

        let messageHash=computeMessageHash(message);


        let signatures=[];
        
        for (let i = 0; i < 1; i++) {
            const element = committeeInfo[i];
            let signature=getSignature(messageHash,element.privateKey);
            signatures.push(signature);
        }
        expect(signatures.length).to.be.equal(1);


        try {
            await program.methods.crossOut(
                chain_id,
                new anchor.BN(nonce),
                message_type,
                version,
                Buffer.from(encodedTokenTransferPayload),
                signatures.map(sig => Buffer.from(sig))
            ).accounts({
                signer: program.provider.wallet.publicKey,
                bridge: benfenBridgePDA,
                processTransfer: processedTransferPDA,
                verifier: messageVerifierPDA,
                committee: committeePDA,
                messageConfig: messageConfigPDA,
                tokenConfig: tokenConfigPDA,
                bridgeConfig: bridgeConfigPDA,
                chainLimit: chainLimitPDA,
                tokenMint: solMint,
                tokenVault: tokenValutPDA,
                tokenAccount: userTokenAccount,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            } as any).rpc();
            expect.fail("Expected crossOut to fail for Total stake is less than minimum required");   
        }catch (error){
            console.log(error)
            expect(error.message).to.contain("Total stake is less than minimum required");
        }
    })

    it("测试领取usdt 不合法的message type",async ()=>{
        let tokenId=TOKEN_IDS.USDT;
        let nonce=100; 
        let amount=5000000+1;
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        let version=1;
        let chain_id=CHAIN_IDS.BENFEN_TESTNET;
        let tokenInfo= getTokenByTokenId(tokenId)
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        let solMint=new anchor.web3.PublicKey(tokenInfo.mintAddress);
        let tokenIdBytes=tokenIdToBytes(tokenId);
        let encodeNonceBytes = nonceToBytes(nonce);

        const benfenAddress='0xaea8ea4ce7c82b9f32835f5cee1057a19dc673cf71b313db8f2bf01f1cc7a91e'
        const benfenAddressBytes = Buffer.from(benfenAddress.slice(2), 'hex');

        //processed_transfer
        const [processedTransferPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.PROCESSED_TRANSFER), Buffer.from([message_type]),Buffer.from([chain_id]), encodeNonceBytes],
            program.programId
        );
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


        await mintTo(
            program.provider.connection,
            program.provider.wallet.payer,
            solMint,
            tokenValutPDA,
            amount,
        );

        await new Promise(resolve => setTimeout(resolve, 1000));



        const mintInfo = await program.provider.connection.getAccountInfo(solMint);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());
        console.log("program.provider.wallet.publicKey:", program.provider.wallet.publicKey.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            solMint,
            program.provider.wallet.publicKey
        );

        let tokenAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
        console.log("userTokenAccount:",tokenAccountInfo.owner.toBase58());

 

        let tokenTransferPayload=createTokenTransferPayload(
            benfenAddressBytes,
            CHAIN_IDS.SOLANA_TESTNET,
            program.provider.wallet.publicKey,
            BigInt(tokenId),
            BigInt(amount),
            Buffer.from('0x1234567890123456789012345678901234567890123456789012345678901234'),
            0
        )
        let encodedTokenTransferPayload=encodeTokenTransferPayload(tokenTransferPayload);

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            encodedTokenTransferPayload
        );

        let messageHash=computeMessageHash(message);


        let signatures=[];
        
        for (let i = 0; i < 3; i++) {
            const element = committeeInfo[i];
            let signature=getSignature(messageHash,element.privateKey);
            signatures.push(signature);
        }
        expect(signatures.length).to.be.equal(3);


        try {
            await program.methods.crossOut(
                chain_id,
                new anchor.BN(nonce),
                MESSAGE_TYPES.ADD_SVM_TOKEN,//mock
                version,
                Buffer.from(encodedTokenTransferPayload),
                signatures.map(sig => Buffer.from(sig))
            ).accounts({
                signer: program.provider.wallet.publicKey,
                bridge: benfenBridgePDA,
                processTransfer: processedTransferPDA,
                verifier: messageVerifierPDA,
                committee: committeePDA,
                messageConfig: messageConfigPDA,
                tokenConfig: tokenConfigPDA,
                bridgeConfig: bridgeConfigPDA,
                chainLimit: chainLimitPDA,
                tokenMint: solMint,
                tokenVault: tokenValutPDA,
                tokenAccount: userTokenAccount,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            } as any).rpc();
            expect.fail("Expected crossOut to fail for Invalid message type");   
        }catch (error){
            console.log(error)
            expect(error.message).to.contain("Invalid message type");
        }
     })

 

    it("从benfen 跨链sol 到 solana",async()=>{
        let tokenId=TOKEN_IDS.SOL;
        let nonce=2;
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        let version=1;
        let chain_id=CHAIN_IDS.BENFEN_TESTNET;
        let tokenInfo= getTokenByTokenId(tokenId)
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        let solMint=new anchor.web3.PublicKey(tokenInfo.mintAddress);
        let tokenIdBytes=tokenIdToBytes(tokenId);
        let encodeNonceBytes = nonceToBytes(nonce);

        const benfenAddress='0xaea8ea4ce7c82b9f32835f5cee1057a19dc673cf71b313db8f2bf01f1cc7a91e'
        const benfenAddressBytes = Buffer.from(benfenAddress.slice(2), 'hex');


        //processed_transfer
        const [processedTransferPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.PROCESSED_TRANSFER), Buffer.from([message_type]),Buffer.from([chain_id]),encodeNonceBytes],
            program.programId
        );

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


        const mintInfo = await program.provider.connection.getAccountInfo(solMint);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());
        console.log("program.provider.wallet.publicKey:", program.provider.wallet.publicKey.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            solMint,
            program.provider.wallet.publicKey
        );

        let tokenAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
        console.log("userTokenAccount:",tokenAccountInfo.owner.toBase58());

 

        let tokenTransferPayload=createTokenTransferPayload(
            benfenAddressBytes,
            CHAIN_IDS.SOLANA_TESTNET,
            program.provider.wallet.publicKey,
            BigInt(tokenId),
            BigInt(100000000),
            Buffer.from('0x1234567890123456789012345678901234567890123456789012345678901234'),
            0
        )
        let encodedTokenTransferPayload=encodeTokenTransferPayload(tokenTransferPayload);

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            encodedTokenTransferPayload
        );

        let messageHash=computeMessageHash(message);


        let signatures=[];
        
        for (let i = 0; i < 3; i++) {
            const element = committeeInfo[i];
            let signature=getSignature(messageHash,element.privateKey);
            signatures.push(signature);
        }
        expect(signatures.length).to.be.equal(3);

         let beforeAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
        console.log("accountAccountInfo:", beforeAccountInfo);
        let beforeSolBalance=await program.provider.connection.getBalance(program.provider.wallet.publicKey);
        console.log("beforeSolBalance:", beforeSolBalance);
        // expect(accountAccountInfo.amount).to.be.equal(100000000);
        let tx=await program.methods.crossOut(
            chain_id,
            new BN(nonce),
            message_type,
            version,
            Buffer.from(encodedTokenTransferPayload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            payer: program.provider.wallet.publicKey,
            bridge: benfenBridgePDA,
            processTransfer: processedTransferPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
            messageConfig: messageConfigPDA,
            tokenConfig: tokenConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            chainLimit: chainLimitPDA,
            tokenMint: solMint,
            tokenVault: tokenValutPDA,
            tokenAccount: userTokenAccount,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        } as any).rpc();
    

        await program.provider.connection.confirmTransaction(tx);

        await new Promise(resolve => setTimeout(resolve, 1000));

        let processedTransferInfo = await program.provider.connection.getAccountInfo(processedTransferPDA);
        expect(processedTransferInfo).to.be.not.null;

        let processedTransfer=await program.account.processTransfer.fetch(processedTransferPDA);
        console.log("processedTransfer:", processedTransfer);
        expect(processedTransfer.isProcessed).to.be.equal(true);

        let accountAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
        console.log("accountAccountInfo:", accountAccountInfo);
        console.log("beforeAccountInfo:", beforeAccountInfo);
        expect(accountAccountInfo.amount).to.be.equal(beforeAccountInfo.amount+BigInt(100000000));

        await unwrapSol(program.provider.connection,program.provider.wallet.payer,BigInt(100000000));

        let afterSolBalance=await program.provider.connection.getBalance(program.provider.wallet.publicKey);
        console.log("afterSolBalance:", afterSolBalance);
       // expect(afterSolBalance).to.be.equal(beforeSolBalance+100000000);
       // let afteraccountAccount=await getAccount(program.provider.connection,userTokenAccount);
        // console.log("afteraccountAccount:", afteraccountAccount);
        expect(afterSolBalance >= beforeAccountInfo.amount).to.be.true;

        // expect(accountAccountInfo.owner.toBase58()).to.be.equal(benfenBridgePDA.toBase58());


    })

    it("从solana 跨链usdc 到 benfen",async()=>{
        let tokenId=TOKEN_IDS.USDT;
        let nonce=1; //nonce 0 already used
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        let version=1;
        let chain_id=CHAIN_IDS.BENFEN_TESTNET;
        let tokenInfo= getTokenByTokenId(tokenId)
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        let solMint=new anchor.web3.PublicKey(tokenInfo.mintAddress);
        let tokenIdBytes=tokenIdToBytes(tokenId);
        let encodeNonceBytes = nonceToBytes(nonce);

        const benfenAddress='0xaea8ea4ce7c82b9f32835f5cee1057a19dc673cf71b313db8f2bf01f1cc7a91e'
        const benfenAddressBytes = Buffer.from(benfenAddress.slice(2), 'hex');

        //processed_transfer
        const [processedTransferPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.PROCESSED_TRANSFER),Buffer.from([message_type]),Buffer.from([chain_id]),encodeNonceBytes],
            program.programId
        );
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

        const mintInfo = await program.provider.connection.getAccountInfo(solMint);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());
        console.log("program.provider.wallet.publicKey:", program.provider.wallet.publicKey.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            solMint,
            program.provider.wallet.publicKey
        );

        let tokenAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
        console.log("userTokenAccount:",tokenAccountInfo.owner.toBase58());







 

        let tokenTransferPayload=createTokenTransferPayload(
            benfenAddressBytes,
            CHAIN_IDS.SOLANA_TESTNET,
            program.provider.wallet.publicKey,
            BigInt(tokenId),
            BigInt(100000000),
            Buffer.from('0x1234567890123456789012345678901234567890123456789012345678901234'),
            0
        )
        let encodedTokenTransferPayload=encodeTokenTransferPayload(tokenTransferPayload);

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            encodedTokenTransferPayload
        );

        let messageHash=computeMessageHash(message);


        let signatures=[];
        
        for (let i = 0; i < 3; i++) {
            const element = committeeInfo[i];
            let signature=getSignature(messageHash,element.privateKey);
            signatures.push(signature);
        }
        expect(signatures.length).to.be.equal(3);

         let beforeAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
        console.log("accountAccountInfo:", beforeAccountInfo);
         let tx=await program.methods.crossOut(
            chain_id,
            new anchor.BN(nonce),
            message_type,
            version,  
            Buffer.from(encodedTokenTransferPayload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            signer: program.provider.wallet.publicKey,
            bridge: benfenBridgePDA,
            processTransfer: processedTransferPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
            messageConfig: messageConfigPDA,
            tokenConfig: tokenConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            chainLimit: chainLimitPDA,
            tokenMint: solMint,
            tokenVault: tokenValutPDA,
            tokenAccount: userTokenAccount,
            tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        } as any).rpc();
    
        await program.provider.connection.confirmTransaction(tx);

        await new Promise(resolve => setTimeout(resolve, 1000));
        let receiver_amount=convertBenfenToSlpDecimal(9,6,BigInt(100000000));

        let processedTransferInfo = await program.provider.connection.getAccountInfo(processedTransferPDA);
        expect(processedTransferInfo).to.be.not.null;

        let processedTransfer=await program.account.processTransfer.fetch(processedTransferPDA);
        console.log("processedTransfer:", processedTransfer);
        expect(processedTransfer.isProcessed).to.be.equal(true);

        let accountAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
        console.log("accountAccountInfo:", accountAccountInfo);
        console.log("beforeAccountInfo:", beforeAccountInfo);
        expect(accountAccountInfo.amount).to.be.equal(beforeAccountInfo.amount+receiver_amount);       
    })
    //testTransferBridgedTokensWithSignaturesTokenDailyLimitExceeded
    it("测试领取usdt 超过24小时限额",async ()=>{
        let tokenId=TOKEN_IDS.USDT;
        let nonce=100; 
        let amount=500*1000000000+1;
        const message_type = MESSAGE_TYPES.TOKEN_TRANSFER;
        let version=1;
        let chain_id=CHAIN_IDS.BENFEN_TESTNET;
        let tokenInfo= getTokenByTokenId(tokenId)
        if (!tokenInfo) {
            throw new Error(`Token with ID ${tokenId} not found in deployTokens.json`);
        }
        let solMint=new anchor.web3.PublicKey(tokenInfo.mintAddress);
        let tokenIdBytes=tokenIdToBytes(tokenId);
        let encodeNonceBytes = nonceToBytes(nonce);

        const benfenAddress='0xaea8ea4ce7c82b9f32835f5cee1057a19dc673cf71b313db8f2bf01f1cc7a91e'
        const benfenAddressBytes = Buffer.from(benfenAddress.slice(2), 'hex');

        //processed_transfer
        const [processedTransferPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.PROCESSED_TRANSFER), Buffer.from([message_type]),Buffer.from([chain_id]),encodeNonceBytes],
            program.programId
        );
        console.log("processedTransferPDA:", processedTransferPDA);
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


        await mintTo(
            program.provider.connection,
            program.provider.wallet.payer,
            solMint,
            tokenValutPDA,
            amount,
        );

        await new Promise(resolve => setTimeout(resolve, 1000));



        const mintInfo = await program.provider.connection.getAccountInfo(solMint);
        console.log("Mint owner program ID:", mintInfo?.owner.toBase58());
        console.log("program.provider.wallet.publicKey:", program.provider.wallet.publicKey.toBase58());

        //token_account 还没有创建创建
        const userTokenAccount = await getAssociatedTokenAddress(
            solMint,
            program.provider.wallet.publicKey
        );

        let tokenAccountInfo=await getAccount(program.provider.connection,userTokenAccount);
        console.log("userTokenAccount:",tokenAccountInfo.owner.toBase58());

 

        let tokenTransferPayload=createTokenTransferPayload(
            benfenAddressBytes,
            CHAIN_IDS.SOLANA_TESTNET,
            program.provider.wallet.publicKey,
            BigInt(tokenId),
            BigInt(amount),
            Buffer.from('0x1234567890123456789012345678901234567890123456789012345678901234'),
            0
        )
        let encodedTokenTransferPayload=encodeTokenTransferPayload(tokenTransferPayload);

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            encodedTokenTransferPayload
        );

        let messageHash=computeMessageHash(message);


        let signatures=[];
        
        for (let i = 0; i < 3; i++) {
            const element = committeeInfo[i];
            let signature=getSignature(messageHash,element.privateKey);
            signatures.push(signature);
        }
        expect(signatures.length).to.be.equal(3);


        try {
            await program.methods.crossOut(
                chain_id,
                new anchor.BN(nonce),
                message_type,
                version,
                Buffer.from(encodedTokenTransferPayload),
                signatures.map(sig => Buffer.from(sig))
            ).accounts({
                signer: program.provider.wallet.publicKey,
                bridge: benfenBridgePDA,
                processTransfer: processedTransferPDA,
                verifier: messageVerifierPDA,
                committee: committeePDA,
                messageConfig: messageConfigPDA,
                tokenConfig: tokenConfigPDA,
                bridgeConfig: bridgeConfigPDA,
                chainLimit: chainLimitPDA,
                tokenMint: solMint,
                tokenVault: tokenValutPDA,
                tokenAccount: userTokenAccount,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
            } as any).rpc();
            expect.fail("Expected crossOut to fail for execeed chain limit");   
        }catch (error){
            console.log(error)
            expect(error.message).to.contain("Cross out amount exceeds limit");
        }
    })

})
