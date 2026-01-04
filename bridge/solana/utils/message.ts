import { PublicKey } from "@solana/web3.js";
import { keccak256 } from "ethers";


export enum BlocklistType {
  BLOCKLIST = 0,    // 添加到黑名单
  UNBLOCKLIST = 1   // 从黑名单移除
}

// Message structure interface
export interface Message {
  messageType: number;
  version: number;
  nonce: bigint;
  chainId: number;
  payload: Uint8Array;
}

// Token transfer payload interface
export interface TokenTransferPayload {
  senderAddressLength: number;
  senderAddress: Uint8Array;
  targetChainId: number;
  recipientAddressLength: number;
  recipientAddress: PublicKey;
  tokenId: bigint;
  amount: bigint;
  txHash: Uint8Array;
  eventIdx: number;
}

// Blocklist payload interface
export interface BlocklistPayload {
  addresses: Uint8Array[];
  isBlocklisted: boolean;
}

// Add token payload interface
export interface AddTokenPayload {
  native: boolean;
  tokenId: bigint;
  tokenAddress: PublicKey;
  benfenDecimal: number;
  tokenPrice: bigint;
}

// Message constants
export const MESSAGE_PREFIX = new TextEncoder().encode("SUI_BRIDGE_MESSAGE");

// Message types
export const MESSAGE_TYPES = {
  TOKEN_TRANSFER: 0,
  BLOCKLIST: 1,
  EMERGENCY_OP: 2,
  UPDATE_BRIDGE_LIMIT: 3,
  UPDATE_TOKEN_PRICE: 4,
  UPGRADE: 5,
  ADD_EVM_TOKENS: 7,
  UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT: 19,
  ADD_SVM_TOKENS: 30,
  EXTEND_PROGRAM: 31,
} as const;

// Required stakes
export const REQUIRED_STAKES = {
  TRANSFER_STAKE_REQUIRED: 3334,
  FREEZING_STAKE_REQUIRED: 450,
  UNFREEZING_STAKE_REQUIRED: 5001,
  UPGRADE_STAKE_REQUIRED: 5001,
  BLOCKLIST_STAKE_REQUIRED: 5001,
  BRIDGE_LIMIT_STAKE_REQUIRED: 5001,
  UPDATE_TOKEN_PRICE_STAKE_REQUIRED: 5001,
  ADD_EVM_TOKENS_STAKE_REQUIRED: 5001,
  UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT_STAKE_REQUIRED: 5001,
} as const;

// Token IDs
export const TOKEN_IDS = {
  SUI: BigInt(0),
  BTC: BigInt(1),
  ETH: BigInt(2),
  USDC: BigInt(3),
  USDT: BigInt(4),
  BNB: BigInt(6),
  OP: BigInt(7),
  ARB: BigInt(8),
  POL: BigInt(9),
  AVAX: BigInt(10),
  SOL: BigInt(14),
} as const;

/**
 * Creates a bridge message
 * @param messageType - Type of the message
 * @param version - Version of the message
 * @param nonce - Nonce for the message
 * @param chainId - Chain ID
 * @param payload - Message payload as Uint8Array
 * @returns Message object
 */
export function createMessage(
  messageType: number,
  version: number,
  nonce: bigint,
  chainId: number,
  payload: Uint8Array
): Message {
  return {
    messageType,
    version,
    nonce,
    chainId,
    payload,
  };
}

/**
 * Encodes a message into bytes
 * @param message - Message to encode
 * @returns Encoded message as Uint8Array
 */
export function encodeMessage(message: Message): Uint8Array {
  const encoded: number[] = [];
  
  // Add prefix
  encoded.push(...MESSAGE_PREFIX);
  
  // Add message type and version
  encoded.push(message.messageType);
  encoded.push(message.version);
  
  // Add nonce (8 bytes, big-endian)
  const nonceBytes = new ArrayBuffer(8);
  const nonceView = new DataView(nonceBytes);
  nonceView.setBigUint64(0, message.nonce, false); // false = big-endian
  encoded.push(...new Uint8Array(nonceBytes));
  
  // Add chain ID
  encoded.push(message.chainId);
  
  // Add payload
  encoded.push(...message.payload);
  
  return new Uint8Array(encoded);
}

