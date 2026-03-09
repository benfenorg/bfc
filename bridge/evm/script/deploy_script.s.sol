// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";

import "../contracts/BridgeConfig.sol";
import "../contracts/BridgeLimiter.sol";
import "../contracts/SuiBridge.sol";

contract DeployScript is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        // BridgeLimiter bridgeLimiter = new BridgeLimiter();
        SuiBridge bridge = new SuiBridge();

        // console.log("[Deployed] BridgeLimiter:", address(bridgeLimiter));
        console.log("[Deployed] SuiBridge:", address(bridge));

        vm.stopBroadcast();
    }
}
//forge script script/deploy_limit.s.sol --rpc-url  https://base-rpc.publicnode.com   --broadcast --verify   