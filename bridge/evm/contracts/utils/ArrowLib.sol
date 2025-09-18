// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IArrow} from "../interfaces/IArrow.sol";


library ArrowLib {

    function deposit(
        address arrowAddr ,
        uint64 protocol_type,
        address asset,
        uint256 amount
    )internal{
        if (arrowAddr == address(0)) {
            return;
        }
        require(amount > 0, "ArrowLib: Amount must be greater than zero");
        require(asset != address(0), "ArrowLib: asset address must be valid");
        require(protocol_type <= type(uint16).max, "ArrowLib: Protocol type out of range");
        IArrow arrow = IArrow(arrowAddr);
        IERC20(asset).approve(arrowAddr,amount);
        if  (protocol_type==0) {
            arrow.depositViaAave(uint16(protocol_type), asset, amount);
        }else if (protocol_type==1) {
            arrow.depositViaCompound(uint16(protocol_type), asset, amount);
        }else if (protocol_type==2) {
            arrow.depositViaMaker(uint16(protocol_type), asset, amount);
        }else{
            revert("SuiBridge:  Invalid Arrow ProtocolType");
        }
    }

    function withdraw(
        address arrowAddr ,
        uint64 protocol_type,
        address lpToken,
        uint256 amount
    ) internal {
        if (arrowAddr == address(0)) {
            return;
        }
        require(amount > 0, "ArrowLib: Amount must be greater than zero");
        require(lpToken != address(0), "ArrowLib: lpToken address must be valid");
        require(protocol_type <= type(uint16).max, "ArrowLib: Protocol type out of range");
        IArrow arrow = IArrow(arrowAddr);
        IERC20(lpToken).approve(arrowAddr,amount);
        if  (protocol_type==0) {
            arrow.withdrawViaAave(uint16(protocol_type), lpToken, amount);
        }else if (protocol_type==1) {
            arrow.withdrawViaCompound(uint16(protocol_type), lpToken, amount);
        }else if (protocol_type==2) {
            arrow.withdrawViaMaker(uint16(protocol_type), lpToken, amount);
        }else{
            revert("SuiBridge:  Invalid Arrow ProtocolType");
        }

    }

}