/**
 * Computes keccak256 hash of data
 * @param data - Data to hash
 * @returns Hash as Uint8Array
 */
export function keccak256Hash(data: Uint8Array): Uint8Array {
  const hash = keccak256(data);
  return new Uint8Array(Buffer.from(hash.slice(2), 'hex'));
}

/**
 * Computes the hash of a bridge message
 * @param message - Message to hash
 * @returns Message hash as Uint8Array
 */
export function computeMessageHash(message: Message): Uint8Array {
  const encoded = encodeMessage(message);
  return keccak256Hash(encoded);
}

/**
 * Computes the required stake for a message type
 * @param message - Message to check
 * @returns Required stake amount
 */
export function computeRequiredStake(message: Message): number {
  switch (message.messageType) {
    case MESSAGE_TYPES.TOKEN_TRANSFER:
      return REQUIRED_STAKES.TRANSFER_STAKE_REQUIRED;
    case MESSAGE_TYPES.BLOCKLIST:
      return REQUIRED_STAKES.BLOCKLIST_STAKE_REQUIRED;
    case MESSAGE_TYPES.EMERGENCY_OP:
      // For emergency operations, we need to decode the payload to determine if it's freezing or unfreezing
      // Assuming first byte indicates the operation (1 = freeze, 0 = unfreeze)
      const opCode = message.payload.length > 0 ? message.payload[0] : 0;
      return opCode === 1 
        ? REQUIRED_STAKES.FREEZING_STAKE_REQUIRED 
        : REQUIRED_STAKES.UNFREEZING_STAKE_REQUIRED;
    case MESSAGE_TYPES.UPDATE_BRIDGE_LIMIT:
      return REQUIRED_STAKES.BRIDGE_LIMIT_STAKE_REQUIRED;
    case MESSAGE_TYPES.UPDATE_TOKEN_PRICE:
      return REQUIRED_STAKES.UPDATE_TOKEN_PRICE_STAKE_REQUIRED;
    case MESSAGE_TYPES.UPGRADE:
      return REQUIRED_STAKES.UPGRADE_STAKE_REQUIRED;
    case MESSAGE_TYPES.ADD_EVM_TOKENS:
      return REQUIRED_STAKES.ADD_EVM_TOKENS_STAKE_REQUIRED;
    case MESSAGE_TYPES.UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT:
      return REQUIRED_STAKES.UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT_STAKE_REQUIRED;
    default:
      return 0;
  }
}

/**
 * Converts a Solana PublicKey to Ethereum address format
 * @param pubkey - 64-byte public key
 * @returns 20-byte Ethereum address
 */
export function pubkeyToEthAddress(pubkey: Uint8Array): Uint8Array {
  if (pubkey.length !== 64) {
    throw new Error('Public key must be 64 bytes');
  }
  
  const hash = keccak256Hash(pubkey);
  return hash.slice(12, 32); // Take last 20 bytes
}

/**
 * Helper function to create a token transfer message
 * @param nonce - Message nonce
 * @param chainId - Source chain ID
 * @param payload - Token transfer payload
 * @returns Token transfer message
 */
export function createTokenTransferMessage(
  nonce: bigint,
  chainId: number,
  payload: Uint8Array
): Message {
  return createMessage(
    MESSAGE_TYPES.TOKEN_TRANSFER,
    1, // version
    nonce,
    chainId,
    payload
  );
}

/**
 * Helper function to create an emergency operation message
 * @param nonce - Message nonce
 * @param chainId - Source chain ID
 * @param isPausing - Whether this is a pausing operation
 * @returns Emergency operation message
 */
