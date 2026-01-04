
use anchor_lang::prelude::*;

#[error_code]
pub enum BridgeUpgradeError{
    #[msg("Invalid buffer owner")]
    InvalidBufferOwner,
    #[msg("Unauthorized Upgrade")]
    UnauthorizedUpgrade,
    #[msg("Invalid program data account")]
    InvalidProgramData,
    #[msg("Program version mismatch")]
    ProgramVersionMismatch,
    #[msg("Upgrade cooldown period not elapsed")]
    UpgradeCooldownNotElapsed,
    #[msg("Missing signature")]
    MissingSignature,
}

#[error_code]
pub enum  BridgeTokenError {
    #[msg("Token not supported")]
    TokenNotSupported,
    #[msg("Invalid token id")]
    InvalidTokenId,
    #[msg("Invalid fungible token decimals")]
    InvalidFungibleTokenDecimals,
    #[msg("Invalid Benfen decimal")]
    InvalidTokenBenfenDecimal,
    #[msg("Invalid token id not supported")]
    InvalidTokenIdNotSupported,
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    #[msg("Invalid token owner")]
    InvalidTokenOwner,
    #[msg("Invalid token price")]
    InvalidTokenPrice,
}

#[error_code]
pub enum BridgeConvertError {
    #[msg("Insufficient amount provided")]
    InsufficientAmount,

    #[msg("Amount is too large")]
    AmountTooLarge,
    #[msg("Amount is too small")]
    AmountTooSmall,
}




#[error_code]
pub enum MessageError {
    #[msg("Invalid message type")]
    InvalidMessageType,
    #[msg("Invalid message nonce")]
    InvalidMessageNonce,
    #[msg("Invalid message chain id")]
    InvalidMessageChainId,
    #[msg("Invalid message verifier")]
    InvalidMessageVerifier,
    #[msg("Invalid sender address length")]
    InvalidSenderAddressLength,
    #[msg("Invalid recipient address length")]
    InvalidRecipientAddressLength,
    #[msg("Invalid payload length")]
    InvalidPayloadLength,
    #[msg("Invalid operation code")]
    InvalidOpCode,
    #[msg("Invalid token id not supported")]
    InvalidTokenIdNotSupported,
    #[msg("Invalid mint address: cannot be default address")]
    InvalidMintAddress,
    #[msg("Invalid token price")]
    InvalidTokenPrice,
}

#[error_code]
pub enum BridgeLimiterError {
    #[msg("Invalid amount is zero")]
    InvalidAmountIsZero,
    #[msg("Exceed window limit")]
    ExceedWindowLimit,
    #[msg("Invalid chain ID")]
    InvalidChainId,
    #[msg("Unsupported chain")]
    UnsupportedChain,
    #[msg("Invalid limiter pubkey")]
    InvalidLimiterPubkey,
    
    #[msg("Invalid hour")]
    InvalidHour,
}

#[error_code]
pub enum BridgeConfigError {
    #[msg("Cannot support self chain")]
    CannotSupportSelf,
    #[msg("Chain already supported")]
    ChainAlreadySupported,
    #[msg("Invalid config pubkey")]
    InvalidConfigPubkey,
}

#[error_code]
pub enum BridgeCommitteeError {
    #[msg("Invalid committee size")]
    InvalidCommitteeSize,
    #[msg("Committee size must be less than 256")]
    CommitteeTooLarge,
    #[msg("Stake calculation overflow")]
    StakeOverflow,
    #[msg("Total stake is less than minimum required")]
    InsufficientStake,
    #[msg("Committee member stake mismatch")]
    CommitteeMemberStakeMismatch,
    #[msg("Committee member index mismatch")]
    CommitteeMemberIndexMismatch,
    #[msg("Committee member is blocklisted mismatch")]
    CommitteeMemberIsBlocklistedMismatch,
    #[msg("Committee size limit exceeded")]
    CommitteeSizeExceeded,
    #[msg("Not committee member")]
    NotCommitteeMember,
    #[msg("Duplicate member address")]
    DuplicateMemberAddress,
    #[msg("Invalid member index")]
    InvalidMemberIndex,

    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Invalid signer")]
    InvalidSigner,
    #[msg("Signer is blocklisted")]
    SignerBlocklisted,
    #[msg("Duplicate signature provided")]
    DuplicateSignature,
    #[msg("Message verifier nonce overflow")]
    MessageVerifierNonceOverflow,
    #[msg("Invalid signature format")]
    InvalidSignatureFormat,
    #[msg("Signature verification failed")]
    SignatureVerificationFailed,
    #[msg("Invalid recovery ID")]
    InvalidRecoveryId,
}

#[error_code]
pub enum AdminError {
    #[msg("Not approved")]
    NotApproved,
}

#[error_code]
pub enum BridgeError {
    #[msg("Bridge is currently paused")]
    BridgePaused,
    #[msg("Invalid recipient address")]
    InvalidRecipientAddress,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid benfen address")]
    InvalidBenfenAddress,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Cross out amount exceeds limit")]
    CrossOutAmountExceedLimit,
    #[msg("Invalid target chain id")]
    InvalidTargetChainId,
    #[msg("Token transfer already processed")]
    TokenTransferAlreadyProcessed,
    #[msg("Unsupported cross to chain id")]
    UnsupportedCrossToChainId,
    #[msg("Invalid bridge config")]
    InvalidBridgeConfig,
    #[msg("Transfer already processed")]
    TransferAlreadyProcessed,
    #[msg("Single transfer amount exceeds limit")]
    SingleTransferAmountExceedsLimit,

    #[msg("Invalid committee")]
    InvalidCommittee,
}