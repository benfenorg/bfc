// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./BridgeBaseTest.t.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "../contracts/interfaces/ISuiBridge.sol";
import "../contracts/interfaces/IBridgeConfig.sol";
import "./mocks/MockSuiBridgeV2.sol";

import {MockLPToken} from "./mocks/MockTokens.sol";
import {MockArrow} from "./mocks/MockArrow.sol";

contract InvestTest is BridgeBaseTest, ISuiBridge {
    // This function is called before each unit test
    function setUp() public {
        setUpBridgeTest();
    }

    function addLpToken(address lpToken,uint64 tokenID) internal {
        // Create update tokens payload
        bool _isNative = true;
        uint8 _numTokenIDs = 1;
        uint64 tokenID1 = tokenID;
        uint8 _numAddresses = 1;
        address address1 = lpToken;
        uint8 _numSuiDecimals = 1;
        uint8 suiDecimal1 = 6;
        uint8 _numPrices = 1;
        uint64 price1 = 100_000 * USD_VALUE_MULTIPLIER;

        bytes memory payload = abi.encodePacked(
            _isNative,
            _numTokenIDs,
            tokenID1,
            _numAddresses,
            address1,
            _numSuiDecimals,
            suiDecimal1,
            _numPrices,
            price1
        );

        // Create transfer message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.ADD_EVM_TOKENS,
            version: 1,
            nonce: 0,
            chainID: 1,
            payload: payload
        });

        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);

        bytes32 messageHash = keccak256(encodedMessage);

        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        config.addTokensWithSignatures(signatures, message);
        assertTrue(config.isTokenSupported(tokenID));
    }

    function addLpTokenID(uint64 protocolType, uint64 tokenID, uint64 lpTokenId) public {
        bytes memory payload = abi.encodePacked(
            protocolType,
            tokenID,
            lpTokenId
        );

        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.ADD_LP_TOKEN_ID,
            version: 1,
            nonce: 0,
            chainID: 1,
            payload: payload
        });


        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);

        bytes32 messageHash = keccak256(encodedMessage);

        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);  
        config.addLpTokenIdWithSignatures(signatures, message);
    }

    function setInvestContract(address investAddress) public {
        bytes memory payload = abi.encodePacked(
           investAddress
        );

        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.UPDATE_INVEST_ADDRESS,
            version: 1,
            nonce: 0,
            chainID: 1,
            payload: payload
        });

        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);

        bytes32 messageHash = keccak256(encodedMessage);

        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);  
        bridge.updateInvestAddressWithSignatures(signatures, message);
    }

    function createLpToken() public returns(address) {
        MockLPToken lpToken = new MockLPToken();
        //lpToken.mint(address(this), 1000000);
        // arrow.setLpToken(USDC, address(lpToken));
        return address(lpToken);
    }


    function mintLpToken(address lpToken, address receiver,  uint256 amount) public {
        MockLPToken(lpToken).mint(address(receiver), amount);
    }


    function testInvestStakedUSDCWithValidSignatures() public {
            //部署invest 合约
            MockArrow arrow = new MockArrow(address(vault));
            uint64 protocolType = 1;
            uint64 tokenID = BridgeUtils.USDC;
            uint64 lpTokenId = 100;

            address lpToken = createLpToken();
            //设置lp token
            //1 aave 2 compound
            arrow.setLpToken(1,USDC, lpToken);
            //给资管合约 mint lpToken
            mintLpToken(lpToken, address(arrow), 1000000000000000);
            //添加lp token
            addLpToken(lpToken, lpTokenId);

            //添加lp token id
            addLpTokenID(protocolType, tokenID, lpTokenId);
            //设置invest合约
            setInvestContract(address(arrow));

            //构造stake calldata
            changePrank(USDCWhale);
            IERC20(USDC).transfer(address(vault), 100_000_000);
            changePrank(deployer);

            // Create Defi payload
            uint8 senderAddressLength = 32;
            bytes memory senderAddress = abi.encode(0);
            uint8 targetChain = chainID;
            uint8 recipientAddressLength = 20;
            address recipientAddress = bridgerA;
            uint64 amount = 1_000_000;
            uint16 eventIdx = 0;
            //uint64 protocolType=1; 
            uint64 protocolVersion=1;
            uint64 protocolTokenID=tokenID;
            uint8 actionType=0;

            bytes memory payload = abi.encodePacked(
                senderAddressLength,
                senderAddress,
                targetChain,
                amount,
                new bytes(0),
                eventIdx,
                protocolType,
                protocolVersion,
                protocolTokenID,
                actionType
            );

            // Create Defi message
            BridgeUtils.Message memory message = BridgeUtils.Message({
                messageType: BridgeUtils.DEFI,
                version: 1,
                nonce: 1,
                chainID: 0,
                payload: payload
            });


            bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
            bytes32 messageHash = keccak256(encodedMessage);

            //signatures
            bytes[] memory signatures = new bytes[](4);

            signatures[0] = getSignature(messageHash, committeeMemberPkA);
            signatures[1] = getSignature(messageHash, committeeMemberPkB);
            signatures[2] = getSignature(messageHash, committeeMemberPkC);
            signatures[3] = getSignature(messageHash, committeeMemberPkD);

            uint256 bbalance=IERC20(lpToken).balanceOf(address(vault));

            //调用
            bridge.investBridgedTokensWithSignatures(signatures, message);

            uint256 abalance=IERC20(lpToken).balanceOf(address(vault));

            assertEq(abalance-bbalance,amount);
    }

    function testInvestStakedUSDCVerifyTokensStakedEvent() public {
            //部署invest 合约
            MockArrow arrow = new MockArrow(address(vault));
            uint64 protocolType = 1;
            uint64 tokenID = BridgeUtils.USDC;
            uint64 lpTokenId = 100;
            uint64 originNonce = 1;

            address lpToken = createLpToken();
            //设置lp token
            //1 aave 2 compound
            arrow.setLpToken(1,USDC, lpToken);
            //给资管合约 mint lpToken
            mintLpToken(lpToken, address(arrow), 1000000000000000);
            //添加lp token
            addLpToken(lpToken, lpTokenId);

            //添加lp token id
            addLpTokenID(protocolType, tokenID, lpTokenId);
            //设置invest合约
            setInvestContract(address(arrow));

            //构造stake calldata
            changePrank(USDCWhale);
            IERC20(USDC).transfer(address(vault), 100_000_000);
            changePrank(deployer);

            // Create Defi payload
            uint8 senderAddressLength = 32;
            bytes memory senderAddress = abi.encode(0);
            uint8 targetChain = chainID;
            uint8 recipientAddressLength = 20;
            address recipientAddress = bridgerA;
            uint64 amount = 1_000_000;
            uint16 eventIdx = 0;
            //uint64 protocolType=1; 
            uint64 protocolVersion=1;
            uint64 protocolTokenID=tokenID;
            uint8 actionType=0;

            bytes memory payload = abi.encodePacked(
                senderAddressLength,
                senderAddress,
                targetChain,
                amount,
                new bytes(0),
                eventIdx,
                protocolType,
                protocolVersion,
                protocolTokenID,
                actionType
            );

            // Create Defi message
            BridgeUtils.Message memory message = BridgeUtils.Message({
                messageType: BridgeUtils.DEFI,
                version: 1,
                nonce: originNonce,
                chainID: 0,
                payload: payload
            });


            bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
            bytes32 messageHash = keccak256(encodedMessage);

            //signatures
            bytes[] memory signatures = new bytes[](4);

            signatures[0] = getSignature(messageHash, committeeMemberPkA);
            signatures[1] = getSignature(messageHash, committeeMemberPkB);
            signatures[2] = getSignature(messageHash, committeeMemberPkC);
            signatures[3] = getSignature(messageHash, committeeMemberPkD);

            uint256 bbalance=IERC20(lpToken).balanceOf(address(vault));


            //期望事件

            vm.expectEmit(true, true, true, true);
            // emit Invested(0, USDC, 1_000_000);
            emit TokensStaked(
                1,// evm
                0, //nonce from evm
                0, //destination chain id (from benfen)
                originNonce, //origin nonce(from benfen)
                senderAddress, //senderAddress
                address(arrow), //recipientAddress (mock arrow)
                0, //erc20AdjustedAmount
                amount, //erc20lpTokenAmount
                protocolType,
                protocolVersion,
                protocolTokenID,
                actionType
            );
            //调用
            bridge.investBridgedTokensWithSignatures(signatures, message);

            uint256 abalance=IERC20(lpToken).balanceOf(address(vault));

            assertEq(abalance-bbalance,amount);

    }

    function testInvestStakedUSDCVerifyTokensStakedeTwiceEvent() public {
            //部署invest 合约
            MockArrow arrow = new MockArrow(address(vault));
            uint64 protocolType = 1;
            uint64 tokenID = BridgeUtils.USDC;
            uint64 lpTokenId = 100;

            uint64 firstOriginNonce = 100;
            uint64 secondOriginNonce = 50;


            address lpToken = createLpToken();
            //设置lp token
            //1 aave 2 compound
            arrow.setLpToken(1,USDC, lpToken);
            //给资管合约 mint lpToken
            mintLpToken(lpToken, address(arrow), 1000000000000000);
            //添加lp token
            addLpToken(lpToken, lpTokenId);

            //添加lp token id
            addLpTokenID(protocolType, tokenID, lpTokenId);
            //设置invest合约
            setInvestContract(address(arrow));

            //构造stake calldata
            changePrank(USDCWhale);
            IERC20(USDC).transfer(address(vault), 100_000_000);
            changePrank(deployer);

            // Create Defi payload
            uint8 senderAddressLength = 32;
            bytes memory senderAddress = abi.encode(0);
            uint8 targetChain = chainID;
            uint8 recipientAddressLength = 20;
            address recipientAddress = bridgerA;
            uint64 amount = 1_000_000;
            uint16 eventIdx = 0;
            //uint64 protocolType=1; 
            uint64 protocolVersion=1;
            uint64 protocolTokenID=tokenID;
            uint8 actionType=0;

            bytes memory payload = abi.encodePacked(
                senderAddressLength,
                senderAddress,
                targetChain,
                amount,
                new bytes(0),
                eventIdx,
                protocolType,
                protocolVersion,
                protocolTokenID,
                actionType
            );

            // Create Defi message
            BridgeUtils.Message memory message = BridgeUtils.Message({
                messageType: BridgeUtils.DEFI,
                version: 1,
                nonce: firstOriginNonce,
                chainID: 0,
                payload: payload
            });


            bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
            bytes32 messageHash = keccak256(encodedMessage);

            //signatures
            bytes[] memory signatures = new bytes[](4);

            signatures[0] = getSignature(messageHash, committeeMemberPkA);
            signatures[1] = getSignature(messageHash, committeeMemberPkB);
            signatures[2] = getSignature(messageHash, committeeMemberPkC);
            signatures[3] = getSignature(messageHash, committeeMemberPkD);

            uint256 bbalance=IERC20(lpToken).balanceOf(address(vault));


            //期望事件

            vm.expectEmit(true, true, true, true);
            // emit Invested(0, USDC, 1_000_000);
            emit TokensStaked(
                1,// evm
                0, //nonce from evm
                0, //destination chain id (from benfen)
                firstOriginNonce, //origin nonce(from benfen) first
                senderAddress, //senderAddress
                address(arrow), //recipientAddress (mock arrow)
                0, //erc20AdjustedAmount
                amount, //erc20lpTokenAmount
                protocolType,
                protocolVersion,
                protocolTokenID,
                actionType
            );
            //调用
            bridge.investBridgedTokensWithSignatures(signatures, message);

            uint256 abalance=IERC20(lpToken).balanceOf(address(vault));

            assertEq(abalance-bbalance,amount);



            message = BridgeUtils.Message({
                messageType: BridgeUtils.DEFI,
                version: 1,
                nonce: secondOriginNonce,
                chainID: 0,
                payload: payload
            });


            encodedMessage = BridgeUtils.encodeMessage(message);
            messageHash = keccak256(encodedMessage);

            //signatures
            // bytes[] memory signatures = new bytes[](4);

            signatures[0] = getSignature(messageHash, committeeMemberPkA);
            signatures[1] = getSignature(messageHash, committeeMemberPkB);
            signatures[2] = getSignature(messageHash, committeeMemberPkC);
            signatures[3] = getSignature(messageHash, committeeMemberPkD);

            bbalance=IERC20(lpToken).balanceOf(address(vault));


            //期望事件

            vm.expectEmit(true, true, true, true);
            emit TokensStaked(
                1,// evm
                1, //nonce from evm (第二笔已经递增)
                0, //destination chain id (from benfen)
                secondOriginNonce, //origin nonce(from benfen) second
                senderAddress, //senderAddress
                address(arrow), //recipientAddress (mock arrow)
                0, //erc20AdjustedAmount
                amount, //erc20lpTokenAmount
                protocolType,
                protocolVersion,
                protocolTokenID,
                actionType
            );
            //调用
            bridge.investBridgedTokensWithSignatures(signatures, message);

            abalance=IERC20(lpToken).balanceOf(address(vault));

            assertEq(abalance-bbalance,amount);
    } 

    function testInvestStakedUSDCUseTwiceSignatures() public {
            //部署invest 合约
            MockArrow arrow = new MockArrow(address(vault));
            uint64 protocolType = 1;
            uint64 tokenID = BridgeUtils.USDC;
            uint64 lpTokenId = 100;

            address lpToken = createLpToken();
            //设置lp token
            //1 aave 2 compound
            arrow.setLpToken(1,USDC, lpToken);
            //给资管合约 mint lpToken
            mintLpToken(lpToken, address(arrow), 1000000000000000);
            //添加lp token
            addLpToken(lpToken, lpTokenId);

            //添加lp token id
            addLpTokenID(protocolType, tokenID, lpTokenId);
            //设置invest合约
            setInvestContract(address(arrow));

            //构造stake calldata
            changePrank(USDCWhale);
            IERC20(USDC).transfer(address(vault), 100_000_000);
            changePrank(deployer);

            // Create Defi payload
            uint8 senderAddressLength = 32;
            bytes memory senderAddress = abi.encode(0);
            uint8 targetChain = chainID;
            uint8 recipientAddressLength = 20;
            address recipientAddress = bridgerA;
            uint64 amount = 1_000_000;
            uint16 eventIdx = 0;
            //uint64 protocolType=1; 
            uint64 protocolVersion=1;
            uint64 protocolTokenID=tokenID;
            uint8 actionType=0;

            bytes memory payload = abi.encodePacked(
                senderAddressLength,
                senderAddress,
                targetChain,
                amount,
                new bytes(0),
                eventIdx,
                protocolType,
                protocolVersion,
                protocolTokenID,
                actionType
            );

            // Create Defi message
            BridgeUtils.Message memory message = BridgeUtils.Message({
                messageType: BridgeUtils.DEFI,
                version: 1,
                nonce: 1,
                chainID: 0,
                payload: payload
            });


            bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
            bytes32 messageHash = keccak256(encodedMessage);

            //signatures
            bytes[] memory signatures = new bytes[](4);

            signatures[0] = getSignature(messageHash, committeeMemberPkA);
            signatures[1] = getSignature(messageHash, committeeMemberPkB);
            signatures[2] = getSignature(messageHash, committeeMemberPkC);
            signatures[3] = getSignature(messageHash, committeeMemberPkD);

            uint256 bbalance=IERC20(lpToken).balanceOf(address(vault));

            //调用
            bridge.investBridgedTokensWithSignatures(signatures, message);

            uint256 abalance=IERC20(lpToken).balanceOf(address(vault));

            assertEq(abalance-bbalance,amount);

            vm.expectRevert("SuiBridge: Message already processed");

            bridge.investBridgedTokensWithSignatures(signatures, message);
         
    }

    function testInvestUnStakedUSDCWithValidSignatures() public {
         //部署invest 合约
        MockArrow arrow = new MockArrow(address(vault));
        uint64 protocolType = 1;
        uint64 tokenID = BridgeUtils.USDC;
        uint64 lpTokenId = 100;

        address lpToken = createLpToken();
        //设置lp token
        arrow.setAsset(1,USDC, lpToken);
        //给资管合约 mint lpToken
        mintLpToken(lpToken, address(vault), 1000000000000000);
        //添加lp token
        addLpToken(lpToken, lpTokenId);

        //添加lp token id
        addLpTokenID(protocolType, tokenID, lpTokenId);
        //设置invest合约
        setInvestContract(address(arrow));

        //构造stake calldata
        changePrank(USDCWhale);
        IERC20(USDC).transfer(address(arrow), 100_000_000_000);
        changePrank(deployer);

        // Create Defi payload
        uint8 senderAddressLength = 32;
        bytes memory senderAddress = abi.encode(0);
        uint8 targetChain = chainID;
        uint8 recipientAddressLength = 20;
        address recipientAddress = bridgerA;
        uint64 amount = 1_000_000;
        uint16 eventIdx = 0;
        //uint64 protocolType=1; 
        uint64 protocolVersion=1;
        uint64 protocolTokenID=tokenID;
        uint8 actionType=1;

        bytes memory payload = abi.encodePacked(
            senderAddressLength,
            senderAddress,
            targetChain,
            amount,
            new bytes(0),
            eventIdx,
            protocolType,
            protocolVersion,
            protocolTokenID,
            actionType
        );

        // Create Defi message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.DEFI,
            version: 1,
            nonce: 1,
            chainID: 0,
            payload: payload
        });


        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(encodedMessage);

        //signatures
        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        uint256 bbalance=IERC20(USDC).balanceOf(address(vault));

        //调用
        bridge.investBridgedTokensWithSignatures(signatures, message);

        uint256 abalance=IERC20(USDC).balanceOf(address(vault));

        assertEq(abalance-bbalance,amount);
    }

    function testInvestUnStakedUSDCVerifyTokensUnStakedEvent() public {
         //部署invest 合约
        MockArrow arrow = new MockArrow(address(vault));
        uint64 protocolType = 1;
        uint64 tokenID = BridgeUtils.USDC;
        uint64 lpTokenId = 100;
        uint64 originNonce = 100;

        address lpToken = createLpToken();
        //设置lp token
        arrow.setAsset(1,USDC, lpToken);
        //给资管合约 mint lpToken
        mintLpToken(lpToken, address(vault), 1000000000000000);
        //添加lp token
        addLpToken(lpToken, lpTokenId);

        //添加lp token id
        addLpTokenID(protocolType, tokenID, lpTokenId);
        //设置invest合约
        setInvestContract(address(arrow));

        //构造stake calldata
        changePrank(USDCWhale);
        IERC20(USDC).transfer(address(arrow), 100_000_000_000);
        changePrank(deployer);

        // Create Defi payload
        uint8 senderAddressLength = 32;
        bytes memory senderAddress = abi.encode(0);
        uint8 targetChain = chainID;
        uint8 recipientAddressLength = 20;
        address recipientAddress = bridgerA;
        uint64 amount = 1_000_000;
        uint16 eventIdx = 0;
        //uint64 protocolType=1; 
        uint64 protocolVersion=1;
        uint64 protocolTokenID=tokenID;
        uint8 actionType=1;

        bytes memory payload = abi.encodePacked(
            senderAddressLength,
            senderAddress,
            targetChain,
            amount,
            new bytes(0),
            eventIdx,
            protocolType,
            protocolVersion,
            protocolTokenID,
            actionType
        );

        // Create Defi message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.DEFI,
            version: 1,
            nonce: originNonce,
            chainID: 0,
            payload: payload
        });


        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(encodedMessage);

        //signatures
        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        uint256 bbalance=IERC20(USDC).balanceOf(address(vault));


        vm.expectEmit(true,true,true,true);
        emit TokensUnStaked(
            chainID,  //sourceChainID: unstake from evm
            0,  //nonce: unstake nonce first
            0, // destinationChainID: benfen chain id
            originNonce, //originNonce :from benfen
            senderAddress, //recipientAddress: benfen address
            address(arrow),  //senderAddress: evm address
            amount, 
            amount, 
            protocolType, 
            protocolVersion, 
            protocolTokenID, 
            actionType
        );

        //调用
        bridge.investBridgedTokensWithSignatures(signatures, message);

        uint256 abalance=IERC20(USDC).balanceOf(address(vault));

        assertEq(abalance-bbalance,amount);
    }


    function testInvestUnStakedUSDCVerifyTokensUnStakedeTwiceEvent() public {
        //部署invest 合约
        MockArrow arrow = new MockArrow(address(vault));
        uint64 protocolType = 1;
        uint64 tokenID = BridgeUtils.USDC;
        uint64 lpTokenId = 100;
        uint64 firstOriginNonce = 100;
        uint64 secondOriginNonce=50;

        address lpToken = createLpToken();
        //设置lp token
        arrow.setAsset(1,USDC, lpToken);
        //给资管合约 mint lpToken
        mintLpToken(lpToken, address(vault), 1000000000000000);
        //添加lp token
        addLpToken(lpToken, lpTokenId);

        //添加lp token id
        addLpTokenID(protocolType, tokenID, lpTokenId);
        //设置invest合约
        setInvestContract(address(arrow));

        //构造stake calldata
        changePrank(USDCWhale);
        IERC20(USDC).transfer(address(arrow), 100_000_000_000);
        changePrank(deployer);

        // Create Defi payload
        uint8 senderAddressLength = 32;
        bytes memory senderAddress = abi.encode(0);
        uint8 targetChain = chainID;
        uint8 recipientAddressLength = 20;
        address recipientAddress = bridgerA;
        uint64 amount = 1_000_000;
        uint16 eventIdx = 0;
        //uint64 protocolType=1; 
        uint64 protocolVersion=1;
        uint64 protocolTokenID=tokenID;
        uint8 actionType=1;

        bytes memory payload = abi.encodePacked(
            senderAddressLength,
            senderAddress,
            targetChain,
            amount,
            new bytes(0),
            eventIdx,
            protocolType,
            protocolVersion,
            protocolTokenID,
            actionType
        );

        // Create Defi message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.DEFI,
            version: 1,
            nonce: firstOriginNonce,
            chainID: 0,
            payload: payload
        });


        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(encodedMessage);

        //signatures
        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        uint256 bbalance=IERC20(USDC).balanceOf(address(vault));


        vm.expectEmit(true,true,true,true);
        emit TokensUnStaked(
            chainID,  //sourceChainID: unstake from evm
            0,  //nonce: unstake nonce first
            0, // destinationChainID: benfen chain id
            firstOriginNonce, //originNonce :from benfen
            senderAddress, //recipientAddress: benfen address
            address(arrow),  //senderAddress: evm address
            amount, 
            amount, 
            protocolType, 
            protocolVersion, 
            protocolTokenID, 
            actionType
        );

        //调用
        bridge.investBridgedTokensWithSignatures(signatures, message);

        uint256 abalance=IERC20(USDC).balanceOf(address(vault));

        assertEq(abalance-bbalance,amount);


        message = BridgeUtils.Message({
            messageType: BridgeUtils.DEFI,
            version: 1,
            nonce: secondOriginNonce,
            chainID: 0,
            payload: payload
        });


        encodedMessage = BridgeUtils.encodeMessage(message);
        messageHash = keccak256(encodedMessage);


        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        bbalance=IERC20(USDC).balanceOf(address(vault));

        //调用第二次
         vm.expectEmit(true, true, true, true);
         emit TokensUnStaked(
            chainID,  //sourceChainID: unstake from evm
            1,  //nonce: unstake nonce second
            0, // destinationChainID: benfen chain id
            secondOriginNonce, //originNonce :from benfen
            senderAddress, //recipientAddress: benfen address
            address(arrow),  //senderAddress: evm address
            amount, 
            amount, 
            protocolType, 
            protocolVersion, 
            protocolTokenID, 
            actionType
        );

        //调用
        bridge.investBridgedTokensWithSignatures(signatures, message);

        abalance=IERC20(USDC).balanceOf(address(vault));
        assertEq(abalance-bbalance,amount);
    }


    function testInvestUnStakedUSDCUseTwiceSignatures() public {
        //部署invest 合约
        MockArrow arrow = new MockArrow(address(vault));
        uint64 protocolType = 1;
        uint64 tokenID = BridgeUtils.USDC;
        uint64 lpTokenId = 100;

        address lpToken = createLpToken();
        //设置lp token
        arrow.setAsset(1,USDC, lpToken);
        //给资管合约 mint lpToken
        mintLpToken(lpToken, address(vault), 1000000000000000);
        //添加lp token
        addLpToken(lpToken, lpTokenId);

        //添加lp token id
        addLpTokenID(protocolType, tokenID, lpTokenId);
        //设置invest合约
        setInvestContract(address(arrow));

        //构造stake calldata
        changePrank(USDCWhale);
        IERC20(USDC).transfer(address(arrow), 100_000_000_000);
        changePrank(deployer);

        // Create Defi payload
        uint8 senderAddressLength = 32;
        bytes memory senderAddress = abi.encode(0);
        uint8 targetChain = chainID;
        uint8 recipientAddressLength = 20;
        address recipientAddress = bridgerA;
        uint64 amount = 1_000_000;
        uint16 eventIdx = 0;
        //uint64 protocolType=1; 
        uint64 protocolVersion=1;
        uint64 protocolTokenID=tokenID;
        uint8 actionType=1;

        bytes memory payload = abi.encodePacked(
            senderAddressLength,
            senderAddress,
            targetChain,
            amount,
            new bytes(0),
            eventIdx,
            protocolType,
            protocolVersion,
            protocolTokenID,
            actionType
        );

        // Create Defi message
        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.DEFI,
            version: 1,
            nonce: 1,
            chainID: 0,
            payload: payload
        });


        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);
        bytes32 messageHash = keccak256(encodedMessage);

        //signatures
        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);

        uint256 bbalance=IERC20(USDC).balanceOf(address(vault));

        //调用
        bridge.investBridgedTokensWithSignatures(signatures, message);

        uint256 abalance=IERC20(USDC).balanceOf(address(vault));

        assertEq(abalance-bbalance,amount);
        vm.expectRevert("SuiBridge: Message already processed");
        bridge.investBridgedTokensWithSignatures(signatures, message);
    }

    function testUpdateInvestAddressWithSignatures() public {
        //部署invest 合约
        MockArrow arrow = new MockArrow(address(vault));
        //设置invest合约


        bytes memory payload = abi.encodePacked(
           address(arrow)
        );

        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.UPDATE_INVEST_ADDRESS,
            version: 1,
            nonce: 0,
            chainID: 1,
            payload: payload
        });

        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);

        bytes32 messageHash = keccak256(encodedMessage);

        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);  
        bridge.updateInvestAddressWithSignatures(signatures, message);
        assertEq(address(arrow), bridge.getInvestAddress());
    }


    function testUpdateInvestAddressVerifyUpdateInvestAddressEvent() public {
        //部署invest 合约
        MockArrow arrow = new MockArrow(address(vault));
        //设置invest合约

         bytes memory payload = abi.encodePacked(
           address(arrow)
        );

        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.UPDATE_INVEST_ADDRESS,
            version: 1,
            nonce: 0,
            chainID: 1,
            payload: payload
        });

        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);

        bytes32 messageHash = keccak256(encodedMessage);

        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);  
        vm.expectEmit(false, false, false, true);
        emit UpdateInvestAddress(message.nonce, address(arrow));
        bridge.updateInvestAddressWithSignatures(signatures, message);
        assertEq(address(arrow), bridge.getInvestAddress());
    }


    function testUpdateInvestAddressWithInvalidNonce() public {
        //部署invest 合约
        MockArrow arrow = new MockArrow(address(vault));



         bytes memory payload = abi.encodePacked(
           address(arrow)
        );

        BridgeUtils.Message memory message = BridgeUtils.Message({
            messageType: BridgeUtils.UPDATE_INVEST_ADDRESS,
            version: 1,
            nonce: 1,// error 正确的应该是0(对于非DEFI和TOKEN_TRANSFER)
            chainID: 1,
            payload: payload
        });

        bytes memory encodedMessage = BridgeUtils.encodeMessage(message);

        bytes32 messageHash = keccak256(encodedMessage);

        bytes[] memory signatures = new bytes[](4);

        signatures[0] = getSignature(messageHash, committeeMemberPkA);
        signatures[1] = getSignature(messageHash, committeeMemberPkB);
        signatures[2] = getSignature(messageHash, committeeMemberPkC);
        signatures[3] = getSignature(messageHash, committeeMemberPkD);  
      
        vm.expectRevert("MessageVerifier: Invalid nonce");
        bridge.updateInvestAddressWithSignatures(signatures, message);
    }



}