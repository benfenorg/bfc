// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

interface IArrow {
    /// @notice Deposit assets into a aave protocol
    /// @param protocol_type Protocol type
    /// @param asset Address of the underlying asset (e.g., USDC, DAI)
    /// @param amount Amount of assets to deposit
    function depositViaAave(
        uint16 protocol_type,
        address asset,
        uint256 amount
    ) external payable;

    /// @notice Withdraw assets from a aave protocol
    /// @param protocol_type Protocol type
    /// @param lpToken Address of the underlying lp lpToken (e.g., lp usdc)
    /// @param amount Amount of assets to withdraw
    function withdrawViaAave(
        uint16 protocol_type,
        address lpToken,
        uint256 amount
    ) external;


    /// @notice Deposit assets into a compound protocol
    /// @param protocol_type Protocol type
    /// @param asset Address of the underlying asset (e.g., USDC, DAI)
    /// @param amount Amount of assets to deposit
    function depositViaCompound(
        uint16 protocol_type,
        address asset,
        uint256 amount
    ) external payable;


    /// @notice Withdraw assets from a compound protocol
    /// @param protocol_type Protocol type
    /// @param lpToken Address of the underlying lp lpToken (e.g., lp usdc)
    /// @param amount Amount of assets to withdraw
    function withdrawViaCompound(
        uint16 protocol_type,
        address lpToken,
        uint256 amount
    ) external;


    /// @notice Deposit assets into a compound protocol
    /// @param protocol_type Protocol type
    /// @param asset Address of the underlying asset (e.g., USDC, DAI)
    /// @param amount Amount of assets to deposit
  function depositViaMaker(
        uint16 protocol_type,
        address asset,
        uint256 amount
    ) external payable;

    /// @notice Withdraw assets from a compound protocol
    /// @param protocol_type Protocol type
    /// @param lpToken Address of the underlying lp lpToken (e.g., lp usdc)
    /// @param amount Amount of assets to withdraw
    function withdrawViaMaker(
        uint16 protocol_type,
        address lpToken,
        uint256 amount
    ) external;

}
