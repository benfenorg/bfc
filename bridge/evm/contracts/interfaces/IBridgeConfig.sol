// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title IBridgeConfig
/// @dev Interface for the BridgeConfig contract.
interface IBridgeConfig {
    /* ========== STRUCTS ========== */

    /// @notice The data struct for the supported bridge tokens.
    struct Token {
        address tokenAddress;
        uint8 suiDecimal;
        bool native;
        uint8 originalDecimal;
    }


    struct BridgeFeeInfo {
        /// @dev 0: fixed fee, 1: dynamic fee
        /// 0 代表收取固定数据
        /// 1 按照百分比
        uint8 mode;
        //具体的值
        uint64 value;
        //上边计算收取的fee至少要超过这个值，否则就要用这个值
        uint64 minFeeValue;
    }

    /* ========== VIEW FUNCTIONS ========== */

    /// @notice Returns the address of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return address of the provided token.
    function tokenAddressOf(uint64 tokenID) external view returns (address);

    /// @notice Returns the sui decimal places of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return amount of sui decimal places of the provided token.
    function tokenSuiDecimalOf(uint64 tokenID) external view returns (uint8);


    /// @notice Returns the original decimal places of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return amount of original decimal places of the provided token.
    function tokenOriginalDecimalOf(uint64 tokenID) external view returns (uint8);

    /// @notice Returns the price of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return price of the provided token.
    function tokenPriceOf(uint64 tokenID) external view returns (uint64);

    /// @notice Returns the supported status of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return true if the token is supported, false otherwise.
    function isTokenSupported(uint64 tokenID) external view returns (bool);

    /// @notice Returns whether a chain is supported in SuiBridge with the given ID.
    /// @param chainId The ID of the chain.
    /// @return true if the chain is supported, false otherwise.
    function isChainSupported(uint8 chainId) external view returns (bool);

    /// @notice Returns the chain ID of the bridge.
    function chainID() external view returns (uint8);


   function investLpTokenIdOf(uint64 protocolType,uint64 underlyingTokenId) external view returns (uint64);
   
   function underlyingTokenIdOf(uint64 protocolType,uint64 lpTokenId) external view returns (uint64);

   function bridgeFeeInfoOf(uint64 tokenID) external view  returns (BridgeFeeInfo memory);

   function bridgeFeeModeOf(uint64 tokenID) external view returns (uint8);
   
   function bridgeFeeValueOf(uint64 tokenID) external view returns (uint64);
   
   function bridgeFeeMinFeeValueOf(uint64 tokenID) external view returns (uint64);

   function calculateBridgeFee(uint64 tokenID, uint256 amount) external view returns (uint256);


    /// @notice Event for the addition of a new token.
    /// @param nonce The governance action nonce.
    /// @param tokenIDs The IDs of the tokens added.
    /// @param tokenAddresses The addresses of the tokens added.
    /// @param suiDecimals The added token's decimal places on Sui.
    /// @param tokenPrices The prices of the tokens added in USD.
    event TokensAddedV2(
        uint64 nonce,
        uint64[] tokenIDs,
        address[] tokenAddresses,
        uint8[] suiDecimals,
        uint64[] tokenPrices
    );


/// @notice Event for the addition of a new token.
    /// @param nonce The governance action nonce.
    /// @param tokenIDs The IDs of the tokens added.
    /// @param tokenAddresses The addresses of the tokens added.
    /// @param suiDecimals The added token's decimal places on Sui.
    /// @param originalDecimals The added token's original decimal places.
    /// @param tokenPrices The prices of the tokens added in USD.
    event TokensAddedV3(
        uint64 nonce,
        uint64[] tokenIDs,
        address[] tokenAddresses,
        uint8[] suiDecimals,
        uint8[] originalDecimals,
        uint64[] tokenPrices
    );


    /// @dev (deprecated in favor of TokensAddedV2)
    event TokenAdded(uint64 tokenID, address tokenAddress, uint8 suiDecimal, uint64 tokenPrice);

    /// @notice Event for the price update of a token.
    /// @param nonce The governance action nonce.
    /// @param tokenID The ID of the token updated.
    /// @param tokenPrice The new price of the token in USD.
    event TokenPriceUpdatedV2(uint64 nonce, uint64 tokenID, uint64 tokenPrice);

    /// @dev (deprecated in favor of TokenPriceUpdatedV2)
    event TokenPriceUpdated(uint64 tokenID, uint64 tokenPrice);


    event LpTokenIdAdded(uint64 nonce,uint64 protocolType,uint64 underlyingTokenId,uint64 lpTokenId);

    event BridgeFeeUpdated(uint64 nonce, uint64 tokenID, uint8 mode, uint64 value, uint64 minFeeValue);
}
