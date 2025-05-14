// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
// import "openzeppelin-foundry-upgrades/Upgrades.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import "openzeppelin-foundry-upgrades/Options.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "../contracts/BridgeCommittee.sol";
import {BridgeVault as BNBBridgeVault} from "../contracts/vault/bnb/BridgeVault.sol";
import {BridgeVault as ETHBridgeVault} from "../contracts/vault/eth/BridgeVault.sol";

import {SuiBridge as BNBSuiBridge}from"../contracts/bridge/bnb/BnbSuiBridge.sol";
import {SuiBridge as ETHSuiBridge}from"../contracts/bridge/eth/EthSuiBridge.sol";


import "../contracts/BridgeConfig.sol";
import "../contracts/BridgeLimiter.sol";


import "../test/mocks/MockTokens.sol";

contract DeployBridge is Script {
    function parseDeployConfig(string memory path) public returns (DeployConfig memory) {
        string memory json = vm.readFile(path);
        DeployConfig memory config;

        config.committeeMemberStake = abi.decode(vm.parseJson(json, ".committeeMemberStake"), (uint256[]));
        config.committeeMembers = abi.decode(vm.parseJson(json, ".committeeMembers"), (address[]));
        config.minCommitteeStakeRequired = abi.decode(vm.parseJson(json, ".minCommitteeStakeRequired"), (uint256));
        config.sourceChainId = abi.decode(vm.parseJson(json, ".sourceChainId"), (uint256));
        config.supportedChainIds = abi.decode(vm.parseJson(json, ".supportedChainIds"), (uint256[]));
        config.supportedChainLimitsInDollars = abi.decode(vm.parseJson(json, ".supportedChainLimitsInDollars"), (uint256[]));
        config.maxUsdLimit = abi.decode(vm.parseJson(json, ".maxUsdLimit"), (uint256));
        config.tokenPrices = abi.decode(vm.parseJson(json, ".tokenPrices"), (uint256[]));
        config.supportedTokens = abi.decode(vm.parseJson(json, ".supportedTokens"), (address[]));
        config.tokenIds = abi.decode(vm.parseJson(json, ".tokenIds"), (uint256[]));
        config.suiDecimals = abi.decode(vm.parseJson(json, ".suiDecimals"), (uint256[]));
        config.weth = abi.decode(vm.parseJson(json, ".weth"), (address));

        return config;
    }

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        string memory chainID = Strings.toString(block.chainid);
        bytes32 chainIDHash = keccak256(abi.encode(chainID));
        bool isLocal = chainIDHash != keccak256(abi.encode("11155111"))
            && chainIDHash != keccak256(abi.encode("1"))&& chainIDHash != keccak256(abi.encode("97")) &&  chainIDHash != keccak256(abi.encode("56"))
            && chainIDHash != keccak256(abi.encode("421614")) && chainIDHash != keccak256(abi.encode("84532")) && chainIDHash != keccak256(abi.encode("420"))
            && chainIDHash != keccak256(abi.encode("43113"))&& chainIDHash != keccak256(abi.encode("80001")) && chainIDHash != keccak256(abi.encode("80002"));
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/deploy_configs/", chainID, ".json");
        // If this is local deployment, we override the path if OVERRIDE_CONFIG_PATH is set.
        // This is useful in integration tests where config path is not fixed.
        if (isLocal) {
            path = vm.envOr("OVERRIDE_CONFIG_PATH", path);
        }

        console.log("config path: ", path);
        DeployConfig memory deployConfig = parseDeployConfig(path);

        // if deploying to local network, deploy mock tokens
        if (isLocal) {
            console.log("Deploying mock tokens for local network");
            // deploy WETH
            deployConfig.weth = address(new WETH());

            // deploy mock tokens
            IERC20 USDC;
            IERC20 USDT;

            if (chainIDHash == keccak256(abi.encode("31339"))){
                USDC = new MockBNBUSDC();
                USDT = new MockBNBUSDT();
            }else{
                USDC = new MockUSDC();
                USDT = new MockUSDT();

            }
            MockWBTC wBTC = new MockWBTC();
            MockKA KA = new MockKA();
            MockBUSD BUSD = new MockBUSD();
            MockBNB BNB = new MockBNB();
            console.log("[Deployed] KA:", address(KA));
            console.log("[Deployed] BNB:", address(BNB));

            // update deployConfig with test values
            deployConfig.supportedTokens = new address[](7);
            deployConfig.supportedTokens[0] = address(0);
            deployConfig.supportedTokens[1] = address(wBTC);
            deployConfig.supportedTokens[2] = deployConfig.weth;
            deployConfig.supportedTokens[3] = address(USDC);
            deployConfig.supportedTokens[4] = address(USDT);
            deployConfig.supportedTokens[5] = address(BUSD);
            deployConfig.supportedTokens[6] = address(BNB);

            deployConfig.tokenIds = new uint256[](7);
            deployConfig.tokenIds[0] = 0;
            deployConfig.tokenIds[1] = 1;
            deployConfig.tokenIds[2] = 2;
            deployConfig.tokenIds[3] = 3;
            deployConfig.tokenIds[4] = 4;
            deployConfig.tokenIds[5] = 5;
            deployConfig.tokenIds[6] = 6;

            deployConfig.suiDecimals = new uint256[](7);
            deployConfig.suiDecimals[0] = 9;
            deployConfig.suiDecimals[1] = 8;
            deployConfig.suiDecimals[2] = 8;
            if (chainIDHash == keccak256(abi.encode("31339"))){
                deployConfig.suiDecimals[3] = 9;
                deployConfig.suiDecimals[4] = 9;
            }else{
                deployConfig.suiDecimals[3] = 6;
                deployConfig.suiDecimals[4] = 6;
            }

            deployConfig.suiDecimals[5] = 9;
            deployConfig.suiDecimals[6] = 8;
        }

        // convert supported chains from uint256 to uint8
        uint8[] memory supportedChainIds = new uint8[](deployConfig.supportedChainIds.length);
        for (uint256 i; i < deployConfig.supportedChainIds.length; i++) {
            supportedChainIds[i] = uint8(deployConfig.supportedChainIds[i]);
        }

        require(
            deployConfig.supportedTokens.length == deployConfig.tokenPrices.length,
            "supportedTokens.length != tokenPrices.length"
        );
        require(
            deployConfig.supportedTokens.length == deployConfig.tokenIds.length,
            "supportedTokens.length != tokenIds.length"
        );
        require(
            deployConfig.supportedTokens.length == deployConfig.suiDecimals.length,
            "supportedTokens.length != suiDecimals.length"
        );

        // deploy Bridge Committee ===================================================================

        // convert committeeMembers stake from uint256 to uint16
        uint16[] memory committeeMemberStake =
            new uint16[](deployConfig.committeeMemberStake.length);
        for (uint256 i; i < deployConfig.committeeMemberStake.length; i++) {
            committeeMemberStake[i] = uint16(deployConfig.committeeMemberStake[i]);
        }

        Options memory opts;
        opts.unsafeSkipAllChecks = true;

        address bridgeCommittee = Upgrades.deployUUPSProxy(
            "BridgeCommittee.sol",
            abi.encodeCall(
                BridgeCommittee.initialize,
                (
                    deployConfig.committeeMembers,
                    committeeMemberStake,
                    uint16(deployConfig.minCommitteeStakeRequired)
                )
            ),
            opts
        );

        // deploy bridge config =====================================================================

        // convert token prices from uint256 to uint64
        uint64[] memory tokenPrices = new uint64[](deployConfig.tokenPrices.length);
        for (uint256 i; i < deployConfig.tokenPrices.length; i++) {
            tokenPrices[i] = uint64(deployConfig.tokenPrices[i]);
        }

        // convert Sui Decimals from uint256 to uint8
        uint8[] memory suiDecimals = new uint8[](deployConfig.suiDecimals.length);
        for (uint256 i; i < deployConfig.suiDecimals.length; i++) {
            suiDecimals[i] = uint8(deployConfig.suiDecimals[i]);
        }

        // convert Token Id from uint256 to uint8
        uint64[] memory tokenIds = new uint64[](deployConfig.tokenIds.length);
        for (uint256 i; i < deployConfig.tokenIds.length; i++) {
            tokenIds[i] = uint64(deployConfig.tokenIds[i]);
        }

        address bridgeConfig = Upgrades.deployUUPSProxy(
            "BridgeConfig.sol",
            abi.encodeCall(
                BridgeConfig.initialize,
                (
                    address(bridgeCommittee),
                    uint8(deployConfig.sourceChainId),
                    deployConfig.supportedTokens,
                    tokenPrices,
                    tokenIds,
                    suiDecimals,
                    supportedChainIds
                )
            ),
            opts
        );

        // initialize config in the bridge committee
        BridgeCommittee(bridgeCommittee).initializeConfig(address(bridgeConfig));
        BridgeCommittee committeeImplementation =
            BridgeCommittee(Upgrades.getImplementationAddress(bridgeCommittee));
        committeeImplementation.initializeConfig(address(bridgeConfig));


        // deploy limiter ===========================================================================

        // convert chain limits from uint256 to uint64[]
        uint64[] memory chainLimits =
            new uint64[](deployConfig.supportedChainLimitsInDollars.length);
        for (uint256 i; i < deployConfig.supportedChainLimitsInDollars.length; i++) {
            chainLimits[i] = uint64(deployConfig.supportedChainLimitsInDollars[i]);
        }
        uint256 maxUsdLimit=deployConfig.maxUsdLimit;

        address limiter = Upgrades.deployUUPSProxy(
            "BridgeLimiter.sol",
            abi.encodeCall(
                BridgeLimiter.initialize, (bridgeCommittee, supportedChainIds, chainLimits,maxUsdLimit)
            ),
            opts
        );

        uint8[] memory _destinationChains = new uint8[](1);
        _destinationChains[0] = 1;

       if (
        keccak256(abi.encode(Strings.toString(block.chainid))) == keccak256(abi.encode("56")) ||
        keccak256(abi.encode(Strings.toString(block.chainid))) == keccak256(abi.encode("97")) ||
        keccak256(abi.encode(Strings.toString(block.chainid))) == keccak256(abi.encode("31339"))
    ){
            // deploy vault =============================================================================
            BNBBridgeVault vault = new BNBBridgeVault(deployConfig.weth);
             // deploy Sui Bridge ========================================================================
            address suiBridge = Upgrades.deployUUPSProxy(
                "BnbSuiBridge.sol",
                abi.encodeCall(BNBSuiBridge.initialize, (bridgeCommittee, address(vault), limiter)),
                opts
            );
            // transfer vault ownership to bridge
            vault.transferOwnership(suiBridge);
            // transfer limiter ownership to bridge
            BridgeLimiter instance = BridgeLimiter(limiter);
            instance.transferOwnership(suiBridge);

            // print deployed addresses for post deployment setup
            console.log("[Deployed] BridgeConfig:", bridgeConfig);
            console.log("[Deployed] SuiBridge:", suiBridge);
            console.log("[Deployed] BridgeLimiter:", limiter);
            console.log("[Deployed] BridgeCommittee:", bridgeCommittee);
            console.log("[Deployed] BridgeVault:", address(vault));
            console.log("[Deployed] BTC:", BridgeConfig(bridgeConfig).tokenAddressOf(1));
            console.log("[Deployed] ETH:", BridgeConfig(bridgeConfig).tokenAddressOf(2));
            console.log("[Deployed] USDC:", BridgeConfig(bridgeConfig).tokenAddressOf(3));
            console.log("[Deployed] USDT:", BridgeConfig(bridgeConfig).tokenAddressOf(4));
            console.log("[Deployed] BNB:", BridgeConfig(bridgeConfig).tokenAddressOf(6));

        }else{

            // deploy vault =============================================================================
            ETHBridgeVault vault = new ETHBridgeVault(deployConfig.weth);
             // deploy Sui Bridge ========================================================================
            address suiBridge = Upgrades.deployUUPSProxy(
                "EthSuiBridge.sol",
                abi.encodeCall(ETHSuiBridge.initialize, (bridgeCommittee, address(vault), limiter)),
                opts
            );
            // transfer vault ownership to bridge
            vault.transferOwnership(suiBridge);

             // transfer limiter ownership to bridge
            BridgeLimiter instance = BridgeLimiter(limiter);
            instance.transferOwnership(suiBridge);

            // print deployed addresses for post deployment setup
            console.log("[Deployed] BridgeConfig:", bridgeConfig);
            console.log("[Deployed] SuiBridge:", suiBridge);
            console.log("[Deployed] BridgeLimiter:", limiter);
            console.log("[Deployed] BridgeCommittee:", bridgeCommittee);
            console.log("[Deployed] BridgeVault:", address(vault));
            console.log("[Deployed] BTC:", BridgeConfig(bridgeConfig).tokenAddressOf(1));
            console.log("[Deployed] ETH:", BridgeConfig(bridgeConfig).tokenAddressOf(2));
            console.log("[Deployed] USDC:", BridgeConfig(bridgeConfig).tokenAddressOf(3));
            console.log("[Deployed] USDT:", BridgeConfig(bridgeConfig).tokenAddressOf(4));
            console.log("[Deployed] BNB:", BridgeConfig(bridgeConfig).tokenAddressOf(6));
        }




        vm.stopBroadcast();
    }

    // used to ignore for forge coverage
    function testSkip() public {}
}

/// check the following for guidelines on updating deploy_configs and references:
/// https://book.getfoundry.sh/cheatcodes/parse-json
struct DeployConfig {
    uint256[] committeeMemberStake;
    address[] committeeMembers;
    uint256 minCommitteeStakeRequired;
    uint256 sourceChainId;
    uint256[] supportedChainIds;
    uint256[] supportedChainLimitsInDollars;
    uint256 maxUsdLimit;
    address[] supportedTokens;
    uint256[] tokenPrices;
    uint256[] tokenIds;
    uint256[] suiDecimals;
    address weth;
}
