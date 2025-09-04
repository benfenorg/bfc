// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
#[allow(unused_field)]
module bridge::bridge {
    use sui::address;
    use std::ascii;
    use std::type_name;
    use sui::clock::Clock;
    use sui::coin::{Coin, TreasuryCap, CoinMetadata};
    use sui::event::emit;
    use sui::linked_table::{Self, LinkedTable};
    use sui::package::UpgradeCap;
    use sui::vec_map::{Self, VecMap};
    use sui::versioned::{Self, Versioned};
    use sui_system::sui_system::SuiSystemState;

    use bridge::chain_ids;
    use bridge::committee::{Self, BridgeCommittee};
    use bridge::limiter::{Self, TransferLimiter};
    use bridge::message::{
        Self, BridgeMessage, BridgeMessageKey, EmergencyOp, UpdateAssetPrice,
        AddExternalCoinAdmin, RemoveExternalCoinAdmin,RefundMessageKey,
        AddExternalCoinWitness,RemoveExternalCoinWitness,
        AddExternalCoinTarget,RemoveExternalCoinTarget,
        UpdateBridgeLimit, AddTokenOnSui, ParsedTokenTransferMessage,
        AddTokenOnTokenList, RemoveTokenOnTokenList,
        SetCrossInBridgeFee, SetCrossOutBridgeFee,
        WithdrawBridgeFee,
        ParsedTokenTransferMessageV2,
        to_parsed_token_transfer_message,
        to_parsed_token_transfer_message_v2,
    };
    use bridge::tokenlist;
    use bridge::bridge_fee;
    use bridge::message_types;
    use bridge::treasury::{Self, BridgeTreasury};
    use sui::hex;
    use sui::vec_set;
    use sui::vec_set::VecSet;
    use std::ascii::String;
    use bfc_system::bfc_system::BfcSystemState;
    use bfc_system::bfc_system_state_inner::BfcSystemModifyCap;
    use bfc_system::busd::BUSD;
    use bridge::limiter_fast_path;
    use bridge::message::TokenTransferInPayload;
    use sui::coin::split;


    //  stable coin id
    const TOKEN_ID_USDC: u64 = 3;
    const TOKEN_ID_USDT: u64 = 4;

    const MESSAGE_VERSION: u8 = 1;
    const MESSAGE_VERSION_V2: u8 = 2;
    const MESSAGE_VERSION_V3: u8 = 3;

    // Transfer Status
    const TRANSFER_STATUS_PENDING: u8 = 0;
    const TRANSFER_STATUS_APPROVED: u8 = 1;
    const TRANSFER_STATUS_CLAIMED: u8 = 2;
    const TRANSFER_STATUS_NOT_FOUND: u8 = 3;

    const EVM_ADDRESS_LENGTH: u64 = 20;

    //////////////////////////////////////////////////////
    // Types
    //

    public struct Bridge has key {
        id: UID,
        inner: Versioned,
    }

    public struct BridgeInner has store {
        bridge_version: u64,
        message_version: u8,
        chain_id: u8,
        // nonce for replay protection
        // key: message type, value: next sequence number
        sequence_nums: VecMap<u8, u64>,
        // committee
        committee: BridgeCommittee,
        // Bridge treasury for mint/burn bridged tokens
        treasury: BridgeTreasury,
        token_transfer_records: LinkedTable<BridgeMessageKey, BridgeRecord>,
        external_bridge_records: LinkedTable<ExternalBridgeMessageKey, ExternalBridgeRecord>,
        // tx hash : [signature addresses]
        pre_deposit_multi_signature_records: LinkedTable<ExternalBridgeMessageKey, VecSet<String>>,
        limiter: TransferLimiter,
        paused: bool,
        refund_records: LinkedTable<RefundMessageKey, BridgeRecord>,
        refund_admins: VecSet<String>,
        // bfc_system_id: UID,
    }

    public struct TokenDepositedEvent has copy, drop {
        seq_num: u64,
        source_chain: u8,
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
    }

     public struct TokenDepositedEventV2 has copy, drop {
        seq_num: u64,
        source_chain: u8,
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount_before_fee: u64,
        amount_after_fee: u64
    }

    public struct TokenSendBackEvent has copy, drop {
        seq_num: u64,
        source_chain: u8,
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u8,
    }

    public struct TokenSendBackEventV2 has copy, drop {
        seq_num: u64,
        source_chain: u8,
        sender_address: vector<u8>,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
    }

    public struct EmergencyOpEvent has copy, drop {
        frozen: bool,
    }

    public struct BridgeRecord has store, drop {
        message: BridgeMessage,
        verified_signatures: Option<vector<vector<u8>>>,
        claimed: bool,
    }

    const EUnexpectedMessageType: u64 = 0;
    const EUnauthorisedClaim: u64 = 1;
    const EMalformedMessageError: u64 = 2;
    const EUnexpectedTokenType: u64 = 3;
    const EUnexpectedChainID: u64 = 4;
    const ENotSystemAddress: u64 = 5;
    const EUnexpectedSeqNum: u64 = 6;
    const EWrongInnerVersion: u64 = 7;
    const EBridgeUnavailable: u64 = 8;
    const EUnexpectedOperation: u64 = 9;
    const EInvariantSuiInitializedTokenTransferShouldNotBeClaimed: u64 = 10;
    const EMessageNotFoundInRecords: u64 = 11;
    const EUnexpectedMessageVersion: u64 = 12;
    const EBridgeAlreadyPaused: u64 = 13;
    const EBridgeNotPaused: u64 = 14;
    const ETokenAlreadyClaimedOrHitLimit: u64 = 15;
    const EInvalidBridgeRoute: u64 = 16;
    const EMustBeTokenMessage: u64 = 17;
    const EInvalidEvmAddress: u64 = 18;
    const ETokenValueIsZero: u64 = 19;

    const EInvalidSender: u64 = 20;
    const EInvalidTxHash: u64 = 21;
    const EDuplicateRefund: u64 = 22;
    const EInvalidTokenIdExpect: u64 = 23;
    const EOnlySupportBusd: u64 = 24;
    const EUseSendBusd: u64 = 25;
    const EUseClaimBusd: u64 = 26;
    const EDuplicatedMessage: u64 = 30;
    const EUnknownExternalCoinOrSender: u64 = 31;
    const EUnpassedMultiSignature: u64 = 32;
    const EUnpassedWitnessSignature: u64=33;
    const EInvalidChainIDAndTokenIDExpect: u64 = 34;
    const EInvalidChainIDOnTokenList : u64=35;
    const EInputAmountLteBridgeFee: u64=36;


    const EUnauthorisedUpdateLimit: u64 = 40;
    const EInvalidMintAmount: u64 = 41;

    const EInvalidMinStakeParticipationPercentage: u64 = 50;
    const EFastPathLimitError: u64 = 51;
    const EOnlySupportTokenTransferIn: u64 = 52;
    const ETransferLimit: u64 = 55;

    const CURRENT_VERSION: u64 = 1;

    public struct TokenTransferApproved has copy, drop {
        message_key: BridgeMessageKey,
    }

    public struct TokenTransferClaimed has copy, drop {
        message_key: BridgeMessageKey,
    }

    public struct TokenTransferAlreadyApproved has copy, drop {
        message_key: BridgeMessageKey,
    }

    public struct TokenTransferAlreadyClaimed has copy, drop {
        message_key: BridgeMessageKey,
    }

    public struct TokenTransferLimitExceed has copy, drop {
        message_key: BridgeMessageKey,
    }

    public struct ExternalPreDepositedEvent has copy, drop {
        tx_hash: ascii::String,
        coin_type: ascii::String,
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        sender: address,
        signatures: vector<u8>,
    }

    public struct ExternalPreDepositedDoneEvent has copy, drop {
        tx_hash: ascii::String,
        coin_type: ascii::String,
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
    }

    public struct ExternalDepositedApprovedEvent has copy, drop {
        tx_hash: ascii::String,
        coin_type: ascii::String,
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
    }

    public struct ExternalDepositStartEvent has copy, drop {
        seq_num: u64,
        tx_hash: ascii::String,
        token_id: u64,
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
    }

    public struct ExternalDepositedEvent has copy, drop {
        tx_hash: ascii::String,
        coin_type: ascii::String,
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
    }

    public struct ExternalDepositedEventV2 has copy, drop {
        tx_hash: ascii::String,
        token_type: u64,
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount_before_fee: u64,
        amount_after_fee: u64
    }

    public struct ExternalWithdrawEvent has copy, drop {
        coin_type: ascii::String,
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
    }

    public struct ExternalWithdrawEventV2 has copy, drop {
        token_type: u64,
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount_before_fee: u64,
        amount_after_fee: u64
    }

    public struct ExternalBridgeMessageKey has copy, drop, store {
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
    }

    public struct ExternalBridgeRecord has store, drop {
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,

        verified_signatures: Option<vector<vector<u8>>>,
        claimed: bool,
    }

    //////////////////////////////////////////////////////
    // Internal initialization functions
    //

