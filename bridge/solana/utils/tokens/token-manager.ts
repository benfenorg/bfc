
import fs from 'fs';


interface TokenInfo {
    tokenType: string;
    tokenId: number;
    mintAddress: string;
    tokenAccountAddress: string;
    timestamp: string;
    index: number;
}

export function saveTokenToJsonArray(tokenData: {
    tokenType: string;
    tokenId: number;
    mintAddress: string;
    tokenAccountAddress: string;
}, filename: string = './deployTokens.json') {
    let existingTokens: any[] = [];
    
    try {
        const fileContent = fs.readFileSync(filename, 'utf8');
        const parsed = JSON.parse(fileContent);
        if (Array.isArray(parsed)) {
            existingTokens = parsed;
        } else {
            console.warn(`Warning: ${filename} is not an array, creating new array`);
            existingTokens = [];
        }
    } catch (error) {
        console.log(`Creating new token array file: ${filename}`);
        existingTokens = [];
    }
    
    const tokenWithTimestamp = {
        ...tokenData,
        timestamp: new Date().toISOString(),
        index: existingTokens.length + 1
    };
    
    existingTokens.push(tokenWithTimestamp);
    
    fs.writeFileSync(filename, JSON.stringify(existingTokens, null, 2));
    
    console.log(`Token saved successfully. Total tokens: ${existingTokens.length}`);
    return existingTokens.length;
}


export function getTokenByTokenId(tokenId: number, filename: string = './deployTokens.json'): TokenInfo | null {
    try {
        // 读取JSON文件
        const fileContent = fs.readFileSync(filename, 'utf8');
        const tokens: TokenInfo[] = JSON.parse(fileContent);
        
        // 确保是数组格式
        if (!Array.isArray(tokens)) {
            console.error(`Error: ${filename} is not an array format`);
            return null;
        }
        
        // 查找匹配的tokenId
        const foundToken = tokens.find(token => token.tokenId === tokenId);
        
        if (foundToken) {
            console.log(`Found token with ID ${tokenId}:`, foundToken);
            return foundToken;
        } else {
            console.log(`No token found with ID: ${tokenId}`);
            return null;
        }
        
    } catch (error) {
        console.error(`Error reading or parsing ${filename}:`, error);
        return null;
    }
}


export function getAllTokens(filename: string = './deployTokens.json'): TokenInfo[] {
    try {
        const fileContent = fs.readFileSync(filename, 'utf8');
        const tokens: TokenInfo[] = JSON.parse(fileContent);
        
        if (!Array.isArray(tokens)) {
            console.error(`Error: ${filename} is not an array format`);
            return [];
        }
        
        return tokens;
    } catch (error) {
        console.error(`Error reading or parsing ${filename}:`, error);
        return [];
    }
}


export function getTokensByType(tokenType: string, filename: string = './deployTokens.json'): TokenInfo[] {
    try {
        const fileContent = fs.readFileSync(filename, 'utf8');
        const tokens: TokenInfo[] = JSON.parse(fileContent);
        
        if (!Array.isArray(tokens)) {
            console.error(`Error: ${filename} is not an array format`);
            return [];
        }
        
        return tokens.filter(token => token.tokenType.toLowerCase() === tokenType.toLowerCase());
    } catch (error) {
        console.error(`Error reading or parsing ${filename}:`, error);
        return [];
    }
}


export function clearTokensFile(filename: string = './deployTokens.json') {
    try {
        // 写入空数组到文件
        fs.writeFileSync(filename, JSON.stringify([], null, 2));
        console.log(`Successfully cleared ${filename}`);
    } catch (error) {
        console.error(`Error clearing ${filename}:`, error);
    }
}


export function resetTokensFile(filename: string = './deployTokens.json') {
    try {
        if (fs.existsSync(filename)) {
            clearTokensFile(filename);
        } else {
            console.log(`File ${filename} does not exist, creating empty array`);
            fs.writeFileSync(filename, JSON.stringify([], null, 2));
        }
    } catch (error) {
        console.error(`Error resetting ${filename}:`, error);
    }
}