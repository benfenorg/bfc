import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS, CHAIN_IDS, TOKEN_IDS, MESSAGE_TYPES } from "../../utils/const";
import { expect } from "chai";
import { createMessage, computeMessageHash } from "../../utils/message";
import { committeeInfo } from "../../utils/committee";
import { getSignature } from "../../utils/signatures";
import { deployUSDC, USDCDeploymentResult } from "../../utils/tokens/deploy_usdc";

describe("BenfenBridge - Update Token Fee Info", () => {
  const program = anchor.workspace.benfenBridge as Program<BenfenBridge>;

  let deployToken: USDCDeploymentResult;

  beforeEach(async () => {
    deployToken = await deployUSDC(
      program.provider.connection,
      program.provider.wallet.payer,
      1_000_000,
      6
    );
  });

  it("update fee info for USDT token", async () => {
    const tokenId = TOKEN_IDS.USDT;
    const encodeTokenId = BigInt(tokenId);
    const tokenIdBytes = Buffer.alloc(8);
    tokenIdBytes.writeBigUInt64BE(encodeTokenId);

    const message_type = MESSAGE_TYPES.UPDATE_TOKEN_FEE_INFO;
    const nonce = 0;
    const version = 1;
    const chain_id = CHAIN_IDS.SOLANA_TESTNET;

    const mode = 1; // percentage mode per EVM logic (mode==0 fixed)
    const feeValue = BigInt(50); // 0.5% (basis points)
    const minFeeValue = BigInt(1000000); // minimum fee

    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.BRIDGE_CONFIG)],
      program.programId
    );

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.COMMITTEE_SEED), bridgeConfigPDA.toBuffer()],
      program.programId
    );

    const [messageVerifierPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.MESSAGE_VERIFIER), committeePDA.toBuffer()],
      program.programId
    );

    const [messageConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.MESSAGE_CONFIG), Buffer.from([message_type]), messageVerifierPDA.toBuffer()],
      program.programId
    );

    const [tokenConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.TOKEN_CONFIG), tokenIdBytes],
      program.programId
    );

    // Ensure token_config account exists: try to fetch; if missing, add USDC token first
    // try {
    //   await program.account.tokenConfigAccount.fetch(tokenConfigPDA);
    // } catch {

    // }

    const payload = (() => {
      const buf = Buffer.alloc(25);
      buf.writeBigUInt64BE(encodeTokenId, 0);
      buf.writeUInt8(mode, 8);
      buf.writeBigUInt64BE(feeValue, 9);
      buf.writeBigUInt64BE(minFeeValue, 17);
      return buf;
    })();

    const message = createMessage(
      message_type,
      version,
      BigInt(nonce),
      chain_id,
      payload
    );

    const messageHash = computeMessageHash(message);
    const signatures: Uint8Array[] = [];
    for (let i = 0; i < 3; i++) {
      signatures.push(getSignature(messageHash, committeeInfo[i].privateKey));
    }
    expect(signatures.length).to.equal(3);

    const tx = await program.methods
      .updateTokenFeeInfo(
        message_type,
        version,
        new BN(nonce),
        chain_id,
        Buffer.from(payload),
        signatures.map((sig) => Buffer.from(sig))
      )
      .accounts({
        messageConfig: messageConfigPDA,
        tokenConfig: tokenConfigPDA,
        bridgeConfig: bridgeConfigPDA,
        verifier: messageVerifierPDA,
        committee: committeePDA,
      } as any)
      .rpc();

    await program.provider.connection.confirmTransaction(tx);

    const tokenInfo = await program.account.tokenConfigAccount.fetch(tokenConfigPDA);
    expect(tokenInfo.tokenId.toNumber()).to.equal(tokenId);
    expect(tokenInfo.mode).to.equal(mode);
    expect(tokenInfo.feeValue.toNumber()).to.equal(Number(feeValue));
    expect(tokenInfo.minFeeValue.toNumber()).to.equal(Number(minFeeValue));

    // // Verify fee calculation: amount=20_000 -> 0.5% = 100; minFeeValue=100 applies
    // const calcAmount = 20_000;
    // const expectedFee = Math.max(
    //   Math.floor((calcAmount * Number(feeValue)) / 10000),
    //   Number(minFeeValue)
    // );
    // // No direct method call from client; just assert logic alignment
    // expect(expectedFee).to.equal(100);
  });
});
