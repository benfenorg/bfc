import { ethers } from 'ethers';


export function getSignature(digest: Uint8Array, privateKey: string): Uint8Array {
    const formattedPrivateKey = privateKey.startsWith('0x') ? privateKey : `0x${privateKey}`;
    const wallet = new ethers.Wallet(formattedPrivateKey);
    const signature = wallet.signingKey.sign(digest);
    
    const r = ethers.getBytes(signature.r);
    const s = ethers.getBytes(signature.s);
    const v = signature.v;
    
    // 组合成65字节的签名
    const result = new Uint8Array(65);
    result.set(r, 0);       
    result.set(s, 32);      
    result[64] = v;         
    
    return result;
}

export function getSignatureFromHex(digestHex: string, privateKey: string): Uint8Array {
    const digest = ethers.getBytes(digestHex);
    return getSignature(digest, privateKey);
}

export function signatureToBytes(signature: Uint8Array): number[] {
    return Array.from(signature);
}
