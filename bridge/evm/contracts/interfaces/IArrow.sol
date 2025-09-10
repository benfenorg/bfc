// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

interface IArrow {

    /// @notice Deposit assets into a specific protocol
    /// @param protocol_type Protocol type (0 = Aave, 1 = Compound, 2 = MakerDAO)
    /// @param asset Address of the underlying asset (e.g., USDC, DAI)
    /// @param amount Amount of assets to deposit
    function deposit(
        uint64 protocol_type,
        address asset,
        uint256 amount
    ) external payable;

    /// @notice Withdraw assets from a specific protocol
    /// @param protocol_type Protocol type
    /// @param asset Address of the underlying asset (LP token)
    /// @param amount Amount of assets to withdraw
    function withdraw(
        uint64 protocol_type,
        address asset,
        uint256 amount
    ) external;

}
