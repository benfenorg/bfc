// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./BridgeBaseTest.t.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "../contracts/interfaces/ISuiBridge.sol";
import "../contracts/interfaces/IBridgeConfig.sol";
import "../contracts/utils/BridgeUtils.sol";
import "forge-std/console.sol";

contract CrossInWithFeeTest is BridgeBaseTest {
    
    function setUp() public {
        setUpBridgeTest();
    }

    function testBridgeETHFeeCalculation() public {
        // 1. Set Bridge Fee for ETH
        uint64 tokenID = BridgeUtils.ETH;
        uint8 mode = 1; // Percentage
        uint64 value = 100000; // 10% (assuming denominator is 100000)
        uint256 minFeeValue = 0.01 ether;

        updateBridgeFee(tokenID, mode, value, minFeeValue);


        // 2. Verify Fee Info
        IBridgeConfig.BridgeFeeInfo memory feeInfo = config.bridgeFeeInfoOf(tokenID);
        assertEq(feeInfo.mode, mode);
        assertEq(feeInfo.value, value);
        assertEq(feeInfo.minFeeValue, minFeeValue);

        // 3. Bridge ETH
        uint256 amount = 1 ether;
        bytes memory recipientAddress = new bytes(32); // Sui address length is 32
        uint8 destinationChainID = 0; // Sui is usually 0

        // Use vm.warp to avoid underflow in BridgeLimiter
        vm.warp(1700000000);

        uint256 initialVaultBalance = IERC20(wETH).balanceOf(address(vault));

        bridge.bridgeETH{value: amount}(recipientAddress, destinationChainID);

        // 4. Verify Vault Balance (Fee should NOT be deducted from deposit to vault)
        // The entire msg.value is sent to vault in bridgeETH and converted to WETH
        assertEq(IERC20(wETH).balanceOf(address(vault)), initialVaultBalance + amount);
    }


    function testBridgeUSDCFeeCalculationWithFeeAndLimit() public {
        changePrank(USDCWhale);

        uint256 amount = 1000000000; // 1000 USDC (6 decimals)
        // approve
        IERC20(USDC).approve(address(bridge), amount);

        // 1. Set Bridge Fee for USDC
        uint64 tokenID = BridgeUtils.USDC;
        uint8 mode = 1; // Percentage
        uint64 value = 10000; // 1% (assuming denominator is 10000)
        uint256 minFeeValue = 1000000; // 1 USDC (6 decimals)
        uint64 limit = 10000000000; // 100 USDC (8 decimals)

        updateBridgeFee(tokenID, mode, value, minFeeValue);
        updateMinLimit(limit);

        // 2. Verify Fee Info And Limit
        IBridgeConfig.BridgeFeeInfo memory feeInfo = config.bridgeFeeInfoOf(tokenID);
        assertEq(feeInfo.mode, mode);
        assertEq(feeInfo.value, value);
        assertEq(feeInfo.minFeeValue, minFeeValue);
        assertEq(limiter.getUsdMinLimit(), uint256(limit));

        // 3. Bridge USDC
       
        bytes memory recipientAddress = new bytes(32); // Sui address length is 32
        uint8 destinationChainID = 0; // Sui is usually 0

        // Use vm.warp to avoid underflow in BridgeLimiter
        vm.warp(1700000000);

        uint256 initialVaultBalance = IERC20(USDC).balanceOf(address(vault));

        // bridge.bridgeETH{value: amount}(recipientAddress, destinationChainID);

        bridge.bridgeERC20WithTargetTokenID(
            BridgeUtils.USDC,
            amount,
            hex"06bb77410cd326430fa2036c8282dbb54a6f8640cea16ef5eff32d638718b3e4",
            0,
            BridgeUtils.BUSD
        );

        // 4. Verify Vault Balance (Fee should NOT be deducted from deposit to vault)
        // The entire msg.value is sent to vault in bridgeETH and converted to WETH
        assertEq(IERC20(USDC).balanceOf(address(vault)), initialVaultBalance + amount);
    }

    function testBridgeUSDCFeeCalculationWithFeeCrossInIsFailed() public {

        changePrank(USDCWhale);

         uint256 amount = 1000000; // 1 USDC (6 decimals)
        // approve
        IERC20(USDC).approve(address(bridge), amount);
         // 1. Set Bridge Fee for USDC
        uint64 tokenID = BridgeUtils.USDC;
        uint8 mode = 0; // 
        uint64 value = 1000000; //  1 USDC
        uint256 minFeeValue = 100000; // 0.1 USDC (6 decimals)
      
        updateBridgeFee(tokenID, mode, value, minFeeValue);

        // 2. Verify Fee Info And Limit
        IBridgeConfig.BridgeFeeInfo memory feeInfo = config.bridgeFeeInfoOf(tokenID);
        assertEq(feeInfo.mode, mode);
        assertEq(feeInfo.value, value);
        assertEq(feeInfo.minFeeValue, minFeeValue);

        // 3. Bridge USDC
        // uint256 amount = 1000000; // 1 USDC (6 decimals)
        bytes memory recipientAddress = new bytes(32); // Sui address length is 32
        uint8 destinationChainID = 0; // Sui is usually 0

        vm.expectRevert("SuiBridge: Insufficient amount for fee");

        bridge.bridgeERC20WithTargetTokenID(
            BridgeUtils.USDC,
            amount,
            hex"06bb77410cd326430fa2036c8282dbb54a6f8640cea16ef5eff32d638718b3e4",
            0,
            BridgeUtils.BUSD
        );

     }


    function testBridgeUSDCFeeCalculationWithLtMinFeeCrossInIsFailed() public {

        changePrank(USDCWhale);

        uint256 amount = 9000000; // 9 USDC (6 decimals)
        // approve
        IERC20(USDC).approve(address(bridge), amount);
         // 1. Set Bridge Fee for USDC
        uint64 tokenID = BridgeUtils.USDC;
        uint8 mode = 1; // 
        uint64 value = 10000; //  1 USDC
        uint256 minFeeValue = 10000000; // 10 USDC (6 decimals)
      
        updateBridgeFee(tokenID, mode, value, minFeeValue);

        // 2. Verify Fee Info And Limit
        IBridgeConfig.BridgeFeeInfo memory feeInfo = config.bridgeFeeInfoOf(tokenID);
        assertEq(feeInfo.mode, mode);
        assertEq(feeInfo.value, value);
        assertEq(feeInfo.minFeeValue, minFeeValue);

        // 3. Bridge USDC
        // uint256 amount = 1000000; // 1 USDC (6 decimals)
        bytes memory recipientAddress = new bytes(32); // Sui address length is 32
        uint8 destinationChainID = 0; // Sui is usually 0

        vm.expectRevert("SuiBridge: Insufficient amount for fee");

        bridge.bridgeERC20WithTargetTokenID(
            BridgeUtils.USDC,
            amount,
            hex"06bb77410cd326430fa2036c8282dbb54a6f8640cea16ef5eff32d638718b3e4",
            0,
            BridgeUtils.BUSD
        );

     }


    function testBridgeUSDCFeeCalculationWithLimitCrossInIsFailed() public {
        changePrank(USDCWhale);

        uint256 amount = 1000000; // 1 USDC (6 decimals)
        // approve
        IERC20(USDC).approve(address(bridge), amount);
         // 1. Set Bridge Fee for USDC
        uint64 tokenID = BridgeUtils.USDC;

        uint64 limit = 100000000; // 1 USDC (8 decimals)
        updateMinLimit(limit);
   
        // 3. Bridge USDC
        // uint256 amount = 1000000; // 1 USDC (6 decimals)
        bytes memory recipientAddress = new bytes(32); // Sui address length is 32
        uint8 destinationChainID = 0; // Sui is usually 0

        vm.expectRevert("SuiBridge: USD Less Than Min Limit");

        bridge.bridgeERC20WithTargetTokenID(
            BridgeUtils.USDC,
            amount,
            hex"06bb77410cd326430fa2036c8282dbb54a6f8640cea16ef5eff32d638718b3e4",
            0,
            BridgeUtils.BUSD
        );

    }

    function testBridgeUSDCFeeCalculationWithLimitCrossInIsSuceess() public {
        changePrank(USDCWhale);

        uint256 amount = 1000000+1; // 1 USDC (6 decimals)
        // approve
        IERC20(USDC).approve(address(bridge), amount);
         // 1. Set Bridge Fee for USDC
        uint64 tokenID = BridgeUtils.USDC;

        uint64 limit = 100000000; // 1 USDC (8 decimals)
        updateMinLimit(limit);
   
        // 3. Bridge USDC
        // uint256 amount = 1000000; // 1 USDC (6 decimals)
        bytes memory recipientAddress = new bytes(32); // Sui address length is 32
        uint8 destinationChainID = 0; // Sui is usually 0

        //vm.expectRevert("SuiBridge: USD Less Than Min Limit");

        bridge.bridgeERC20WithTargetTokenID(
            BridgeUtils.USDC,
            amount,
            hex"06bb77410cd326430fa2036c8282dbb54a6f8640cea16ef5eff32d638718b3e4",
            0,
            BridgeUtils.BUSD
        );

    }

    function testBridgeETHMinLimitRevert() public {
        // 1. Set Min Limit
        uint64 minLimit = 100 * 100000000; // 100 USD (8 decimals)
        updateMinLimit(minLimit);

        assertEq(limiter.getUsdMinLimit(), uint256(minLimit));

        // 2. Try to bridge amount less than min limit
        // ETH price in BridgeBaseTest is 2596.96 USD (8 decimals)
        // 1 ETH = 2596.96 USD
        // 0.01 ETH = 25.96 USD < 100 USD
        uint256 amount = 0.01 ether; 
        bytes memory recipientAddress = new bytes(32);
        uint8 destinationChainID = 0;

        vm.warp(1700000000);

        vm.expectRevert("SuiBridge: USD Less Than Min Limit");
        bridge.bridgeETH{value: amount}(recipientAddress, destinationChainID);
    }

    function testBridgeETHFeeNotDeducted() public {
        // 1. Set Bridge Fee
        uint64 tokenID = BridgeUtils.ETH;
        uint8 mode = 1; // Percentage
        uint64 value = 1000; // 10%
        uint256 minFeeValue = 0;
        updateBridgeFee(tokenID, mode, value, minFeeValue);

        // 2. Bridge ETH
        uint256 amount = 1 ether;
        bytes memory recipientAddress = new bytes(32);
        uint8 destinationChainID = 0;

        vm.warp(1700000000);

        // 3. Capture event to verify amount or check logs
        // Since we cannot easily check event args without defining event, 
        // we verify that the transaction succeeds and vault receives full amount.
        
        uint256 initialVaultBalance = IERC20(wETH).balanceOf(address(vault));
        bridge.bridgeETH{value: amount}(recipientAddress, destinationChainID);
        assertEq(IERC20(wETH).balanceOf(address(vault)), initialVaultBalance + amount);
    }

    function testBridgeETHInsufficientFee() public {
        // 1. Set Bridge Fee (Min Fee)
        uint64 tokenID = BridgeUtils.ETH;
        uint8 mode = 1; // Percentage
        uint64 value = 0; // 0%
        uint256 minFeeValue = 0.5 ether; // High min fee
        
        updateBridgeFee(tokenID, mode, value, minFeeValue);

        // 2. Send amount less than fee
        // Send 0.1 ether. Fee should be max(0, 0.5) = 0.5 ether.
        // 0.1 < 0.5. Should revert.
        bytes memory recipientAddress = new bytes(32);
        uint8 destinationChainID = 0;
        vm.warp(1700000000);
        
        vm.expectRevert("SuiBridge: Insufficient amount for fee");
        bridge.bridgeETH{value: 0.1 ether}(recipientAddress, destinationChainID);
        
        // // 3. Send amount equal to fee
        // // Send 0.5 ether. Fee = 0.5. Amount >= Fee. Should pass.
        // // And full amount should be deposited.
        // uint256 initialVaultBalance = IERC20(wETH).balanceOf(address(vault));
        // bridge.bridgeETH{value: 0.5 ether}(recipientAddress, destinationChainID);
        // assertEq(IERC20(wETH).balanceOf(address(vault)), initialVaultBalance + 0.5 ether);
    }

    // Helper to update bridge fee
    function updateBridgeFee(uint64 tokenID, uint8 mode, uint64 value, uint256 minFeeValue) internal {
        bytes memory payload = abi.encodePacked(
            tokenID,
            mode,
            value,
            minFeeValue
        );
        
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.UPDATE_BRIDGE_FEE,
            version: 1,
            nonce: 0, 
            chainID: chainID,
            payload: payload
        });
        
        // Need to fetch current nonce for this message type
        // BridgeConfig inherits from CommitteeUpgradeable which has nonces
        // Wait, BridgeConfig inherits from CommitteeUpgradeable?
        // Let's check BridgeConfig inheritance.
        // BridgeConfig is OwnableUpgradeable, UUPSUpgradeable.
        // It does NOT inherit from CommitteeUpgradeable.
        // It uses verifyMessageAndSignatures modifier which checks nonces in `committee`.
        // Wait, verifyMessageAndSignatures is in BridgeConfig?
        // Let's check BridgeConfig.sol again for verifyMessageAndSignatures.
        // Ah, BridgeConfig has `verifyMessageAndSignatures` modifier?
        // Yes, it does.
        
        // And it checks `committee.nonces(messageType)`.
        // So I should check `committee.nonces`.
        
        uint64 nonce = committee.nonces(BridgeUtils.UPDATE_BRIDGE_FEE);
        message.nonce = nonce;

        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(encodedMessage);

        bytes[] memory signatures = new bytes[](4);
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        config.updateBridgeFeeWithSignatures(signatures, message);
    }

    // Helper to update min limit
    function updateMinLimit(uint64 newLimit) internal {
        // Payload structure based on updated BridgeUtils.decodeUpdateMinLimitPayload:
        // uint8 senderChainID (1 byte)
        // uint64 newLimit (8 bytes)
        // Total: 9 bytes
        
        // Use a supported chain ID (e.g. 0 for Sui)
        uint8 sourceChainID = 0; 
        
        bytes memory payload = abi.encodePacked(sourceChainID, newLimit);
        
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.UPDATE_MIN_LIMIT,
            version: 1,
            nonce: 0,
            chainID: chainID,
            payload: payload
        });

        // Limiter has nonces?
        // BridgeLimiter inherits CommitteeUpgradeable?
        // Yes, BridgeLimiter.sol: contract BridgeLimiter is Initializable, CommitteeUpgradeable...
        // So it has nonces mapping itself?
        // Let's check BridgeLimiter.sol
        // `contract BridgeLimiter is Initializable, CommitteeUpgradeable, OwnableUpgradeable, UUPSUpgradeable`
        // So it has `nonces` mapping.
        
        uint64 nonce = limiter.nonces(BridgeUtils.UPDATE_MIN_LIMIT);
        message.nonce = nonce;

        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(encodedMessage);

        bytes[] memory signatures = new bytes[](4);
        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        limiter.updateMinLimitWithSignatures(signatures, message);
    }
}
