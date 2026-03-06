// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/interfaces/IERC20Metadata.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "./utils/CommitteeUpgradeable.sol";
import "./interfaces/ISuiBridge.sol";
import "./interfaces/IBridgeVault.sol";
import "./interfaces/IBridgeLimiter.sol";
import "./interfaces/IBridgeConfig.sol";
import {ArrowLib} from "./utils/ArrowLib.sol";
import {BridgeLib} from "./utils/BridgeLib.sol";

/// @title SuiBridge
/// @notice This contract implements a token bridge that enables users to deposit and withdraw
/// supported tokens to and from other chains. The bridge supports the transfer of Ethereum and ERC20
/// tokens. Bridge operations are managed by a committee of Sui validators that are responsible
/// for verifying and processing bridge messages. The bridge is designed to be upgradeable and
/// can be paused in case of an emergency. The bridge also enforces limits on the amount of
/// assets that can be withdrawn to prevent abuse.
contract SuiBridge is ISuiBridge, CommitteeUpgradeable, PausableUpgradeable {
    /* ========== STATE VARIABLES ========== */

    mapping(uint64 nonce => bool isProcessed) public isTransferProcessed;
    IBridgeVault public vault;
    IBridgeLimiter public limiter;

    uint8 constant SUI_ADDRESS_LENGTH = 32; 

    mapping (uint64 nonce => bool isProcessed) public isInvestProcessed;

    address investAddress;

    /* ========== INITIALIZER ========== */

    /// @notice Initializes the SuiBridge contract with the provided parameters.
    /// @dev this function should be called directly after deployment (see OpenZeppelin upgradeable standards).
    /// @param _committee The address of the committee contract.
    /// @param _vault The address of the bridge vault contract.
    /// @param _limiter The address of the bridge limiter contract.
    function initialize(address _committee, address _vault, address _limiter,address _investAddress)
        external
        initializer
    {
        __CommitteeUpgradeable_init(_committee);
        __Pausable_init();
        vault = IBridgeVault(_vault);
        limiter = IBridgeLimiter(_limiter);
        investAddress = _investAddress;
    }

    /* ========== EXTERNAL FUNCTIONS ========== */

    /// @notice Allows the caller to provide signatures that enable the transfer of tokens to
    /// the recipient address indicated within the message payload.
    /// @dev `message.chainID` represents the sending chain ID. Receiving chain ID needs to match
    /// this bridge's chain ID (this chain).
    /// @param signatures The array of signatures.
    /// @param message The BridgeUtils containing the transfer details.
    function transferBridgedTokensWithSignatures(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    )
        external
        nonReentrant
        verifyMessageAndSignatures(message, signatures, BridgeUtils.TOKEN_TRANSFER)
        onlySupportedChain(message.chainID)
    {
        require(message.messageType == BridgeUtils.TOKEN_TRANSFER,"SuiBridge: Invalid message type");
        // verify that message has not been processed
        require(!isTransferProcessed[message.nonce], "SuiBridge: Message already processed");

        IBridgeConfig config = committee.config();

        
        if (message.version==1){
            BridgeUtils.TokenTransferPayload memory tokenTransferPayload =
            BridgeUtils.decodeTokenTransferPayload(message.payload);

            // verify target chain ID is this chain ID
            require(
                tokenTransferPayload.targetChain == config.chainID(), "SuiBridge: Invalid target chain"
            );

            // convert amount to ERC20 token decimals
            uint256 erc20AdjustedAmount = BridgeUtils.convertSuiToERC20Decimal(
                IERC20Metadata(config.tokenAddressOf(tokenTransferPayload.tokenID)).decimals(),
                config.tokenSuiDecimalOf(tokenTransferPayload.tokenID),
                tokenTransferPayload.amount
            );

            // mark message as processed
            isTransferProcessed[message.nonce] = true;

            _transferTokensFromVault(
                message.chainID,
                tokenTransferPayload.tokenID,
                tokenTransferPayload.recipientAddress,
                erc20AdjustedAmount
            );


            emit TokensClaimed(
                message.chainID,
                message.nonce,
                config.chainID(),
                tokenTransferPayload.tokenID,
                erc20AdjustedAmount,
                tokenTransferPayload.senderAddress,
                tokenTransferPayload.recipientAddress
            );
        }else if (message.version==3) {
             BridgeUtils.TokenTransferPayloadV2 memory tokenTransferPayload =
            BridgeUtils.decodeTokenTransferPayloadV2(message.payload);

            // verify target chain ID is this chain ID
            require(
                tokenTransferPayload.targetChain == config.chainID(), "SuiBridge: Invalid target chain"
            );

            // convert amount to ERC20 token decimals
            uint256 erc20AdjustedAmount = BridgeUtils.convertSuiToERC20Decimal(
                IERC20Metadata(config.tokenAddressOf(tokenTransferPayload.tokenID)).decimals(),
                config.tokenSuiDecimalOf(tokenTransferPayload.tokenID),
                tokenTransferPayload.amount
            );

            // mark message as processed
            isTransferProcessed[message.nonce] = true;

            _transferTokensFromVault(
                message.chainID,
                tokenTransferPayload.tokenID,
                tokenTransferPayload.recipientAddress,
                erc20AdjustedAmount
            );


            emit TokensClaimed(
                message.chainID,
                message.nonce,
                config.chainID(),
                tokenTransferPayload.tokenID,
                erc20AdjustedAmount,
                tokenTransferPayload.senderAddress,
                tokenTransferPayload.recipientAddress
            );
        }else{

            BridgeUtils.TokenTransferPayloadV2 memory tokenTransferPayload =
            BridgeUtils.decodeTokenTransferPayloadV2(message.payload);

            // verify target chain ID is this chain ID
            require(
                tokenTransferPayload.targetChain == config.chainID(), "SuiBridge: Invalid target chain"
            );

            // convert amount to ERC20 token decimals
            uint256 erc20AdjustedAmount = BridgeUtils.convertSuiToERC20Decimal(
                IERC20Metadata(config.tokenAddressOf(tokenTransferPayload.tokenID)).decimals(),
                config.tokenOriginalDecimalOf(tokenTransferPayload.tokenID),
                tokenTransferPayload.amount
            );

            // mark message as processed
            isTransferProcessed[message.nonce] = true;

            _transferTokensFromVault(
                message.chainID,
                tokenTransferPayload.tokenID,
                tokenTransferPayload.recipientAddress,
                erc20AdjustedAmount
            );


            emit TokensClaimed(
                message.chainID,
                message.nonce,
                config.chainID(),
                tokenTransferPayload.tokenID,
                erc20AdjustedAmount,
                tokenTransferPayload.senderAddress,
                tokenTransferPayload.recipientAddress
            );
        }
    }

    function investBridgedTokensWithSignatures(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    )  external
       nonReentrant
       verifyMessageAndSignatures(message, signatures, BridgeUtils.DEFI)
       onlySupportedChain(message.chainID)
    {
        require(message.messageType == BridgeUtils.DEFI,"SuiBridge: Invalid message type");
        require(!isInvestProcessed[message.nonce], "SuiBridge: Message already processed");

        IBridgeConfig config = committee.config();

        BridgeUtils.DefiTransferPayload memory defiTransferPayload =
            BridgeUtils.decodeDefiTransferPayload(message.payload);

        require(
            defiTransferPayload.targetChain == config.chainID(), "SuiBridge: Invalid target chain"
        );
        require(investAddress != address(0), "SuiBridge: invest address not set");   

        if (defiTransferPayload.actionType==0) {
             defiStake(message,defiTransferPayload,config);
        }else if (defiTransferPayload.actionType==1){
             defiUnStake(message,defiTransferPayload,config);
        }else{
            revert("SuiBridge: Invalid actionType");
        }    
    }

    function defiStake(
        BridgeUtils.Message memory message,
        BridgeUtils.DefiTransferPayload memory defiTransferPayload,
        IBridgeConfig config
    )internal {
        uint64 lpTokenId = config.investLpTokenIdOf(defiTransferPayload.protocolType,defiTransferPayload.protocolTokenID);
        address lpTokenAddress = config.tokenAddressOf(lpTokenId);
        // convert token amount(USDT/USDC) to ERC20 token decimals
        uint256 erc20AdjustedAmount = BridgeUtils.convertSuiToERC20Decimal(
            IERC20Metadata(config.tokenAddressOf(defiTransferPayload.protocolTokenID)).decimals(),
            config.tokenSuiDecimalOf(defiTransferPayload.protocolTokenID),
            defiTransferPayload.amount
        );

        _transferTokensFromVault(
            message.chainID,
            defiTransferPayload.protocolTokenID,
            address(this), 
            erc20AdjustedAmount
        );
        //deposit
        uint256 beforeLpTokenAmount=IERC20(lpTokenAddress).balanceOf(address(vault));
        ArrowLib.deposit(
            investAddress,
            defiTransferPayload.protocolType,
            config.tokenAddressOf(defiTransferPayload.protocolTokenID),
            erc20AdjustedAmount
        );
        uint256 afterLpTokenAmount=IERC20(lpTokenAddress).balanceOf(address(vault));

        require(afterLpTokenAmount>beforeLpTokenAmount,"SuiBridge: Invalid stake amount");

        uint256 lpAmount=afterLpTokenAmount-beforeLpTokenAmount;
        //LPToken Amount to Benfen Decimal
        uint64 suiLpTokenAmount = BridgeUtils.convertERC20ToSuiDecimal(
            IERC20Metadata(lpTokenAddress).decimals(),
            config.tokenSuiDecimalOf(lpTokenId),
            lpAmount
        );

        // mark message as processed
        isInvestProcessed[message.nonce] = true;

        emit TokensStaked(
            config.chainID(),
            nonces[BridgeUtils.DEFI],
            message.chainID,
            message.nonce,
            defiTransferPayload.senderAddress,
            investAddress,
            defiTransferPayload.amount,
            suiLpTokenAmount,
            defiTransferPayload.protocolType,
            defiTransferPayload.protocolVersion,
            defiTransferPayload.protocolTokenID,
            defiTransferPayload.principalAmount,
            defiTransferPayload.actionType
        );
        nonces[BridgeUtils.DEFI]++;
    }


    function defiUnStake(
        BridgeUtils.Message memory message,
        BridgeUtils.DefiTransferPayload memory defiTransferPayload,
        IBridgeConfig config
    )internal {
        uint64 tokenId = defiTransferPayload.protocolTokenID;
        uint64 lpTokenId = config.investLpTokenIdOf(defiTransferPayload.protocolType,tokenId);
        // convert token amount(LPToken) to ERC20 token decimals
        uint256 erc20AdjustedAmount = BridgeUtils.convertSuiToERC20Decimal(
            IERC20Metadata(config.tokenAddressOf(lpTokenId)).decimals(),
            config.tokenSuiDecimalOf(lpTokenId),
            defiTransferPayload.amount
        );

        _transferTokensFromVault(
            message.chainID,
            lpTokenId,
            address(this), 
            erc20AdjustedAmount
        );
        // address lpTokenAddress = config.tokenAddressOf(lpTokenId);
        address tokenAddress = config.tokenAddressOf(tokenId);
        uint256 beforeTokenAmount=IERC20(tokenAddress).balanceOf(address(vault));
        //withdraw
        ArrowLib.withdraw(
            investAddress,
            defiTransferPayload.protocolType,
            config.tokenAddressOf(lpTokenId), //lp token
            erc20AdjustedAmount
        );
        uint256 afterTokenAmount=IERC20(tokenAddress).balanceOf(address(vault));

        require(afterTokenAmount>beforeTokenAmount,"SuiBridge: Invalid unstake amount");

        uint256 tokenAmount=afterTokenAmount-beforeTokenAmount;

        //Token Amount(USDT/USDC) to Benfen Decimal
        uint64 suiTokenAmount = BridgeUtils.convertERC20ToSuiDecimal(
            IERC20Metadata(tokenAddress).decimals(),
            config.tokenSuiDecimalOf(tokenId),
            tokenAmount
        );

        // mark message as processed
        isInvestProcessed[message.nonce] = true;
            
        emit TokensUnStaked(
            config.chainID(),
            nonces[BridgeUtils.DEFI],
            message.chainID,
            message.nonce,
            defiTransferPayload.senderAddress,
            investAddress, 
            suiTokenAmount, // aave redeem token (usdc/usdt)
            defiTransferPayload.amount, //lp token
            defiTransferPayload.protocolType,
            defiTransferPayload.protocolVersion,
            defiTransferPayload.protocolTokenID,
            defiTransferPayload.principalAmount,
            defiTransferPayload.actionType
        );
        nonces[BridgeUtils.DEFI]++;
    }


    function updateInvestAddressWithSignatures(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    ) external
      nonReentrant
      verifyMessageAndSignatures(message, signatures, BridgeUtils.UPDATE_INVEST_ADDRESS)
    {
        address _investAddress = BridgeUtils.decodeInvestAddressPayload(message.payload);
        investAddress = _investAddress;

        emit UpdateInvestAddress(message.nonce, _investAddress);
    }


    function getInvestAddress() external view returns (address) {
        return investAddress;
    }


    /// @notice Executes an emergency operation with the provided signatures and message.
    /// @dev If the given operation is to freeze and the bridge is already frozen, the operation
    /// will revert.
    /// @param signatures The array of signatures to verify.
    /// @param message The BridgeUtils containing the details of the operation.
    function executeEmergencyOpWithSignatures(
        bytes[] memory signatures,
        BridgeUtils.Message memory message
    )
        external
        nonReentrant
        verifyMessageAndSignatures(message, signatures, BridgeUtils.EMERGENCY_OP)
    {
        // decode the emergency op message
        bool isFreezing = BridgeUtils.decodeEmergencyOpPayload(message.payload);

        if (isFreezing) _pause();
        else _unpause();

        emit EmergencyOperation(message.nonce, isFreezing);
    }


    /// @notice Enables the caller to deposit supported tokens to be bridged to a given
    /// destination chain.
    /// @dev The provided tokenID and destinationChainID must be supported. The caller must
    /// have approved this contract to transfer the given token.
    /// @param tokenID The ID of the token to be bridged.
    /// @param amount The amount of tokens to be bridged.
    /// @param recipientAddress The address on the Sui chain where the tokens will be sent.
    /// @param destinationChainID The ID of the destination chain.
    function bridgeERC20(
        uint64 tokenID,
        uint256 amount,
        bytes memory recipientAddress,
        uint8 destinationChainID
    ) external whenNotPaused nonReentrant onlySupportedChain(destinationChainID) {
        IBridgeConfig config = committee.config();
        uint8 decimal = config.tokenSuiDecimalOf(tokenID);
        
        BridgeLib.BridgeERC20Args memory args = BridgeLib.BridgeERC20Args({
            tokenID: tokenID,
            amount: amount,
            recipientAddress: recipientAddress,
            destinationChainID: destinationChainID,
            targetTokenID: 5, // busd
            decimal: decimal,
            nonce: nonces[BridgeUtils.TOKEN_TRANSFER],
            config: config,
            limiter: limiter,
            vault: vault
        });
        
        BridgeLib.bridgeERC20Common(args);
        
        // increment token transfer nonce
        nonces[BridgeUtils.TOKEN_TRANSFER]++;
    }

    /// @notice Enables the caller to deposit supported tokens to be bridged to a given
    /// destination chain.
    /// @dev The provided tokenID and destinationChainID must be supported. The caller must
    /// have approved this contract to transfer the given token.
    /// @param tokenID The ID of the token to be bridged.
    /// @param amount The amount of tokens to be bridged.
    /// @param recipientAddress The address on the Sui chain where the tokens will be sent.
    /// @param destinationChainID The ID of the destination chain.
    /// @param targetTokenID The ID of the target token.
    function bridgeERC20WithTargetTokenID(
        uint64 tokenID,
        uint256 amount,
        bytes memory recipientAddress,
        uint8 destinationChainID,
        uint64 targetTokenID
    ) external whenNotPaused nonReentrant onlySupportedChain(destinationChainID) {
        IBridgeConfig config = committee.config();
        uint8 decimal = config.tokenSuiDecimalOf(tokenID);
        if (tokenID == 3 || tokenID == 4) {
            // USDT/USDC only support cross to SUI
            require(
                targetTokenID == tokenID || targetTokenID == 5,
                "SuiBridge: Invalid target token ID"
            );
            if (targetTokenID == tokenID) {
                decimal = config.tokenOriginalDecimalOf(tokenID);
            }
        } else {
            require(tokenID == targetTokenID, "SuiBridge: Invalid target token ID");
        }
        
        BridgeLib.BridgeERC20Args memory args = BridgeLib.BridgeERC20Args({
            tokenID: tokenID,
            amount: amount,
            recipientAddress: recipientAddress,
            destinationChainID: destinationChainID,
            targetTokenID: targetTokenID,
            decimal: decimal,
            nonce: nonces[BridgeUtils.TOKEN_TRANSFER],
            config: config,
            limiter: limiter,
            vault: vault
        });
        
        BridgeLib.bridgeERC20Common(args);
        
        // increment token transfer nonce
        nonces[BridgeUtils.TOKEN_TRANSFER]++;
    }

    function _getNativeTokenID(uint8 chainID) private pure returns (uint64) {
        if (chainID == 30 || chainID == 31 || chainID == 32) {
            return BridgeUtils.BNB;
        } else if (chainID == 39 || chainID == 40 || chainID == 41) {
            return BridgeUtils.POL;
        } else if (chainID == 45 || chainID == 46 || chainID == 47) {
            return BridgeUtils.AVAX;
        } else {
            return BridgeUtils.ETH;
        }
    }

    /// @notice Enables the caller to deposit Eth to be bridged to a given destination chain.
    /// @dev The provided destinationChainID must be supported.
    /// @param recipientAddress The address on the destination chain where Eth will be sent.
    /// @param destinationChainID The ID of the destination chain.
    function bridgeETH(bytes memory recipientAddress, uint8 destinationChainID)
        external
        payable
        whenNotPaused
        nonReentrant
        onlySupportedChain(destinationChainID)
    {
        require(
            recipientAddress.length == SUI_ADDRESS_LENGTH,
            "SuiBridge: Invalid recipient address length"
        );

        uint256 amount = msg.value;

        // Transfer the unwrapped ETH to the target address
        (bool success,) = payable(address(vault)).call{value: amount}("");
        require(success, "SuiBridge: Failed to transfer ETH to vault");

        // Adjust the amount to emit.
        IBridgeConfig config = committee.config();
        uint8 chainid = config.chainID();
        
        uint64 tokenID = _getNativeTokenID(chainid);

        // Adjust the amount
        require(limiter.calculateAmountInUSD(tokenID, amount) < limiter.getUsdMaxLimit(), "SuiBridge: USD Exceed Limit");
        require(limiter.calculateAmountInUSD(tokenID, amount) > limiter.getUsdMinLimit(), "SuiBridge: USD Less Than Min Limit");
        
        // Calculate bridge fee
        IBridgeConfig.BridgeFeeInfo memory feeInfo = config.bridgeFeeInfoOf(tokenID);
        uint256 fee = BridgeLib.calculateBridgeFee(feeInfo, amount);
        require(amount > fee, "SuiBridge: Insufficient amount for fee");
        
        uint64 suiAdjustedAmount = BridgeUtils.convertERC20ToSuiDecimal(
            IERC20Metadata(config.tokenAddressOf(tokenID)).decimals(),
            config.tokenSuiDecimalOf(tokenID),
            amount
        );
        emit TokensDeposited(
            config.chainID(),
            nonces[BridgeUtils.TOKEN_TRANSFER],
            destinationChainID,
            tokenID,
            tokenID,
            suiAdjustedAmount,
            msg.sender,
            recipientAddress
        );

        // increment token transfer nonce
        nonces[BridgeUtils.TOKEN_TRANSFER]++;
    }

    /* ========== INTERNAL FUNCTIONS ========== */

    /// @dev Transfers tokens from the vault to a target address.
    /// @param sendingChainID The ID of the chain from which the tokens are being transferred.
    /// @param tokenID The ID of the token being transferred.
    /// @param recipientAddress The address to which the tokens are being transferred.
    /// @param amount The amount of tokens being transferred.
    function _transferTokensFromVault(
        uint8 sendingChainID,
        uint64 tokenID,
        address recipientAddress,
        uint256 amount
    ) private whenNotPaused limitNotExceeded(sendingChainID, tokenID, amount) {
        address tokenAddress = committee.config().tokenAddressOf(tokenID);

        // Check that the token address is supported
        require(tokenAddress != address(0), "SuiBridge: Unsupported token");
        uint8 chainid = committee.config().chainID();
        
        uint64 nativeTokenID = _getNativeTokenID(chainid);

        if (tokenID == nativeTokenID) {
            vault.transferETH(payable(recipientAddress), amount);
        } else {
            // transfer tokens from vault to target address
            vault.transferERC20(tokenAddress, recipientAddress, amount);
        }

        // update amount bridged
        limiter.recordBridgeTransfers(sendingChainID, tokenID, amount);
    }

    /* ========== MODIFIERS ========== */

    /// @dev Requires the amount being transferred does not exceed the bridge limit in
    /// the last 24 hours.
    /// @param tokenID The ID of the token being transferred.
    /// @param amount The amount of tokens being transferred.
    modifier limitNotExceeded(uint8 chainID, uint64 tokenID, uint256 amount) {
        _checkLimitNotExceeded(chainID, tokenID, amount);
        _;
    }

    function _checkLimitNotExceeded(uint8 chainID, uint64 tokenID, uint256 amount) internal view {
        require(
            !limiter.willAmountExceedLimit(chainID, tokenID, amount),
            "SuiBridge: Amount exceeds bridge limit"
        );
    }

    /// @dev Requires the target chain ID is supported.
    /// @param targetChainID The ID of the target chain.
    modifier onlySupportedChain(uint8 targetChainID) {
        _checkSupportedChain(targetChainID);
        _;
    }

    function _checkSupportedChain(uint8 targetChainID) internal view {
        require(
            committee.config().isChainSupported(targetChainID),
            "SuiBridge: Target chain not supported"
        );
    }
}
