// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @title IWBNB
/// @notice Interface for the WBNB contract.
interface IWBNB is IERC20 {
    /// @notice Deposit BNB to get wrapped BNB
    /// @dev This function enables users to deposit BNB and receive wrapped BNB tokens in return.
    /// @dev The amount of BNB to be deposited should be sent along with the function call.
    function deposit() external payable;

    /// @notice Withdraw wrapped BNB to get BNB
    /// @dev This function allows users to withdraw a specified amount of wrapped BNB and receive BNB in return.
    /// @param wad The amount of wrapped BNB to be withdrawn.
    function withdraw(uint256 wad) external;
}
