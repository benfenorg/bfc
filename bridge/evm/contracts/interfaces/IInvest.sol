
interface IInvest {

    /// @notice Deposit assets into a specific protocol
    /// @param protocol_type Protocol type (1 = Aave, 2 = Compound, 3 = MakerDAO)
    /// @param asset Address of the underlying asset (e.g., USDC, DAI)
    /// @param amount Amount of assets to deposit
    /// @param recipientAddress The recipient address on Benfen chain to receive the generated LP tokens
    function deposit(
        uint64 protocol_type,
        address asset,
        uint256 amount,
        bytes memory recipientAddress
    ) external payable;

    /// @notice Withdraw assets from a specific protocol
    /// @param protocol_type Protocol type
    /// @param asset Address of the underlying asset
    /// @param amount Amount of assets to withdraw
    /// @param recipientAddress The recipient address on Benfen chain to receive the withdrawn USDT/USDC
    function withdraw(
        uint64 protocol_type,
        address asset,
        uint256 amount,
        bytes memory recipientAddress
    ) external;

}
