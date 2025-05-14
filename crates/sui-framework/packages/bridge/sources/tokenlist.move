module bridge::tokenlist {

    use sui::table::{Self, Table};
    use sui::dynamic_field;

    use bridge::chain_ids;

    const KEY: vector<u8> = b"bridge_token_list";

    const EChainIDAndTokenIDNotExist: u64=0;
    const EBridgeTokenListRegistryAlreadyExists: u64=1;

    /// Metadata for a bridged token.
    public struct TokenInfo has store, copy, drop {
        /// The originating chain's ID.
        chain_id: u64,
        /// The token's unique identifier on the originating chain.
        token_id: u64,
        /// Number of decimal places used by the token.
        decimal: u64,
    }

    // Stores all the supported bridged tokens and their metadata,
    /// including those that can be transferred from or to the Benfen chain.
    public struct BridgeTokenList has store {
        /// Tokens supported for bridging **from** Benfen to external chains.
        /// Mapping: chain_id => (token_id => true)
        from_benfen: Table<u64, Table<u64, bool>>,

        /// Tokens supported for bridging **to** Benfen from external chains.
        /// Mapping: chain_id => (token_id => true)
        to_benfen: Table<u64, Table<u64, bool>>,

        /// Metadata for supported tokens.
        /// Mapping: chain_id => (token_id => TokenInfo)
        tokens: Table<u64, Table<u64, TokenInfo>>,

    }

     public(package) fun new_tokenlist_registry(parent_id: &mut UID,ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, KEY),
            EBridgeTokenListRegistryAlreadyExists // TODO - add custom error type
        );
        dynamic_field::add(
            parent_id,
            KEY,
            new(ctx),
        );
        initial_token_list(parent_id,ctx);
    }

    /// Initializes an empty `BridgeTokenList` with all tables.
    public(package) fun new(ctx: &mut TxContext): BridgeTokenList {
        let (from_benfen, to_benfen, tokens) = empty(ctx);
        BridgeTokenList {
            from_benfen,
            to_benfen,
            tokens,
        }
    }

    public(package) fun initial_token_list(parent_id: &mut UID,ctx: &mut TxContext){
        //btc cross in benfen
        //bitcoin
        add_token_to_benfen(parent_id,chain_ids::btc_mainnet() as u64,1,ctx);
        add_token_to_benfen(parent_id,chain_ids::btc_testnet() as u64,1,ctx);

        //eth cross in benfen
        //eth
        add_token_to_benfen(parent_id,chain_ids::eth_mainnet() as u64,2,ctx);
        add_token_to_benfen(parent_id,chain_ids::eth_sepolia() as u64,2,ctx);
        add_token_to_benfen(parent_id,chain_ids::eth_custom() as u64,2,ctx);

        //usdc
        add_token_to_benfen(parent_id,chain_ids::eth_mainnet() as u64,3,ctx);
        add_token_to_benfen(parent_id,chain_ids::eth_sepolia() as u64,3,ctx);
        add_token_to_benfen(parent_id,chain_ids::eth_custom() as u64,3,ctx);

        //usdt
        add_token_to_benfen(parent_id,chain_ids::eth_mainnet() as u64,4,ctx);
        add_token_to_benfen(parent_id,chain_ids::eth_sepolia() as u64,4,ctx);
        add_token_to_benfen(parent_id,chain_ids::eth_custom() as u64,4,ctx);



        //bsc cross in  benfen
        //usdc
        add_token_to_benfen(parent_id,chain_ids::bsc_mainnet() as u64,3,ctx);
        add_token_to_benfen(parent_id,chain_ids::bsc_testnet() as u64,3,ctx);
        add_token_to_benfen(parent_id,chain_ids::bsc_custom() as u64,3,ctx);

        //usdt
        add_token_to_benfen(parent_id,chain_ids::bsc_mainnet() as u64,4,ctx);
        add_token_to_benfen(parent_id,chain_ids::bsc_testnet() as u64,4,ctx);
        add_token_to_benfen(parent_id,chain_ids::bsc_custom() as u64,4,ctx);

        //bnb
        add_token_to_benfen(parent_id,chain_ids::bsc_mainnet() as u64,6,ctx);
        add_token_to_benfen(parent_id,chain_ids::bsc_testnet() as u64,6,ctx);
        add_token_to_benfen(parent_id,chain_ids::bsc_custom() as u64,6,ctx);


        //base cross in benfen
        //usdc
        add_token_to_benfen(parent_id,chain_ids::base_mainnet() as u64,3,ctx);
        add_token_to_benfen(parent_id,chain_ids::base_testnet() as u64,3,ctx);
        add_token_to_benfen(parent_id,chain_ids::base_custom() as u64,3,ctx);

        //usdt
        add_token_to_benfen(parent_id,chain_ids::base_mainnet() as u64,4,ctx);
        add_token_to_benfen(parent_id,chain_ids::base_testnet() as u64,4,ctx);
        add_token_to_benfen(parent_id,chain_ids::base_custom() as u64,4,ctx);

        //eth
        add_token_to_benfen(parent_id,chain_ids::base_mainnet() as u64,2,ctx);
        add_token_to_benfen(parent_id,chain_ids::base_testnet() as u64,2,ctx);
        add_token_to_benfen(parent_id,chain_ids::base_custom() as u64,2,ctx);



        //op cross in  benfen
        //usdc
        add_token_to_benfen(parent_id,chain_ids::op_mainnet() as u64,3,ctx);
        add_token_to_benfen(parent_id,chain_ids::op_testnet() as u64,3,ctx);
        add_token_to_benfen(parent_id,chain_ids::op_custom() as u64,3,ctx);

        //usdt
        add_token_to_benfen(parent_id,chain_ids::op_mainnet() as u64,4,ctx);
        add_token_to_benfen(parent_id,chain_ids::op_testnet() as u64,4,ctx);
        add_token_to_benfen(parent_id,chain_ids::op_custom() as u64,4,ctx);

        //eth
        add_token_to_benfen(parent_id,chain_ids::op_mainnet() as u64,2,ctx);
        add_token_to_benfen(parent_id,chain_ids::op_testnet() as u64,2,ctx);
        add_token_to_benfen(parent_id,chain_ids::op_custom() as u64,2,ctx);

        //op
        add_token_to_benfen(parent_id,chain_ids::op_mainnet() as u64,7,ctx);
        add_token_to_benfen(parent_id,chain_ids::op_testnet() as u64,7,ctx);
        add_token_to_benfen(parent_id,chain_ids::op_custom() as u64,7,ctx);


        //cross out  bitcoin
        //bitcoin
        add_token_from_benfen(parent_id,chain_ids::btc_mainnet() as u64,1,ctx);
        add_token_from_benfen(parent_id,chain_ids::btc_testnet() as u64,1,ctx);


        //cross out eth
        //eth
        add_token_from_benfen(parent_id,chain_ids::eth_mainnet() as u64,2,ctx);
        add_token_from_benfen(parent_id,chain_ids::eth_sepolia() as u64,2,ctx);
        add_token_from_benfen(parent_id,chain_ids::eth_custom() as u64,2,ctx);

        //usdc
        add_token_from_benfen(parent_id,chain_ids::eth_mainnet() as u64,3,ctx);
        add_token_from_benfen(parent_id,chain_ids::eth_sepolia() as u64,3,ctx);
        add_token_from_benfen(parent_id,chain_ids::eth_custom() as u64,3,ctx);

        //usdt
        add_token_from_benfen(parent_id,chain_ids::eth_mainnet() as u64,4,ctx);
        add_token_from_benfen(parent_id,chain_ids::eth_sepolia() as u64,4,ctx);
        add_token_from_benfen(parent_id,chain_ids::eth_custom() as u64,4,ctx);


        //cross out bsc
        //usdc
        add_token_from_benfen(parent_id,chain_ids::bsc_mainnet() as u64,3,ctx);
        add_token_from_benfen(parent_id,chain_ids::bsc_testnet() as u64,3,ctx);
        add_token_from_benfen(parent_id,chain_ids::bsc_custom() as u64,3,ctx);

        //usdt
        add_token_from_benfen(parent_id,chain_ids::bsc_mainnet() as u64,4,ctx);
        add_token_from_benfen(parent_id,chain_ids::bsc_testnet() as u64,4,ctx);
        add_token_from_benfen(parent_id,chain_ids::bsc_custom() as u64,4,ctx);

        //bnb
        add_token_from_benfen(parent_id,chain_ids::bsc_mainnet() as u64,6,ctx);
        add_token_from_benfen(parent_id,chain_ids::bsc_testnet() as u64,6,ctx);
        add_token_from_benfen(parent_id,chain_ids::bsc_custom() as u64,6,ctx);


        //cross out base
        //usdc
        add_token_from_benfen(parent_id,chain_ids::base_mainnet() as u64,3,ctx);
        add_token_from_benfen(parent_id,chain_ids::base_testnet() as u64,3,ctx);
        add_token_from_benfen(parent_id,chain_ids::base_custom() as u64,3,ctx);

        //usdt
        add_token_from_benfen(parent_id,chain_ids::base_mainnet() as u64,4,ctx);
        add_token_from_benfen(parent_id,chain_ids::base_testnet() as u64,4,ctx);
        add_token_from_benfen(parent_id,chain_ids::base_custom() as u64,4,ctx);

        //eth
        add_token_from_benfen(parent_id,chain_ids::base_mainnet() as u64,2,ctx);
        add_token_from_benfen(parent_id,chain_ids::base_testnet() as u64,2,ctx);
        add_token_from_benfen(parent_id,chain_ids::base_custom() as u64,2,ctx);


        //cross out op
        //usdc
        add_token_from_benfen(parent_id,chain_ids::op_mainnet() as u64,3,ctx);
        add_token_from_benfen(parent_id,chain_ids::op_testnet() as u64,3,ctx);
        add_token_from_benfen(parent_id,chain_ids::op_custom() as u64,3,ctx);

        //usdt
        add_token_from_benfen(parent_id,chain_ids::op_mainnet() as u64,4,ctx);
        add_token_from_benfen(parent_id,chain_ids::op_testnet() as u64,4,ctx);
        add_token_from_benfen(parent_id,chain_ids::op_custom() as u64,4,ctx);

        //eth
        add_token_from_benfen(parent_id,chain_ids::op_mainnet() as u64,2,ctx);
        add_token_from_benfen(parent_id,chain_ids::op_testnet() as u64,2,ctx);
        add_token_from_benfen(parent_id,chain_ids::op_custom() as u64,2,ctx);

        //op
        add_token_from_benfen(parent_id,chain_ids::op_mainnet() as u64,7,ctx);
        add_token_from_benfen(parent_id,chain_ids::op_testnet() as u64,7,ctx);
        add_token_from_benfen(parent_id,chain_ids::op_custom() as u64,7,ctx);

    }

    public(package) fun borrow(parent_id: &UID): &BridgeTokenList{
        dynamic_field::borrow<vector<u8>,BridgeTokenList>(parent_id, KEY)
    }

    public(package) fun borrow_mut(parent_id: &mut UID): &mut BridgeTokenList{
        dynamic_field::borrow_mut<vector<u8>,BridgeTokenList>(parent_id, KEY)
    }

    /// Internal helper to initialize nested tables.
    fun empty(
        ctx: &mut TxContext
    ): (
        Table<u64, Table<u64, bool>>,
        Table<u64, Table<u64, bool>>,
        Table<u64, Table<u64, TokenInfo>>,
    ) {
        let from_benfen = table::new<u64, Table<u64, bool>>(ctx);
        let to_benfen = table::new<u64, Table<u64, bool>>(ctx);
        let tokens = table::new<u64, Table<u64, TokenInfo>>(ctx);
        (from_benfen, to_benfen, tokens)
    }

    /// Adds a token to the list of those supported **to** Benfen.
    public(package) fun add_token_to_benfen(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64,
        ctx: &mut TxContext
    ) {
        let self=borrow_mut(parent_id);
        if (self.is_supported_to_benfen_internal(chain_id, token_id)) {
            return
        };
        if (!self.to_benfen.contains(chain_id)) {
            self.to_benfen.add(chain_id, table::new(ctx));
        };
        self.to_benfen.borrow_mut(chain_id).add(token_id, true);
    }

    /// Adds a token to the list of those supported **from** Benfen.
    public(package) fun add_token_from_benfen(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64,
        ctx: &mut TxContext
    ) {
        let self=borrow_mut(parent_id);
        if (self.is_supported_from_benfen_internal(chain_id, token_id)) {
            return
        };
        if (!self.from_benfen.contains(chain_id)) {
            self.from_benfen.add(chain_id, table::new(ctx));
        };
        self.from_benfen.borrow_mut(chain_id).add(token_id, true);
    }

    /// Removes a token from the **to Benfen** supported list.
    public(package) fun remove_token_to_benfen(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64
    ) {
        let self=borrow_mut(parent_id);
        if (!self.is_supported_to_benfen_internal(chain_id, token_id)) {
            return
        };
        self.to_benfen.borrow_mut(chain_id).remove(token_id);
    }

    /// Removes a token from the **from Benfen** supported list.
    public(package) fun remove_token_from_benfen(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64
    ) {
        let self=borrow_mut(parent_id);
        if (!self.is_supported_from_benfen_internal(chain_id, token_id)) {
            return
        };
        self.from_benfen.borrow_mut(chain_id).remove(token_id);
    }


    /// Adds token metadata if not already present.
    public(package) fun add_token_info(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64,
        decimal: u64,
        ctx: &mut TxContext
    ) {
        let self=borrow_mut(parent_id);
        if (!self.tokens.contains(chain_id)) {
            self.tokens.add(chain_id, table::new(ctx));
        };
        if (!self.tokens.borrow(chain_id).contains(token_id)) {
            let info = TokenInfo {
                chain_id,
                token_id,
                decimal,
            };
            self.tokens.borrow_mut(chain_id).add(token_id, info);
        };
    }

    /// Removes token metadata if exists.
    public(package) fun remove_token_info(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64
    ) {
        let self=borrow_mut(parent_id);
        if (!self.is_exist_token_info_internal(chain_id, token_id)) {
            return
        };
        self.tokens.borrow_mut(chain_id).remove(token_id);
    }

     /// Fetches metadata of a specific token. Panics if not found.
    public fun get_token_info(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64
    ): TokenInfo {
        let self=borrow(parent_id);
        assert!(self.is_exist_token_info_internal(chain_id, token_id), EChainIDAndTokenIDNotExist);
        *self.tokens.borrow(chain_id).borrow(token_id)
    }

    public fun is_supported_to_benfen(
        parnet_id: &UID,
        chain_id: u64,
        token_id: u64
    ):bool{
        let self=borrow(parnet_id);
        self.is_supported_to_benfen_internal(chain_id,token_id)

    }

     public fun is_supported_from_benfen(
        parnet_id: &UID,
        chain_id: u64,
        token_id: u64
    ):bool{
        let self=borrow(parnet_id);
        self.is_supported_from_benfen_internal(chain_id,token_id)

    }

    public fun is_exist_token_info(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64
    ):bool{
        let self=borrow(parent_id);
        self.is_exist_token_info_internal(chain_id,token_id)
    }


    /// Returns whether token metadata exists.
    fun is_exist_token_info_internal(
        self: &BridgeTokenList,
        chain_id: u64,
        token_id: u64
    ): bool {
        if (!self.tokens.contains(chain_id)) {
            return false
        };
        self.tokens.borrow(chain_id).contains(token_id)
    }

    /// Checks whether a token is supported for bridging **into** Benfen.
    fun is_supported_to_benfen_internal(
        self: &BridgeTokenList,
        chain_id: u64,
        token_id: u64
    ): bool {
        if (!self.to_benfen.contains(chain_id)) {
            return false
        };
        let inner = self.to_benfen.borrow(chain_id);
        if (!inner.contains(token_id)) {
            return false
        };
        *inner.borrow(token_id)
    }


    /// Checks whether a token is supported for bridging **from** Benfen.
    fun is_supported_from_benfen_internal(
        self: &BridgeTokenList,
        chain_id: u64,
        token_id: u64
    ): bool {
        if (!self.from_benfen.contains(chain_id)) {
            return false
        };
        let inner = self.from_benfen.borrow(chain_id);
        if (!inner.contains(token_id)) {
            return false
        };
        *inner.borrow(token_id)
    }


    #[test_only]
    public(package) fun new_tokenlist_registry_for_testing(parent_id: &mut UID,ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, KEY),
            EBridgeTokenListRegistryAlreadyExists // TODO - add custom error type
        );
        dynamic_field::add(
            parent_id,
            KEY,
            new(ctx),
        );
    }

}
