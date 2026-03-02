// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/interfaces/IERC20Metadata.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/math/Math.sol";
import "../interfaces/ISuiBridge.sol";
import "../interfaces/IBridgeVault.sol";
import "../interfaces/IBridgeLimiter.sol";
import "../interfaces/IBridgeConfig.sol";
import "./BridgeUtils.sol";


library BridgeLib {
    struct BridgeERC20Args {
        uint64 tokenID;
        uint256 amount;
        bytes recipientAddress;
        uint8 destinationChainID;
        uint64 targetTokenID;
        uint8 decimal;
        uint64 nonce;
        IBridgeConfig config;
        IBridgeLimiter limiter;
        IBridgeVault vault;
    }

    event TokensDeposited(
        uint8 indexed sourceChainID,
        uint64 indexed nonce,
        uint8 indexed destinationChainID,
        uint64 tokenID,
        uint64 targetTokenID,
        uint64 suiAdjustedAmount,
        address senderAddress,
        bytes recipientAddress
    );
   
    function calculateBridgeFee(IBridgeConfig.BridgeFeeInfo memory feeInfo, uint256 amount) internal pure returns (uint256) {
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

    function bridgeERC20Common(BridgeERC20Args memory args) public {
        require(
            args.recipientAddress.length == 32, // SUI_ADDRESS_LENGTH
            "SuiBridge: Invalid recipient address length"
        );
        require(args.config.isTokenSupported(args.tokenID), "SuiBridge: Unsupported token");

        // get bridge fee
        IBridgeConfig.BridgeFeeInfo memory feeInfo = args.config.bridgeFeeInfoOf(args.tokenID);

        address tokenAddress = args.config.tokenAddressOf(args.tokenID);

        // check that the bridge contract has allowance to transfer the tokens
        require(
            IERC20(tokenAddress).allowance(msg.sender, address(this)) >= args.amount,
            "SuiBridge: Insufficient allowance"
        );

        require(
            args.limiter.calculateAmountInUSD(args.tokenID, args.amount) > args.limiter.getUsdMinLimit(),
            "SuiBridge: USD Less Than Min Limit"
        );

        require(
            args.limiter.calculateAmountInUSD(args.tokenID, args.amount) < args.limiter.getUsdMaxLimit(),
            "SuiBridge: USD Exceed Limit"
        );

        // calculate old vault balance
        uint256 oldBalance = IERC20(tokenAddress).balanceOf(address(args.vault));

        // Transfer the tokens from the contract to the vault
        SafeERC20.safeTransferFrom(IERC20(tokenAddress), msg.sender, address(args.vault), args.amount);

        // calculate new vault balance
        uint256 newBalance = IERC20(tokenAddress).balanceOf(address(args.vault));

        // calculate the amount transferred
        uint256 amountTransfered = newBalance - oldBalance;

        // Calculate bridge fee
        uint256 fee = calculateBridgeFee(feeInfo, amountTransfered);

        require(amountTransfered > fee, "SuiBridge: Insufficient amount for fee");
        

        // Adjust the amount
        uint64 suiAdjustedAmount = BridgeUtils.convertERC20ToSuiDecimal(
            IERC20Metadata(tokenAddress).decimals(),
            args.decimal,
            amountTransfered
        );

        emit TokensDeposited(
            args.config.chainID(),
            args.nonce,
            args.destinationChainID,
            args.tokenID,
            args.targetTokenID,
            suiAdjustedAmount,
            msg.sender,
            args.recipientAddress
        );
    }
}