export function createEmergencyOpMessage(
  nonce: bigint,
  chainId: number,
  isPausing: boolean
): Message {
  const payload = new Uint8Array([isPausing ? 1 : 0]);
  return createMessage(
    MESSAGE_TYPES.EMERGENCY_OP,
    1, // version
    nonce,
    chainId,
    payload
  );
}

/**
 * Creates an UpdateTokenPrice payload serialized as bytes
 * Based on the Rust decode_update_token_price_payload function
 * Payload format: token_id(8) + token_price(8) = 16 bytes
 * @param tokenId - Token ID (8 bytes, big-endian)
 * @param tokenPrice - Token price (8 bytes, big-endian)
 * @returns Serialized payload as Uint8Array
 */
export function createUpdateTokenPricePayload(
  tokenId: bigint,
  tokenPrice: bigint
): Uint8Array {
  // Total payload size: 8 + 8 = 16 bytes
  const payload = new Uint8Array(16);
  
  // Write token ID (8 bytes, big-endian)
  const tokenIdBuffer = new ArrayBuffer(8);
  const tokenIdView = new DataView(tokenIdBuffer);
  tokenIdView.setBigUint64(0, tokenId, false); // false = big-endian
  payload.set(new Uint8Array(tokenIdBuffer), 0);
  
  // Write token price (8 bytes, big-endian)
  const priceBuffer = new ArrayBuffer(8);
  const priceView = new DataView(priceBuffer);
  priceView.setBigUint64(0, tokenPrice, false); // false = big-endian
  payload.set(new Uint8Array(priceBuffer), 8);
  
  return payload;
}

/**
 * Creates an AddTokenPayload serialized as bytes
 * Based on the Rust decode_add_token_payload function
 * Payload format: native(1) + token_id(8) + token_address(32) + benfen_decimal(1) + token_price(8) = 50 bytes
 * @param native - Whether the token is native
 * @param tokenId - Token ID (8 bytes, big-endian)
 * @param tokenAddress - Token address (32 bytes)
 * @param benfenDecimal - Benfen decimal places (1 byte)
 * @param tokenPrice - Token price (8 bytes, big-endian)
 * @returns Serialized payload as Uint8Array
 */
export function createAddSlpTokenPayload(
  native: boolean,
  tokenId: bigint,
  tokenAddress: PublicKey,
  benfenDecimal: number,
  tokenPrice: bigint
): Uint8Array {
  // Total payload size: 1 + 8 + 32 + 1 + 8 = 50 bytes
  const payload = new Uint8Array(50);
  let offset = 0;

  // Write native flag (1 byte)
  payload[offset] = native ? 1 : 0;
  offset += 1;

  // Write token ID (8 bytes, big-endian)
  const tokenIdBuffer = new ArrayBuffer(8);
  const tokenIdView = new DataView(tokenIdBuffer);
  tokenIdView.setBigUint64(0, tokenId, false); // false = big-endian
  payload.set(new Uint8Array(tokenIdBuffer), offset);
  offset += 8;

  // Write token address (32 bytes)
  payload.set(tokenAddress.toBytes(), offset);
  offset += 32;

  // Write benfen decimal (1 byte)
  payload[offset] = benfenDecimal;
  offset += 1;

  // Write token price (8 bytes, big-endian)
  const priceBuffer = new ArrayBuffer(8);
  const priceView = new DataView(priceBuffer);
  priceView.setBigUint64(0, tokenPrice, false); // false = big-endian
  payload.set(new Uint8Array(priceBuffer), offset);

  return payload;
}

/**
 * Helper function to create an add token message
 * @param nonce - Message nonce
 * @param chainId - Source chain ID
 * @param tokenPayload - Add token payload
 * @returns Add token message
 */
export function createAddTokenMessage(
  nonce: bigint,
  chainId: number,
  tokenPayload: AddTokenPayload
): Message {
  const payload = createAddSlpTokenPayload(
    tokenPayload.native,
    tokenPayload.tokenId,
    tokenPayload.tokenAddress,
    tokenPayload.benfenDecimal,
    tokenPayload.tokenPrice
  );
  
  return createMessage(
    MESSAGE_TYPES.ADD_SVM_TOKENS,
    1, // version
    nonce,
    chainId,
    payload
  );
}

