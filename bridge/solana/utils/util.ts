export function benfenAddressToBytes(benfenAddress: string,vaild:boolean = true): Buffer {
    // 移除 0x 前缀（如果存在）
    const cleanAddress = benfenAddress.startsWith('0x') 
        ? benfenAddress.slice(2) 
        : benfenAddress;
    
    if (vaild){
        if (!/^[0-9a-fA-F]{64}$/.test(cleanAddress)) {
            throw new Error('Invalid benfen address format. Expected 64 hex characters.');
        }
    }
    return Buffer.from(cleanAddress, 'hex');
}


export function getDefaultBenfenAddress(): string {
    return '0xaea8ea4ce7c82b9f32835f5cee1057a19dc673cf71b313db8f2bf01f1cc7a91e';
}


export function getDefaultBenfenAddressBytes(): Buffer {
    const benfenAddress = getDefaultBenfenAddress();
    return benfenAddressToBytes(benfenAddress);
}

const benfenAddress = '0xaea8ea4ce7c82b9f32835f5cee1057a19dc673cf71b313db8f2bf01f1cc7a91e';
const benfenAddressBytes = Buffer.from(benfenAddress.slice(2), 'hex');




export function tokenIdToBytes(tokenId: number | bigint): Buffer {
    const encodeTokenId = BigInt(tokenId);
    // 把 token_id 转换成 big-endian Uint8Array (8 字节)
    const tokenIdBytes = Buffer.alloc(8);
    tokenIdBytes.writeBigUInt64BE(encodeTokenId);
    return tokenIdBytes;
}


export function bytesToTokenId(tokenIdBytes: Buffer): bigint {
    if (tokenIdBytes.length !== 8) {
        throw new Error('Token ID bytes must be exactly 8 bytes long');
    }
    return tokenIdBytes.readBigUInt64BE(0);
}


export function isValidTokenId(tokenId: number | bigint): boolean {
    try {
        const id = BigInt(tokenId);
        return id >= 0 && id <= BigInt(Number.MAX_SAFE_INTEGER);
    } catch {
        return false;
    }
}


export function nonceToBytes(nonce: number | bigint): Buffer {
    const encodeNonce = BigInt(nonce);
    const nonceBytes = Buffer.alloc(8);
    nonceBytes.writeBigUInt64BE(encodeNonce);
    return nonceBytes;
}

export { benfenAddress, benfenAddressBytes };

