import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BenfenBridge } from "../../target/types/benfen_bridge";
import { SEEDS } from "../../utils/const";
import { expect } from "chai";


describe("Transfer Authority", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BenfenBridge as Program<BenfenBridge>;

  it("Transfer program upgrade authority to PDA", async () => {
    const bpfLoaderUpgradeable = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");
    // 计算committee PDA
    const [bridgeConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.BRIDGE_CONFIG)],
        program.programId
    );

    const [committeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from(SEEDS.COMMITTEE_SEED),bridgeConfigPDA.toBuffer()],
          program.programId
    );

    // 计算升级权限PDA
    const [upgradeAuthorityPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.UPGRADE_AUTHORITY),committeePDA.toBuffer()],
      program.programId
    );

    const upgradeAuthorityPDA1 = await program.provider.connection.getAccountInfo(upgradeAuthorityPDA);
    expect(upgradeAuthorityPDA1).to.be.not.null;

    // const programDataAddress = new anchor.web3.PublicKey("7cVwth8zehVouPkHvpXoavKKCT7Xqk15F34bms269SJq");

    const [programDataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [program.programId.toBuffer()],
      bpfLoaderUpgradeable
    );

    console.log("programDataAddress",programDataAddress.toString());
    console.log("upgradeAuthorityPDA",upgradeAuthorityPDA.toString());


    // 调用指令
    await program.methods
      .transferUpgradeAuthority(
      )
      .accounts({
        oldUpgradeAuthority: provider.wallet.publicKey,
        programData: programDataAddress,
        newUpgradeAuthority: upgradeAuthorityPDA,
        bpfLoaderUpgradeable: new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111"),
        program: program.programId,
      } as any)
      .rpc()
    })


    

});