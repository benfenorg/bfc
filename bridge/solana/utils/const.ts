export const SEEDS = {
  BRIDGE_CONFIG: "bridge_config",
  TOKEN_CONFIG: "token_config",
  COMMITTEE_SEED: "committee",
  MESSAGE_VERIFIER: "message_verifier",
  MESSAGE_CONFIG: "message_config",
  CHAIN_LIMIT: "chain_limit",
  BENFEN_BRIDGE: "benfen_bridge",
  UPDATE_BRIDGE_LIMIT: "update_bridge_limit",
  VAULT: "vault",
  PROCESSED_TRANSFER: "processed_transfer",
  UPGRADE_AUTHORITY: "upgrade_authority",
} as const;

export const CHAIN_IDS = {
  SOLANA_MAINNET: 60,
  SOLANA_TESTNET: 61,
  SOLANA_CUSTOM: 62,
  BENFEN_MAINNET: 0,
  BENFEN_TESTNET: 2,  
  BENFEN_CUSTOM: 3, 
} as const;


export const TokenAddrInMainnet = {
  SOL: "So11111111111111111111111111111111111111111",
  USDC: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  USDT: "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
}


export const TOKEN_IDS={
  USDC: 3,
  USDT: 4,
  SOL: 16,
}

export const MESSAGE_TYPES={
  TOKEN_TRANSFER: 0,
  BLOCKLIST: 1,
  EMERGENCY_OP:2,
  UPDATE_BRIDGE_LIMIT:3,
  UPDATE_TOKEN_PRICE: 4,
  UPGRADE:32,
  
  UPDATE_BRIDGE_SINGLE_TRANSFER_LIMIT:19,
  ADD_SVM_TOKEN: 30,
  EXTEND_PROGRAM: 31,
  
}


export const DEFAULTS = {
  AIRDROP_AMOUNT: 1000000000,
} as const;

export type SeedTypes = typeof SEEDS[keyof typeof SEEDS];
export type ChainIdTypes = typeof CHAIN_IDS[keyof typeof CHAIN_IDS];