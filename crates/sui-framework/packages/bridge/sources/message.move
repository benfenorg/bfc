// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::message {
    use std::ascii::{Self, String};
    use sui::bcs::{Self, BCS};


    use bridge::chain_ids;
    use bridge::message_types;

    const CURRENT_MESSAGE_VERSION: u8 = 1;
    const CURRENT_MESSAGE_VERSION_V2: u8 = 2;
    const CURRENT_MESSAGE_VERSION_V3: u8 = 3;
    const ECDSA_ADDRESS_LENGTH: u64 = 20;

    const ETrailingBytes: u64 = 0;
    const EInvalidAddressLength: u64 = 1;
    const EEmptyList: u64 = 2;
    const EInvalidMessageType: u64 = 3;
    const EInvalidEmergencyOpType: u64 = 4;
    // const EInvalidPayloadLength: u64 = 5;
    const EMustBeTokenMessage: u64 = 6;
    const EInvalidOperationType: u64 = 7;
    const EMustBeDefiMessage: u64 = 8;
    // Emergency Op types
    const PAUSE: u8 = 0;
    const UNPAUSE: u8 = 1;

    const ADD: u8 =0;
    const REMOVE: u8 = 1;

    //////////////////////////////////////////////////////
    // Types
    //

    public struct BridgeMessage has copy, drop, store {
        message_type: u8,
        message_version: u8,
        seq_num: u64,
        source_chain: u8,
        payload: vector<u8>
    }

     public struct BitcoinMessage has copy, drop, store {
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: vector<u8>,
        coin_type: vector<u8>
    }

    public struct BridgeMessageKey has copy, drop, store {
        source_chain: u8,
        message_type: u8,
        bridge_seq_num: u64
    }

    public struct RefundMessageKey has copy, drop, store {
        tx_hash: vector<u8>,
    }

    public struct FastPathLimitPayload has drop{
        chain_id: u8,
        token_id: u64,
        amount: u64,
    }

    public fun chain_id(self: &FastPathLimitPayload): u8 {
        self.chain_id
    }

    public fun token_id(self: &FastPathLimitPayload): u64 {
        self.token_id
    }

    public fun amount(self: &FastPathLimitPayload): u64 {
        self.amount
    }
    public struct TokenTransferPayload has drop {
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u8,
    }

    public struct TokenTransferPayloadV2 has drop {
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
    }

    public struct DefiTransferOutPayload has drop {
        sender_address: vector<u8>,
        target_chain: u8,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
        protocol_type: u64,// 0:aave, 1:compound, 2:curve 等等
        protocol_version: u64,
        protocol_token_id: u64,
        action_type: u8, // 0:stake, 1:unstake
        principal_amount: u64,
    }

    public struct DefiTransferInPayload has drop {
        sender_address: vector<u8>,
        target_chain: u8,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
        fast_path_selector: u8,
        protocol_type: u64,// 0:aave, 1:compound, 2:curve 等等
        protocol_version: u64,
        protocol_token_id: u64,
        original_seq_num: u64,
        action_type: u8, // 0:stake, 1:unstake
        lp_token_amount: u64,
        principal_amount: u64,
    }

    public struct TokenTransferInPayload has drop {
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
        fast_path_selector: u8,
    }

    

    public struct EmergencyOp has drop {
        op_type: u8
    }

    public struct Blocklist has drop {
        blocklist_type: u8,
        validator_eth_addresses: vector<vector<u8>>
    }

    public struct RefundAdmin has drop {
        op_type: u8,
        sui_address: String
    }

    // Update the limit for route from sending_chain to receiving_chain
    // This message is supposed to be processed by `chain` or the receiving chain
    public struct UpdateBridgeLimit has drop {
        // The receiving chain, also the chain that checks and processes this message
        receiving_chain: u8,
        // The sending chain
        sending_chain: u8,
        limit: u64
    }

    public struct UpdateAssetPrice has drop {
        token_id: u64,
        new_price: u64
    }

    public struct AddExternalCoinAdmin has drop {
        coin_type: String,
        admin_address: String,
    }

    public struct RemoveExternalCoinAdmin has drop {
        coin_type: String,
        admin_address: String,
    }

    public struct AddExternalCoinWitness has drop {
        coin_type: String,
        witness_address: vector<u8>,
    }

    public struct RemoveExternalCoinWitness has drop {
        coin_type: String,
        witness_address: vector<u8>,
    }

    public struct AddExternalCoinTarget has drop {
        coin_type: String,
        target_address: String,
    }

    public struct RemoveExternalCoinTarget has drop {
        coin_type: String,
        target_address: String,
    }

    public struct AddTokenOnSui has drop {
        native_token: bool,
        token_ids: vector<u64>,
        token_type_names: vector<String>,
        token_prices: vector<u64>
    }

    // For read
    public struct ParsedTokenTransferMessage has drop {
        message_version: u8,
        seq_num: u64,
        source_chain: u8,
        payload: vector<u8>,
        parsed_payload: TokenTransferPayload,
    }
    public struct ParsedTokenTransferMessageV2 has drop {
        message_version: u8,
        seq_num: u64,
        source_chain: u8,
        payload: vector<u8>,
        parsed_payload: TokenTransferPayloadV2,
    }

    public struct ParsedTokenTransferInMessage has drop {
        message_version: u8,
        seq_num: u64,
        source_chain: u8,
        payload: vector<u8>,
        parsed_payload: TokenTransferInPayload,
    }

    public struct ParsedDefiTransferOutMessage has drop {
        message_version: u8,
        seq_num: u64,
        source_chain: u8,
        payload: vector<u8>,
        parsed_payload: DefiTransferOutPayload,
    }

    public struct  AddTokenOnTokenList has drop {
        from_chain_id :u8,
        to_chain_id: u8,
        token_id: u64,
    }

    public struct  RemoveTokenOnTokenList has drop {
        from_chain_id :u8,
        to_chain_id: u8,
        token_id: u64,
    }

    public struct SetCrossOutBridgeFee has drop {
        chain_id: u8,
        token_id: u64,
        mode: u64,
        amount: u64
    }

    public struct SetCrossInBridgeFee has drop {
        chain_id: u8,
        token_id: u64,
        mode: u64,
        amount: u64
    }

    public struct WithdrawBridgeFee has drop{
        recipient :address,
        coin_type: String,
        amount: u64
    }


    //////////////////////////////////////////////////////
    // Public functions
    //

    // Note: `bcs::peel_vec_u8` *happens* to work here because
    // `sender_address` and `target_address` are no longer than 255 bytes.
    // Therefore their length can be represented by a single byte.
    // See `create_token_bridge_message` for the actual encoding rule.
    public fun extract_token_bridge_payload(message: &BridgeMessage): TokenTransferPayload {
        let mut bcs = bcs::new(message.payload);
        let sender_address = bcs.peel_vec_u8();
        let target_chain = bcs.peel_u8();
        let target_address = bcs.peel_vec_u8();
        let token_type = peel_u64_be(&mut bcs);
        let amount = peel_u64_be(&mut bcs);
        let tx_hash = bcs.peel_vec_u8();
        let event_idx = bcs.peel_u8();
        chain_ids::assert_valid_chain_id(target_chain);
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        TokenTransferPayload {
            sender_address,
            target_chain,
            target_address,
            token_type,
            amount,
            tx_hash,
            event_idx
        }
    }

    public fun extract_fast_path_limit_payload(message: &BridgeMessage): FastPathLimitPayload {
        let mut bcs = bcs::new(message.payload);
        let chain_id = bcs.peel_u8();
        let token_id = peel_u64_be(&mut bcs);
        let amount = peel_u64_be(&mut bcs);
        chain_ids::assert_valid_chain_id(chain_id);
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        FastPathLimitPayload {
            chain_id,
            token_id,
            amount,
        }
    }

    // Note: `bcs::peel_vec_u8` *happens* to work here because
    // `sender_address` and `target_address` are no longer than 255 bytes.
    // Therefore their length can be represented by a single byte.
    // See `create_token_bridge_message` for the actual encoding rule.
    public fun extract_token_bridge_payload_v2(message: &BridgeMessage): TokenTransferPayloadV2 {
        let mut bcs = bcs::new(message.payload);
        let sender_address = bcs.peel_vec_u8();
        let target_chain = bcs.peel_u8();
        let target_address = bcs.peel_vec_u8();
        let token_type = peel_u64_be(&mut bcs);
        let amount = peel_u64_be(&mut bcs);
        let tx_hash = bcs.peel_vec_u8();
        let event_idx = bcs.peel_u16();
        chain_ids::assert_valid_chain_id(target_chain);
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        TokenTransferPayloadV2 {
            sender_address,
            target_chain,
            target_address,
            token_type,
            amount,
            tx_hash,
            event_idx,
        }
    }

    public fun extract_token_bridge_in_payload(message: &BridgeMessage): TokenTransferInPayload {
        let mut bcs = bcs::new(message.payload);
        let sender_address = bcs.peel_vec_u8();
        let target_chain = bcs.peel_u8();
        let target_address = bcs.peel_vec_u8();
        let token_type = peel_u64_be(&mut bcs);
        let amount = peel_u64_be(&mut bcs);
        let tx_hash = bcs.peel_vec_u8();
        let event_idx = bcs.peel_u16();
        let fast_path_selector = bcs.peel_u8();
        chain_ids::assert_valid_chain_id(target_chain);
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        TokenTransferInPayload {
            sender_address,
            target_chain,
            target_address,
            token_type,
            amount,
            tx_hash,
            event_idx,
            fast_path_selector
        }
    }

    public fun extract_add_witness_poyload(message: &BridgeMessage): AddExternalCoinWitness {
        let mut bcs = bcs::new(message.payload);
        let coin_type = ascii::string(bcs.peel_vec_u8());
        let (mut witness_address, mut i) = (vector[], 0);
        while (i < ECDSA_ADDRESS_LENGTH) {
                witness_address.push_back(bcs.peel_u8());
                i = i + 1;
        };
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);
         AddExternalCoinWitness {
            coin_type,
            witness_address
         }
    }

    public fun extract_remove_witness_poyload(message: &BridgeMessage): RemoveExternalCoinWitness {
        let mut bcs = bcs::new(message.payload);
        let coin_type = ascii::string(bcs.peel_vec_u8());
        let (mut witness_address, mut i) = (vector[], 0);
         while (i < ECDSA_ADDRESS_LENGTH) {
                witness_address.push_back(bcs.peel_u8());
                i = i + 1;
        };
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);
        RemoveExternalCoinWitness {
            coin_type,
            witness_address
        }
    }

      public fun extract_add_external_target_address_poyload(message: &BridgeMessage): AddExternalCoinTarget {
        let mut bcs = bcs::new(message.payload);
        let coin_type = ascii::string(bcs.peel_vec_u8());
        let length=bcs.peel_u8();

        let (mut target_address, mut i) = (vector[], 0);
        while (i < length) {
                target_address.push_back(bcs.peel_u8());
                i = i + 1;
        };
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        let target_address = ascii::string(target_address);

         AddExternalCoinTarget {
            coin_type,
            target_address
         }
    }

    public fun extract_remove_external_target_address_poyload(message: &BridgeMessage): RemoveExternalCoinTarget {
        let mut bcs = bcs::new(message.payload);
        let coin_type = ascii::string(bcs.peel_vec_u8());
        let length=bcs.peel_u8();
        let (mut target_address, mut i) = (vector[], 0);

        while (i < length) {
                target_address.push_back(bcs.peel_u8());
                i = i + 1;
        };
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);
        let target_address = ascii::string(target_address);
        RemoveExternalCoinTarget {
            coin_type,
            target_address
        }
    }


    public fun extract_add_token_on_token_list_poyload(message: &BridgeMessage): AddTokenOnTokenList{
           let mut bcs = bcs::new(message.payload);
           let from_chain_id=bcs.peel_u8();
           let to_chain_id=bcs.peel_u8();
           let token_id=peel_u64_be(&mut bcs);

           AddTokenOnTokenList{
            from_chain_id,
            to_chain_id,
            token_id
           }
    }

    public fun extract_remove_token_on_token_list_poyload(message: &BridgeMessage): RemoveTokenOnTokenList{
           let mut bcs = bcs::new(message.payload);
           let from_chain_id=bcs.peel_u8();
           let to_chain_id=bcs.peel_u8();
           let token_id=peel_u64_be(&mut bcs);

           RemoveTokenOnTokenList{
            from_chain_id,
            to_chain_id,
            token_id
           }
    }
    //SetCrossOutBridgeFee
    public fun extract_set_cross_out_bridge_fee_poyload(message: &BridgeMessage): SetCrossOutBridgeFee{
           let mut bcs = bcs::new(message.payload);
           let chain_id=bcs.peel_u8();
           let token_id=peel_u64_be(&mut bcs);
           let mode=peel_u64_be(&mut bcs);
           let amount=peel_u64_be(&mut bcs);


           SetCrossOutBridgeFee{
            chain_id,
            token_id,
            mode,
            amount
           }
    }

    public fun extract_set_cross_in_bridge_fee_poyload(message: &BridgeMessage): SetCrossInBridgeFee{
           let mut bcs = bcs::new(message.payload);
           let chain_id=bcs.peel_u8();
           let token_id=peel_u64_be(&mut bcs);
           let mode=peel_u64_be(&mut bcs);
           let amount=peel_u64_be(&mut bcs);

           SetCrossInBridgeFee{
            chain_id,
            token_id,
            mode,
            amount
           }
    }

    public fun extract_withdraw_bridge_fee(message: &BridgeMessage) :WithdrawBridgeFee{
        let mut bcs = bcs::new(message.payload);
        let recipient=bcs.peel_address();
        let coin_type = ascii::string(bcs.peel_vec_u8());
        let amount=peel_u64_be(&mut bcs);

        WithdrawBridgeFee{
            recipient,
            coin_type,
            amount,
        }
    }

    /// Emergency op payload is just a single byte
    public fun extract_emergency_op_payload(message: &BridgeMessage): EmergencyOp {
        assert!(message.payload.length() == 1, ETrailingBytes);
        EmergencyOp { op_type: message.payload[0] }
    }

    public fun extract_blocklist_payload(message: &BridgeMessage): Blocklist {
        // blocklist payload should consist of one byte blocklist type, and list of 20 bytes evm addresses
        // derived from ECDSA public keys
        let mut bcs = bcs::new(message.payload);
        let blocklist_type = bcs.peel_u8();
        let mut address_count = bcs.peel_u8();

        assert!(address_count != 0, EEmptyList);

        let mut validator_eth_addresses = vector[];
        while (address_count > 0) {
            let (mut address, mut i) = (vector[], 0);
            while (i < ECDSA_ADDRESS_LENGTH) {
                address.push_back(bcs.peel_u8());
                i = i + 1;
            };
            validator_eth_addresses.push_back(address);
            address_count = address_count - 1;
        };

        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        Blocklist {
            blocklist_type,
            validator_eth_addresses
        }
    }

    public fun extract_refund_admin_payload(message: &BridgeMessage): RefundAdmin {
        // blocklist payload should consist of one byte blocklist type, and list of 20 bytes evm addresses
        // derived from ECDSA public keys
        let mut bcs = bcs::new(message.payload);
        let op_type = bcs.peel_u8();
        let sui_address = ascii::string(bcs.peel_vec_u8());

        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        assert!(op_type == ADD || op_type == REMOVE, EInvalidOperationType);

        RefundAdmin {
            op_type,
            sui_address
        }
    }

    public fun extract_update_bridge_limit(message: &BridgeMessage): UpdateBridgeLimit {
        let mut bcs = bcs::new(message.payload);
        let sending_chain = bcs.peel_u8();
        let limit = peel_u64_be(&mut bcs);

        chain_ids::assert_valid_chain_id(sending_chain);
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        UpdateBridgeLimit {
            receiving_chain: message.source_chain,
            sending_chain,
            limit
        }
    }

    public fun extract_add_external_coin_admin(message: &BridgeMessage): AddExternalCoinAdmin {
        let mut bcs = bcs::new(message.payload);
        let coin_type = ascii::string(bcs.peel_vec_u8());
        let admin_address = ascii::string(bcs.peel_vec_u8());

        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        AddExternalCoinAdmin {
            coin_type,
            admin_address
        }
    }

    public fun extract_remove_external_coin_admin(message: &BridgeMessage): RemoveExternalCoinAdmin {
        let mut bcs = bcs::new(message.payload);
        let coin_type = ascii::string(bcs.peel_vec_u8());
        let admin_address = ascii::string(bcs.peel_vec_u8());
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);
        RemoveExternalCoinAdmin {
            coin_type,
            admin_address
        }
    }

    public fun extract_update_asset_price(message: &BridgeMessage): UpdateAssetPrice {
        let mut bcs = bcs::new(message.payload);
        let token_id = peel_u64_be(&mut bcs);
        let new_price = peel_u64_be(&mut bcs);

        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        UpdateAssetPrice {
            token_id,
            new_price
        }
    }

    public fun extract_add_tokens_on_sui(message: &BridgeMessage): AddTokenOnSui {
        let mut bcs = bcs::new(message.payload);
        let native_token = bcs.peel_bool();
        let token_ids = bcs.peel_vec_u64();
        let token_type_names_bytes = bcs.peel_vec_vec_u8();
        let token_prices = bcs.peel_vec_u64();

        let mut n = 0;
        let mut token_type_names = vector[];
        while (n < token_type_names_bytes.length()){
            token_type_names.push_back(ascii::string(*token_type_names_bytes.borrow(n)));
            n = n + 1;
        };
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);
        AddTokenOnSui {
            native_token,
            token_ids,
            token_type_names,
            token_prices
        }
    }

    public fun serialize_message(message: BridgeMessage): vector<u8> {
        let BridgeMessage {
            message_type,
            message_version,
            seq_num,
            source_chain,
            payload
        } = message;

        let mut message = vector[
            message_type,
            message_version,
        ];

        // bcs serializes u64 as 8 bytes
        message.append(reverse_bytes(bcs::to_bytes(&seq_num)));
        message.push_back(source_chain);
        message.append(payload);
        message
    }

    public fun serialize_bitcoin_message(message: BitcoinMessage): vector<u8>{
        let BitcoinMessage {
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
        coin_type,
        } = message;
         let mut message=vector[
            source_chain,
         ];
        message.append(reverse_bytes(bcs::to_bytes(&amount)));
        message.append(source_address);
        message.append(target_address);
        message.append(tx_hash);
        message.append(coin_type);
        message
    }

    public fun create_bitcoin_message (
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: vector<u8>,
        coin_type: vector<u8>
    ):BitcoinMessage{
        BitcoinMessage{
            source_chain,
            source_address,
            target_address,
            amount,
            tx_hash,
            coin_type,
        }
    }

    /// Token Transfer Message Format:
    /// [message_type: u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [source_chain: u8]
    /// [sender_address_length:u8]
    /// [sender_address: byte[]]
    /// [target_chain:u8]
    /// [target_address_length:u8]
    /// [target_address: byte[]]
    /// [token_type:u8]
    /// [amount:u64]
    public fun create_token_bridge_message(
        source_chain: u8,
        seq_num: u64,
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u8,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);
        chain_ids::assert_valid_chain_id(target_chain);

        let mut payload = vector[];

        // sender address should be less than 255 bytes so can fit into u8
        payload.push_back((vector::length(&sender_address) as u8));
        payload.append(sender_address);
        payload.push_back(target_chain);
        // target address should be less than 255 bytes so can fit into u8
        payload.push_back((vector::length(&target_address) as u8));
        payload.append(target_address);
        // bcs serialzies u64 as 8 bytes
        payload.append(reverse_bytes(bcs::to_bytes(&token_type)));
        payload.append(reverse_bytes(bcs::to_bytes(&amount)));

        // btc address len is different from eth address len, so we can't assert palyload length
        // assert!(vector::length(&payload) == 71, EInvalidPayloadLength);
        payload.push_back((vector::length(&tx_hash) as u8));
        payload.append(tx_hash);
        payload.push_back(event_idx);
        BridgeMessage {
            message_type: message_types::token(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }

    //bridge to benfen message
    public fun create_token_bridge_in_message(
        source_chain: u8,
        seq_num: u64,
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
        fast_path_selector: u8,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);
        chain_ids::assert_valid_chain_id(target_chain);

        let mut payload = vector[];

        // sender address should be less than 255 bytes so can fit into u8
        payload.push_back((vector::length(&sender_address) as u8));
        payload.append(sender_address);
        payload.push_back(target_chain);
        // target address should be less than 255 bytes so can fit into u8
        payload.push_back((vector::length(&target_address) as u8));
        payload.append(target_address);
        // bcs serialzies u64 as 8 bytes
        payload.append(reverse_bytes(bcs::to_bytes(&token_type)));
        payload.append(reverse_bytes(bcs::to_bytes(&amount)));

        // btc address len is different from eth address len, so we can't assert palyload length
        // assert!(vector::length(&payload) == 71, EInvalidPayloadLength);
        payload.push_back((vector::length(&tx_hash) as u8));
        payload.append(tx_hash);
        payload.append(reverse_bytes(bcs::to_bytes(&event_idx)));
        payload.push_back(fast_path_selector);
        BridgeMessage {
            message_type: message_types::token(),
            message_version: CURRENT_MESSAGE_VERSION_V2,
            seq_num,
            source_chain,
            payload,
        }
    }

    public fun create_token_bridge_message_v2(
        source_chain: u8,
        seq_num: u64,
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);
        chain_ids::assert_valid_chain_id(target_chain);

        let mut payload = vector[];

        // sender address should be less than 255 bytes so can fit into u8
        payload.push_back((vector::length(&sender_address) as u8));
        payload.append(sender_address);
        payload.push_back(target_chain);
        // target address should be less than 255 bytes so can fit into u8
        payload.push_back((vector::length(&target_address) as u8));
        payload.append(target_address);
        // bcs serialzies u64 as 8 bytes
        payload.append(reverse_bytes(bcs::to_bytes(&token_type)));
        payload.append(reverse_bytes(bcs::to_bytes(&amount)));

        // btc address len is different from eth address len, so we can't assert palyload length
        // assert!(vector::length(&payload) == 71, EInvalidPayloadLength);
        payload.push_back((vector::length(&tx_hash) as u8));
        payload.append(tx_hash);
        payload.append(reverse_bytes(bcs::to_bytes(&event_idx)));
        BridgeMessage {
            message_type: message_types::token(),
            message_version: CURRENT_MESSAGE_VERSION_V3,
            seq_num,
            source_chain,
            payload,
        }
    }

    /// Emergency Op Message Format:
    /// [message_type: u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [chain_id: u8]
    /// [op_type: u8]
    public fun create_emergency_op_message(
        source_chain: u8,
        seq_num: u64,
        op_type: u8,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);

        BridgeMessage {
            message_type: message_types::emergency_op(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload: vector[op_type],
        }
    }

    /// Blocklist Message Format:
    /// [message_type: u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [chain_id: u8]
    /// [blocklist_type: u8]
    /// [validator_length: u8]
    /// [validator_ecdsa_addresses: byte[][]]
    public fun create_blocklist_message(
        source_chain: u8,
        seq_num: u64,
        // 0: block, 1: unblock
        blocklist_type: u8,
        validator_ecdsa_addresses: vector<vector<u8>>,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);

        let address_length = validator_ecdsa_addresses.length();
        let mut payload = vector[blocklist_type, (address_length as u8)];
        let mut i = 0;

        while (i < address_length) {
            let address = validator_ecdsa_addresses[i];
            assert!(address.length() == ECDSA_ADDRESS_LENGTH, EInvalidAddressLength);
            payload.append(address);

            i = i + 1;
        };

        BridgeMessage {
            message_type: message_types::committee_blocklist(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }

    /// Blocklist Message Format:
    /// [message_type: u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [chain_id: u8]
    /// [op_type: u8]
    /// [address_admin: byte[][]]
    public fun create_refund_admin_message(
        source_chain: u8,
        seq_num: u64,
        // 0: add, 1: del
        op_type: u8,
        admin_address: String,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);
        let mut payload = bcs::to_bytes(&op_type);
        payload.append(bcs::to_bytes(&admin_address));

        BridgeMessage {
            message_type: message_types::refund_admin_operate(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }

    /// Blocklist Message Format:
    /// [message_type: u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [chain_id: u8]
    /// [token_id: u64]
    /// [amount: u64]
    /// 
    public fun create_fast_path_limit_message(
        seq_num: u64,
        chain_id: u8,
        token_id: u64,
        amount: u64,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(chain_id);
        let mut payload = bcs::to_bytes(&chain_id);
        payload.append(bcs::to_bytes(&chain_id));
        payload.append(bcs::to_bytes(&token_id));
        payload.append(bcs::to_bytes(&amount));

        BridgeMessage {
            message_type: message_types::update_bridge_limit_fast_path(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain: chain_id,
            payload,
        }
    }
    
    public fun create_fast_path_limit_message_v2(
        seq_num: u64,
        chain_id: u8,
        token_id: u64,
        amount: u64,
        chain_id_evm: u8,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(chain_id);
        chain_ids::assert_valid_chain_id(chain_id_evm);
        let mut payload = bcs::to_bytes(&chain_id_evm);
        payload.append(reverse_bytes(bcs::to_bytes(&token_id)));
        payload.append(reverse_bytes(bcs::to_bytes(&amount)));

        BridgeMessage {
            message_type: message_types::update_bridge_limit_fast_path(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain: chain_id,
            payload,
        }
    }

    /// Update bridge limit Message Format:
    /// [message_type: u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [receiving_chain_id: u8]
    /// [sending_chain_id: u8]
    /// [new_limit: u64]
    public fun create_update_bridge_limit_message(
        receiving_chain: u8,
        seq_num: u64,
        sending_chain: u8,
        new_limit: u64,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(receiving_chain);
        chain_ids::assert_valid_chain_id(sending_chain);

        let mut payload = vector[sending_chain];
        payload.append(reverse_bytes(bcs::to_bytes(&new_limit)));

        BridgeMessage {
            message_type: message_types::update_bridge_limit(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain: receiving_chain,
            payload,
        }
    }

    /// Update asset price message
    /// [message_type: u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [chain_id: u8]
    /// [token_id: u64]
    /// [new_price:u64]
    public fun create_update_asset_price_message(
        token_id: u64,
        source_chain: u8,
        seq_num: u64,
        new_price: u64,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);

        let mut payload = reverse_bytes(bcs::to_bytes(&token_id));
        payload.append(reverse_bytes(bcs::to_bytes(&new_price)));
        BridgeMessage {
            message_type: message_types::update_asset_price(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }

    public fun create_add_external_coin_admin_message(
        source_chain: u8,
        seq_num: u64,
        coin_type: String,
        admin_address: String,
    )   : BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);
        let mut payload = bcs::to_bytes(&coin_type);
        payload.append(bcs::to_bytes(&admin_address));

        BridgeMessage {
            message_type: message_types::add_external_coin_admin(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }

    public fun create_add_external_coin_witness_message(
        source_chain: u8,
        seq_num: u64,
        coin_type: String,
        witness_address: vector<u8>,
    ): BridgeMessage{
         chain_ids::assert_valid_chain_id(source_chain);
        let mut payload = bcs::to_bytes(&coin_type);
        payload.append(witness_address);

        BridgeMessage {
            message_type: message_types::add_external_coin_witness(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }

    }

    public fun create_remove_external_coin_witness_message(
        source_chain: u8,
        seq_num: u64,
        coin_type: String,
        witness_address: vector<u8>,
    ):BridgeMessage{
         chain_ids::assert_valid_chain_id(source_chain);
        let mut payload = bcs::to_bytes(&coin_type);
        payload.append(witness_address);

        BridgeMessage {
            message_type: message_types::remove_external_coin_witness(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }

    }

     public fun create_add_external_coin_target_message(
        source_chain: u8,
        seq_num: u64,
        coin_type: String,
        target_address: vector<u8>,
    ): BridgeMessage{
         chain_ids::assert_valid_chain_id(source_chain);
        let mut payload = bcs::to_bytes(&coin_type);
        payload.push_back(target_address.length() as u8);
        payload.append(target_address);

        BridgeMessage {
            message_type: message_types::add_external_coin_target(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }

    }

    public fun create_remove_external_coin_target_message(
        source_chain: u8,
        seq_num: u64,
        coin_type: String,
        target_address: vector<u8>,
    ):BridgeMessage{
         chain_ids::assert_valid_chain_id(source_chain);
        let mut payload = bcs::to_bytes(&coin_type);
        payload.push_back(target_address.length() as u8);
        payload.append(target_address);

        BridgeMessage {
            message_type: message_types::remove_external_coin_target(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }

    }

    public fun create_remove_external_coin_admin_message(
        source_chain: u8,
        seq_num: u64,
        coin_type: String,
        admin_address: String,
    )   : BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);
        let mut payload = bcs::to_bytes(&coin_type);
        payload.append(bcs::to_bytes(&admin_address));

        BridgeMessage {
            message_type: message_types::remove_external_coin_admin(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }


    public fun create_add_token_on_token_list(
        source_chain: u8,
        seq_num: u64,
        from_chain: u8,
        target_chain: u8,
        token_id: u64
    ): BridgeMessage{
        chain_ids::assert_valid_chain_id(source_chain);
        chain_ids::assert_valid_chain_id(from_chain);
        chain_ids::assert_valid_chain_id(target_chain);
        let mut payload = reverse_bytes(bcs::to_bytes(&from_chain));
        payload.append(reverse_bytes(bcs::to_bytes(&target_chain)));
        payload.append(reverse_bytes(bcs::to_bytes(&token_id)));

        BridgeMessage {
            message_type: message_types::add_token_on_token_list(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }


    public fun create_remove_token_on_token_list(
        source_chain: u8,
        seq_num: u64,
        from_chain: u8,
        target_chain: u8,
        token_id: u64
    ): BridgeMessage{
        chain_ids::assert_valid_chain_id(source_chain);
        chain_ids::assert_valid_chain_id(from_chain);
        chain_ids::assert_valid_chain_id(target_chain);
        let mut payload = reverse_bytes(bcs::to_bytes(&from_chain));
        payload.append(reverse_bytes(bcs::to_bytes(&target_chain)));
        payload.append(reverse_bytes(bcs::to_bytes(&token_id)));
        BridgeMessage {
            message_type: message_types::remove_token_on_token_list(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }

    }

    public fun  create_set_cross_in_bridge_fee(
        source_chain: u8,
        seq_num: u64,
        from_chain: u8,
        token_id: u64,
        mode: u64,
        amount: u64,
    ): BridgeMessage{
        let mut payload = reverse_bytes(bcs::to_bytes(&from_chain));
        payload.append(reverse_bytes(bcs::to_bytes(&token_id)));
        payload.append(reverse_bytes(bcs::to_bytes(&mode)));
        payload.append(reverse_bytes(bcs::to_bytes(&amount)));
        BridgeMessage{
            message_type: message_types::set_cross_in_bridge_fee(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload
        }
    }


    public fun  create_set_cross_out_bridge_fee(
        source_chain: u8,
        seq_num: u64,
        to_chain: u8,
        token_id: u64,
        mode: u64,
        amount: u64,
    ): BridgeMessage{
        let mut payload = reverse_bytes(bcs::to_bytes(&to_chain));
        payload.append(reverse_bytes(bcs::to_bytes(&token_id)));
        payload.append(reverse_bytes(bcs::to_bytes(&mode)));
        payload.append(reverse_bytes(bcs::to_bytes(&amount)));
        BridgeMessage{
            message_type: message_types::set_cross_out_bridge_fee(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload
        }
    }

    public fun create_withdraw_fee_cap(
        source_chain: u8,
        seq_num: u64,
        addr: address,
        coin_type: String,
        amount: u64
    ): BridgeMessage{
        let mut payload =bcs::to_bytes(&addr);
        payload.append(bcs::to_bytes(&coin_type));
        payload.append(reverse_bytes(bcs::to_bytes(&amount)));

        BridgeMessage{
            message_type: message_types::withdraw_bridge_fee(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload
        }
    }

    /// Update Sui token message
    /// [message_type:u8]
    /// [version:u8]
    /// [nonce:u64]
    /// [chain_id: u8]
    /// [native_token:bool]
    /// [token_ids:vector<u64>]
    /// [token_type_name:vector<String>]
    /// [token_prices:vector<u64>]
    public fun create_add_tokens_on_sui_message(
        source_chain: u8,
        seq_num: u64,
        native_token: bool,
        token_ids: vector<u64>,
        type_names: vector<String>,
        token_prices: vector<u64>,
    ): BridgeMessage {
        chain_ids::assert_valid_chain_id(source_chain);
        let mut payload = bcs::to_bytes(&native_token);
        payload.append(bcs::to_bytes(&token_ids));
        payload.append(bcs::to_bytes(&type_names));
        payload.append(bcs::to_bytes(&token_prices));
        BridgeMessage {
            message_type: message_types::add_tokens_on_sui(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }

    public fun create_defi_transfer_in_message(
        source_chain: u8,
        seq_num: u64,
        benfen_address: vector<u8>,
        target_chain: u8,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
        fast_path_selector: u8,
        protocol_type: u64,
        protocol_version: u64,
        protocol_token_id: u64,
        original_seq_num: u64,
        action_type: u8,
        lp_token_amount:u64,
        principal_amount: u64,
    ): BridgeMessage{
        chain_ids::assert_valid_chain_id(source_chain);
        chain_ids::assert_valid_chain_id(target_chain);

        let mut payload = vector[];

        // sender address should be less than 255 bytes so can fit into u8
        payload.push_back((vector::length(&benfen_address) as u8));
        payload.append(benfen_address);
        payload.push_back(target_chain);
        // bcs serializes u64 as 8 bytes
        payload.append(reverse_bytes(bcs::to_bytes(&amount)));

        // tx_hash length prefix
        payload.push_back((vector::length(&tx_hash) as u8));
        payload.append(tx_hash);
        payload.append(reverse_bytes(bcs::to_bytes(&event_idx)));
        payload.push_back(fast_path_selector);
        payload.append(reverse_bytes(bcs::to_bytes(&protocol_type)));
        payload.append(reverse_bytes(bcs::to_bytes(&protocol_version)));
        payload.append(reverse_bytes(bcs::to_bytes(&protocol_token_id)));
        payload.append(reverse_bytes(bcs::to_bytes(&original_seq_num)));
        payload.push_back(action_type);
        payload.append(reverse_bytes(bcs::to_bytes(&lp_token_amount)));
        payload.append(reverse_bytes(bcs::to_bytes(&principal_amount)));
        BridgeMessage {
            message_type: message_types::defi(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }

    public fun create_defi_transfer_out_message(
        source_chain: u8,
        seq_num: u64,
        sender_address: vector<u8>,
        target_chain: u8,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
        protocol_type: u64,
        protocol_version: u64,
        protocol_token_id: u64,
        action_type: u8,
        principal_amount: u64,
    ): BridgeMessage{
        chain_ids::assert_valid_chain_id(source_chain);
        chain_ids::assert_valid_chain_id(target_chain);

        let mut payload = vector[];

        // sender address should be less than 255 bytes so can fit into u8
        payload.push_back((vector::length(&sender_address) as u8));
        payload.append(sender_address);
        payload.push_back(target_chain);
        // bcs serializes u64 as 8 bytes
        payload.append(reverse_bytes(bcs::to_bytes(&amount)));

        // tx_hash length prefix
        payload.push_back((vector::length(&tx_hash) as u8));
        payload.append(tx_hash);
        payload.append(reverse_bytes(bcs::to_bytes(&event_idx)));
        payload.append(reverse_bytes(bcs::to_bytes(&protocol_type)));
        payload.append(reverse_bytes(bcs::to_bytes(&protocol_version)));
        payload.append(reverse_bytes(bcs::to_bytes(&protocol_token_id)));
        payload.push_back(action_type);
        //principal amount
        payload.append(reverse_bytes(bcs::to_bytes(&principal_amount)));
        BridgeMessage {
            message_type: message_types::defi(),
            message_version: CURRENT_MESSAGE_VERSION,
            seq_num,
            source_chain,
            payload,
        }
    }

    public fun extract_defi_transfer_in_payload(message: &BridgeMessage): DefiTransferInPayload {
        let mut bcs = bcs::new(message.payload);
        let sender_address = bcs.peel_vec_u8();
        let target_chain = bcs.peel_u8();
        let amount = peel_u64_be(&mut bcs);
        let tx_hash = bcs.peel_vec_u8();
        let event_idx = bcs.peel_u16();
        let fast_path_selector = bcs.peel_u8();
        let protocol_type = peel_u64_be(&mut bcs);
        let protocol_version = peel_u64_be(&mut bcs);
        let protocol_token_id = peel_u64_be(&mut bcs);
        let original_seq_num = peel_u64_be(&mut bcs);
        let action_type = bcs.peel_u8();
        let lp_token_amount = peel_u64_be(&mut bcs);
        chain_ids::assert_valid_chain_id(target_chain);
        let principal_amount = peel_u64_be(&mut bcs);
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        DefiTransferInPayload {
            sender_address,
            target_chain,
            amount,
            tx_hash,
            event_idx,
            fast_path_selector,
            protocol_type,
            protocol_version,
            protocol_token_id,
            original_seq_num,
            action_type,
            lp_token_amount,
            principal_amount,
        }
    }

    public fun extract_defi_transfer_out_payload(message: &BridgeMessage): DefiTransferOutPayload {
        let mut bcs = bcs::new(message.payload);
        let sender_address = bcs.peel_vec_u8();
        let target_chain = bcs.peel_u8();
        let amount = peel_u64_be(&mut bcs);
        let tx_hash = bcs.peel_vec_u8();
        let event_idx = bcs.peel_u16();
        let protocol_type = peel_u64_be(&mut bcs);
        let protocol_version = peel_u64_be(&mut bcs);
        let protocol_token_id = peel_u64_be(&mut bcs);
        let action_type = bcs.peel_u8();
        chain_ids::assert_valid_chain_id(target_chain);
        let principal_amount = peel_u64_be(&mut bcs);
        assert!(bcs.into_remainder_bytes().is_empty(), ETrailingBytes);

        DefiTransferOutPayload {
            sender_address,
            target_chain,
            amount,
            tx_hash,
            event_idx,
            protocol_type,
            protocol_version,
            protocol_token_id,
            action_type,
            principal_amount
        }
    }

    public fun create_key(source_chain: u8, message_type: u8, bridge_seq_num: u64): BridgeMessageKey {
        BridgeMessageKey { source_chain, message_type, bridge_seq_num }
    }

    public fun key(self: &BridgeMessage): BridgeMessageKey {
        create_key(self.source_chain, self.message_type, self.seq_num)
    }

    public fun key_refund(tx_hash: vector<u8>): RefundMessageKey {
        RefundMessageKey { tx_hash }
    }

    // BridgeMessage getters
    public fun message_version(self: &BridgeMessage): u8 {
        self.message_version
    }

    public fun message_type(self: &BridgeMessage): u8 {
        self.message_type
    }

    public fun seq_num(self: &BridgeMessage): u64 {
        self.seq_num
    }

    public fun source_chain(self: &BridgeMessage): u8 {
        self.source_chain
    }

    public fun payload(self: &BridgeMessage): vector<u8> {
        self.payload
    }

    public fun token_sender_address(self: &TokenTransferPayload): vector<u8> {
        self.sender_address
    }

    public fun token_target_chain(self: &TokenTransferPayload): u8 {
        self.target_chain
    }

    public fun token_target_address(self: &TokenTransferPayload): vector<u8> {
        self.target_address
    }

    public fun token_type(self: &TokenTransferPayload): u64 {
        self.token_type
    }

    public fun token_amount(self: &TokenTransferPayload): u64 {
        self.amount
    }

    public fun token_tx_hash(self: &TokenTransferPayload): vector<u8> {
        self.tx_hash
    }

    public fun token_event_idx(self: &TokenTransferPayload): u8 {
        self.event_idx
    }

    public fun token_sender_address_v2(self: &TokenTransferPayloadV2): vector<u8> {
        self.sender_address
    }

    public fun token_target_chain_v2(self: &TokenTransferPayloadV2): u8 {
        self.target_chain
    }

    public fun token_target_address_v2(self: &TokenTransferPayloadV2): vector<u8> {
        self.target_address
    }

    public fun token_type_v2(self: &TokenTransferPayloadV2): u64 {
        self.token_type
    }

    public fun token_amount_v2(self: &TokenTransferPayloadV2): u64 {
        self.amount
    }

    public fun token_tx_hash_v2(self: &TokenTransferPayloadV2): vector<u8> {
        self.tx_hash
    }

    public fun token_event_idx_v2(self: &TokenTransferPayloadV2): u16 {
        self.event_idx
    }

    public fun token_sender_address_in(self: &TokenTransferInPayload): vector<u8> {
        self.sender_address
    }
    public fun token_target_chain_in(self: &TokenTransferInPayload): u8 {
        self.target_chain
    }

    public fun token_target_address_in(self: &TokenTransferInPayload): vector<u8> {
        self.target_address
    }

    public fun token_type_in(self: &TokenTransferInPayload): u64 {
        self.token_type
    }

    public fun token_amount_in(self: &TokenTransferInPayload): u64 {
        self.amount
    }

    public fun token_tx_hash_in(self: &TokenTransferInPayload): vector<u8> {
        self.tx_hash
    }

    public fun token_event_idx_in(self: &TokenTransferInPayload): u16 {
        self.event_idx
    }

    public fun token_fast_path_selector_in(self: &TokenTransferInPayload): u8 {
        self.fast_path_selector
    }

    public fun sender_address_defi_out(self: &DefiTransferOutPayload): vector<u8> {
        self.sender_address
    }

    public fun target_chain_defi_out(self: &DefiTransferOutPayload): u8 {
        self.target_chain
    }

    public fun token_amount_defi_out(self: &DefiTransferOutPayload): u64 {
        self.amount
    }

    public fun token_tx_hash_defi_out(self: &DefiTransferOutPayload): vector<u8> {
        self.tx_hash
    }

    public fun token_event_idx_defi_out(self: &DefiTransferOutPayload): u16 {
        self.event_idx
    }

    public fun protocol_type_defi_out(self: &DefiTransferOutPayload): u64 {
        self.protocol_type
    }

    public fun protocol_version_defi_out(self: &DefiTransferOutPayload): u64 {
        self.protocol_version
    }

    public fun action_type_defi_out(self: &DefiTransferOutPayload): u8 {
        self.action_type
    }

    public fun protocol_token_id_defi_out(self: &DefiTransferOutPayload): u64 {
        self.protocol_token_id
    }

    public fun sender_address_defi_in(self: &DefiTransferInPayload): vector<u8> {
        self.sender_address
    }

    public fun target_chain_defi_in(self: &DefiTransferInPayload): u8 {
        self.target_chain
    }

    public fun amount_defi_in(self: &DefiTransferInPayload): u64 {
        self.amount
    }

    public fun lp_token_amount_defi_in(self: &DefiTransferInPayload): u64 {
        self.lp_token_amount
    }

    public fun tx_hash_defi_in(self: &DefiTransferInPayload): vector<u8> {
        self.tx_hash
    }

    public fun event_idx_defi_in(self: &DefiTransferInPayload): u16 {
        self.event_idx
    }

    public fun fast_path_selector_defi_in(self: &DefiTransferInPayload): u8 {
        self.fast_path_selector
    }

    public fun protocol_type_defi_in(self: &DefiTransferInPayload): u64 {
        self.protocol_type
    }

    public fun protocol_version_defi_in(self: &DefiTransferInPayload): u64 {
        self.protocol_version
    }

    public fun protocol_token_id_defi_in(self: &DefiTransferInPayload): u64 {
        self.protocol_token_id
    }

    public fun original_seq_num_defi_in(self: &DefiTransferInPayload): u64 {
        self.original_seq_num
    }

    public fun principal_amount_defi_in(self: &DefiTransferInPayload): u64 {
        self.principal_amount
    }

    public fun action_type_defi_in(self: &DefiTransferInPayload): u8 {
        self.action_type
    }
    // EmergencyOpPayload getters
    public fun emergency_op_type(self: &EmergencyOp): u8 {
        self.op_type
    }

    public fun blocklist_type(self: &Blocklist): u8 {
        self.blocklist_type
    }

    public fun blocklist_validator_addresses(self: &Blocklist): &vector<vector<u8>> {
        &self.validator_eth_addresses
    }

    public fun refund_admin_op_type(self: &RefundAdmin): u8 {
        self.op_type
    }

    public fun refund_admin_add(): u8 {
        ADD
    }

    public fun refund_admin_remove(): u8 {
        REMOVE
    }

    public fun refund_admin_sui_address(self: &RefundAdmin): &String {
        &self.sui_address
    }

    public fun update_bridge_limit_payload_sending_chain(self: &UpdateBridgeLimit): u8 {
        self.sending_chain
    }

    public fun update_bridge_limit_payload_receiving_chain(self: &UpdateBridgeLimit): u8 {
        self.receiving_chain
    }

    public fun update_bridge_limit_payload_limit(self: &UpdateBridgeLimit): u64 {
        self.limit
    }

    public fun update_asset_price_payload_token_id(self: &UpdateAssetPrice): u64 {
        self.token_id
    }

    public fun update_asset_price_payload_new_price(self: &UpdateAssetPrice): u64 {
        self.new_price
    }

    public fun add_external_coin_witness_payload_coin_type(self: &AddExternalCoinWitness): String {
        self.coin_type
    }

    public fun add_external_coin_witness_payload_witness_address(self: &AddExternalCoinWitness): vector<u8> {
        self.witness_address
    }

    public fun remove_external_coin_witness_payload_coin_type(self: &RemoveExternalCoinWitness): String {
        self.coin_type
    }

    public fun remove_external_coin_witness_payload_witness_address(self: &RemoveExternalCoinWitness): vector<u8> {
        self.witness_address
    }

    public fun add_external_coin_target_payload_coin_type(self: &AddExternalCoinTarget): String {
        self.coin_type
    }

    public fun add_external_coin_target_payload_target_address(self: &AddExternalCoinTarget): String {
        self.target_address
    }

    public fun remove_external_coin_target_payload_coin_type(self: &RemoveExternalCoinTarget): String {
        self.coin_type
    }

    public fun remove_external_coin_target_payload_target_address(self: &RemoveExternalCoinTarget): String {
        self.target_address
    }


    public fun add_external_coin_admin_payload_coin_type(self: &AddExternalCoinAdmin): String {
        self.coin_type
    }

    public fun add_external_coin_admin_payload_admin_address(self: &AddExternalCoinAdmin): String {
        self.admin_address
    }

    public fun remove_external_coin_admin_payload_coin_type(self: &RemoveExternalCoinAdmin): String {
        self.coin_type
    }

    public fun remove_external_coin_admin_payload_admin_address(self: &RemoveExternalCoinAdmin): String {
        self.admin_address
    }

    public fun  set_cross_in_bridge_fee_poyload(self: &SetCrossInBridgeFee):(u8,u64,u64,u64){
        (self.chain_id,self.token_id,self.mode,self.amount)
    }

    public fun  set_cross_out_bridge_fee_poyload(self: &SetCrossOutBridgeFee):(u8,u64,u64,u64){
        (self.chain_id,self.token_id,self.mode,self.amount)
    }

    public fun withdraw_bridge_fee_polyload(self: &WithdrawBridgeFee):(address,String,u64){
        (self.recipient,self.coin_type,self.amount)
    }

    public fun add_token_on_token_list_payload_from_chain_id(self: &AddTokenOnTokenList): u8 {
        self.from_chain_id
    }

    public fun add_token_on_token_list_payload_to_chain_id(self: &AddTokenOnTokenList): u8 {
        self.to_chain_id
    }

    public fun add_token_on_token_list_payload_token_id(self: &AddTokenOnTokenList): u64 {
        self.token_id
    }

    public fun remove_token_on_token_list_payload_from_chain_id(self: &RemoveTokenOnTokenList): u8 {
       self.from_chain_id
    }

    public fun remove_token_on_token_list_payload_to_chain_id(self: &RemoveTokenOnTokenList): u8 {
        self.to_chain_id
    }

    public fun remove_token_on_token_list_payload_token_id(self: &RemoveTokenOnTokenList): u64 {
        self.token_id
    }

    public fun is_native(self: &AddTokenOnSui): bool {
        self.native_token
    }

    public fun token_ids(self: &AddTokenOnSui): vector<u64> {
        self.token_ids
    }

    public fun token_type_names(self: &AddTokenOnSui): vector<String> {
        self.token_type_names
    }

    public fun token_prices(self: &AddTokenOnSui): vector<u64> {
        self.token_prices
    }

    public fun emergency_op_pause(): u8 {
        PAUSE
    }

    public fun emergency_op_unpause(): u8 {
        UNPAUSE
    }

    /// Return the required signature threshold for the message, values are voting power in the scale of 10000
    public fun required_voting_power(self: &BridgeMessage): u64 {
        let message_type = message_type(self);

        if (message_type == message_types::token()) {
            3334
        } else if (message_type == message_types::emergency_op()) {
            let payload = extract_emergency_op_payload(self);
            if (payload.op_type == PAUSE) {
                450
            } else if (payload.op_type == UNPAUSE) {
                5001
            } else {
                abort EInvalidEmergencyOpType
            }
        } else if (message_type == message_types::committee_blocklist()) {
            5001
        } else if (message_type == message_types::update_asset_price()) {
            5001
        } else if (message_type == message_types::update_bridge_limit()) {
            5001
        } else if (message_type == message_types::add_tokens_on_sui()) {
            5001
        } else if (message_type == message_types::add_external_coin_admin()) {
            5001
        } else if (message_type == message_types::remove_external_coin_admin()) {
            5001
        } else if (message_type == message_types::refund_admin_operate()) {
            5001
        }else if (message_type == message_types::add_external_coin_witness()) {
            5001
        }else if (message_type == message_types::remove_external_coin_witness()) {
            5001
        } else if (message_type == message_types::add_external_coin_target()) {
            5001
        }else if (message_type == message_types::remove_external_coin_target()) {
            5001
        } else if (message_type == message_types::add_token_on_token_list()) {
            5001
        }else if (message_type == message_types::remove_token_on_token_list()) {
            5001
        }else if (message_type == message_types::set_cross_in_bridge_fee()) {
            5001
        }else if (message_type == message_types::set_cross_out_bridge_fee()) {
            5001
        }else if (message_type == message_types::withdraw_bridge_fee()) {
            5001
        }else if (message_type == message_types::fast_path_limit_update()) {
            5001
        }else if (message_type == message_types::defi()) {
            3334
        }
        else {
            abort EInvalidMessageType
        }
    }

    // Convert BridgeMessage to ParsedTokenTransferMessage
    public fun to_parsed_token_transfer_message(
        message: &BridgeMessage,
    ): ParsedTokenTransferMessage {
        assert!(message.message_type() == message_types::token(), EMustBeTokenMessage);
        let payload = message.extract_token_bridge_payload();
        ParsedTokenTransferMessage {
            message_version: message.message_version(),
            seq_num: message.seq_num(),
            source_chain: message.source_chain(),
            payload: message.payload(),
            parsed_payload: payload,
        }
    }

    public fun to_parsed_token_transfer_message_v2(
        message: &BridgeMessage,
    ): ParsedTokenTransferMessageV2 {
        assert!(message.message_type() == message_types::token(), EMustBeTokenMessage);
        let payload = message.extract_token_bridge_payload_v2();
        ParsedTokenTransferMessageV2 {
            message_version: message.message_version(),
            seq_num: message.seq_num(),
            source_chain: message.source_chain(),
            payload: message.payload(),
            parsed_payload: payload,
        }
    }

    public fun to_parsed_token_transfer_in_message(
        message: &BridgeMessage,
    ): ParsedTokenTransferInMessage {
        assert!(message.message_type() == message_types::token(), EMustBeTokenMessage);
        let payload = message.extract_token_bridge_in_payload();
        ParsedTokenTransferInMessage {
            message_version: message.message_version(),
            seq_num: message.seq_num(),
            source_chain: message.source_chain(),
            payload: message.payload(),
            parsed_payload: payload,
        }
    }

    public fun to_parsed_defi_transfer_out_message(
        message: &BridgeMessage,
    ): ParsedDefiTransferOutMessage {
        assert!(message.message_type() == message_types::defi(), EMustBeDefiMessage);
        let payload = message.extract_defi_transfer_out_payload();
        ParsedDefiTransferOutMessage {
            message_version: message.message_version(),
            seq_num: message.seq_num(),
            source_chain: message.source_chain(),
            payload: message.payload(),
            parsed_payload: payload,
        }
    }
    //////////////////////////////////////////////////////
    // Internal functions
    //

    fun reverse_bytes(mut bytes: vector<u8>): vector<u8> {
        vector::reverse(&mut bytes);
        bytes
    }

    fun peel_u64_be(bcs: &mut BCS): u64 {
        let (mut value, mut i) = (0u64, 64u8);
        while (i > 0) {
            i = i - 8;
            let byte = (bcs::peel_u8(bcs) as u64);
            value = value + (byte << i);
        };
        value
    }

    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test_only]
    public(package) fun peel_u64_be_for_testing(bcs: &mut BCS): u64 {
        peel_u64_be(bcs)
    }

    #[test_only]
    public(package) fun make_generic_message(
        message_type: u8,
        message_version: u8,
        seq_num: u64,
        source_chain: u8,
        payload: vector<u8>,
    ): BridgeMessage {
        BridgeMessage {
            message_type,
            message_version,
            seq_num,
            source_chain,
            payload,
        }
    }

    #[test_only]
    public(package) fun make_payload(
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u8,
    ): TokenTransferPayload {
        TokenTransferPayload {
            sender_address,
            target_chain,
            target_address,
            token_type,
            amount,
            tx_hash,
            event_idx,
        }
    }
    #[test_only]
    public(package) fun make_payload_v2(
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
    ): TokenTransferPayloadV2 {
        TokenTransferPayloadV2 {
            sender_address,
            target_chain,
            target_address,
            token_type,
            amount,
            tx_hash,
            event_idx,
        }
    }

    #[test_only]
    public(package) fun make_payload_4_defi_transfer_out(
        sender_address: vector<u8>,
        target_chain: u8,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
        protocol_type: u64,
        protocol_version: u64,
        protocol_token_id: u64,
        action_type: u8,
        principal_amount: u64,
    ): DefiTransferOutPayload {
        DefiTransferOutPayload {
            sender_address,
            target_chain,
            amount,
            tx_hash,
            event_idx,
            protocol_type,
            protocol_version,
            protocol_token_id,
            action_type,
            principal_amount,
        }
    }

    #[test_only]
    public(package) fun make_payload_4_defi_transfer_in(
        sender_address: vector<u8>,
        target_chain: u8,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
        fast_path_selector: u8,
        protocol_type: u64,// 0:aave, 1:compound, 2:curve 等等
        protocol_version: u64,
        protocol_token_id: u64,
        original_seq_num: u64,
        action_type: u8, // 0:stake, 1:unstake
        lp_token_amount:u64,
        principal_amount: u64,
    ): DefiTransferInPayload {
        DefiTransferInPayload {
            sender_address,
            target_chain,
            amount,
            tx_hash,
            event_idx,
            fast_path_selector,
            protocol_type,
            protocol_version,
            protocol_token_id,
            original_seq_num,
            action_type,
            lp_token_amount,
            principal_amount,
        }
    }

    #[test_only]
    public(package) fun deserialize_message_test_only(message: vector<u8>): BridgeMessage {
        let mut bcs = bcs::new(message);
        let message_type = bcs::peel_u8(&mut bcs);
        let message_version = bcs::peel_u8(&mut bcs);
        let seq_num = peel_u64_be_for_testing(&mut bcs);
        let source_chain = bcs::peel_u8(&mut bcs);
        let payload = bcs::into_remainder_bytes(bcs);
        make_generic_message(
            message_type,
            message_version,
            seq_num,
            source_chain,
            payload,
        )
    }

    #[test_only]
    public(package) fun reverse_bytes_test(bytes: vector<u8>): vector<u8> {
        reverse_bytes(bytes)
    }

    #[test_only]
    public(package) fun set_payload(message: &mut BridgeMessage, bytes: vector<u8>) {
        message.payload = bytes;
    }

    #[test_only]
    public(package) fun make_add_token_on_sui(
        native_token: bool,
        token_ids: vector<u64>,
        token_type_names: vector<String>,
        token_prices: vector<u64>,
    ): AddTokenOnSui {
        AddTokenOnSui {
            native_token,
            token_ids,
            token_type_names,
            token_prices,
        }
    }

    #[test_only]
    public(package) fun unpack_message(msg: BridgeMessageKey): (u8, u8, u64) {
        (msg.source_chain, msg.message_type, msg.bridge_seq_num)
    }
}