/**
 * Decodes an AddTokenPayload from bytes (reverse of createAddSlpTokenPayload)
 * @param payload - Serialized payload bytes
 * @returns Decoded AddTokenPayload
 */
export function decodeAddTokenPayload(payload: Uint8Array): AddTokenPayload {
  if (payload.length < 50) {
    throw new Error('Invalid payload length: minimum 50 bytes required');
  }

  let offset = 0;

  // Read native flag (1 byte)
  const native = payload[offset] !== 0;
  offset += 1;

  // Read token ID (8 bytes, big-endian)
  const tokenIdView = new DataView(payload.buffer, payload.byteOffset + offset, 8);
  const tokenId = tokenIdView.getBigUint64(0, false); // false = big-endian
  offset += 8;

  // Read token address (32 bytes)
  const tokenAddressBytes = payload.slice(offset, offset + 32);
  const tokenAddress = new PublicKey(tokenAddressBytes);
  offset += 32;

  // Read benfen decimal (1 byte)
  const benfenDecimal = payload[offset];
  offset += 1;

  // Read token price (8 bytes, big-endian)
  const priceView = new DataView(payload.buffer, payload.byteOffset + offset, 8);
  const tokenPrice = priceView.getBigUint64(0, false); // false = big-endian

  return {
    native,
    tokenId,
    tokenAddress,
    benfenDecimal,
    tokenPrice,
  };
}



/**
 * Creates an UpdateLimit payload serialized as bytes
 * Based on the Rust decode_update_limit_payload function
 * Payload format: sender_chain_id(1) + new_limit(8) = 9 bytes
 * @param senderChainId - Sender chain ID (1 byte)
 * @param newLimit - New limit value (8 bytes, big-endian)
 * @returns Serialized payload as Uint8Array
 */
export function createUpdateLimitPayload(
  senderChainId: number,
  newLimit: bigint
): Uint8Array {
  // Total payload size: 1 + 8 = 9 bytes
  const payload = new Uint8Array(9);
  
  // Write sender chain ID (1 byte)
  payload[0] = senderChainId;
  
  // Write new limit (8 bytes, big-endian)
  const limitBuffer = new ArrayBuffer(8);
  const limitView = new DataView(limitBuffer);
  limitView.setBigUint64(0, newLimit, false); // false = big-endian
  payload.set(new Uint8Array(limitBuffer), 1);
  
  return payload;
}


/**
 * 创建更新单笔转账限制的payload
 * 对应Rust中的decode_update_single_transfer_limit_payload函数
 * @param senderChainId - 发送方链ID (1字节)
 * @param newLimit - 新的限制金额 (8字节，大端序u64)
 * @returns 9字节的payload buffer
 */
export function createUpdateSingleTransferLimitPayload(
  senderChainId: number,
  newLimit: bigint
): Buffer {
  // 验证参数
  if (senderChainId < 0 || senderChainId > 255) {
    throw new Error('senderChainId must be between 0 and 255');
  }
  if (newLimit < 0) {
    throw new Error('newLimit must be non-negative');
  }

  // 创建9字节的buffer (1字节sender_chain_id + 8字节new_limit)
  const buffer = Buffer.alloc(9);
  
  // 写入sender_chain_id (1字节)
  buffer.writeUInt8(senderChainId, 0);
  
  // 写入new_limit (8字节，大端序)
  buffer.writeBigUInt64BE(newLimit, 1);
  
  return buffer;
}


/**
 * 创建黑名单操作的payload
 * 对应Rust中的decode_blocklist_payload函数
 * @param addresses - 地址列表（hex格式，每个地址20字节）
 * @param isBlocklisted - true表示添加到黑名单，false表示移除
 * @returns payload buffer
 */
