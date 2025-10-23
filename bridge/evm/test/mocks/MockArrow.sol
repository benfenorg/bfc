// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IArrow} from "../../contracts/interfaces/IArrow.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {console2} from "forge-std/console2.sol";

contract MockArrow is IArrow {
    address vault;
    constructor(address _vault){
        vault=_vault;
    }

    //protocol_type => asset => lpToken
    mapping(uint64 => mapping(address=>address)) lpTokens;
    //protocol_type => asset => lpToken
    mapping(uint64 => mapping(address=>address)) assets;

    function setLpToken(uint64 protocol_type,address asset,address lpToken) external{
        lpTokens[protocol_type][asset]=lpToken;
    }

    function setAsset(uint64 protocol_type,address asset,address lpToken) external{
        assets[protocol_type][lpToken]=asset;
    }

    //lpToken
    function getLpToken(uint64 protocol_type,address asset) external view returns(address){
        return lpTokens[protocol_type][asset];
    }

    //asset
    function getAsset(uint64 protocol_type,address lpToken) external view returns(address){
        return assets[protocol_type][lpToken];
    }


    /// @notice Deposit assets into a aave protocol
    /// @param protocol_type Protocol type
    /// @param asset Address of the underlying asset (e.g., USDC, DAI)
    /// @param amount Amount of assets to deposit
    function depositViaAave(
        uint16 protocol_type,
        address asset,
        uint256 amount
    ) external payable{
        address lpToken=lpTokens[protocol_type][asset];

        require(lpToken!=address(0),"lpToken not set");
        //检查是否转进来 amount 数量的asset
        uint256 oldBalance = IERC20(asset).balanceOf(address(this));

        SafeERC20.safeTransferFrom(IERC20(asset), msg.sender, address(this), amount);

        uint256 newBalance = IERC20(asset).balanceOf(address(this));

        require(newBalance-oldBalance>=amount,"asset not enough");

        require(IERC20(lpToken).balanceOf(address(this))>=amount,"lpToken not enough");
        IERC20(lpToken).transfer(vault, amount);
    }

    /// @notice Withdraw assets from a aave protocol
    /// @param protocol_type Protocol type
    /// @param lpToken Address of the underlying lp lpToken (e.g., lp usdc)
    /// @param amount Amount of assets to withdraw
    function withdrawViaAave(
        uint16 protocol_type,
        address lpToken,
        uint256 amount
    ) external{
        //asset
        address asset=assets[protocol_type][lpToken];
        require(asset!=address(0),"asset not set");

        uint256 oldBalance = IERC20(lpToken).balanceOf(address(this));

        SafeERC20.safeTransferFrom(IERC20(lpToken), msg.sender, address(this), amount);

        uint256 newBalance = IERC20(lpToken).balanceOf(address(this));

        require(newBalance-oldBalance>=amount,"lpToken not enough");

        require(IERC20(asset).balanceOf(address(this))>=amount,"asset not enough");

        IERC20(asset).transfer(vault, amount + (amount / 10));
    }


    /// @notice Deposit assets into a compound protocol
    /// @param protocol_type Protocol type
    /// @param asset Address of the underlying asset (e.g., USDC, DAI)
    /// @param amount Amount of assets to deposit
    function depositViaCompound(
        uint16 protocol_type,
        address asset,
        uint256 amount
    ) external payable{

    }


    /// @notice Withdraw assets from a compound protocol
    /// @param protocol_type Protocol type
    /// @param lpToken Address of the underlying lp lpToken (e.g., lp usdc)
    /// @param amount Amount of assets to withdraw
    function withdrawViaCompound(
        uint16 protocol_type,
        address lpToken,
        uint256 amount
    ) external{

    }


    /// @notice Deposit assets into a compound protocol
    /// @param protocol_type Protocol type
    /// @param asset Address of the underlying asset (e.g., USDC, DAI)
    /// @param amount Amount of assets to deposit
  function depositViaMaker(
        uint16 protocol_type,
        address asset,
        uint256 amount
    ) external payable{

    }

    /// @notice Withdraw assets from a compound protocol
    /// @param protocol_type Protocol type
    /// @param lpToken Address of the underlying lp lpToken (e.g., lp usdc)
    /// @param amount Amount of assets to withdraw
    function withdrawViaMaker(
        uint16 protocol_type,
        address lpToken,
        uint256 amount
    ) external{

    }

}