// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../../contracts/interfaces/IArrow.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {console2} from "forge-std/console2.sol";





contract MockArrow is IArrow{
    address vault;
    constructor(address _vault){
        vault=_vault;
    }

    mapping(address=>address) lpTokens;
    mapping(address=>address) assets;

    function setLpToken(address asset,address lpToken) external{
        lpTokens[asset]=lpToken;
    }

    function setAsset(address asset,address lpToken) external{
        assets[lpToken]=asset;
    }

    //lpToken
    function getLpToken(address asset) external view returns(address){
        return lpTokens[asset];
    }

    //asset
    function getAsset(address lpToken) external view returns(address){
        return assets[lpToken];
    }

    //stake
    function deposit(
        uint64 _protocolType,
        address asset,
        uint256 amount
    ) external payable{
        //lpToken
        address lpToken=lpTokens[asset];
        require(lpToken!=address(0),"lpToken not set");
        //检查是否转进来 amount 数量的asset
        uint256 oldBalance = IERC20(asset).balanceOf(address(this));

        SafeERC20.safeTransferFrom(IERC20(asset), msg.sender, address(this), amount);

        uint256 newBalance = IERC20(asset).balanceOf(address(this));

        require(newBalance-oldBalance>=amount,"asset not enough");

        require(IERC20(lpToken).balanceOf(address(this))>=amount,"lpToken not enough");
        IERC20(lpToken).transfer(vault, amount);
    }

    //unstake
    function withdraw(
        uint64 _protocolType,
        address lpToken,
        uint256 amount
    ) external{
        //asset
        address asset=assets[lpToken];
        require(asset!=address(0),"asset not set");

        uint256 oldBalance = IERC20(lpToken).balanceOf(address(this));

        SafeERC20.safeTransferFrom(IERC20(lpToken), msg.sender, address(this), amount);

        uint256 newBalance = IERC20(lpToken).balanceOf(address(this));

        require(newBalance-oldBalance>=amount,"lpToken not enough");

        require(IERC20(asset).balanceOf(address(this))>=amount,"asset not enough");


        IERC20(asset).transfer(vault, amount);
    }


}