export function createBlocklistPayload(
  addresses: string[],
  isBlocklisted: boolean
): Buffer {
  // 验证参数
  if (addresses.length === 0) {
    throw new Error('addresses array cannot be empty');
  }
  if (addresses.length > 255) {
    throw new Error('too many addresses: maximum 255 addresses allowed');
  }

  // 验证地址格式
  for (const address of addresses) {
    const cleanAddress = address.startsWith('0x') ? address.slice(2) : address;
    if (cleanAddress.length !== 40) {
      throw new Error(`Invalid address length: ${address} (expected 40 hex characters)`);
    }
    if (!/^[0-9a-fA-F]+$/.test(cleanAddress)) {
      throw new Error(`Invalid address format: ${address} (must be hex)`);
    }
  }

  // 计算总长度：2字节头部 + (地址数量 * 20字节)
  const totalLength = 2 + (addresses.length * 20);
  const buffer = Buffer.alloc(totalLength);
  
  let offset = 0;
  
  // 写入操作类型 (1字节)
  // blocklistType: 0 = blocklist, 1 = unblocklist
  buffer.writeUInt8(isBlocklisted ? BlocklistType.BLOCKLIST : BlocklistType.UNBLOCKLIST, offset);
  offset += 1;
  
  // 写入成员数量 (1字节)
  buffer.writeUInt8(addresses.length, offset);
  offset += 1;
  
  // 写入地址列表 (每个地址20字节)
  for (const address of addresses) {
    const cleanAddress = address.startsWith('0x') ? address.slice(2) : address;
    const addressBuffer = Buffer.from(cleanAddress, 'hex');
    addressBuffer.copy(buffer, offset);
    offset += 20;
  }
  
  return buffer;
}


// ... existing code ...

/**
 * 编码TokenTransferPayload为字节数组
 * 对应Rust中的decode_token_transfer_payload函数
 * 字节布局：
 * - byte 0: sender_address_length (1字节，固定为32)
 * - bytes 1-32: sender_address (32字节)
 * - byte 33: target_chain_id (1字节)
 * - byte 34: recipient_address_length (1字节，固定为32)
 * - bytes 35-66: recipient_address (32字节)
 * - bytes 67-74: token_id (8字节，大端序u64)
 * - bytes 75-82: amount (8字节，大端序u64)
 * - bytes 83 to (length-2): tx_hash (可变长度)
 * - last 2 bytes: event_idx (2字节，大端序u16)
 * @param payload - TokenTransferPayload对象
 * @returns 编码后的字节数组
 */
export function encodeTokenTransferPayload(payload: TokenTransferPayload): Uint8Array {
  // 验证参数
  if (payload.senderAddress.length !== 32) {
    throw new Error('Sender address must be 32 bytes');
  }
  if (payload.recipientAddress.toBytes().length !== 32) {
    throw new Error('Recipient address must be 32 bytes');
  }
  if (payload.targetChainId < 0 || payload.targetChainId > 255) {
    throw new Error('Target chain ID must be between 0 and 255');
  }
  if (payload.eventIdx < 0 || payload.eventIdx > 65535) {
    throw new Error('Event index must be between 0 and 65535');
  }

  // 计算总长度：83字节固定部分 + tx_hash长度
  const fixedLength = 83; // 1+32+1+1+32+8+8 = 83
  const totalLength = fixedLength + payload.txHash.length + 2; // +2 for event_idx
  
  const buffer = new Uint8Array(totalLength);
  let offset = 0;

  // byte 0: sender_address_length (固定为32)
  buffer[offset] = 32;
  offset += 1;

  // bytes 1-32: sender_address
  buffer.set(payload.senderAddress, offset);
  offset += 32;

  // byte 33: target_chain_id
  buffer[offset] = payload.targetChainId;
  offset += 1;

  // byte 34: recipient_address_length (固定为32)
  buffer[offset] = 32;
  offset += 1;

  // bytes 35-66: recipient_address
  buffer.set(payload.recipientAddress.toBytes(), offset);
  offset += 32;

  // bytes 67-74: token_id (8字节，大端序)
  const tokenIdBuffer = new ArrayBuffer(8);
  const tokenIdView = new DataView(tokenIdBuffer);
  tokenIdView.setBigUint64(0, payload.tokenId, false); // false = big-endian
  buffer.set(new Uint8Array(tokenIdBuffer), offset);
  offset += 8;

  // bytes 75-82: amount (8字节，大端序)
  const amountBuffer = new ArrayBuffer(8);
  const amountView = new DataView(amountBuffer);
  amountView.setBigUint64(0, payload.amount, false); // false = big-endian
  buffer.set(new Uint8Array(amountBuffer), offset);
  offset += 8;

  // bytes 83 to (length-2): tx_hash
  buffer.set(payload.txHash, offset);
  offset += payload.txHash.length;

  // last 2 bytes: event_idx (大端序u16)
  const eventIdxBuffer = new ArrayBuffer(2);
  const eventIdxView = new DataView(eventIdxBuffer);
  eventIdxView.setUint16(0, payload.eventIdx, false); // false = big-endian
  buffer.set(new Uint8Array(eventIdxBuffer), offset);

  return buffer;
}

