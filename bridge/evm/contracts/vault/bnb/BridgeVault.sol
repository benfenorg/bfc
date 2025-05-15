// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./../../interfaces/vault/bnb/IBridgeVault.sol";
import "./../../interfaces/IWNative.sol";
/// @title BridgeVault
/// @notice A contract that acts as a vault for transferring ERC20 tokens and BNB. It enables the owner
/// (intended to be the SuiBridge contract) to transfer tokens to a target address. It also supports
/// unwrapping WBNB (Wrapped Ether) and transferring the unwrapped BNB.
/// @dev The contract is initialized with the deployer as the owner. The ownership is intended to be
/// transferred to the SuiBridge contract after the bridge contract is deployed.
contract BridgeVault is Ownable, IBridgeVault, ReentrancyGuard {
    /* ========== STATE VARIABLES ========== */

    IWNative public immutable wNative;

    /* ========== CONSTRUCTOR ========== */

    /// @notice Constructor function for the BridgeVault contract.
    /// @param _wNative The address of the Wrapped Native Token (_wNative) contract.
    constructor(address _wNative) Ownable(msg.sender) ReentrancyGuard() {
        // Set the WBNB address
        wNative = IWNative(_wNative);
    }

    /// @notice Transfers ERC20 tokens from the contract to a target address. Only the owner of
    /// the contract can call this function.
    /// @dev This function is intended to only be called by the SuiBridge contract.
    /// @param tokenAddress The address of the ERC20 token.
    /// @param recipientAddress The address to transfer the tokens to.
    /// @param amount The amount of tokens to transfer.
    function transferERC20(address tokenAddress, address recipientAddress, uint256 amount)
        external
        override
        onlyOwner
        nonReentrant
    {
        // Transfer the tokens from the contract to the target address
        SafeERC20.safeTransfer(IERC20(tokenAddress), recipientAddress, amount);
    }

    /// @notice Unwraps stored wrapped BNB and transfers the newly withdrawn BNB to the provided target
    /// address. Only the owner of the contract can call this function.
    /// @dev This function is intended to only be called by the SuiBridge contract.
    /// @param recipientAddress The address to transfer the BNB to.
    /// @param amount The amount of BNB to transfer.
    function transferNativeToken(address payable recipientAddress, uint256 amount)
        external
        override
        onlyOwner
        nonReentrant
    {
        // Unwrap the WBNB
        wNative.withdraw(amount);

        // Transfer the unwrapped BNB to the target address
        (bool success,) = recipientAddress.call{value: amount}("");
        require(success, "BNB transfer failed");
    }

    /// @notice Wraps as eth sent to this contract.
    /// @dev skip if sender is wBNB contract to avoid infinite loop.
    receive() external payable {
        if (msg.sender != address(wNative)) {
            wNative.deposit{value: msg.value}();
        }
    }
}
