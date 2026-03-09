module bridge::bridge_fee{

    use sui::table::{Self, Table};
    use sui::dynamic_field;
    use sui::bag::{Self,Bag};
    use std::type_name::{Self,TypeName};
    use sui::balance::{Self,Balance};
    use sui::coin::{Self,Coin};


    use bridge::chain_ids;
    use bridge::tokenlist;
    use std::ascii::{String};

    const KEY: vector<u8> = b"bridge_fee";
    const EBridgeFeeRegistryAlreadyExists: u64=0;
    const EBridgeFeeTypeNotSupport: u64=1;
    const EBridgeFeeChainIDAndTokenIDNotExpect: u64=3;
    const EBridgeFeeWithdrawCoinTypeNotMatch: u64=4;
    const EBridgeFeeSettingWrong: u64=4;


    /// Fee rates are expressed in millionths (1e6 precision)
    /// e.g. 1% = 10000; 0.01% = 100; 0.0001% = 1
    const FEE_RATE_PRECISION: u64 = 1_000_000;

    public struct WithdrawBridgeFeeCap has key,store {
        id: UID,
        coin_type: String,
        amount: u64,
    }

    /// Bridge fee configuration struct, manages fee settings for cross-chain in and out operations,
    /// as well as a funds pool to hold coin balances.
    public struct BridgeFee has store {
        /// Cross-chain out fee configuration:
        /// Maps chain_id -> token_id -> FeeInfo
        from_benfen: Table<u64, Table<u64, FeeInfo>>,

        /// Cross-chain in fee configuration:
        /// Maps chain_id -> token_id -> FeeInfo
        to_benfen: Table<u64, Table<u64, FeeInfo>>,

        /// Funds pool to store balances of various coin types,
        /// e.g., funds[TypeName] = Balance<T>
        funds: Bag,
    }

    /// Fee information struct supports fixed and percentage fee modes.
    public struct FeeInfo has copy, drop, store {
        /// Fee mode, either Fixed or Percentage.
        mode: u64,

        /// Fee value:
        /// - If mode == Fixed(0), this is the fixed fee amount (same unit as token amount).
        /// - If mode == Percentage(1), this is the fee rate in parts per million (ppm).
        value: u64,
    }


     public(package) fun new_bridge_fee_registry(parent_id: &mut UID,ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, KEY),
            EBridgeFeeRegistryAlreadyExists // TODO - add custom error type
        );
        dynamic_field::add(
            parent_id,
            KEY,
            new(ctx),
        );
        initial_bridge_fee(parent_id,ctx);
    }

    public(package) fun new(ctx: &mut TxContext): BridgeFee {
        let (from_benfen, to_benfen, funds) = empty(ctx);
        BridgeFee {
            from_benfen,
            to_benfen,
            funds,
        }
    }

    fun empty(
        ctx: &mut TxContext
    ): (
        Table<u64, Table<u64, FeeInfo>>,
        Table<u64, Table<u64, FeeInfo>>,
        Bag,
    ) {
        let from_benfen = table::new<u64, Table<u64, FeeInfo>>(ctx);
        let to_benfen = table::new<u64, Table<u64, FeeInfo>>(ctx);
        let funds = bag::new(ctx);
        (from_benfen, to_benfen, funds)
    }

    fun initial_bridge_fee(parent_id: &mut UID,ctx: &mut TxContext) {
        //todo btc
        set_fee_in_cross_out(parent_id,chain_ids::btc_mainnet() as u64,tokenlist::get_btc_token_id() ,0,43202,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::btc_testnet() as u64,tokenlist::get_btc_token_id() ,0,43202,ctx);

        // tron usdt
        set_fee_in_cross_out(parent_id,chain_ids::tron_mainnet() as u64,tokenlist::get_usdt_token_id() ,0,5_000_000_000,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::tron_testnet() as u64,tokenlist::get_usdt_token_id() ,0,5_000_000_000,ctx);

        //ETH usdc
        set_fee_in_cross_out(parent_id,chain_ids::eth_mainnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::eth_sepolia() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::eth_custom() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        //ETH usdt
        set_fee_in_cross_out(parent_id,chain_ids::eth_mainnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::eth_sepolia() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::eth_custom() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        //bsc usdc
        set_fee_in_cross_out(parent_id,chain_ids::bsc_mainnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::bsc_testnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::bsc_custom() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        //bsc usdt
        set_fee_in_cross_out(parent_id,chain_ids::bsc_mainnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::bsc_testnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::bsc_custom() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);

        //base usdc
        set_fee_in_cross_out(parent_id,chain_ids::base_mainnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::base_testnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::base_custom() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        //base usdt
        set_fee_in_cross_out(parent_id,chain_ids::base_mainnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::base_testnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::base_custom() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);

        //op usdc
        set_fee_in_cross_out(parent_id,chain_ids::op_mainnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::op_testnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::op_custom() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        //op usdt
        set_fee_in_cross_out(parent_id,chain_ids::op_mainnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::op_testnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::op_custom() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);

        //polygon usdc
        set_fee_in_cross_out(parent_id,chain_ids::pol_mainnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::pol_testnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::pol_custom() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        //polygon usdt
        set_fee_in_cross_out(parent_id,chain_ids::pol_mainnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::pol_testnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::pol_custom() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);

        //arb usdc
        set_fee_in_cross_out(parent_id,chain_ids::arb_mainnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::arb_testnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::arb_custom() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        //arb usdt
        set_fee_in_cross_out(parent_id,chain_ids::arb_mainnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::arb_testnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::arb_custom() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);

        //avax usdc
        set_fee_in_cross_out(parent_id,chain_ids::avax_mainnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::avax_testnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::avax_custom() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        //avax usdt
        set_fee_in_cross_out(parent_id,chain_ids::avax_mainnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::avax_testnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::avax_custom() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);

        //sol usdc
        set_fee_in_cross_out(parent_id,chain_ids::solana_mainnet()as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::solana_testnet() as u64,tokenlist::get_usdc_token_id() ,1,500,ctx);
        //sol usdt
        set_fee_in_cross_out(parent_id,chain_ids::solana_mainnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);
        set_fee_in_cross_out(parent_id,chain_ids::solana_testnet() as u64,tokenlist::get_usdt_token_id() ,1,500,ctx);

    }



    public(package) fun borrow(parent_id: &UID): &BridgeFee{
        dynamic_field::borrow<vector<u8>,BridgeFee>(parent_id, KEY)
    }

    public(package) fun borrow_mut(parent_id: &mut UID): &mut BridgeFee{
        dynamic_field::borrow_mut<vector<u8>,BridgeFee>(parent_id, KEY)
    }

    public(package) fun set_fee_in_cross_in(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64,
        mode: u64,
        value: u64,
        ctx: &mut TxContext
    ) {
        assert!(mode < 2,EBridgeFeeTypeNotSupport);
        let self=borrow_mut(parent_id);
        if (!self.to_benfen.contains(chain_id)) {
            self.to_benfen.add(chain_id, table::new(ctx));
        };
        let new_fee_info=new_fee_info(mode,value);
        assert!(new_fee_info.mode < 2,EBridgeFeeTypeNotSupport);
        if (!self.to_benfen.borrow(chain_id).contains(token_id)){
            self.to_benfen.borrow_mut(chain_id).add(token_id, new_fee_info);
        }else{
           *self.to_benfen.borrow_mut(chain_id).borrow_mut(token_id)=new_fee_info
        }
    }


    public(package) fun set_fee_in_cross_out(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64,
        mode: u64,
        value: u64,
        ctx: &mut TxContext
    ) {
        assert!(mode < 2,EBridgeFeeTypeNotSupport);
        let self=borrow_mut(parent_id);
        if (!self.from_benfen.contains(chain_id)) {
            self.from_benfen.add(chain_id, table::new(ctx));
        };
        let new_fee_info=new_fee_info(mode,value);
        assert!(new_fee_info.mode < 2,EBridgeFeeTypeNotSupport);
        if (!self.from_benfen.borrow(chain_id).contains(token_id)){
            self.from_benfen.borrow_mut(chain_id).add(token_id, new_fee_info);
        }else{
           *self.from_benfen.borrow_mut(chain_id).borrow_mut(token_id)=new_fee_info
        }
    }

    /// Compute the net amount after applying the fee
    public fun get_cross_out_amount_after_fee(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64,
        amount: u64
    ): u64 {
        if (!is_token_id_supported_in_cross_out_internal(borrow(parent_id), chain_id, token_id)) {
            abort EBridgeFeeChainIDAndTokenIDNotExpect
        };
        let fee = calculate_cross_out_fee_amount(parent_id, chain_id, token_id, amount);
        if (amount > fee) {
            amount - fee
        } else {
            0
        }
    }

    public fun get_cross_in_amount_after_fee(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64,
        amount: u64
    ): u64 {
        if (!is_token_id_supported_in_cross_in_internal(borrow(parent_id), chain_id, token_id)) {
            abort EBridgeFeeChainIDAndTokenIDNotExpect
        };
        let fee = calculate_cross_in_fee_amount(parent_id, chain_id, token_id, amount);
        if (amount > fee) {
            amount - fee
        } else {
            0
        }
    }

    public(package) fun get_fee_info_cross_in(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64,
    ): (u64,u64){
        let self = borrow(parent_id);
        let fee_info= get_fee_info_to_benfen(self, chain_id, token_id);
        (fee_info.mode,fee_info.value)
    }


    public(package) fun get_fee_info_cross_out(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64,
    ): (u64,u64){
        let self = borrow(parent_id);
        let fee_info=get_fee_info_from_benfen(self, chain_id, token_id);
        (fee_info.mode,fee_info.value)
    }


    public fun calculate_cross_out_fee_amount(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64,
        amount: u64
    ): u64 {
        let self = borrow(parent_id);

        let fee_info = get_fee_info_from_benfen(self, chain_id, token_id);

        // Fixed fee mode
        if (fee_info.mode == 0) {
            return fee_info.value
        };

        // Percentage-based fee mode
        if (fee_info.mode == 1) {
            return calculate_fee(amount, fee_info.value)
        };
        0
    }


     public fun calculate_cross_in_fee_amount(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64,
        amount: u64
    ): u64 {
        let self = borrow(parent_id);

        let fee_info = get_fee_info_to_benfen(self, chain_id, token_id);

        // Fixed fee mode
        if (fee_info.mode == 0) {
            return fee_info.value

        };

        // Percentage-based fee mode
        if (fee_info.mode == 1) {
            return calculate_fee(amount, fee_info.value)
        };
        0
    }


    public fun deposit_fee<T>(parent_id: &mut UID,coin: Coin<T>){
        let coin_type=type_name::get<T>();
        let self=borrow_mut(parent_id);
        if (!self.is_asset_supported(coin_type)){
            //create funds store
            bag::add(&mut self.funds, coin_type, balance::zero<T>())
        };
        let bal=bag::borrow_mut<TypeName,Balance<T>>(&mut self.funds, coin_type);
        bal.join(coin.into_balance());
    }


    public(package) fun withdraw_fee<T>(parent_id: &mut UID, cap: WithdrawBridgeFeeCap,ctx: &mut TxContext): Coin<T>{
        let input_coin_type=type_name::get<T>();
        let WithdrawBridgeFeeCap{
            id,
            coin_type,
            amount
        }=cap;
        let input_coin_type_str =input_coin_type.into_string();

        assert!(input_coin_type_str==coin_type,EBridgeFeeWithdrawCoinTypeNotMatch);
        object::delete(id);
        let self=borrow_mut(parent_id);
        if (!self.is_asset_supported(input_coin_type)){
            return coin::zero<T>(ctx)
        };
        let bal=bag::borrow_mut<TypeName,Balance<T>>(&mut self.funds, input_coin_type);
        bal.split(amount).into_coin(ctx)
    }

    public(package) fun create_withdraw_fee_cap(coin_type: String,amount: u64,ctx: &mut TxContext):WithdrawBridgeFeeCap{
        WithdrawBridgeFeeCap {
            id: object::new(ctx),
            coin_type,
            amount
        }
    }

    public fun get_withdraw_cap_coin_type(cap: &WithdrawBridgeFeeCap): String{
        cap.coin_type
    }
    public fun get_withdraw_cap_amount(cap: &WithdrawBridgeFeeCap): u64{
        cap.amount
    }

    public fun get_unclaimed_bridge_fee<T>(parent_id: &UID): u64{
        let coin_type=type_name::get<T>();
        let self=borrow(parent_id);
        if (!self.is_asset_supported(coin_type)){
            return 0
        };
        bag::borrow<TypeName,Balance<T>>(&self.funds, coin_type).value()
    }

    public fun is_asset_supported(self :&BridgeFee,coin_type: TypeName):bool{
        bag::contains(&self.funds, coin_type)
    }

    fun is_token_id_supported_in_cross_out_internal(self :&BridgeFee,chain_id: u64, token_id: u64):bool{
        if (!self.from_benfen.contains(chain_id)) {
            return false
        };
        let inner = self.from_benfen.borrow(chain_id);
        if (!inner.contains(token_id)) {
            return false
        };
        true
    }

    fun is_token_id_supported_in_cross_in_internal(self :&BridgeFee,chain_id: u64, token_id: u64):bool{
        if (!self.to_benfen.contains(chain_id)) {
            return false
        };
        let inner = self.to_benfen.borrow(chain_id);
        if (!inner.contains(token_id)) {
            return false
        };
        true
    }
    

    fun get_fee_info_from_benfen(self :&BridgeFee,chain_id: u64, token_id: u64): FeeInfo{
         if (!self.from_benfen.contains(chain_id)) {
           return default_fee_info()
        };
        let inner = self.from_benfen.borrow(chain_id);
        if (!inner.contains(token_id)) {
          return default_fee_info()
        };
        *inner.borrow(token_id)
    }

     fun get_fee_info_to_benfen(self :&BridgeFee,chain_id: u64, token_id: u64): FeeInfo{
         if (!self.to_benfen.contains(chain_id)) {
           return default_fee_info()
        };
        let inner = self.to_benfen.borrow(chain_id);
        if (!inner.contains(token_id)) {
          return default_fee_info()
        };
        *inner.borrow(token_id)
    }
    fun default_fee_info():FeeInfo{
        FeeInfo { mode:2, value:0 }
    }

    fun new_fee_info(
        mode: u64,
        value: u64
    ): FeeInfo{
         FeeInfo { mode, value}
    }

    fun calculate_fee(amount: u64,fee_rate: u64) : u64 {
        assert!(fee_rate < FEE_RATE_PRECISION, EBridgeFeeSettingWrong);
        (((amount as u128)*( fee_rate as u128)) /(FEE_RATE_PRECISION as u128)) as u64
    }

}