/**
 * 创建TokenTransferPayload对象的辅助函数
 * @param senderAddress - 发送方地址 (32字节)
 * @param targetChainId - 目标链ID
 * @param recipientAddress - 接收方地址 (PublicKey)
 * @param tokenId - Token ID
 * @param amount - 转账金额
 * @param txHash - 交易哈希
 * @param eventIdx - 事件索引
 * @returns TokenTransferPayload对象
 */
export function createTokenTransferPayload(
  senderAddress: Uint8Array,
  targetChainId: number,
  recipientAddress: PublicKey,
  tokenId: bigint,
  amount: bigint,
  txHash: Uint8Array,
  eventIdx: number
): TokenTransferPayload {
  return {
    senderAddressLength: 32,
    senderAddress,
    targetChainId,
    recipientAddressLength: 32,
    recipientAddress,
    tokenId,
    amount,
    txHash,
    eventIdx,
  };
}

/**
 * 十六进制字符串转字节数组的辅助函数
 * @param hex - 十六进制字符串（可选0x前缀）
 * @returns 字节数组
 */
export function hexToBytes(hex: string): Uint8Array {
  const cleanHex = hex.startsWith('0x') ? hex.slice(2) : hex;
  if (cleanHex.length % 2 !== 0) {
    throw new Error('Invalid hex string length');
  }
  const bytes = new Uint8Array(cleanHex.length / 2);
  for (let i = 0; i < cleanHex.length; i += 2) {
    bytes[i / 2] = parseInt(cleanHex.substr(i, 2), 16);
  }
  return bytes;
}

export function encodeEmergencyOpPayload(isFreezing: boolean): Uint8Array {
    // 根据Rust代码逻辑：0 = blocklist(冻结), 1 = unblocklist(解冻)
    const opCode = isFreezing ? 0 : 1;
    return new Uint8Array([opCode]);
}

export function createUpgradePayload(
  proxy: PublicKey,
  implementation: PublicKey,
  version: number
): Uint8Array {
  const payload = new Uint8Array(65);
  
  // 设置代理地址 (0-31 字节)
  payload.set(proxy.toBytes(), 0);
  
  // 设置实现地址 (32-63 字节)
  payload.set(implementation.toBytes(), 32);
  
  // 设置版本号 (64 字节)
  payload[64] = version;
  
  return payload;
}

export function createExtendPayload(
  proxy: PublicKey,
  size: number, //u32
): Uint8Array{
  const payload = new Uint8Array(36);
   payload.set(proxy.toBytes(), 0);
   const sizeBytes = Buffer.alloc(4);
   sizeBytes.writeUInt32BE(size,0);
   payload.set(sizeBytes, 32);
   return payload;
}

