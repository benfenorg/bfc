import {
  Connection,
  PublicKey,
  Signer,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  createCloseAccountInstruction,
  mintTo as splMintTo,
  NATIVE_MINT,
  createSyncNativeInstruction,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
  getAccount,

} from "@solana/spl-token";

export interface WSOLResult {
  wsolTokenAccount: PublicKey;
  wsolAmount: number;
}


export async function wrapSol(
  connection: Connection,
  payer: Signer,
  solAmount: bigint = BigInt(1 * LAMPORTS_PER_SOL)
): Promise<PublicKey> {
  // WSOL 的 mint 地址是固定的 NATIVE_MINT
  const wsolMint = NATIVE_MINT;
  
  // 获取或创建 WSOL token account
  const wsolTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    wsolMint,
    payer.publicKey
  );

  let currentBalance = BigInt(0);
  try {
    const accountInfo = await getAccount(connection, wsolTokenAccount.address);
    currentBalance = accountInfo.amount ;
    console.log(`Current WSOL balance: ${Number(currentBalance) / LAMPORTS_PER_SOL} SOL`);
  } catch (error) {
    console.log("WSOL token account does not exist",error);
  }
  const neededAmount = solAmount - currentBalance;

  if (neededAmount <= 0) {
    console.log(`Sufficient WSOL balance (${currentBalance} SOL). No conversion needed.`);
    return wsolTokenAccount.address
  }

  console.log(`Need to convert ${neededAmount} SOL to WSOL`);

  const lamports = neededAmount;

  const tokenAccountInfo = await getAccount(connection, wsolTokenAccount.address);
  if (!tokenAccountInfo) {
    throw new Error("WSOL token account does not exist");
  }

  const transaction = new Transaction();

  // 转账 SOL 到 WSOL token account
  transaction.add(
    SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: wsolTokenAccount.address,
      lamports: lamports,
    })
  );

  // 同步 native account (将 SOL 转换为 WSOL)
  transaction.add(
    createSyncNativeInstruction(wsolTokenAccount.address)
  );

  // 发送并确认交易
  const signature = await connection.sendTransaction(transaction, [payer]);
  await connection.confirmTransaction(signature);

  console.log(`Successfully wrapped ${Number(solAmount) / LAMPORTS_PER_SOL} SOL to WSOL`);
  console.log(`WSOL Token Account: ${wsolTokenAccount.address.toString()}`);
  console.log(`Transaction Signature: ${signature}`);

  return wsolTokenAccount.address;
}


export async function unwrapSol(
  connection: Connection,
  payer: Signer,
  solAmount?: bigint,
): Promise<PublicKey> {
  // 如果没有提供 WSOL token account，使用默认的关联账户
 const wsolMint = NATIVE_MINT;
  
  // 获取或创建 WSOL token account
  const wsolTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    payer,
    wsolMint,
    payer.publicKey
  );

let currentBalance = BigInt(0);
  try {
    const accountInfo = await getAccount(connection, wsolTokenAccount.address);
    currentBalance = accountInfo.amount ;
    console.log(`Current WSOL balance: ${Number(currentBalance) / LAMPORTS_PER_SOL} SOL`);
  } catch (error) {
    console.log("WSOL token account does not exist",error);
  }
  if (!(currentBalance>=solAmount)){
    throw new Error("WSOL balance not enough");
  }

  console.log("enter")

//   const neededAmount = solAmount - currentBalance;

//   if (neededAmount <= 0) {
//      return wsolTokenAccount.address
//   }

  const transaction = new Transaction();

  //将wsolTokenAccount的wsol 转到用户手里变sol
  transaction.add(
    createCloseAccountInstruction(
      wsolTokenAccount.address,  // 要关闭的 token 账户
      payer.publicKey,          // lamports 的接收者
      payer.publicKey           // 账户的 owner
    )
  );


//   // 同步 native account (将 SOL 转换为 WSOL)
//   transaction.add(
//     createSyncNativeInstruction(wsolTokenAccount.address)
//   );
  //
  const signature = await connection.sendTransaction(transaction, [payer]);
  await connection.confirmTransaction(signature);

  console.log(`Successfully unwrapped WSOL back to SOL`);
  console.log(`Transaction Signature: ${signature}`);

    return wsolTokenAccount.address
}

/**
 * 检查用户的 WSOL 余额
 * @param connection Solana 连接
 * @param userPublicKey 用户公钥
 * @returns WSOL 余额（以 SOL 为单位）
 */
export async function getWsolBalance(
  connection: Connection,
  userPublicKey: PublicKey
): Promise<number> {
  try {
    const wsolTokenAccount = await getAssociatedTokenAddress(
      NATIVE_MINT,
      userPublicKey
    );
    
    const accountInfo = await getAccount(connection, wsolTokenAccount);
    return Number(accountInfo.amount) / LAMPORTS_PER_SOL;
  } catch (error) {
    console.log('WSOL token account does not exist');
    return 0;
  }
}