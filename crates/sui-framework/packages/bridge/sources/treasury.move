// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::treasury {
    use std::ascii;
    use std::ascii::String;
    use std::type_name;
    use std::type_name::TypeName;

    use sui::address;
    use sui::bag;
    use sui::bag::Bag;
    use sui::coin::{Self, Coin, TreasuryCap, CoinMetadata};
    use sui::event::emit;
    use sui::hex;
    use sui::object_bag::{Self, ObjectBag};
    use sui::package;
    use sui::package::UpgradeCap;
    use sui::vec_map;
    use sui::vec_map::VecMap;
    use sui::vec_set;
    use sui::vec_set::VecSet;
    use sui::ecdsa_k1;
    use sui::hash;
    use bridge::crypto;
    use bridge::message;

    const EUnsupportedTokenType: u64 = 1;
    const EInvalidUpgradeCap: u64 = 2;
    const ETokenSupplyNonZero: u64 = 3;
    const EInvalidNotionalValue: u64 = 4;

    #[test_only]
    const USD_VALUE_MULTIPLIER: u64 = 100000000; // 8 DP accuracy

    //////////////////////////////////////////////////////
    // Types
    //

    public struct BridgeTreasury has store {
        // “0x00::btc::BTC”:Set<address>
        external_coin_admin_address: VecMap<String, VecSet<String>>,
        external_coin_target_address: VecMap<String, VecSet<String>>,

        external_coin_witness_address: VecMap<String,VecSet<vector<u8>>>,
        // token treasuries, values are TreasuryCaps for native bridge V1.
        treasuries: ObjectBag,
        supported_tokens: VecMap<TypeName, BridgeTokenMetadata>,
        // Mapping token id to type name
        id_token_type_map: VecMap<u64, TypeName>,
        // Bag for storing potential new token waiting to be approved
        waiting_room: Bag
    }

    public struct BridgeTokenMetadata has store, copy, drop {
        id: u64,
        decimal_multiplier: u64,
        notional_value: u64,
        native_token: bool
    }

    public struct ForeignTokenRegistration has store {
        type_name: TypeName,
        uc: UpgradeCap,
        decimal: u8,
    }

    public struct UpdateTokenPriceEvent has copy, drop {
        token_id: u64,
        new_price: u64,
    }

    public struct NewTokenEvent has copy, drop {
        token_id: u64,
        type_name: TypeName,
        native_token: bool,
        decimal_multiplier: u64,
        notional_value: u64
    }

    public struct TokenRegistrationEvent has copy, drop {
        type_name: TypeName,
        decimal: u8,
        native_token: bool
    }

    public struct AddExternalCoinAdminEvent has copy, drop {
        coin_type_name: String,
        address: String,
    }

    public struct RemoveExternalCoinAdminEvent has copy, drop {
    coin_type_name: String,
    address: String,
    }

    public struct AddExternalCoinWitnessEvent has copy, drop {
    coin_type_name: String,
    address: vector<u8>,
    }

    public struct RemoveExternalCoinWitnessEvent has copy, drop {
    coin_type_name: String,
    address: vector<u8>,
    }

    public struct AddExternalCoinTargetEvent has copy, drop {
    coin_type_name: String,
    address: String,
}

    public struct RemoveExternalCoinTargetEvent has copy, drop {
    coin_type_name: String,
    address: String,
}

    public fun token_id<T>(self: &BridgeTreasury): u64 {
        let metadata = self.get_token_metadata<T>();
        metadata.id
    }

    public fun decimal_multiplier<T>(self: &BridgeTreasury): u64 {
        let metadata = self.get_token_metadata<T>();
        metadata.decimal_multiplier
    }

    public fun notional_value<T>(self: &BridgeTreasury): u64 {
        let metadata = self.get_token_metadata<T>();
        metadata.notional_value
    }

    //////////////////////////////////////////////////////
    // Internal functions
    //

    public(package) fun register_foreign_token<T>(
        self: &mut BridgeTreasury,
        tc: TreasuryCap<T>,
        uc: UpgradeCap,
        metadata: &CoinMetadata<T>,
    ) {
        // Make sure TreasuryCap has not been minted before.
        assert!(coin::total_supply(&tc) == 0, ETokenSupplyNonZero);
        let type_name = type_name::get<T>();
        let address_bytes = hex::decode(ascii::into_bytes(type_name::get_address(&type_name)));
        let coin_address = address::from_bytes(address_bytes);
        // Make sure upgrade cap is for the Coin package
        // FIXME: add test
        assert!(
            object::id_to_address(&package::upgrade_package(&uc))
                == coin_address, EInvalidUpgradeCap
        );
        let registration = ForeignTokenRegistration {
            type_name,
            uc,
            decimal: coin::get_decimals(metadata),
        };
        self.waiting_room.add(type_name::into_string(type_name), registration);
        self.treasuries.add(type_name, tc);

        emit(TokenRegistrationEvent{
            type_name,
            decimal: coin::get_decimals(metadata),
            native_token: false
        });
    }

    public(package) fun add_new_token(
        self: &mut BridgeTreasury,
        token_name: String,
        token_id: u64,
        native_token: bool,
        notional_value: u64,
    ) {
        if (!native_token){
            assert!(notional_value > 0, EInvalidNotionalValue);
            let ForeignTokenRegistration{
                type_name,
                uc,
                decimal,
            } = self.waiting_room.remove<String, ForeignTokenRegistration>(token_name);
            let decimal_multiplier = 10u64.pow(decimal);
            self.supported_tokens.insert(
                type_name,
                BridgeTokenMetadata{
                    id: token_id,
                    decimal_multiplier,
                    notional_value,
                    native_token
                },
            );
            self.id_token_type_map.insert(token_id, type_name);

            // Freeze upgrade cap to prevent changes to the coin
            transfer::public_freeze_object(uc);

            emit(NewTokenEvent{
                token_id,
                type_name,
                native_token,
                decimal_multiplier,
                notional_value
            })
        } else {
            // Not implemented for V1
        }
    }

    public(package) fun create(ctx: &mut TxContext): BridgeTreasury {
        BridgeTreasury {
            external_coin_admin_address: vec_map::empty(),
            external_coin_target_address: vec_map::empty(),
            external_coin_witness_address: vec_map::empty(),
            treasuries: object_bag::new(ctx),
            supported_tokens: vec_map::empty(),
            id_token_type_map: vec_map::empty(),
            waiting_room: bag::new(ctx),
        }
    }

    public(package) fun is_external_coin_admin(
        self: &BridgeTreasury,
        coin_type_name: String,
        address: String,
    ): bool {
        let admins = self.external_coin_admin_address.try_get(&coin_type_name);
        if (admins.is_none()) {
            return false
        };
        let admins = admins.destroy_some();
        admins.contains(&address)
    }

      public(package) fun is_external_coin_witness(
        self: &BridgeTreasury,
        coin_type_name: String,
        addr: vector<u8>,
    ): bool {
        let admins = self.external_coin_witness_address.try_get(&coin_type_name);
        if (admins.is_none()) {
            return false
        };
        let admins = admins.destroy_some();
        admins.contains(&addr)
    }

    public(package) fun external_coin_admin_count(
        self: &BridgeTreasury,
        coin_type_name: String,
    ): u64 {
        let admins = self.external_coin_admin_address.try_get(&coin_type_name);
        if (admins.is_none()) {
            return 0
        };
        let admins = admins.destroy_some();
        return admins.size()
    }

    public(package) fun add_external_coin_admin(
        self: &mut BridgeTreasury,
        coin_type_name: String,
        address: String,
    ) {
        let admins = self.external_coin_admin_address.try_get(&coin_type_name);
        if (admins.is_none()) {
            self.external_coin_admin_address.insert(coin_type_name, vec_set::empty());
        };
        let admins = self.external_coin_admin_address.get_mut(&coin_type_name);
        if (!admins.contains(&address)) {
            admins.insert(address);
        };
        emit(AddExternalCoinAdminEvent{
            coin_type_name,
            address
        })
    }

    public(package) fun add_external_coin_target(
        self: &mut BridgeTreasury,
        coin_type_name: String,
        addr: String,
    ) {
        let admins = self.external_coin_target_address.try_get(&coin_type_name);
        if (admins.is_none()) {
            self.external_coin_target_address.insert(coin_type_name, vec_set::empty());
        };
        let admins = self.external_coin_target_address.get_mut(&coin_type_name);
        if (!admins.contains(&addr)) {
            admins.insert(addr);
        };
        emit(AddExternalCoinTargetEvent{
            coin_type_name,
            address: addr
        })

    }

    public(package) fun add_external_coin_witness(
        self: &mut BridgeTreasury,
        coin_type_name: String,
        addr: vector<u8>,
  ) {
        let admins = self.external_coin_witness_address.try_get(&coin_type_name);
        if (admins.is_none()) {
            self.external_coin_witness_address.insert(coin_type_name, vec_set::empty());
        };
        let admins = self.external_coin_witness_address.get_mut(&coin_type_name);
        if (!admins.contains(&addr)) {
            admins.insert(addr);
        };
        emit(AddExternalCoinWitnessEvent{
            coin_type_name,
            address: addr
        })
    }

    public(package) fun remove_external_coin_witness(
        self: &mut BridgeTreasury,
        coin_type_name: String,
        addr: vector<u8>,
     ) {
        let admins = self.external_coin_witness_address.try_get(&coin_type_name);
        if (admins.is_none()) {
            return
        };
        let admins = self.external_coin_witness_address.get_mut(&coin_type_name);
        if (admins.contains(&addr)) {
            admins.remove(&addr);
            if (admins.size() == 0) {
                self.external_coin_witness_address.remove(&coin_type_name);
            };
            emit(RemoveExternalCoinWitnessEvent{
                coin_type_name,
                address: addr
            })
        }
    }


     public fun verify_bitcoin_signatures<T>(
        self: &BridgeTreasury,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
        signatures: vector<u8>,
    ):bool{
        let coin_type = type_name::into_string(type_name::get<T>());

        let bitcoin_message=message::create_bitcoin_message(
            source_chain,
            source_address,
            target_address,
            amount,
            *ascii::as_bytes(&tx_hash),
            *ascii::as_bytes(&coin_type)
        );
        let msg=hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
        let pubkey =
            ecdsa_k1::decompress_pubkey(&ecdsa_k1::secp256k1_ecrecover(&signatures, &msg, 0));
        let addr=crypto::ecdsa_pub_key_to_eth_address(&pubkey);
        self.is_external_coin_witness(coin_type, addr)
    }

    public(package) fun remove_external_coin_admin(
        self: &mut BridgeTreasury,
        coin_type_name: String,
        address: String,
    ) {
        let admins = self.external_coin_admin_address.try_get(&coin_type_name);
        if (admins.is_none()) {
            return
        };
        let admins = self.external_coin_admin_address.get_mut(&coin_type_name);
        if (admins.contains(&address)) {
            admins.remove(&address);
            if (admins.size() == 0) {
                self.external_coin_admin_address.remove(&coin_type_name);
            };
            emit(RemoveExternalCoinAdminEvent{
                coin_type_name,
                address
            })
        }
    }

     public(package) fun remove_external_coin_target(
        self: &mut BridgeTreasury,
        coin_type_name: String,
        addr: String,
    ) {
        let admins = self.external_coin_target_address.try_get(&coin_type_name);
        if (admins.is_none()) {
            return
        };
        let admins = self.external_coin_target_address.get_mut(&coin_type_name);
        if (admins.contains(&addr)) {
            admins.remove(&addr);
            if (admins.size() == 0) {
                self.external_coin_target_address.remove(&coin_type_name);
            };
            emit(RemoveExternalCoinTargetEvent{
                coin_type_name,
                address: addr
            })
        }
    }

    public(package) fun burn<T>(self: &mut BridgeTreasury, token: Coin<T>) {
        let treasury = &mut self.treasuries[type_name::get<T>()];
        coin::burn(treasury, token);
    }

    public(package) fun mint<T>(
        self: &mut BridgeTreasury,
        amount: u64,
        ctx: &mut TxContext,
    ): Coin<T> {
        let treasury = &mut self.treasuries[type_name::get<T>()];
        coin::mint(treasury, amount, ctx)
    }

    public(package)  fun calculate_amount_in_usd<T>(
        treasury: &BridgeTreasury,
        amount: u64): u64 {
        let metadata = treasury.get_token_metadata<T>();
        ((amount as u128) * (metadata.notional_value as u128) / (metadata.decimal_multiplier as u128)) as u64
    }


    public(package) fun update_asset_notional_price(
        self: &mut BridgeTreasury,
        token_id: u64,
        new_usd_price: u64,
    ) {
        let type_name = self.id_token_type_map.try_get(&token_id);
        assert!(type_name.is_some(), EUnsupportedTokenType);
        assert!(new_usd_price > 0, EInvalidNotionalValue);
        let type_name = type_name.destroy_some();
        let metadata = self.supported_tokens.get_mut(&type_name);
        metadata.notional_value = new_usd_price;

        emit(UpdateTokenPriceEvent {
            token_id,
            new_price: new_usd_price,
        })
    }

    fun get_token_metadata<T>(self: &BridgeTreasury): BridgeTokenMetadata {
        let coin_type = type_name::get<T>();
        let metadata = self.supported_tokens.try_get(&coin_type);
        assert!(metadata.is_some(), EUnsupportedTokenType);
        metadata.destroy_some()
    }

    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test_only]
    public struct ETH has drop {}
    #[test_only]
    public struct BTC has drop {}
    #[test_only]
    public struct USDT has drop {}
    #[test_only]
    public struct USDC has drop {}

    #[test_only]
    public struct BUSD has drop {}

    #[test_only]
    public struct LTC has drop {}


    #[test_only]
    public fun new_for_testing(ctx: &mut TxContext): BridgeTreasury {
        create(ctx)
    }

    #[test_only]
    public fun mock_for_test(ctx: &mut TxContext): BridgeTreasury {
        let mut treasury = new_for_testing(ctx);
        treasury.setup_for_testing();
        treasury
    }

    #[test_only]
    public fun setup_for_testing(treasury: &mut BridgeTreasury) {
        treasury.supported_tokens.insert(type_name::get<BTC>(), BridgeTokenMetadata{
            id: 1,
            decimal_multiplier: 100_000_000,
            notional_value: 50_000 * USD_VALUE_MULTIPLIER,
            native_token: false,
        });
        treasury.supported_tokens.insert(type_name::get<ETH>(), BridgeTokenMetadata{
            id: 2,
            decimal_multiplier: 100_000_000,
            notional_value: 3_000 * USD_VALUE_MULTIPLIER,
            native_token: false,
        });
        treasury.supported_tokens.insert(type_name::get<USDC>(), BridgeTokenMetadata{
            id: 3,
            decimal_multiplier: 1_000_000,
            notional_value: USD_VALUE_MULTIPLIER,
            native_token: false,
        });
        treasury.supported_tokens.insert(type_name::get<USDT>(), BridgeTokenMetadata{
            id: 4,
            decimal_multiplier: 1_000_000,
            notional_value: USD_VALUE_MULTIPLIER,
            native_token: false,
        });
        treasury.supported_tokens.insert(type_name::get<BUSD>(), BridgeTokenMetadata{
            id: 5,
            decimal_multiplier: 1_000_000_000,
            notional_value: USD_VALUE_MULTIPLIER,
            native_token: false,
        });

        treasury.supported_tokens.insert(type_name::get<LTC>(), BridgeTokenMetadata{
            id: 6,
            decimal_multiplier: 1_000_000_000_000,
            notional_value: USD_VALUE_MULTIPLIER,
            native_token: false,
        });

        treasury.id_token_type_map.insert(1, type_name::get<BTC>());
        treasury.id_token_type_map.insert(2, type_name::get<ETH>());
        treasury.id_token_type_map.insert(3, type_name::get<USDC>());
        treasury.id_token_type_map.insert(4, type_name::get<USDT>());
        treasury.id_token_type_map.insert(5, type_name::get<BUSD>());
        treasury.id_token_type_map.insert(6, type_name::get<LTC>());
    }

    #[test_only]
    public fun waiting_room(treasury: &BridgeTreasury): &Bag {
        &treasury.waiting_room
    }

    #[test_only]
    public fun treasuries(treasury: &BridgeTreasury): &ObjectBag {
        &treasury.treasuries
    }

    #[test_only]
    public fun external_coin_admin_address(treasury: &BridgeTreasury): &VecMap<String, VecSet<String>> {
        &treasury.external_coin_admin_address
    }

    #[test_only]
    public fun external_coin_witness_address(treasury: &BridgeTreasury): &VecMap<String,VecSet<vector<u8>>> {
        &treasury.external_coin_witness_address
    }

    #[test_only]
    public fun external_coin_target_address(treasury: &BridgeTreasury): &VecMap<String,VecSet<String>> {
        &treasury.external_coin_target_address
    }

    #[test_only]
    public fun unwrap_update_event(event: UpdateTokenPriceEvent): (u64, u64) {
        (event.token_id, event.new_price)
    }

    #[test_only]
    public fun unwrap_new_token_event(event: NewTokenEvent): (u64, TypeName, bool, u64, u64) {
        (event.token_id, event.type_name, event.native_token, event.decimal_multiplier, event.notional_value)
    }

    #[test_only]
    public fun unwrap_registration_event(event: TokenRegistrationEvent): (TypeName, u8, bool) {
        (event.type_name, event.decimal, event.native_token)
    }
}
