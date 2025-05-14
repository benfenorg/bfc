// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";

import {MockUSDC,MockUSDT,WETH} from "../test/mocks/MockTokens.sol";


contract DeployScript is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        WETH weth =new WETH();
        MockUSDC usdc = new MockUSDC();
        MockUSDT usdt = new MockUSDT();
        console.log("[Deployed] WETH:", address(weth));

        console.log("[Deployed] USDC:", address(usdc));
        console.log("[Deployed] USDT:", address(usdt));

        vm.stopBroadcast();
    }
}
