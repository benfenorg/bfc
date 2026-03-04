// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "./utils/CommitteeUpgradeable.sol";
import "./interfaces/IBridgeConfig.sol";
import "@openzeppelin/contracts/utils/math/Math.sol";


/// @title BridgeConfig
/// @notice This contract manages a registry of supported tokens and supported chain IDs for the SuiBridge.
/// It also provides functions to convert token amounts to Sui decimal adjusted amounts and vice versa.
contract BridgeConfig is IBridgeConfig, CommitteeUpgradeable {
    /* ========== STATE VARIABLES ========== */

    uint8 public chainID;
    mapping(uint64 tokenID => Token) public supportedTokens;
    // price in USD (8 decimal precision) (e.g. 1 ETH = 2000 USD => 2000_00000000)
    mapping(uint64 tokenID => uint64 tokenPrice) public tokenPrices;
    mapping(uint8 chainId => bool isSupported) public supportedChains;

    //invest info
    //protocolType => tokenID => LPTokenID
    mapping(uint64 => mapping(uint64 => uint64)) public tokenToLP;
    //protocolType => LPTokenID => tokenID
    mapping(uint64 => mapping(uint64 => uint64)) public lpToToken;
    /* ========== INITIALIZER ========== */

    
    //store bridge fee
    mapping(uint64 tokenID => BridgeFeeInfo) public bridgeFeeInfos;


    /// @notice Constructor function for the BridgeConfig contract.
    /// @dev the provided arrays must have the same length.
    /// @param _committee The address of the BridgeCommittee contract.
    /// @param _chainID The ID of the chain this contract is deployed on.
    /// @param _supportedTokens The addresses of the supported tokens.
    /// @param _tokenPrices An array of token prices (with 8 decimal precision).
    /// @param _supportedChains array of supported chain IDs.
    function initialize(
        address _committee,
        uint8 _chainID,
        address[] memory _supportedTokens,
        uint64[] memory _tokenPrices,
        uint64[] memory _tokenIds,
        uint8[] memory _suiDecimals,
        uint8[] memory _supportedChains,
        uint64 _protocolType,
        uint64 _tokenID,
        uint64 _lpTokenId
    ) external initializer {
        __CommitteeUpgradeable_init(_committee);
        require(
            _supportedTokens.length == _tokenPrices.length, "BridgeConfig: Invalid token prices"
        );
        require(
            _supportedTokens.length == _tokenIds.length, "BridgeConfig: Invalid token IDs"
        );
        require(
            _supportedTokens.length == _suiDecimals.length, "BridgeConfig: Invalid Sui decimals"
        );

        for (uint8 i; i < _tokenIds.length; i++) {
            // `is_native` is hardcoded to `true` because we only support Eth native tokens
            // at the moment. This needs to change when we support tokens native on other chains.
            //todo 这里先默认原始decimal是6 后续再改
            
            uint8 originalDecimal = _suiDecimals[i];
            if(_tokenIds[i]==3 || _tokenIds[i]==4){
                originalDecimal=6;
            }

            supportedTokens[_tokenIds[i]] = Token(_supportedTokens[i], _suiDecimals[i], true, originalDecimal);
        }

        for (uint8 i; i < _supportedChains.length; i++) {
            require(_supportedChains[i] != _chainID, "BridgeConfig: Cannot support self");
            supportedChains[_supportedChains[i]] = true;
        }

        for (uint8 i; i < _tokenPrices.length; i++) {
            tokenPrices[_tokenIds[i]] = _tokenPrices[i];
        }

        tokenToLP[_protocolType][_tokenID] = _lpTokenId;
        lpToToken[_protocolType][_lpTokenId] = _tokenID;
        chainID = _chainID;
    }

    /* ========== VIEW FUNCTIONS ========== */

    /// @notice Returns the address of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return address of the provided token.
    function tokenAddressOf(uint64 tokenID) public view override returns (address) {
        return supportedTokens[tokenID].tokenAddress;
    }

    /// @notice Returns the sui decimal places of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return amount of sui decimal places of the provided token.
    function tokenSuiDecimalOf(uint64 tokenID) public view override returns (uint8) {
        return supportedTokens[tokenID].suiDecimal;
    }

    /// @notice Returns the original decimal places of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return amount of original decimal places of the provided token.
    function tokenOriginalDecimalOf(uint64 tokenID) public view override returns (uint8) {
        return supportedTokens[tokenID].originalDecimal;
    }

    /// @notice Returns the price of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return price of the provided token.
    function tokenPriceOf(uint64 tokenID) public view override returns (uint64) {
        return tokenPrices[tokenID];
    }

    function investLpTokenIdOf(uint64 protocolType,uint64 underlyingTokenId) public view override returns (uint64) {
        return tokenToLP[protocolType][underlyingTokenId];
    }

    function underlyingTokenIdOf(uint64 protocolType,uint64 lpTokenId) public view override returns (uint64) {
        return lpToToken[protocolType][lpTokenId];
    }

    /// @notice Returns the bridge fee info of the token with the given ID.
    /// @param tokenID The ID of the token.
    /// @return BridgeFeeInfo of the provided token.
    function bridgeFeeInfoOf(uint64 tokenID) public view override returns (BridgeFeeInfo memory) {
        return bridgeFeeInfos[tokenID];
    }

    
    function bridgeFeeModeOf(uint64 tokenID) public view returns (uint8) {
        return bridgeFeeInfos[tokenID].mode;
    }

    function bridgeFeeValueOf(uint64 tokenID) public view returns (uint64) {
        return bridgeFeeInfos[tokenID].value;
    }

    function bridgeFeeMinFeeValueOf(uint64 tokenID) public view returns (uint64) {
        return bridgeFeeInfos[tokenID].minFeeValue;
    }


    function calculateBridgeFee(uint64 tokenID, uint256 amount) public view returns (uint256) {
        BridgeFeeInfo memory feeInfo = bridgeFeeInfos[tokenID];
        uint256 fee;
        if (feeInfo.mode == 0) {
            // Fixed fee
            fee = feeInfo.value;
        } else {
            // Percentage fee
            // e.g. 1% = 10000; 0.01% = 100; 0.0001% = 1
            fee = Math.mulDiv(amount, feeInfo.value, 1000000);
        }

        if (fee < feeInfo.minFeeValue) {
            fee = feeInfo.minFeeValue;
        }
        return fee;
    }


    /// @notice Returns whether a token is supported in SuiBridge with the given ID.
    /// @param tokenID The ID of the token.
    /// @return true if the token is supported, false otherwise.
    function isTokenSupported(uint64 tokenID) public view override returns (bool) {
        return supportedTokens[tokenID].tokenAddress != address(0);
    }

    /// @notice Returns whether a chain is supported in SuiBridge with the given ID.
    /// @param chainId The ID of the chain.
    /// @return true if the chain is supported, false otherwise.
    function isChainSupported(uint8 chainId) public view override returns (bool) {
        return supportedChains[chainId];
    }

    /* ========== MUTATIVE FUNCTIONS ========== */

    // 本次升级要初始化的
    function settingOriginalDecimal(uint8 decimal) external{
        supportedTokens[2].originalDecimal = 8;
        supportedTokens[3].originalDecimal = 6;
        supportedTokens[4].originalDecimal = 6;
        supportedTokens[6].originalDecimal = 8;
    }

    /// @notice Adds a LP token ID with the provided message if the provided signatures are valid.
    /// @param signatures array of signatures to validate the message.
    /// @param message BridgeMessage containing the add LP token payload.
    /// @dev The function will revert if the payload length is invalid.
    ///     Add LP token payload is 24 bytes.
    ///     bytes 0-7    : protocol type (uint64)
    ///     bytes 8-15   : token ID (uint64)
    ///     bytes 16-23  : LP token ID (uint64)
    /// @dev The function will revert if the token is already added.
    function addLpTokenIdWithSignatures(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    )
        external 
        nonReentrant  
        verifyMessageAndSignatures(message, signatures, BridgeUtils.ADD_LP_TOKEN_ID)
    {
        (uint64 protocolType, uint64 tokenID, uint64 lpTokenId) = BridgeUtils.decodeAddLpTokenPayload(message.payload);
        require(tokenToLP[protocolType][tokenID] == 0, "BridgeConfig: LPToken already added");
        tokenToLP[protocolType][tokenID] = lpTokenId;
        lpToToken[protocolType][lpTokenId] = tokenID;

        emit LpTokenIdAdded(message.nonce,protocolType,tokenID,lpTokenId);
    }

    /// @notice Updates the token price with the provided message if the provided signatures are valid.
    /// @param signatures array of signatures to validate the message.
    /// @param message BridgeMessage containing the update token price payload.
    function updateTokenPriceWithSignatures(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    )
        external
        nonReentrant
        verifyMessageAndSignatures(message, signatures, BridgeUtils.UPDATE_TOKEN_PRICE)
    {
        // decode the update token payload
        (uint64 tokenID, uint64 price) = BridgeUtils.decodeUpdateTokenPricePayload(message.payload);

        _updateTokenPrice(tokenID, price);

        emit TokenPriceUpdatedV2(message.nonce, tokenID, price);
    }

    /// @notice Updates the bridge fee info with the provided message if the provided signatures are valid.
    /// @param signatures array of signatures to validate the message.
    /// @param message BridgeMessage containing the update bridge fee payload.
    function updateBridgeFeeWithSignatures(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    )
        external
        nonReentrant
        verifyMessageAndSignatures(message, signatures, BridgeUtils.UPDATE_BRIDGE_FEE)
    {
        (uint8 sourceChainID,uint64 tokenID, uint8 mode, uint64 value, uint64 minFeeValue) = BridgeUtils.decodeUpdateBridgeFeePayload(message.payload);
        require(
            isChainSupported(sourceChainID),
            "BridgeConfig: Source chain not supported"
        ); 
        bridgeFeeInfos[tokenID] = BridgeFeeInfo(mode, value, minFeeValue);

        emit BridgeFeeUpdated(message.nonce, tokenID, mode, value, minFeeValue);
    }

    function addTokensWithSignatures(bytes[] memory signatures, BridgeUtils.Message memory message)
        external
        nonReentrant
        verifyMessageAndSignatures(message, signatures, BridgeUtils.ADD_EVM_TOKENS)
    {
        // decode the update token payload
        (
            bool native,
            uint64[] memory tokenIDs,
            address[] memory tokenAddresses,
            uint8[] memory suiDecimals,
            uint8[] memory originalDecimals,
            uint64[] memory _tokenPrices
        ) = BridgeUtils.decodeAddTokensPayload(message.payload);

        // update the token
        for (uint8 i; i < tokenIDs.length; i++) {
            _addToken(tokenIDs[i], tokenAddresses[i], suiDecimals[i], originalDecimals[i],_tokenPrices[i], native);
        }

        emit TokensAddedV3(message.nonce, tokenIDs, tokenAddresses, suiDecimals, originalDecimals, _tokenPrices);
    }

    /* ========== PRIVATE FUNCTIONS ========== */

    /// @notice Updates the price of the token with the provided ID.
    /// @param tokenID The ID of the token to update.
    /// @param tokenPrice The price of the token.
    function _updateTokenPrice(uint64 tokenID, uint64 tokenPrice) private {
        require(isTokenSupported(tokenID), "BridgeConfig: Unsupported token");
        require(tokenPrice > 0, "BridgeConfig: Invalid token price");

        tokenPrices[tokenID] = tokenPrice;
    }

    /// @notice Updates the token with the provided ID.
    /// @param tokenID The ID of the token to update.
    /// @param tokenAddress The address of the token.
    /// @param suiDecimal The decimal places of the token.
    /// @param tokenPrice The price of the token.
    /// @param native Whether the token is native to the chain.
    function _addToken(
        uint64 tokenID,
        address tokenAddress,
        uint8 suiDecimal,
        uint8 originalDecimal,
        uint64 tokenPrice,
        bool native
    ) private {
        require(tokenAddress != address(0), "BridgeConfig: Invalid token address");
        require(suiDecimal > 0, "BridgeConfig: Invalid Sui decimal");
        require(tokenPrice > 0, "BridgeConfig: Invalid token price");
        
        //todo 这里先默认原始decimal是6 后续再改
        supportedTokens[tokenID] = Token(tokenAddress, suiDecimal, native, originalDecimal);
        tokenPrices[tokenID] = tokenPrice;
    }

    /* ========== MODIFIERS ========== */

    /// @notice Requires the given token to be supported.
    /// @param tokenID The ID of the token to check.
    modifier tokenSupported(uint64 tokenID) {
        require(isTokenSupported(tokenID), "BridgeConfig: Unsupported token");
        _;
    }
}
