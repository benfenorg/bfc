import {
  Connection,
  PublicKey,
  Signer,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo as splMintTo,
} from "@solana/spl-token";

export interface USDCDeploymentResult {
  mintAddress: PublicKey;
  tokenAccountAddress: PublicKey;
}

/**
 * 获取SOL
 */
async function airdropSol(
  connection: Connection,
  publicKey: PublicKey,
  amountSOL: number = 2
): Promise<void> {
  const signature = await connection.requestAirdrop(
    publicKey,
    amountSOL * LAMPORTS_PER_SOL
  );
  await connection.confirmTransaction(signature);
}

/**
 * 部署模拟 USDC：
 */
export async function deployUSDC(
  connection: Connection,
  payer: Signer,
  mintAmount: number = 10000000000,
  decimals: number = 6
): Promise<USDCDeploymentResult> {
  await airdropSol(connection, payer.publicKey);

  const mint = await createMint(
    connection,
    payer,
    payer.publicKey,
    null,
    decimals
  );

  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    mint,
    payer.publicKey
  );

  const amount = mintAmount * 10 ** decimals;
  await splMintTo(connection, payer, mint, tokenAccount.address, payer, amount);

  return {
    mintAddress: mint,
    tokenAccountAddress: tokenAccount.address,
  };
}

/**
 * 单独 Mint 代币到指定账户
 * @param connection 
 * @param payer 付款钱包
 * @param mint Token Mint 地址
 * @param destinationTokenAccount 收币账户地址
 * @param amount Mint 数量（不含小数位）
 */
export async function mintTo(
  connection: Connection,
  payer: Signer,
  mint: PublicKey,
  destinationTokenAccount: PublicKey,
  amount: number
) {
  // USDC 是6位小数
  const decimals = 6;
  const rawAmount = amount * 10 ** decimals;

  await splMintTo(connection, payer, mint, destinationTokenAccount, payer, rawAmount);
}
