// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/interfaces/IERC20Metadata.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
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

    function bridgeERC20Common(BridgeERC20Args memory args) public {
        require(
            args.recipientAddress.length == 32, // SUI_ADDRESS_LENGTH
            "SuiBridge: Invalid recipient address length"
        );
        require(args.config.isTokenSupported(args.tokenID), "SuiBridge: Unsupported token");

        address tokenAddress = args.config.tokenAddressOf(args.tokenID);

        // check that the bridge contract has allowance to transfer the tokens
        require(
            IERC20(tokenAddress).allowance(msg.sender, address(this)) >= args.amount,
            "SuiBridge: Insufficient allowance"
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
