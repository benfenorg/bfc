import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, DEFAULTS, TOKEN_IDS, MESSAGE_TYPES, TokenAddrInMainnet } from "../../utils/const";
import { expect } from "chai";
import { createMessage,createAddSlpTokenPayload, computeMessageHash,createUpdateSingleTransferLimitPayload } from "../../utils/message";
import { committeeInfo } from "../../utils/committee";
import { getSignature } from "../../utils/signatures";
import { deployUSDC, USDCDeploymentResult} from "../../utils/tokens/deploy_usdc";
import { Connection } from "@solana/web3.js";

describe("BenfenBridge - Update Single Transfer Limit", () => {
      const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

      let deployToken : USDCDeploymentResult;

    beforeEach(async()=>{
        // const connection = new Connection("http://localhost:8899", "confirmed");
        // const payer = program.provider.wallet.publicKey

        deployToken=await deployUSDC(program.provider.connection,program.provider.wallet.payer,1_000_000,6);
    })


    it("update single transfer limit",async()=>{

        let message_type=MESSAGE_TYPES.UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT;
        let nonce = 0; 
        let version=1;
        let chain_id=CHAIN_IDS.SOLANA_TESTNET;

        let limit=500000*100000000;
        let targetChainId=CHAIN_IDS.BENFEN_TESTNET;
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
      
        let updateSingleTransferLimitPayload= createUpdateSingleTransferLimitPayload(
            targetChainId,
            BigInt(limit),
        );


        let message=createMessage(
            message_type,
            version,
            BigInt(nonce),
            chain_id,
            updateSingleTransferLimitPayload
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

         let tx=(await program.methods.updateSingleTransferLimit(
            message_type,
            version,
            new BN(nonce),
            chain_id,
            Buffer.from(updateSingleTransferLimitPayload),
            signatures.map(sig => Buffer.from(sig))
        ).accounts({
            messageConfig: messageConfigPDA,
            bridgeConfig: bridgeConfigPDA,
            chainLimit: chainLimitPDA,
            verifier: messageVerifierPDA,
            committee: committeePDA,
        }as any).rpc());

        await program.provider.connection.confirmTransaction(tx);

        let chainLimit = await program.account.chainLimit.fetch(chainLimitPDA);
        expect(chainLimit.maxUsdLimit.toNumber()).to.be.equal(limit);

        let messageConfig1 = await program.account.messageConfig.fetch(messageConfigPDA);
        expect(messageConfig1.messageType).to.be.equal(MESSAGE_TYPES.UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT);
        expect(messageConfig1.nonce.toNumber()).to.be.equal(nonce+1);
      
    })
})