// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @title IWNative
/// @notice Interface for the IWNative contract.
interface IWNative is IERC20 {
    /// @notice Deposit Native Token to get wrapped Native Token
    /// @dev This function enables users to deposit Native Token and receive wrapped Native Token tokens in return.
    /// @dev The amount of Native Token to be deposited should be sent along with the function call.
    function deposit() external payable;

    /// @notice Withdraw wrapped Native Token to get Native Token
    /// @dev This function allows users to withdraw a specified amount of wrapped Native Token and receive Native Token in return.
    /// @param wad The amount of wrapped Native Token to be withdrawn.
    function withdraw(uint256 wad) external;
}
