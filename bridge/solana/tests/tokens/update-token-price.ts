import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, TOKEN_IDS, MESSAGE_TYPES } from "../../utils/const";
import { expect } from "chai";
import { createMessage, computeMessageHash, createUpdateTokenPricePayload } from "../../utils/message";
import { committeeInfo } from "../../utils/committee";
import { getSignature } from "../../utils/signatures";

describe("BenfenBridge - Update Token Price", () => {
    const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;


    it("stake is 5000",async()=>{
        let tokenId=TOKEN_IDS.USDC;
        const encodeTokenId = BigInt(tokenId); // Replace with actual token ID
        let message_type=MESSAGE_TYPES.UPDATE_TOKEN_PRICE;
        let nonce = 0; 
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let targetChainId=CHAIN_IDS.BENFEN_TESTNET;
        let price =200000000;
          // 把 token_id 转换成 big-endian Uint8Array (8 字节)
        const tokenIdBytes = Buffer.alloc(8);
        tokenIdBytes.writeBigUInt64BE(encodeTokenId); 

        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );
        expect(bridgeConfigPDA).to.be.not.null;

        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        expect(committeePDA).to.be.not.null;

        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([targetChainId]),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        expect(chainLimitPDA).to.be.not.null;

        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );

        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );
      
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );

         let updatePricePayload= createUpdateTokenPricePayload(
            BigInt(tokenId),
            BigInt(price),
         );

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            updatePricePayload
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

        try {

            let tx=(await program.methods.updateTokenPrice(
                message_type,
                version,
                new BN(nonce),
                chain_id,
                Buffer.from(updatePricePayload),
                signatures.map(sig => Buffer.from(sig))
            ).accounts({
                messageConfig: messageConfigPDA,
                tokenConfig: tokenConfigPDA,
                bridgeConfig: bridgeConfigPDA,
                verifier: messageVerifierPDA,
                committee: committeePDA,
            }as any).rpc());
            expect.fail("stake is 5000")
        }catch(error) {
            console.log(error.message)
            expect(error.message).to.include("Total stake is less than minimum required");
            console.log (error)
        }        
    })


    it("update token price",async()=>{
        let tokenId=TOKEN_IDS.USDC;
        const encodeTokenId = BigInt(tokenId); // Replace with actual token ID
        let message_type=MESSAGE_TYPES.UPDATE_TOKEN_PRICE;
        let nonce = 0; 
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let targetChainId=CHAIN_IDS.BENFEN_TESTNET;
        let price =200000000;
          // 把 token_id 转换成 big-endian Uint8Array (8 字节)
        const tokenIdBytes = Buffer.alloc(8);
        tokenIdBytes.writeBigUInt64BE(encodeTokenId); 

        const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.BRIDGE_CONFIG)],
            program.programId
        );
        expect(bridgeConfigPDA).to.be.not.null;

        const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
            program.programId
        );
        expect(committeePDA).to.be.not.null;

        const [chainLimitPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.CHAIN_LIMIT),Buffer.from([targetChainId]),bridgeConfigPDA.toBuffer()],
            program.programId
        );

        expect(chainLimitPDA).to.be.not.null;

        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );

        const [tokenConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.TOKEN_CONFIG),tokenIdBytes],
            program.programId
        );
      
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );

         let updatePricePayload= createUpdateTokenPricePayload(
            BigInt(tokenId),
            BigInt(price),
         );

        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            updatePricePayload
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

        let tx=(await program.methods.updateTokenPrice(
            message_type,
            version,
            new BN(nonce),
            chain_id,
            Buffer.from(updatePricePayload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            messageConfig: messageConfigPDA,
            tokenConfig: tokenConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
        }as any).rpc());

        let tokenConfig = await program.account.tokenConfigAccount.fetch(tokenConfigPDA);
        expect(tokenConfig.price.toNumber()).to.be.equal(price);
        expect(tokenConfig.tokenId.toNumber()).to.be.equal(tokenId);

        let messageConfig1 = await program.account.messageConfig.fetch(messageConfigPDA);
        expect(messageConfig1.messageType).to.be.equal(MESSAGE_TYPES.UPDATE_TOKEN_PRICE);
        expect(messageConfig1.nonce.toNumber()).to.be.equal(nonce+1);
    })
})