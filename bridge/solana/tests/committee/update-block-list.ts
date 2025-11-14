import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, MESSAGE_TYPES } from "../../utils/const";
import { expect } from "chai";
import { createMessage, computeMessageHash,createBlocklistPayload } from "../../utils/message";
import { committeeInfo } from "../../utils/committee";
import { getSignature } from "../../utils/signatures";

describe("BenfenBridge - Update Block List", () => {
    const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

    it("update block list with  invalid message type",async ()=>{

        let message_type=MESSAGE_TYPES.ADD_SVM_TOKEN;
        let nonce = 0; 
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let index=committeeInfo.length-1;
        let addr=committeeInfo[index].address;
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
        
      

        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([MESSAGE_TYPES.BLOCKLIST]),messageVerifierPDA.toBuffer()],
            program.programId
        );
      
        let blocklistPayload= createBlocklistPayload(
            [addr],
            true ,
        );


        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            blocklistPayload
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


        try {
            let tx=(await program.methods.updateBlockList(
            message_type,
            version,
            new BN(nonce),
            chain_id,
            Buffer.from(blocklistPayload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            messageConfig: messageConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
        }as any).rpc());
         expect.fail("Expected update block list  with invalid nonce");
        }catch(error){
            console.log(error.message)
            expect(error.message).to.include("Invalid message typ");
        } 
    })

    it("update block list with invalid  message nonce",async ()=>{

        let message_type=MESSAGE_TYPES.BLOCKLIST;
        let nonce = 1; 
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let index=committeeInfo.length-1;
        let addr=committeeInfo[index].address;
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
        
      

        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
      
        let blocklistPayload= createBlocklistPayload(
            [addr],
            true ,
        );


        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            blocklistPayload
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


        try {
            let tx=(await program.methods.updateBlockList(
            message_type,
            version,
            new BN(nonce),
            chain_id,
            Buffer.from(blocklistPayload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            messageConfig: messageConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
        }as any).rpc());
         expect.fail("Expected update block list  with Invalid message nonce");
        }catch(error){
            console.log(error.message)
            expect(error.message).to.include("Invalid message nonce");
        } 
    })

    

    it("update block list",async()=>{

        let message_type=MESSAGE_TYPES.BLOCKLIST;
        let nonce = 0; 
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let index=committeeInfo.length-1;
        let addr=committeeInfo[index].address;
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
        
      

        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
      
        let blocklistPayload= createBlocklistPayload(
            [addr],
            true ,
        );


        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            blocklistPayload
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

         let tx=(await program.methods.updateBlockList(
            message_type,
            version,
            new BN(nonce),
            chain_id,
            Buffer.from(blocklistPayload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            messageConfig: messageConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
        }as any).rpc());

        await program.provider.connection.confirmTransaction(tx);

        let committee = await program.account.committee.fetch(committeePDA);
        console.log(committee.members);
        console.log(committee.members[index]);
        expect(committee.members[index].isBlocklisted).to.be.equal(0);
        for (let i = 0; i < committeeInfo.length; i++) {
            if (i!=index) {
                expect(committee.members[i].isBlocklisted).to.be.equal(1);
            }else{
                expect(committee.members[i].isBlocklisted).to.be.equal(0);
            }
        }
    })

    it("测试签名不在 committee 中",async ()=>{

         let message_type=MESSAGE_TYPES.BLOCKLIST;
        let nonce = 1; 
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;
        let index=committeeInfo.length-1;
        let addr=committeeInfo[index].address;
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

        const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_VERIFIER),committeePDA.toBuffer()],
            program.programId
        );
        
        const [messageConfigPDA]= anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.MESSAGE_CONFIG),Buffer.from([message_type]),messageVerifierPDA.toBuffer()],
            program.programId
        );
      
        let blocklistPayload= createBlocklistPayload(
            [addr],
            true ,
        );


        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            blocklistPayload
        );

        let messageHash=computeMessageHash(message);
        let signatures=[];
        // committeeInfo 2 stake is 5000
        for (let i = 0; i < committeeInfo.length; i++) {
            const element = committeeInfo[i];
            let signature=getSignature(messageHash,element.privateKey);
            signatures.push(signature);
        }
        expect(signatures.length).to.be.equal(4);

        try{

        await program.methods.updateBlockList(
            message_type,
            version,
            new BN(nonce),
            chain_id,
            Buffer.from(blocklistPayload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            messageConfig: messageConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
        }as any).rpc();

         expect.fail("Expected update block list  with invalid signer");

        }catch(error) {
            console.log("error.message",error.message)
            expect(error.message).to.include("Signer is blocklisted");
        }

       

    })
})