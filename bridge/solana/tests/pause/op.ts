import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, DEFAULTS, TOKEN_IDS, MESSAGE_TYPES, TokenAddrInMainnet } from "../../utils/const";
import { expect } from "chai";
import { createMessage,createAddSlpTokenPayload, computeMessageHash,createUpdateLimitPayload,encodeEmergencyOpPayload,createTokenTransferPayload ,encodeTokenTransferPayload} from "../../utils/message";
import { committeeInfo } from "../../utils/committee";
import { getSignature } from "../../utils/signatures";
import { deployUSDC, USDCDeploymentResult} from "../../utils/tokens/deploy_usdc";
import { Connection } from "@solana/web3.js";
import { isToken } from "typescript";
import { benfenAddressToBytes, getDefaultBenfenAddressBytes,tokenIdToBytes,nonceToBytes } from "../../utils/util";
import { getAssociatedTokenAddress, createAssociatedTokenAccountInstruction,getAccount,NATIVE_MINT } from '@solana/spl-token';
import { getTokenByTokenId } from "../../utils/tokens/token-manager";


describe("BenfenBridge - Pause", () => {
    const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

    it("update op ",async ()=>{
        let message_type=MESSAGE_TYPES.EMERGENCY_OP;
        let nonce = 0; 
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let targetChainId=CHAIN_IDS.BENFEN_TESTNET;
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
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([MESSAGE_TYPES.EMERGENCY_OP]),messageVerifierPDA.toBuffer()],
            program.programId
        );

        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
            program.programId
        );

        let payload=encodeEmergencyOpPayload(
            true
        );

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
        await program.methods.updateEmergencyOp(
            message_type,
            version,
            new BN(nonce),
            chain_id,
            Buffer.from(payload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            messageConfig: messageConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
            bridge: benfenBridgePDA,
        }as any).rpc();

        let freezeAccount=await program.account.benfenBridge.fetch(benfenBridgePDA);
        expect(freezeAccount.isPaused).to.be.true;
    })   
    
    
    it("pause after not cross in",async ()=>{
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


        


        // 检查关联token账户是否存在
        const tokenAccountInfo = await program.provider.connection.getAccountInfo(userTokenAccount);
        expect(tokenAccountInfo).to.be.not.null;



        try {

             let crossInTx=await program.methods.crossTokenToBridge(
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
            expect.fail("Bridge is currently paused")
        }catch(error) {
            expect(error.message).to.contain("Bridge is currently paused")
            console.log(error)
        }
    })


    it("pause after not cross out",async ()=>{
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


        try {
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
        expect.fail("Bridge is currently paused")

        }catch(error) {
            expect(error.message).to.contain("Bridge is currently paused")
            console.log(error)
        }

    })

    it("unpasue ",async ()=>{

        let message_type=MESSAGE_TYPES.EMERGENCY_OP;
        let nonce = 1; 
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let targetChainId=CHAIN_IDS.BENFEN_TESTNET;
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
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([MESSAGE_TYPES.EMERGENCY_OP]),messageVerifierPDA.toBuffer()],
            program.programId
        );

        const [benfenBridgePDA]=anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BENFEN_BRIDGE),committeePDA.toBuffer()],
            program.programId
        );

        let payload=encodeEmergencyOpPayload(
            false
        );

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
        await program.methods.updateEmergencyOp(
            message_type,
            version,
            new BN(nonce),
            chain_id,
            Buffer.from(payload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            messageConfig: messageConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
            bridge: benfenBridgePDA,
        }as any).rpc();

        let freezeAccount=await program.account.benfenBridge.fetch(benfenBridgePDA);
        expect(freezeAccount.isPaused).to.be.false;
    })

    // it("unpause cross out",async ()=>{

    // })

    // it("unpause cross in",async ()=>{

    // })

})