    // this method is called once in end of epoch tx to create the bridge
    #[allow(unused_function)]
    fun create(id: UID, chain_id: u8, ctx: &mut TxContext) {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);
        let bridge_inner = BridgeInner {
            bridge_version: CURRENT_VERSION,
            message_version: MESSAGE_VERSION,
            chain_id,
            sequence_nums: vec_map::empty(),
            committee: committee::create(ctx),
            treasury: treasury::create(ctx),
            token_transfer_records: linked_table::new(ctx),
            external_bridge_records: linked_table::new(ctx),
            pre_deposit_multi_signature_records: linked_table::new(ctx),
            limiter: limiter::new(),
            paused: false,
            refund_records: linked_table::new(ctx),
            refund_admins: vec_set::empty(),
        };
        let bridge = Bridge {
            id,
            inner: versioned::create(CURRENT_VERSION, bridge_inner, ctx)
        };
        transfer::share_object(bridge);
    }

    #[allow(unused_function)]
    fun init_bridge_committee(
        bridge: &mut Bridge,
        active_validator_voting_power: VecMap<address, u64>,
        min_stake_participation_percentage: u64,
        ctx: &TxContext
    ) {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);
        let inner = load_inner_mut(bridge);
        assert!(min_stake_participation_percentage>=7500, EInvalidMinStakeParticipationPercentage);
        if (inner.committee.committee_members().is_empty()) {
            inner.committee.try_create_next_committee(
                active_validator_voting_power,
                min_stake_participation_percentage,
                ctx,
            )
        }
    }

    public entry fun migrate(
        bridge: &mut Bridge,
        ctx: &mut TxContext
    ){
        bridge_fee::new_bridge_fee_registry(&mut bridge.id, ctx);
        limiter_fast_path::registry(&mut bridge.id, ctx);
        limiter::new_external_limits(&mut bridge.id, ctx);
    }

    public fun init_token_list(
        bridge: &mut Bridge,
        ctx: &mut TxContext
    ){
        tokenlist::new_tokenlist_registry(&mut bridge.id, ctx);
        tokenlist::add_center_token_list(&mut bridge.id, ctx);
        limiter::update_transfer_limits(&mut load_inner_mut(bridge).limiter);
    }

    public fun update_external_out_limit(
        bridge: &mut Bridge,
        bfc_system_state: &BfcSystemState,
        cap: &BfcSystemModifyCap,
        target_chain: u8,
        limit: u64,
        ctx: &mut TxContext,
    ) {
        let (inner,parent_id) = load_inner_mut_and_uid(bridge);
        assert!(bfc_system_state.verify_capability(cap, ctx), EUnauthorisedUpdateLimit);
        let route = chain_ids::get_route(inner.chain_id, target_chain);
        limiter::update_external_out_limit(
            parent_id,
            &route,
            limit
        );
    }

    //////////////////////////////////////////////////////
    // Public functions
    //

    public fun committee_registration(
        bridge: &mut Bridge,
        system_state: &mut SuiSystemState,
        bridge_pubkey_bytes: vector<u8>,
        http_rest_url: vector<u8>,
        ctx: &TxContext
    ) {
        load_inner_mut(bridge)
            .committee
            .register(system_state, bridge_pubkey_bytes, http_rest_url, ctx);
    }

    public fun update_node_url(bridge: &mut Bridge, new_url: vector<u8>, ctx: &TxContext) {
        load_inner_mut(bridge).committee.update_node_url(new_url, ctx);
    }

    public fun register_foreign_token<T>(
        bridge: &mut Bridge,
        tc: TreasuryCap<T>,
        uc: UpgradeCap,
        metadata: &CoinMetadata<T>,
    ) {
        load_inner_mut(bridge)
            .treasury
            .register_foreign_token<T>(tc, uc, metadata)
    }


    public fun withdraw<T>(
        bridge: &mut Bridge,
        target_chain: u8,
        target_address: vector<u8>,
        mut token: Coin<T>,
        ctx: &mut TxContext
    ) {
        token.destroy_zero();
    }

    // Create bridge request to send token to other chain, the request will be in
    // pending state until approved
    public fun send_token<T>(
        bridge: &mut Bridge,
        target_chain: u8,
        target_address: vector<u8>,
        mut token: Coin<T>,
        ctx: &mut TxContext
    ) {
        let (inner,parent_id) = load_inner_mut_and_uid(bridge);
        assert!(!inner.paused, EBridgeUnavailable);
        assert!(chain_ids::is_valid_route(inner.chain_id, target_chain), EInvalidBridgeRoute);
        assert!(target_address.length() == EVM_ADDRESS_LENGTH, EInvalidEvmAddress);

        let bridge_seq_num = inner.get_current_seq_num_and_increment(message_types::token());
        let token_id = inner.treasury.token_id<T>();
        let token_amount = token.balance().value();
        assert!(token_amount > 0, ETokenValueIsZero);
        assert!(token_id != 5, EUseSendBusd);

        assert!(tokenlist::is_supported_from_benfen(parent_id, target_chain as u64, token_id),EInvalidChainIDAndTokenIDExpect);

        let fee=bridge_fee::calculate_cross_out_fee_amount(parent_id,target_chain as u64,token_id,token_amount);
        assert!(token_amount>fee,EInputAmountLteBridgeFee);
        let fee_coin=token.split<T>(fee, ctx);
        bridge_fee::deposit_fee(parent_id, fee_coin);
        let amount_after_fee=token_amount-fee;
        let route = chain_ids::get_route(inner.chain_id, target_chain);
        let amount_in_usd = inner.treasury.calculate_amount_in_usd<T>(amount_after_fee);
        assert!(amount_in_usd <= limiter::get_external_out_limit(parent_id, &route), ETransferLimit);
        // create bridge message
        let message = message::create_token_bridge_message_v2(
            inner.chain_id,
            bridge_seq_num,
            address::to_bytes(ctx.sender()),
            target_chain,
            target_address,
            token_id,
            amount_after_fee,
            hex::decode(b""),
            0u16, // event_idx
        );

        // burn / escrow token, unsupported coins will fail in this step
        inner.treasury.burn(token);

        // Store pending bridge request
        inner.token_transfer_records.push_back(
            message.key(),
            BridgeRecord {
                message,
                verified_signatures: option::none(),
                claimed: false,
            },
        );

        // emit event
        emit(
            TokenDepositedEventV2 {
                seq_num: bridge_seq_num,
                source_chain: inner.chain_id,
                sender_address: address::to_bytes(ctx.sender()),
                target_chain,
                target_address,
                token_type: token_id,
                amount_before_fee: token_amount,
                amount_after_fee,
            },
        );
    }

    public fun send_busd<T>(
        bridge: &mut Bridge,
        bfc_system_state: &mut BfcSystemState,
        target_chain: u8,
        target_address: vector<u8>,
        mut token: Coin<T>,
        token_id_expect: u64,
        ctx: &mut TxContext
    ) {

        let (inner,bridge_id) = load_inner_mut_and_uid(bridge);
        assert!(tokenlist::is_supported_from_benfen(bridge_id, target_chain as u64, token_id_expect),EInvalidChainIDAndTokenIDExpect);
        assert!(!inner.paused, EBridgeUnavailable);
        assert!(chain_ids::is_valid_route(inner.chain_id, target_chain), EInvalidBridgeRoute);
        assert!(target_address.length() == EVM_ADDRESS_LENGTH, EInvalidEvmAddress);
        let is_busd = type_name::get<T>() == type_name::get<BUSD>();
        assert!(is_busd, EOnlySupportBusd);
        assert!(token_id_expect == 3 || token_id_expect == 4, EInvalidTokenIdExpect);

        let bridge_seq_num = inner.get_current_seq_num_and_increment(message_types::token());
        // let token_id_origin = inner.treasury.token_id<T>();
        // assert!(token_id_origin == 5, EOnlySupportBusd);
        let token_id = token_id_expect;
        let token_amount=if (target_chain==chain_ids::eth_mainnet() || target_chain==chain_ids::eth_sepolia() || target_chain==chain_ids::eth_custom()) {
             token.balance().value()/1000u64
        }else{
             token.balance().value()
        };
        assert!(token_amount > 0, ETokenValueIsZero);
        let fee=bridge_fee::calculate_cross_out_fee_amount(bridge_id,target_chain as u64,token_id,token_amount);
        assert!(token_amount>fee,EInputAmountLteBridgeFee);
        let amount_after_fee=token_amount-fee;
        let fee_coin=token.split<T>(fee, ctx);
        bridge_fee::deposit_fee(bridge_id, fee_coin);
        // create bridge message
        let message = message::create_token_bridge_message_v2(
            inner.chain_id,
            bridge_seq_num,
            address::to_bytes(ctx.sender()),
            target_chain,
            target_address,
            token_id,
            amount_after_fee,
            hex::decode(b""),
            0u16, // event_idx
        );

        // burn / escrow token, unsupported coins will fail in this step
        bfc_system_state.burn_stable(token, ctx);

        // Store pending bridge request
        inner.token_transfer_records.push_back(
            message.key(),
            BridgeRecord {
                message,
                verified_signatures: option::none(),
                claimed: false,
            },
        );

        // emit event
        emit(
            TokenDepositedEventV2 {
                seq_num: bridge_seq_num,
                source_chain: inner.chain_id,
                sender_address: address::to_bytes(ctx.sender()),
                target_chain,
                target_address,
                token_type: token_id,
                amount_before_fee: token_amount,
                amount_after_fee,
            },
        );
    }

    // Create bridge request to send token back to EVM, the request will be in
    // pending state until approved
    public fun send_back_token(
        bridge: &mut Bridge,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        token_amount: u64,
        tx_hash: vector<u8>,
        event_idx: u8,
        ctx: &mut TxContext
    ) {
        let inner = load_inner_mut(bridge);
        assert!(!inner.paused, EBridgeUnavailable);
        assert!(chain_ids::is_valid_route(inner.chain_id, target_chain), EInvalidBridgeRoute);
        assert!(!inner.refund_records.contains(message::key_refund(tx_hash)), EDuplicateRefund);
        assert!(target_address.length() == EVM_ADDRESS_LENGTH, EInvalidEvmAddress);
        assert!(token_amount > 0, ETokenValueIsZero);
        assert!(tx_hash.length() >= 1, EInvalidTxHash);
        assert!(inner.is_refund_admin(ctx.sender().to_ascii_string()), EInvalidSender);
        let bridge_seq_num = inner.get_current_seq_num_and_increment(message_types::token());
        // create bridge message
        let message = message::create_token_bridge_message(
            inner.chain_id,
            bridge_seq_num,
            address::to_bytes(ctx.sender()),
            target_chain,
            target_address,
            token_type,
            token_amount,
            tx_hash,
            event_idx,
        );
        // Store pending bridge request
        inner.token_transfer_records.push_back(
            message.key(),
            BridgeRecord {
                message,
                verified_signatures: option::none(),
                claimed: false,
            },
        );
        //store for idempotency
        inner.refund_records.push_back(
            message::key_refund(tx_hash),
            BridgeRecord {
                message,
                verified_signatures: option::none(),
                claimed: false,
            },
        );

        // emit event
        emit(
            TokenSendBackEvent {
                seq_num: bridge_seq_num,
                source_chain: inner.chain_id,
                sender_address: address::to_bytes(ctx.sender()),
                target_chain,
                target_address,
                token_type: token_type,
                amount: token_amount,
                tx_hash,
                event_idx,
            },
        );
    }
    public fun send_back_token_v2(
        bridge: &mut Bridge,
        target_chain: u8,
        target_address: vector<u8>,
        token_type: u64,
        token_amount: u64,
        tx_hash: vector<u8>,
        event_idx: u16,
        ctx: &mut TxContext
    ) {
        let inner = load_inner_mut(bridge);
        assert!(!inner.paused, EBridgeUnavailable);
        assert!(chain_ids::is_valid_route(inner.chain_id, target_chain), EInvalidBridgeRoute);
        assert!(!inner.refund_records.contains(message::key_refund(tx_hash)), EDuplicateRefund);
        assert!(target_address.length() == EVM_ADDRESS_LENGTH, EInvalidEvmAddress);
        assert!(token_amount > 0, ETokenValueIsZero);
        assert!(tx_hash.length() >= 1, EInvalidTxHash);
        assert!(inner.is_refund_admin(ctx.sender().to_ascii_string()), EInvalidSender);
        let bridge_seq_num = inner.get_current_seq_num_and_increment(message_types::token());
        // create bridge message
        let message = message::create_token_bridge_message_v2(
            inner.chain_id,
            bridge_seq_num,
            address::to_bytes(ctx.sender()),
            target_chain,
            target_address,
            token_type,
            token_amount,
            tx_hash,
            event_idx,
        );
        // Store pending bridge request
        inner.token_transfer_records.push_back(
            message.key(),
            BridgeRecord {
                message,
                verified_signatures: option::none(),
                claimed: false,
            },
        );
        //store for idempotency
        inner.refund_records.push_back(
            message::key_refund(tx_hash),
            BridgeRecord {
                message,
                verified_signatures: option::none(),
                claimed: false,
            },
        );

        // emit event
        emit(
            TokenSendBackEventV2 {
                seq_num: bridge_seq_num,
                source_chain: inner.chain_id,
                sender_address: address::to_bytes(ctx.sender()),
                target_chain,
                target_address,
                token_type: token_type,
                amount: token_amount,
                tx_hash,
                event_idx,
            },
        );
    }

    public fun fast_path_limit_by_sender(
        bridge: &mut Bridge,
        sender_address: vector<u8>,
        chain_id: u8,
        token_id: u64,
        clock: &Clock,
        _ctx: &mut TxContext
    ): u64 {
        let (_,uid) = load_inner_mut_and_uid(bridge);
        limiter_fast_path::get_user_remaining_limit(uid, sender_address, chain_id, token_id, clock)
    }

    public fun fast_path_limit_config_info(
        bridge: &mut Bridge,
        chain_id: u8,
        token_id: u64,
    ): u64 {
        let (_,uid) = load_inner_mut_and_uid(bridge);
        limiter_fast_path::get_limit_config_info(uid, chain_id, token_id)
    }

    fun is_refund_admin(inner: &BridgeInner, address: String): bool {
        inner.refund_admins.contains(&address)
    }

    // Record bridge message approvals in Sui, called by the bridge client
    // If already approved, return early instead of aborting.
    public fun approve_token_transfer(
        bridge: &mut Bridge,
        message: BridgeMessage,
        signatures: vector<vector<u8>>,
    ) {
        let inner = load_inner_mut(bridge);
        assert!(!inner.paused, EBridgeUnavailable);
        // verify signatures
        inner.committee.verify_signatures(message, signatures);

        assert!(message.message_type() == message_types::token(), EMustBeTokenMessage);
        assert!(message.message_version() == MESSAGE_VERSION, EUnexpectedMessageVersion);
        let token_payload = message.extract_token_bridge_payload();
        let target_chain = token_payload.token_target_chain();
        assert!(
            message.source_chain() == inner.chain_id || target_chain == inner.chain_id,
            EUnexpectedChainID,
        );

        let message_key = message.key();
        // retrieve pending message if source chain is Sui, the initial message
        // must exist on chain
        if (message.source_chain() == inner.chain_id) {
            let record = &mut inner.token_transfer_records[message_key];

            assert!(record.message == message, EMalformedMessageError);
            assert!(!record.claimed, EInvariantSuiInitializedTokenTransferShouldNotBeClaimed);

            // If record already has verified signatures, it means the message has been approved
            // Then we exit early.
            if (record.verified_signatures.is_some()) {
                emit(TokenTransferAlreadyApproved { message_key });
                return
            };
            // Store approval
            record.verified_signatures = option::some(signatures)
        } else {
            // At this point, if this message is in token_transfer_records, we know
            // it's already approved because we only add a message to token_transfer_records
            // after verifying the signatures
            if (inner.token_transfer_records.contains(message_key)) {
                emit(TokenTransferAlreadyApproved { message_key });
                return
            };
            //idempotency for SendBack and ETHToSui
            let tx_hash = token_payload.token_tx_hash();
            if (inner.refund_records.contains(message::key_refund(tx_hash))) {
                emit(TokenTransferAlreadyApproved { message_key });
                return
            };
            // Store message and approval
            inner.token_transfer_records.push_back(
                message_key,
                BridgeRecord {
                    message,
                    verified_signatures: option::some(signatures),
                    claimed: false
                },
            );
        };

        emit(TokenTransferApproved { message_key });
    }

    // Record bridge message approvals in Sui, called by the bridge client
    // If already approved, return early instead of aborting.
    public fun approve_token_transfer_v2(
        bridge: &mut Bridge,
        message: BridgeMessage,
        signatures: vector<vector<u8>>,
    ) {
        let inner = load_inner_mut(bridge);
        assert!(!inner.paused, EBridgeUnavailable);
        // verify signatures
        inner.committee.verify_signatures(message, signatures);

        assert!(message.message_type() == message_types::token(), EMustBeTokenMessage);
        assert!(message.message_version() == MESSAGE_VERSION_V3, EUnexpectedMessageVersion);
        let token_payload = message.extract_token_bridge_payload_v2();
        let target_chain = token_payload.token_target_chain_v2();
        assert!(
            message.source_chain() == inner.chain_id || target_chain == inner.chain_id,
            EUnexpectedChainID,
        );

        let message_key = message.key();
        // retrieve pending message if source chain is Sui, the initial message
        // must exist on chain
        if (message.source_chain() == inner.chain_id) {
            let record = &mut inner.token_transfer_records[message_key];

            assert!(record.message == message, EMalformedMessageError);
            assert!(!record.claimed, EInvariantSuiInitializedTokenTransferShouldNotBeClaimed);

            // If record already has verified signatures, it means the message has been approved
            // Then we exit early.
            if (record.verified_signatures.is_some()) {
                emit(TokenTransferAlreadyApproved { message_key });
                return
            };
            // Store approval
            record.verified_signatures = option::some(signatures)
        } else {
            // At this point, if this message is in token_transfer_records, we know
            // it's already approved because we only add a message to token_transfer_records
            // after verifying the signatures
            if (inner.token_transfer_records.contains(message_key)) {
                emit(TokenTransferAlreadyApproved { message_key });
                return
            };
            //idempotency for SendBack and ETHToSui
            let tx_hash = token_payload.token_tx_hash_v2();
            if (inner.refund_records.contains(message::key_refund(tx_hash))) {
                emit(TokenTransferAlreadyApproved { message_key });
                return
            };
            // Store message and approval
            inner.token_transfer_records.push_back(
                message_key,
                BridgeRecord {
                    message,
                    verified_signatures: option::some(signatures),
                    claimed: false
                },
            );
        };

        emit(TokenTransferApproved { message_key });
    }

    public fun approve_token_transfer_in(
        bridge: &mut Bridge,
        message: BridgeMessage,
        signatures: vector<vector<u8>>,
    ) {
        let inner = load_inner_mut(bridge);
        assert!(!inner.paused, EBridgeUnavailable);
        // verify signatures
        inner.committee.verify_signatures(message, signatures);

        assert!(message.message_type() == message_types::token(), EMustBeTokenMessage);
        assert!(message.message_version() == MESSAGE_VERSION_V2, EUnexpectedMessageVersion);
        let token_payload = message.extract_token_bridge_in_payload();
        let target_chain = token_payload.token_target_chain_in();
        assert!(
            message.source_chain() == inner.chain_id || target_chain == inner.chain_id,
            EUnexpectedChainID,
        );

        let message_key = message.key();
        // retrieve pending message if source chain is Sui, the initial message
        // must exist on chain
        //only support token transfer in
        assert!(message.source_chain() != inner.chain_id, EOnlySupportTokenTransferIn);
        // At this point, if this message is in token_transfer_records, we know
        // it's already approved because we only add a message to token_transfer_records
        // after verifying the signatures
        if (inner.token_transfer_records.contains(message_key)) {
            emit(TokenTransferAlreadyApproved { message_key });
            return
        };
        //idempotency for SendBack and ETHToSui
        let tx_hash = token_payload.token_tx_hash_in();
        if (inner.refund_records.contains(message::key_refund(tx_hash))) {
                emit(TokenTransferAlreadyApproved { message_key });
                return
        };
        // Store message and approval
        inner.token_transfer_records.push_back(
            message_key,
            BridgeRecord {
                message,
                verified_signatures: option::some(signatures),
                claimed: false
            },
        );

        emit(TokenTransferApproved { message_key });
    }

    // Get the max mint BUSD amount
    public fun get_max_mint_busd_amount(bridge: &Bridge): u64 {
        let inner = load_inner(bridge);
        inner.limiter.get_mint_busd_max_limit()
    }

    // Set the max mint BUSD amount
    public fun set_max_mint_busd_amount(
        bridge: &mut Bridge,
        bfc_system_state: &BfcSystemState,
        cap: &BfcSystemModifyCap,
        new_limit: u64,
        ctx: &mut TxContext,
        ) {
        let inner = load_inner_mut(bridge);
        assert!(bfc_system_state.verify_capability(cap, ctx), EUnauthorisedUpdateLimit);
        inner.limiter.set_mint_busd_max_limit(new_limit);
    }

    // This function can only be called by the token recipient
    // Abort if the token has already been claimed or hits limiter currently,
    // in which case, no event will be emitted and only abort code will be returned.
    public fun claim_token<T>(
        bridge: &mut Bridge,
        clock: &Clock,
        source_chain: u8,
        bridge_seq_num: u64,
        ctx: &mut TxContext,
    ): Coin<T> {
        let (maybe_token, owner) = bridge.claim_token_internal<T>(
            clock,
            source_chain,
            bridge_seq_num,
            ctx,
        );
        // Only token owner can claim the token
        assert!(ctx.sender() == owner, EUnauthorisedClaim);
        assert!(maybe_token.is_some(), ETokenAlreadyClaimedOrHitLimit);
        maybe_token.destroy_some()
    }

    // This function can be called by anyone to claim and transfer the token to the recipient
    // If the token has already been claimed or hits limiter currently, it will return instead of aborting.
    public fun claim_and_transfer_token<T>(
        bridge: &mut Bridge,
        clock: &Clock,
        source_chain: u8,
        bridge_seq_num: u64,
        ctx: &mut TxContext,
    ) {
        let (token, owner) = bridge.claim_token_internal<T>(clock, source_chain, bridge_seq_num, ctx);
        if (token.is_some()) {
            transfer::public_transfer(token.destroy_some(), owner)
        } else {
            token.destroy_none();
        };
    }

    public fun claim_and_transfer_busd<T>(
        bridge: &mut Bridge,
        bfc_system_state: &mut BfcSystemState,
        clock: &Clock,
        source_chain: u8,
        bridge_seq_num: u64,
        cap: &BfcSystemModifyCap,
        ctx: &mut TxContext,
    ) {
        let (token, owner) = bridge.claim_stable_token_internal<T>(bfc_system_state, clock, source_chain, bridge_seq_num, cap, ctx);
        if (token.is_some()) {
            transfer::public_transfer(token.destroy_some(), owner)
        } else {
            token.destroy_none();
        };
    }

    public fun execute_system_message_with_ctx(
        bridge: &mut Bridge,
        message: BridgeMessage,
        signatures: vector<vector<u8>>,
        ctx: &mut TxContext,
    ){
         let message_type = message.message_type();

        // TODO: test version mismatch
        assert!(message.message_version() == MESSAGE_VERSION, EUnexpectedMessageVersion);
        let (inner,bridge_id) = load_inner_mut_and_uid(bridge);

        assert!(message.source_chain() == inner.chain_id, EUnexpectedChainID);

        // check system ops seq number and increment it
        let expected_seq_num = inner.get_current_seq_num_and_increment(message_type);
        assert!(message.seq_num() == expected_seq_num, EUnexpectedSeqNum);

        inner.committee.verify_signatures(message, signatures);
        if (message_type == message_types::add_token_on_token_list()){
            let payload = message.extract_add_token_on_token_list_poyload();
            execute_add_token_on_token_list(bridge_id,payload,ctx);
        }else if (message_type == message_types::remove_token_on_token_list()){
            let payload = message.extract_remove_token_on_token_list_poyload();
            execute_remove_token_on_token_list(bridge_id,payload);
        }else if (message_type == message_types::set_cross_out_bridge_fee()){
            let payload = message.extract_set_cross_out_bridge_fee_poyload();
            execute_set_cross_out(bridge_id, payload, ctx)
        }else if (message_type == message_types::set_cross_in_bridge_fee()){
            let payload = message.extract_set_cross_in_bridge_fee_poyload();
            execute_set_cross_in(bridge_id, payload, ctx)
        }else if (message_type == message_types::withdraw_bridge_fee()){
            let payload = message.extract_withdraw_bridge_fee();
            execute_withdraw_bridge_fee(payload, ctx)
        }else{
            abort EUnexpectedMessageType
        };
    }

    public fun execute_system_message(
        bridge: &mut Bridge,
        message: BridgeMessage,
        signatures: vector<vector<u8>>,
    ) {
        let message_type = message.message_type();

        // TODO: test version mismatch
        assert!(message.message_version() == MESSAGE_VERSION, EUnexpectedMessageVersion);
        let (inner, uid) = load_inner_mut_and_uid(bridge);

        assert!(message.source_chain() == inner.chain_id, EUnexpectedChainID);

        // check system ops seq number and increment it
        let expected_seq_num = inner.get_current_seq_num_and_increment(message_type);
        assert!(message.seq_num() == expected_seq_num, EUnexpectedSeqNum);

        inner.committee.verify_signatures(message, signatures);

        if (message_type == message_types::emergency_op()) {
            let payload = message.extract_emergency_op_payload();
            inner.execute_emergency_op(payload);
        } else if (message_type == message_types::committee_blocklist()) {
            let payload = message.extract_blocklist_payload();
            inner.committee.execute_blocklist(payload);
        } else if (message_type == message_types::update_bridge_limit()) {
            let payload = message.extract_update_bridge_limit();
            inner.execute_update_bridge_limit(payload);
        } else if (message_type == message_types::update_asset_price()) {
            let payload = message.extract_update_asset_price();
            inner.execute_update_asset_price(payload);
        } else if (message_type == message_types::add_external_coin_admin()) {
            let payload = message.extract_add_external_coin_admin();
            inner.execute_add_external_coin_admin(payload);
        } else if (message_type == message_types::remove_external_coin_admin()) {
            let payload = message.extract_remove_external_coin_admin();
            inner.execute_remove_external_coin_admin(payload);
        } else if (message_type == message_types::add_tokens_on_sui()) {
            let payload = message.extract_add_tokens_on_sui();
            inner.execute_add_tokens_on_sui(payload);
        } else if (message_type == message_types::refund_admin_operate()) {
            let payload = message.extract_refund_admin_payload();
            inner.execute_refund_admin_operate(payload);
        } else if  (message_type == message_types::add_external_coin_witness()){
            let payload = message.extract_add_witness_poyload();
            inner.execute_add_external_coin_witness(payload);

        }else if  (message_type == message_types::remove_external_coin_witness()){
            let payload = message.extract_remove_witness_poyload();
            inner.execute_remove_external_coin_witness(payload);

        }else if  (message_type == message_types::add_external_coin_target()){
            let payload = message.extract_add_external_target_address_poyload();
            inner.execute_add_external_coin_target_payload(payload);
        }else if  (message_type == message_types::remove_external_coin_target()){
            let payload = message.extract_remove_external_target_address_poyload();
            inner.execute_remove_external_coin_target_payload(payload);
        }else if  (message_type == message_types::update_bridge_limit_fast_path()){
            let payload = message.extract_fast_path_limit_payload();
            limiter_fast_path::add_limiter(uid, payload.chain_id(),payload.token_id(),payload.amount());
        }else {
            abort EUnexpectedMessageType
        };
    }

    public fun get_available_claim_amount<T>(
          bridge: &Bridge,
          source_chain: u8,
    ): u128 {
        let inner = load_inner(bridge);
        let route = chain_ids::get_route(source_chain, inner.chain_id);
        inner.limiter.get_available_claim_amount<T>(&inner.treasury, route)
    }

    public fun pre_deposit_external_coin<T>(
        bridge: &mut Bridge,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
        signatures: vector<u8>,
        ctx: &mut TxContext
    ) {
        let sender = ctx.sender();
        let sender_str = sender.to_ascii_string();
        let coin_type = type_name::into_string(type_name::get<T>());

        let inner = load_inner_mut(bridge);
        assert!(inner.treasury.verify_bitcoin_signatures<T>(source_chain, source_address, target_address, amount, tx_hash, signatures),EUnpassedWitnessSignature);
        assert!(!inner.paused, EBridgeUnavailable);
        assert!(chain_ids::is_valid_route(source_chain, inner.chain_id), EInvalidBridgeRoute);
        if (!inner.treasury.is_external_coin_admin(coin_type, sender_str)) {
            abort EUnknownExternalCoinOrSender
        };

        // check then add to pre_deposit_multi_signature_records
        let key = ExternalBridgeMessageKey{
            source_chain,
            source_address,
            target_address,
            amount,
            tx_hash,
        };
        if (inner.pre_deposit_multi_signature_records.contains(key)) {
            let records = &mut inner.pre_deposit_multi_signature_records[key];
            if (records.contains(&sender_str)) {
                return
            };

            records.insert(sender_str);
        } else {
            let mut records = vec_set::empty();
            records.insert(sender_str);
            inner.pre_deposit_multi_signature_records.push_back(key, records);
        };

        emit(
            ExternalPreDepositedEvent {
                tx_hash,
                coin_type,
                source_chain,
                target_chain: inner.chain_id,
                source_address,
                target_address,
                amount,
                sender,
                signatures,
            }
        );

        if (inner.multi_signature_passed(key, coin_type)) {
            emit(
                ExternalPreDepositedDoneEvent {
                    tx_hash,
                    coin_type,
                    source_chain,
                    target_chain: inner.chain_id,
                    source_address,
                    target_address,
                    amount,
                },
            )
        }
    }

    public fun deposit_external_coin<T>(
        bridge: &mut Bridge,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
        signatures: vector<u8>,
        ctx: &mut TxContext
    ) {
        let sender = ctx.sender();
        let coin_type = type_name::into_string(type_name::get<T>());

        let inner = load_inner_mut(bridge);

        assert!(inner.treasury.verify_bitcoin_signatures<T>(source_chain, source_address, target_address, amount, tx_hash, signatures),EUnpassedWitnessSignature);
        assert!(!inner.paused, EBridgeUnavailable);
        assert!(chain_ids::is_valid_route(source_chain, inner.chain_id), EInvalidBridgeRoute);
        if (!inner.treasury.is_external_coin_admin(coin_type, sender.to_ascii_string())) {
            abort EUnknownExternalCoinOrSender
        };

        let key = ExternalBridgeMessageKey{
            source_chain,
            source_address,
            target_address,
            amount,
            tx_hash,
        };
        if (!inner.multi_signature_passed(key, coin_type)) {
            abort EUnpassedMultiSignature
        };

        // check records
        let key = ExternalBridgeMessageKey{
            source_chain,
            source_address,
            target_address,
            amount,
            tx_hash,
        };
        if (inner.external_bridge_records.contains(key)) {
            abort EDuplicatedMessage
        };

        // // v1
        // let token = inner.treasury.mint<T>(amount, ctx);
        // transfer::public_transfer(token, address::from_bytes(target_address));

        // inner.external_bridge_records.push_back(
        //     key,
        //     ExternalBridgeRecord {
        //         source_chain,
        //         target_chain: inner.chain_id,
        //         source_address,
        //         target_address,
        //         amount,
        //         verified_signatures: option::none(),
        //         claimed: true,
        //     },
        // );

        // emit(
        //     ExternalDepositedEvent {
        //         tx_hash,
        //         coin_type,
        //         source_chain,
        //         target_chain: inner.chain_id,
        //         source_address,
        //         target_address,
        //         amount,
        //     },
        // )

        // v2
        let seq_num = inner.get_current_seq_num_and_increment(message_types::token());
        let token_id = inner.treasury.token_id<T>();

        emit(
            ExternalDepositStartEvent {
                seq_num,
                tx_hash,
                token_id,
                source_chain,
                target_chain: inner.chain_id,
                source_address,
                target_address,
                amount,
            },
        )
    }

    // for v2
    public fun approval_and_claimed_external_coin<T>(
        bridge: &mut Bridge,
        message: BridgeMessage,
        signatures: vector<vector<u8>>,
        ctx: &mut TxContext
    ) {
        let (inner,parent_id) = load_inner_mut_and_uid(bridge);
        assert!(!inner.paused, EBridgeUnavailable);

        // verify signatures
        inner.committee.verify_signatures(message, signatures);

        assert!(message.message_type() == message_types::token(), EMustBeTokenMessage);
        assert!(message.message_version() == MESSAGE_VERSION, EUnexpectedMessageVersion);
        let token_payload = message.extract_token_bridge_payload();
        let target_chain = token_payload.token_target_chain();
        assert!(
            message.source_chain() == inner.chain_id || target_chain == inner.chain_id,
            EUnexpectedChainID,
        );

        let coin_type = type_name::into_string(type_name::get<T>());
        let token_id=treasury::token_id<T>(&inner.treasury);
        // check records
        let tx_hash = ascii::string(token_payload.token_tx_hash());
        let source_chain = message.source_chain();
        let target_chain = token_payload.token_target_chain();
        let source_address = token_payload.token_sender_address();
        let target_address = token_payload.token_target_address();
        let amount = token_payload.token_amount();
        let key = ExternalBridgeMessageKey{
            source_chain,
            source_address,
            target_address,
            amount,
            tx_hash,
        };
        if (inner.external_bridge_records.contains(key)) {
            emit(ExternalDepositedApprovedEvent{
                tx_hash,
                coin_type,
                source_chain: source_chain,
                target_chain: target_chain,
                source_address: source_address,
                target_address: target_address,
                amount: token_payload.token_amount(),
               });

            return
        };
        assert!(token_payload.token_amount() > 0, ETokenValueIsZero);

        let mut token = inner.treasury.mint<T>(amount, ctx);

        let fee=bridge_fee::calculate_cross_in_fee_amount(parent_id,source_chain as u64,token_id,amount);
        assert!(amount>fee,EInputAmountLteBridgeFee);

        if (fee != 0){
              let fee_coin=token.split<T>(fee, ctx);
              bridge_fee::deposit_fee(parent_id, fee_coin);
        };
        transfer::public_transfer(token, address::from_bytes(target_address));

        inner.external_bridge_records.push_back(
            key,
            ExternalBridgeRecord {
                source_chain,
                target_chain: inner.chain_id,
                source_address,
                target_address,
                amount,
                verified_signatures: option::some(signatures),
                claimed: true,
            },
        );

        emit(
            ExternalDepositedEventV2 {
                tx_hash,
                token_type: token_id,
                source_chain,
                target_chain: inner.chain_id,
                source_address,
                target_address,
                amount_before_fee: amount,
                amount_after_fee: amount - fee
            },
        )
    }

    public fun approval_and_claimed_external_busd_coin<T>(
        bridge: &mut Bridge,
        message: BridgeMessage,
        signatures: vector<vector<u8>>,
        bfc_system_state: &mut BfcSystemState,
        cap: &BfcSystemModifyCap,
        ctx: &mut TxContext
    ) {
        let (inner,parent_id) = load_inner_mut_and_uid(bridge);
        assert!(!inner.paused, EBridgeUnavailable);

        // verify signatures
        inner.committee.verify_signatures(message, signatures);

        assert!(message.message_type() == message_types::token(), EMustBeTokenMessage);
        assert!(message.message_version() == MESSAGE_VERSION, EUnexpectedMessageVersion);
        let token_payload = message.extract_token_bridge_payload();
        let target_chain = token_payload.token_target_chain();
        assert!(
            message.source_chain() == inner.chain_id || target_chain == inner.chain_id,
            EUnexpectedChainID,
        );

        let coin_type = type_name::into_string(type_name::get<T>());
        let token_id=treasury::token_id<T>(&inner.treasury);

        // check records
        let tx_hash = ascii::string(token_payload.token_tx_hash());
        let source_chain = message.source_chain();
        let target_chain = token_payload.token_target_chain();
        let source_address = token_payload.token_sender_address();
        let target_address = token_payload.token_target_address();
        let amount = token_payload.token_amount();
        let key = ExternalBridgeMessageKey{
            source_chain,
            source_address,
            target_address,
            amount,
            tx_hash,
        };
        if (inner.external_bridge_records.contains(key)) {
            emit(ExternalDepositedApprovedEvent{
                tx_hash,
                coin_type,
                source_chain: source_chain,
                target_chain: target_chain,
                source_address: source_address,
                target_address: target_address,
                amount: token_payload.token_amount(),
            });

            return
        };
        assert!(token_payload.token_amount() > 0, ETokenValueIsZero);
        let mut token =bfc_system_state.mint_stable<BUSD>(amount, cap,  ctx);
        //address::from_bytes(target_address),

        let fee=bridge_fee::calculate_cross_in_fee_amount(parent_id,source_chain as u64,token_id,amount);
        assert!(amount>fee,EInputAmountLteBridgeFee);

        if (fee != 0){
              let fee_coin=token.split<BUSD>(fee, ctx);
              bridge_fee::deposit_fee(parent_id, fee_coin);
        };
        transfer::public_transfer(token, address::from_bytes(target_address));

        inner.external_bridge_records.push_back(
            key,
            ExternalBridgeRecord {
                source_chain,
                target_chain: inner.chain_id,
                source_address,
                target_address,
                amount,
                verified_signatures: option::some(signatures),
                claimed: true,
            },
        );

        emit(
            ExternalDepositedEventV2 {
                tx_hash,
                token_type: token_id,
                source_chain,
                target_chain: inner.chain_id,
                source_address,
                target_address,
                amount_before_fee: amount,
                amount_after_fee: amount - fee
            },
        )
    }

    public fun withdraw_external_busd_coin<T>(
        bridge: &mut Bridge,
        target_chain: u8,
        target_address: vector<u8>,
        mut token: Coin<T>,
        token_id_expect: u64,
        bfc_system_state: &mut BfcSystemState,
        ctx: &mut TxContext
    ) {
        let (inner,parent_id) = load_inner_mut_and_uid(bridge);

        assert!(tokenlist::is_supported_from_benfen(
            parent_id, target_chain as u64, token_id_expect),EInvalidChainIDAndTokenIDExpect);
        assert!(token_id_expect == TOKEN_ID_USDC || token_id_expect == TOKEN_ID_USDT, EInvalidTokenIdExpect);
        assert!(type_name::get<T>() == type_name::get<BUSD>(), EOnlySupportBusd);
        assert!(!inner.paused, EBridgeUnavailable);
        assert!(chain_ids::is_valid_route(inner.chain_id, target_chain), EInvalidBridgeRoute);

        let amount = token.balance().value();
        assert!(amount > 0, ETokenValueIsZero);
        let fee=bridge_fee::calculate_cross_out_fee_amount(parent_id,target_chain as u64,token_id_expect,amount);
        assert!(amount>fee,EInputAmountLteBridgeFee);
        let fee_coin=token.split<T>(fee, ctx);
        bridge_fee::deposit_fee(parent_id, fee_coin);
        let amount_after_fee=amount-fee;

        bfc_system_state.burn_stable(token, ctx);

        // emit event
       emit(
            ExternalWithdrawEventV2 {
                token_type: token_id_expect,
                source_chain: inner.chain_id,
                target_chain,
                source_address: address::to_bytes(ctx.sender()),
                target_address,
                amount_before_fee: amount,
                amount_after_fee,
            },
        );
    }

    public fun withdraw_external_coin<T>(
        bridge: &mut Bridge,
        target_chain: u8,
        target_address: vector<u8>,
        mut token: Coin<T>,
        ctx: &mut TxContext
    ) {
        let (inner,parent_id) = load_inner_mut_and_uid(bridge);
        let token_id=treasury::token_id<T>(&inner.treasury);
        assert!(tokenlist::is_supported_from_benfen(
            parent_id, target_chain as u64, token_id),EInvalidChainIDAndTokenIDExpect);

        assert!(!inner.paused, EBridgeUnavailable);
        assert!(chain_ids::is_valid_route(inner.chain_id, target_chain), EInvalidBridgeRoute);

        let amount = token.balance().value();
        assert!(amount > 0, ETokenValueIsZero);
        let fee=bridge_fee::calculate_cross_out_fee_amount(parent_id,target_chain as u64,token_id,amount);
        assert!(amount>fee,EInputAmountLteBridgeFee);
        let fee_coin=token.split<T>(fee, ctx);
        bridge_fee::deposit_fee(parent_id, fee_coin);
        let amount_after_fee=amount-fee;
        let route = chain_ids::get_route(inner.chain_id, target_chain);
        let amount_in_usd = inner.treasury.calculate_amount_in_usd<T>(amount_after_fee);
        assert!(amount_in_usd <= limiter::get_external_out_limit(parent_id, &route), ETransferLimit);
        inner.treasury.burn(token);

        // emit event
        emit(
            ExternalWithdrawEventV2 {
                token_type: token_id,
                source_chain: inner.chain_id,
                target_chain,
                source_address: address::to_bytes(ctx.sender()),
                target_address,
                amount_before_fee: amount,
                amount_after_fee,
            },
        );
    }

    //////////////////////////////////////////////////////
    // DevInspect Functions for Read
    //


    public fun get_unclaimed_bridge_fee<T>(
         bridge: &Bridge,
    ):u64{
        bridge_fee::get_unclaimed_bridge_fee<T>(&bridge.id)
    }


    public fun get_cross_out_fee_amount<T>(
         bridge: &Bridge,
         chain_id: u64,
         amount: u64,
    ):u64{
        let (inner,parent_id) = load_inner_and_uid(bridge);
        let token_id = inner.treasury.token_id<T>();
        bridge_fee::calculate_cross_out_fee_amount(parent_id,chain_id,token_id,amount)
    }

    public fun get_cross_in_fee_amount<T>(
         bridge: &Bridge,
         chain_id: u64,
         amount: u64,
    ):u64{
        let (inner,parent_id) = load_inner_and_uid(bridge);
        let token_id = inner.treasury.token_id<T>();
        bridge_fee::calculate_cross_in_fee_amount(parent_id,chain_id,token_id,amount)
    }

    #[allow(unused_function)]
    fun get_token_transfer_action_status(
        bridge: &Bridge,
        source_chain: u8,
        bridge_seq_num: u64,
    ): u8 {
        let inner = load_inner(bridge);
        let key = message::create_key(
            source_chain,
            message_types::token(),
            bridge_seq_num
        );

        if (!inner.token_transfer_records.contains(key)) {
            return TRANSFER_STATUS_NOT_FOUND
        };

        let record = &inner.token_transfer_records[key];
        if (record.claimed) {
            return TRANSFER_STATUS_CLAIMED
        };

        if (record.verified_signatures.is_some()) {
            return TRANSFER_STATUS_APPROVED
        };

        TRANSFER_STATUS_PENDING
    }

    #[allow(unused_function)]
    fun get_external_token_transfer_action_status(
        bridge: &Bridge,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
    ): u8 {
        let inner = load_inner(bridge);

         let key = ExternalBridgeMessageKey{
            source_chain,
            source_address,
            target_address,
            amount,
            tx_hash,
        };

        if (!inner.external_bridge_records.contains(key)) {
            return TRANSFER_STATUS_NOT_FOUND
        };

        let record = &inner.external_bridge_records[key];
        if (record.claimed) {
            return TRANSFER_STATUS_CLAIMED
        };

        if (record.verified_signatures.is_some()) {
            return TRANSFER_STATUS_APPROVED
        };

        TRANSFER_STATUS_PENDING
    }

    // just for idempotent check
    #[allow(unused_function)]
    fun get_send_back_status(
        bridge: &Bridge,
        tx_hash: vector<u8>,
    ): u8 {
        let inner = load_inner(bridge);
        let key = message::key_refund(tx_hash);

        if (!inner.refund_records.contains(key)) {
            return TRANSFER_STATUS_NOT_FOUND
        };

        let record = &inner.refund_records[key];
        if (record.claimed) {
            return TRANSFER_STATUS_CLAIMED
        };
        TRANSFER_STATUS_PENDING
    }

    #[allow(unused_function)]
    fun get_token_transfer_action_signatures(
        bridge: &Bridge,
        source_chain: u8,
        bridge_seq_num: u64,
    ): Option<vector<vector<u8>>> {
        let inner = load_inner(bridge);
        let key = message::create_key(
            source_chain,
            message_types::token(),
            bridge_seq_num
        );

        if (!inner.token_transfer_records.contains(key)) {
            return option::none()
        };

        let record = &inner.token_transfer_records[key];
        record.verified_signatures
    }

    //////////////////////////////////////////////////////
    // Internal functions
    //
    fun multi_signature_passed(
        inner: & BridgeInner,
        key: ExternalBridgeMessageKey,
        coin_type: String,
    ): bool {
        // check then add to pre_deposit_multi_signature_records
        if (inner.pre_deposit_multi_signature_records.contains(key)) {
            let records = inner.pre_deposit_multi_signature_records[key];
            // if pre_deposit_multi_signature_records > 50%
            let signed = records.size();
            let len = inner.treasury.external_coin_admin_count(coin_type);
            if (signed * 2 > len) {
                return true
            };
        };

        false
    }

    fun load_inner(
        bridge: &Bridge,
    ): &BridgeInner {
        let version = bridge.inner.version();

        // TODO: Replace this with a lazy update function when we add a new version of the inner object.
        assert!(version == CURRENT_VERSION, EWrongInnerVersion);
        let inner: &BridgeInner = bridge.inner.load_value();
        assert!(inner.bridge_version == version, EWrongInnerVersion);
        inner
    }

    fun load_inner_mut(bridge: &mut Bridge): &mut BridgeInner {
        let version = bridge.inner.version();
        // TODO: Replace this with a lazy update function when we add a new version of the inner object.
        assert!(version == CURRENT_VERSION, EWrongInnerVersion);
        let inner: &mut BridgeInner = bridge.inner.load_value_mut();
        assert!(inner.bridge_version == version, EWrongInnerVersion);
        inner
    }

    fun load_inner_mut_and_uid(bridge: &mut Bridge): (&mut BridgeInner ,&mut UID){
        let version = bridge.inner.version();
        // TODO: Replace this with a lazy update function when we add a new version of the inner object.
        assert!(version == CURRENT_VERSION, EWrongInnerVersion);
        let inner: &mut BridgeInner = bridge.inner.load_value_mut();
        assert!(inner.bridge_version == version, EWrongInnerVersion);
        (inner,&mut bridge.id)
    }

    fun load_inner_and_uid(bridge: &Bridge): (&BridgeInner ,&UID){
        let version = bridge.inner.version();
        // TODO: Replace this with a lazy update function when we add a new version of the inner object.
        assert!(version == CURRENT_VERSION, EWrongInnerVersion);
        let inner = bridge.inner.load_value<BridgeInner>();
        assert!(inner.bridge_version == version, EWrongInnerVersion);
        (inner,&bridge.id)
    }

    // Claim token from approved bridge message
    // Returns Some(Coin) if coin can be claimed. If already claimed, return None
    fun claim_token_internal<T>(
        bridge: &mut Bridge,
        clock: &Clock,
        source_chain: u8,
        bridge_seq_num: u64,
        ctx: &mut TxContext,
    ): (Option<Coin<T>>, address) {
        let (inner,parent_id) = load_inner_mut_and_uid(bridge);
        assert!(!inner.paused, EBridgeUnavailable);
        let is_busd = type_name::get<T>() == type_name::get<BUSD>();
        assert!(!is_busd, EUseClaimBusd);
        let key = message::create_key(source_chain, message_types::token(), bridge_seq_num);
        assert!(inner.token_transfer_records.contains(key), EMessageNotFoundInRecords);

        // retrieve approved bridge message
        let record = &mut inner.token_transfer_records[key];
        // ensure this is a token bridge message
        assert!(
            &record.message.message_type() == message_types::token(),
            EUnexpectedMessageType,
        );
        // Ensure it's signed
        assert!(record.verified_signatures.is_some(), EUnauthorisedClaim);

        // extract token message
        let token_payload = record.message.extract_token_bridge_in_payload();
        // get owner address
        let owner = address::from_bytes(token_payload.token_target_address_in());

        // If already claimed, exit early
        if (record.claimed) {
            emit(TokenTransferAlreadyClaimed { message_key: key });
            return (option::none(), owner)
        };

        let target_chain = token_payload.token_target_chain_in();
        // ensure target chain matches bridge.chain_id
        assert!(target_chain == inner.chain_id, EUnexpectedChainID);

        // TODO: why do we check validity of the route here? what if inconsistency?
        // Ensure route is valid
        // TODO: add unit tests
        // `get_route` abort if route is invalid
        let route = chain_ids::get_route(source_chain, target_chain);
        // check token type
        assert!(
            treasury::token_id<T>(&inner.treasury) == token_payload.token_type_in(),
            EUnexpectedTokenType,
        );

        let amount = token_payload.token_amount_in();
        let fee=bridge_fee::calculate_cross_in_fee_amount(parent_id,source_chain as u64,token_payload.token_type_in(),amount);
        assert!(amount>fee,EInputAmountLteBridgeFee);

        // Make sure transfer is within limit.
        if (!inner
            .limiter
            .check_and_record_sending_transfer<T>(
            &inner.treasury,
            clock,
            route,
            amount,
        )
        ) {
            emit(TokenTransferLimitExceed { message_key: key });
            return (option::none(), owner)
        };

        let mut token = inner.treasury.mint<T>(amount, ctx);
        if (fee!=0){
              let fee_coin=token.split<T>(fee, ctx);
              bridge_fee::deposit_fee(parent_id, fee_coin);
        };
        // Record changes
        record.claimed = true;
        emit(TokenTransferClaimed { message_key: key });

        (option::some(token), owner)
    }

    fun check_fast_path_limit(
        bridge_id: &mut UID,
        clock: &Clock,
        token_payload: TokenTransferInPayload,
    ) {
        //fast path checker
        if (token_payload.token_fast_path_selector_in() != 2) { // 2 is finalized,0 and 1 is fast path
            let amount = token_payload.token_amount_in();
            let chain_id = token_payload.token_target_chain_in();
            let token_id = token_payload.token_type_in();
            let sender_address = token_payload.token_sender_address_in();
            let remaining_limit = limiter_fast_path::check_and_record_user_limit(bridge_id, sender_address, chain_id, token_id, amount, clock);
            assert!(remaining_limit, EFastPathLimitError);
        };
    }

    fun claim_stable_token_internal<T>(
        bridge: &mut Bridge,
        bfc_system_state: &mut BfcSystemState,
        clock: &Clock,
        source_chain: u8,
        bridge_seq_num: u64,
        cap: &BfcSystemModifyCap,
        ctx: &mut TxContext,
    ): (Option<Coin<T>>, address) {
        let (inner,parent_id) = load_inner_mut_and_uid(bridge);
        assert!(!inner.paused, EBridgeUnavailable);

        let key = message::create_key(source_chain, message_types::token(), bridge_seq_num);
        assert!(inner.token_transfer_records.contains(key), EMessageNotFoundInRecords);

        // retrieve approved bridge message
        let record = &mut inner.token_transfer_records[key];
        // ensure this is a token bridge message
        assert!(
            &record.message.message_type() == message_types::token(),
            EUnexpectedMessageType,
        );
        // Ensure it's signed
        assert!(record.verified_signatures.is_some(), EUnauthorisedClaim);

        // extract token message
        let token_payload = record.message.extract_token_bridge_in_payload();
        // get owner address
        let owner = address::from_bytes(token_payload.token_target_address_in());
        // get token type
        let token_id = token_payload.token_type_in();
        assert!(token_id == 5, EOnlySupportBusd);

        // If already claimed, exit early
        if (record.claimed) {
            emit(TokenTransferAlreadyClaimed { message_key: key });
            return (option::none(), owner)
        };

        let target_chain = token_payload.token_target_chain_in();
        // ensure target chain matches bridge.chain_id
        assert!(target_chain == inner.chain_id, EUnexpectedChainID);

        // `get_route` abort if route is invalid
        let route = chain_ids::get_route(source_chain, target_chain);
        // check token type
        assert!(
            treasury::token_id<T>(&inner.treasury) == token_payload.token_type_in(),
            EUnexpectedTokenType,
        );

        let amount = token_payload.token_amount_in();
        assert!(amount <= inner.limiter.get_mint_busd_max_limit(), EInvalidMintAmount);
        // Make sure transfer is within limit.
        if (!inner
            .limiter
            .check_and_record_sending_transfer<T>(
            &inner.treasury,
            clock,
            route,
            amount,
        )
        ) {
            emit(TokenTransferLimitExceed { message_key: key });
            return (option::none(), owner)
        };
        let token_id=token_payload.token_type_in();
        let fee=bridge_fee::calculate_cross_in_fee_amount(parent_id,source_chain as u64,token_id,amount);
        assert!(amount>fee,EInputAmountLteBridgeFee);
        let amount_after_fee=amount-fee;
        check_fast_path_limit(parent_id, clock, token_payload);
        // claim from treasury
        //transfer busd to owner
        bfc_system_state.mint_stable_entry_to_address<BUSD>(amount_after_fee, cap, owner, ctx);
        if (fee != 0){
            let fee_coin=bfc_system_state.mint_stable<BUSD>(fee,cap, ctx);
            bridge_fee::deposit_fee(parent_id, fee_coin);
        };
        record.claimed = true;
        emit(TokenTransferClaimed { message_key: key });
        (option::none(), owner)
    }

    fun execute_emergency_op(inner: &mut BridgeInner, payload: EmergencyOp) {
        let op = payload.emergency_op_type();
        if (op == message::emergency_op_pause()) {
            assert!(!inner.paused, EBridgeAlreadyPaused);
            inner.paused = true;
            emit(EmergencyOpEvent { frozen: true });
        } else if (op == message::emergency_op_unpause()) {
            assert!(inner.paused, EBridgeNotPaused);
            inner.paused = false;
            emit(EmergencyOpEvent { frozen: false });
        } else {
            abort EUnexpectedOperation
        };
    }

    fun execute_refund_admin_operate(inner: &mut BridgeInner, payload: message::RefundAdmin) {
        let op = payload.refund_admin_op_type();
        if (op == message::refund_admin_add()) {
            let sui_address = payload.refund_admin_sui_address();
            inner.add_refund_admin(sui_address);
        } else if (op == message::refund_admin_remove()) {
            let sui_address = payload.refund_admin_sui_address();
            inner.remove_refund_admin(sui_address);
        } else {
            abort EUnexpectedOperation
        };
    }

    fun add_refund_admin(inner: &mut BridgeInner, address: &String) {
        if (!inner.refund_admins.contains(address)) {
            inner.refund_admins.insert(*address);
        }
    }

    fun remove_refund_admin(inner: &mut BridgeInner, address: &String) {
        if (inner.refund_admins.contains(address)) {
            inner.refund_admins.remove(address);
        }
    }

    fun execute_update_bridge_limit(inner: &mut BridgeInner, payload: UpdateBridgeLimit) {
        let receiving_chain = payload.update_bridge_limit_payload_receiving_chain();
        assert!(receiving_chain == inner.chain_id, EUnexpectedChainID);
        let route = chain_ids::get_route(
            payload.update_bridge_limit_payload_sending_chain(),
            receiving_chain
        );

        inner.limiter.update_route_limit(
            &route,
            payload.update_bridge_limit_payload_limit()
        )
    }

    fun execute_update_asset_price(inner: &mut BridgeInner, payload: UpdateAssetPrice) {
        inner.treasury.update_asset_notional_price(
            payload.update_asset_price_payload_token_id(),
            payload.update_asset_price_payload_new_price()
        )
    }

    fun execute_add_external_coin_admin(inner: &mut BridgeInner, payload: AddExternalCoinAdmin) {
        inner.treasury.add_external_coin_admin(
            payload.add_external_coin_admin_payload_coin_type(),
            payload.add_external_coin_admin_payload_admin_address(),
        )
    }

    fun execute_remove_external_coin_admin(inner: &mut BridgeInner, payload: RemoveExternalCoinAdmin) {
        inner.treasury.remove_external_coin_admin(
            payload.remove_external_coin_admin_payload_coin_type(),
            payload.remove_external_coin_admin_payload_admin_address(),
        )
    }

    fun execute_add_external_coin_target_payload(inner: &mut BridgeInner, payload: AddExternalCoinTarget) {
        inner.treasury.add_external_coin_target(
            payload.add_external_coin_target_payload_coin_type(),
            payload.add_external_coin_target_payload_target_address(),
        )
    }

    fun execute_remove_external_coin_target_payload(inner: &mut BridgeInner, payload: RemoveExternalCoinTarget) {
        inner.treasury.remove_external_coin_target(
            payload.remove_external_coin_target_payload_coin_type(),
            payload.remove_external_coin_target_payload_target_address(),
        )
    }

    fun execute_add_external_coin_witness(inner: &mut BridgeInner, payload: AddExternalCoinWitness) {
        inner.treasury.add_external_coin_witness(
            payload.add_external_coin_witness_payload_coin_type(),
            payload.add_external_coin_witness_payload_witness_address(),
        )
    }

    fun execute_remove_external_coin_witness(inner: &mut BridgeInner, payload: RemoveExternalCoinWitness) {
        inner.treasury.remove_external_coin_witness(
            payload.remove_external_coin_witness_payload_coin_type(),
            payload.remove_external_coin_witness_payload_witness_address(),
        )
    }

    fun execute_set_cross_in(parent_id: &mut UID,payload:SetCrossInBridgeFee,ctx: &mut TxContext){
        let (chain_id,token_id,mode,amount)=payload.set_cross_in_bridge_fee_poyload();
        bridge_fee::set_fee_in_cross_in(parent_id,chain_id as u64,token_id,mode,amount,ctx);
    }
    fun execute_set_cross_out(parent_id: &mut UID,payload:SetCrossOutBridgeFee,ctx: &mut TxContext){
        let (chain_id,token_id,mode,amount)=payload.set_cross_out_bridge_fee_poyload();
        bridge_fee::set_fee_in_cross_out(parent_id,chain_id as u64,token_id,mode,amount,ctx);
    }

    fun execute_withdraw_bridge_fee(payload: WithdrawBridgeFee,ctx: &mut TxContext){
       let(recipient,coin_type,amount)=payload.withdraw_bridge_fee_polyload();
       let cap=bridge_fee::create_withdraw_fee_cap(coin_type, amount, ctx);
       transfer::public_transfer(cap,recipient);
    }


    fun execute_add_token_on_token_list(parent_id: &mut UID,payload: AddTokenOnTokenList,ctx: &mut TxContext){
        let source_chain=payload.add_token_on_token_list_payload_from_chain_id();
        let target_chain=payload.add_token_on_token_list_payload_to_chain_id();
        let token_id=payload.add_token_on_token_list_payload_token_id();

        if (target_chain==chain_ids::sui_mainnet() || target_chain==chain_ids::sui_testnet() || target_chain==chain_ids::sui_custom()) {
            tokenlist::add_token_to_benfen(parent_id,source_chain as u64,token_id,ctx);
        }else if (source_chain==chain_ids::sui_mainnet() || source_chain==chain_ids::sui_testnet() || source_chain==chain_ids::sui_custom())  {
            tokenlist::add_token_from_benfen(parent_id,target_chain as u64,token_id,ctx);
        }else{
            abort EInvalidChainIDOnTokenList
        }

    }

    fun execute_remove_token_on_token_list(parent_id: &mut UID,payload: RemoveTokenOnTokenList){
        let source_chain=payload.remove_token_on_token_list_payload_from_chain_id();
        let target_chain=payload.remove_token_on_token_list_payload_to_chain_id();
        let token_id=payload.remove_token_on_token_list_payload_token_id();

        if (target_chain==chain_ids::sui_mainnet() || target_chain==chain_ids::sui_testnet() || target_chain==chain_ids::sui_custom()) {
            tokenlist::remove_token_to_benfen(parent_id,source_chain as u64,token_id);
        }else if (source_chain==chain_ids::sui_mainnet() || source_chain==chain_ids::sui_testnet() || source_chain==chain_ids::sui_custom())  {
            tokenlist::remove_token_from_benfen(parent_id,target_chain as u64,token_id);
        }else{
            abort EInvalidChainIDOnTokenList
        }

    }


    fun execute_add_tokens_on_sui(inner: &mut BridgeInner, payload: AddTokenOnSui) {
        // FIXME: assert native_token to be false and add test
        let native_token = payload.is_native();
        let mut token_ids = payload.token_ids();
        let mut token_type_names = payload.token_type_names();
        let mut token_prices = payload.token_prices();

        // Make sure token data is consistent
        assert!(token_ids.length() == token_type_names.length(), EMalformedMessageError);
        assert!(token_ids.length() == token_prices.length(), EMalformedMessageError);

        while (token_ids.length() > 0) {
            let token_id = token_ids.pop_back();
            let token_type_name = token_type_names.pop_back();
            let token_price = token_prices.pop_back();
            inner.treasury.add_new_token(token_type_name, token_id, native_token, token_price)
        }
    }

    // Verify seq number matches the next expected seq number for the message type,
    // and increment it.
    fun get_current_seq_num_and_increment(bridge: &mut BridgeInner, msg_type: u8): u64 {
        if (!bridge.sequence_nums.contains(&msg_type)) {
            bridge.sequence_nums.insert(msg_type, 1);
            return 0
        };

        let entry = &mut bridge.sequence_nums[&msg_type];
        let seq_num = *entry;
        *entry = seq_num + 1;
        seq_num
    }

    #[allow(unused_function)]
    fun get_parsed_token_transfer_message(
        bridge: &Bridge,
        source_chain: u8,
        bridge_seq_num: u64,
    ): Option<ParsedTokenTransferMessage> {
        let inner = load_inner(bridge);
        let key = message::create_key(
            source_chain,
            message_types::token(),
            bridge_seq_num
        );

        if (!inner.token_transfer_records.contains(key)) {
            return option::none()
        };

        let record = &inner.token_transfer_records[key];
        let message = &record.message;
        option::some(to_parsed_token_transfer_message(message))
    }

    #[allow(unused_function)]
    fun get_parsed_token_transfer_message_v2(
        bridge: &Bridge,
        source_chain: u8,
        bridge_seq_num: u64,
    ): Option<ParsedTokenTransferMessageV2> {
        let inner = load_inner(bridge);
        let key = message::create_key(
            source_chain,
            message_types::token(),
            bridge_seq_num
        );

        if (!inner.token_transfer_records.contains(key)) {
            return option::none()
        };

        let record = &inner.token_transfer_records[key];
        let message = &record.message;
        option::some(to_parsed_token_transfer_message_v2(message))
    }

    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test_only]
    public fun add_external_coin_admin_for_testing(
        bridge: &mut Bridge,
        coin_type: ascii::String,
        admin_address: ascii::String,
    ) {
        bridge.load_inner_mut().treasury.add_external_coin_admin(coin_type, admin_address)
    }

    #[test_only]
    public fun remove_external_coin_admin_for_testing(
        bridge: &mut Bridge,
        coin_type: ascii::String,
        admin_address: ascii::String,
    ) {
        bridge.load_inner_mut().treasury.remove_external_coin_admin(coin_type, admin_address)
    }

    #[test_only]
    public fun find_external_bridge_record(
        bridge: &Bridge,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
    )   : Option<ExternalBridgeRecord> {
        let key = ExternalBridgeMessageKey{
            source_chain,
            source_address,
            target_address,
            amount,
            tx_hash,
        };
        let inner = load_inner(bridge)  ;
        if (!inner.external_bridge_records.contains(key)) {
            return option::none()
        };
        let record = &inner.external_bridge_records[key];
        return option::some(ExternalBridgeRecord{
            source_chain: record.source_chain,
            target_chain: record.target_chain,
            source_address: record.source_address,
            target_address: record.target_address,
            amount: record.amount,
            verified_signatures: record.verified_signatures,
            claimed: record.claimed,
        })
    }

    #[test_only]
    public fun create_bridge_for_testing(id: UID, chain_id: u8, ctx: &mut TxContext) {
        create(id, chain_id, ctx);
    }

    #[test_only]
    public fun new_for_testing(chain_id: u8, ctx: &mut TxContext): Bridge {
        let id = object::new(ctx);
        let bridge_inner = BridgeInner {
            bridge_version: CURRENT_VERSION,
            message_version: MESSAGE_VERSION,
            chain_id,
            sequence_nums: vec_map::empty(),
            committee: committee::create(ctx),
            treasury: treasury::create(ctx),
            token_transfer_records: linked_table::new(ctx),
            external_bridge_records: linked_table::new(ctx),
            pre_deposit_multi_signature_records: linked_table::new(ctx),
            limiter: limiter::new(),
            paused: false,
            refund_records: linked_table::new(ctx),
            refund_admins: vec_set::empty(),
        };
        let mut bridge = Bridge {
            id,
            inner: versioned::create(CURRENT_VERSION, bridge_inner, ctx),
        };
        bridge.setup_treasury_for_testing();
        bridge
    }

    #[test_only]
    public fun setup_refund_admin_for_testing(bridge: &mut Bridge, address: String) {
        let inner = load_inner_mut(bridge);
        inner.add_refund_admin(&address);
    }

    #[test_only]
    public fun setup_treasury_for_testing(bridge: &mut Bridge) {
        bridge.load_inner_mut().treasury.setup_for_testing();
    }

    #[test_only]
    public fun test_init_bridge_committee(
        bridge: &mut Bridge,
        active_validator_voting_power: VecMap<address, u64>,
        min_stake_participation_percentage: u64,
        ctx: &TxContext
    ) {
        init_bridge_committee(
            bridge,
            active_validator_voting_power,
            min_stake_participation_percentage,
            ctx,
        );
    }

    #[test_only]
    public fun new_bridge_record_for_testing(
        message: BridgeMessage,
        verified_signatures: Option<vector<vector<u8>>>,
        claimed: bool,
    ): BridgeRecord {
        BridgeRecord {
            message,
            verified_signatures,
            claimed
        }
    }

    #[test_only]
    public fun test_load_inner_mut(bridge: &mut Bridge): &mut BridgeInner {
        bridge.load_inner_mut()
    }

    #[test_only]
    public fun test_load_inner(bridge: &Bridge): &BridgeInner {
        bridge.load_inner()
    }

    #[test_only]
    public fun test_load_limiter(bridge: &Bridge): &TransferLimiter {
        &bridge.load_inner().limiter
    }

    #[test_only]
    public fun test_load_mut_uid(bridge: &mut Bridge): &mut UID {
       &mut bridge.id
    }

    #[test_only]
    public fun test_get_token_transfer_action_status(
        bridge: &mut Bridge,
        source_chain: u8,
        bridge_seq_num: u64,
    ): u8 {
        bridge.get_token_transfer_action_status(source_chain, bridge_seq_num)
    }

    #[test_only]
     public fun test_get_external_token_transfer_action_status(
        bridge: &Bridge,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
    ): u8 {
        bridge.get_external_token_transfer_action_status(source_chain, source_address,target_address,amount,tx_hash)
    }



    #[test_only]
    public fun test_get_token_transfer_action_signatures(
        bridge: &mut Bridge,
        source_chain: u8,
        bridge_seq_num: u64,
    ): Option<vector<vector<u8>>> {
        bridge.get_token_transfer_action_signatures(source_chain, bridge_seq_num)
    }

    #[test_only]
    public fun test_get_parsed_token_transfer_message(
        bridge: &Bridge,
        source_chain: u8,
        bridge_seq_num: u64,
    ): Option<ParsedTokenTransferMessage> {
        bridge.get_parsed_token_transfer_message(source_chain, bridge_seq_num)
    }

    #[test_only]
    public fun test_get_parsed_token_transfer_message_v2(
        bridge: &Bridge,
        source_chain: u8,
        bridge_seq_num: u64,
    ): Option<ParsedTokenTransferMessageV2> {
        bridge.get_parsed_token_transfer_message_v2(source_chain, bridge_seq_num)
    }

    #[test_only]
    public fun inner_limiter(bridge_inner: &BridgeInner): &TransferLimiter {
        &bridge_inner.limiter
    }

    #[test_only]
    public fun inner_treasury(bridge_inner: &BridgeInner): &BridgeTreasury {
        &bridge_inner.treasury
    }

    #[test_only]
    public fun inner_treasury_mut(bridge_inner: &mut BridgeInner): &mut BridgeTreasury {
        &mut bridge_inner.treasury
    }

    #[test_only]
    public fun inner_paused(bridge_inner: &BridgeInner): bool {
        bridge_inner.paused
    }

    #[test_only]
    public fun inner_token_transfer_records(
        bridge_inner: &BridgeInner,
    ): &LinkedTable<BridgeMessageKey, BridgeRecord> {
        &bridge_inner.token_transfer_records
    }

    #[test_only]
    public fun inner_token_transfer_records_mut(
        bridge_inner: &mut BridgeInner,
    ): &mut LinkedTable<BridgeMessageKey, BridgeRecord> {
        &mut bridge_inner.token_transfer_records
    }

    #[test_only]
    public fun test_execute_emergency_op(
        bridge_inner: &mut BridgeInner,
        payload: EmergencyOp,
    ) {
        bridge_inner.execute_emergency_op(payload)
    }

    #[test_only]
    public fun sequence_nums(bridge_inner: &BridgeInner): &VecMap<u8, u64> {
        &bridge_inner.sequence_nums
    }

    #[test_only]
    public fun assert_paused(bridge_inner: &BridgeInner, error: u64) {
        assert!(bridge_inner.paused, error);
    }

    #[test_only]
    public fun assert_not_paused(bridge_inner: &BridgeInner, error: u64) {
        assert!(!bridge_inner.paused, error);
    }

    #[test_only]
    public fun test_get_current_seq_num_and_increment(
        bridge_inner: &mut BridgeInner,
        msg_type: u8,
    ): u64 {
        get_current_seq_num_and_increment(bridge_inner, msg_type)
    }

    #[test_only]
    public fun test_execute_update_bridge_limit(
        inner: &mut BridgeInner,
        payload: UpdateBridgeLimit,
    ) {
        execute_update_bridge_limit(inner, payload)
    }

    #[test_only]
    public fun test_execute_update_asset_price(
        inner: &mut BridgeInner,
        payload: UpdateAssetPrice,
    ) {
        execute_update_asset_price(inner, payload)
    }

    #[test_only]
    public fun transfer_status_pending(): u8 {
        TRANSFER_STATUS_PENDING
    }

    #[test_only]
    public fun transfer_status_approved(): u8 {
        TRANSFER_STATUS_APPROVED
    }

    #[test_only]
    public fun transfer_status_claimed(): u8 {
        TRANSFER_STATUS_CLAIMED
    }

    #[test_only]
    public fun transfer_status_not_found(): u8 {
        TRANSFER_STATUS_NOT_FOUND
    }

    #[test_only]
    public fun test_execute_add_tokens_on_sui(bridge: &mut Bridge, payload: AddTokenOnSui) {
        let inner = load_inner_mut(bridge);
        inner.execute_add_tokens_on_sui(payload);
    }

    #[test_only]
    public fun get_seq_num_for(bridge: &mut Bridge, message_type: u8): u64 {
        let inner = load_inner_mut(bridge);
        let seq_num = if (inner.sequence_nums.contains(&message_type)) {
            inner.sequence_nums[&message_type]
        } else {
            inner.sequence_nums.insert(message_type, 0);
            0
        };
        seq_num
    }

    #[test_only]
    public fun get_seq_num_inc_for(bridge: &mut Bridge, message_type: u8): u64 {
        let inner = load_inner_mut(bridge);
        inner.get_current_seq_num_and_increment(message_type)
    }

    #[test_only]
    public fun transfer_approve_key(event: TokenTransferApproved): BridgeMessageKey {
        event.message_key
    }

    #[test_only]
    public fun transfer_claimed_key(event: TokenTransferClaimed): BridgeMessageKey {
        event.message_key
    }

    #[test_only]
    public fun transfer_already_approved_key(event: TokenTransferAlreadyApproved): BridgeMessageKey {
        event.message_key
    }

    #[test_only]
    public fun transfer_already_claimed_key(event: TokenTransferAlreadyClaimed): BridgeMessageKey {
        event.message_key
    }

    #[test_only]
    public fun transfer_limit_exceed_key(event: TokenTransferLimitExceed): BridgeMessageKey {
        event.message_key
    }

    #[test_only]
    public fun unwrap_external_deposited_approved_event(event: ExternalDepositedApprovedEvent):  (ascii::String, ascii::String, u8, u8, vector<u8>, vector<u8>, u64)  {
        (
            event.tx_hash,
            event.coin_type,
            event.source_chain,
            event.target_chain,
            event.source_address,
            event.target_address,
            event.amount,
        )
    }

    #[test_only]
    public fun unwrap_external_deposited_event(event: ExternalDepositedEvent):  (ascii::String, ascii::String, u8, u8, vector<u8>, vector<u8>, u64)  {
        (
            event.tx_hash,
            event.coin_type,
            event.source_chain,
            event.target_chain,
            event.source_address,
            event.target_address,
            event.amount,
        )
    }

    #[test_only]
    public fun unwrap_external_deposited_event_v2(event: ExternalDepositedEventV2):  (ascii::String, u64, u8, u8, vector<u8>, vector<u8>, u64,u64)  {
        (
            event.tx_hash,
            event.token_type,
            event.source_chain,
            event.target_chain,
            event.source_address,
            event.target_address,
            event.amount_before_fee,
            event.amount_after_fee,
        )
    }

    #[test_only]
    public fun unwrap_external_withdrawn_event(event: ExternalWithdrawEvent): (ascii::String, u8, u8, vector<u8>, vector<u8>, u64) {
        (
            event.coin_type,
            event.source_chain,
            event.target_chain,
            event.source_address,
            event.target_address,
            event.amount,
        )
    }

    #[test_only]
    public fun unwrap_external_withdrawn_v2_event(event: ExternalWithdrawEventV2): (u64, u8,u8,vector<u8>,vector<u8>, u64,u64){
        (
            event.token_type,
            event.source_chain,
            event.target_chain,
            event.source_address,
            event.target_address,
            event.amount_before_fee,
            event.amount_after_fee,
        )
    }

    #[test_only]
    public fun unwrap_external_bridge_record(record: ExternalBridgeRecord): (u8, u8, vector<u8>, vector<u8>, u64) {
        (
            record.source_chain,
            record.target_chain,
            record.source_address,
            record.target_address,
            record.amount,
        )
    }

    #[test_only]
    public fun unwrap_deposited_event(event: TokenDepositedEvent): (u64, u8, vector<u8>, u8, vector<u8>, u64, u64) {
        (
            event.seq_num,
            event.source_chain,
            event.sender_address,
            event.target_chain,
            event.target_address,
            event.token_type,
            event.amount,
        )
    }

    #[test_only]
    public fun unwrap_deposited_event_v2(event: TokenDepositedEventV2): (u64, u8, vector<u8>, u8, vector<u8>, u64, u64,u64) {
        (
            event.seq_num,
            event.source_chain,
            event.sender_address,
            event.target_chain,
            event.target_address,
            event.token_type,
            event.amount_before_fee,
            event.amount_after_fee,
        )
    }

    #[test_only]
    public fun unwrap_send_back_event(event: TokenSendBackEvent): (u64, u8, vector<u8>, u8, vector<u8>, u64, u64, vector<u8>) {
        (
            event.seq_num,
            event.source_chain,
            event.sender_address,
            event.target_chain,
            event.target_address,
            event.token_type,
            event.amount,
            event.tx_hash,
        )
    }

    #[test_only]
    public fun unwrap_send_back_event_v2(event: TokenSendBackEventV2): (u64, u8, vector<u8>, u8, vector<u8>, u64, u64, vector<u8>) {
        (
            event.seq_num,
            event.source_chain,
            event.sender_address,
            event.target_chain,
            event.target_address,
            event.token_type,
            event.amount,
            event.tx_hash,
        )
    }
    #[test_only]
    public fun unwrap_emergency_op_event(event: EmergencyOpEvent): bool {
        event.frozen
    }
}
