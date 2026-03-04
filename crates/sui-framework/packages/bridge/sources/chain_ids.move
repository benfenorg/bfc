// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::chain_ids {

    // Chain IDs
    const SuiMainnet: u8 = 0;
    const SuiTestnet: u8 = 1;
    const SuiCustom: u8 = 2;

    const EthMainnet: u8 = 10;
    const EthSepolia: u8 = 11;
    const EthCustom: u8 = 12;

    const BtcMainnet: u8 = 20;
    const BtcTestnet: u8 = 21;

    const BscMainnet: u8 = 30;
    const BscTestnet: u8 = 31;
    const BscCustom: u8 = 32;

    const OPMainnet: u8 = 33;
    const OPTestnet: u8 = 34;
    const OPCustom: u8 = 35;

    const ArbMainnet: u8 = 36;
    const ArbTestnet: u8 = 37;
    const ArbCustom: u8 = 38;

    const PolMainnet: u8 = 39;
    const PolTestnet: u8 = 40;
    const PolCustom: u8 = 41;

    const BaseMainnet: u8 = 42;
    const BaseTestnet: u8 = 43;
    const BaseCustom: u8 = 44;

    const AvaxMainnet: u8 = 45;
    const AvaxTestnet: u8 = 46;
    const AvaxCustom: u8 = 47;

    const TronMainnet: u8 = 48;
    const TronTestnet: u8 = 49;

    const SolanaMainnet: u8 = 50;
    const SolanaTestnet: u8 = 51;

    const LTCMainnet: u8 = 52;
    const LTCTestnet: u8 = 53;

    const DogeMainnet: u8 = 54;
    const DogeTestnet: u8 = 55;

    const SuiOfficialMainnet: u8 = 56;
    const SuiOfficialTestnet: u8 = 57;

    const AptosMainnet: u8 = 58;
    const AptosTestnet: u8 = 59;


    const EInvalidBridgeRoute: u64 = 0;

    //////////////////////////////////////////////////////
    // Types
    //

    public struct BridgeRoute has copy, drop, store {
        source: u8,
        destination: u8,
    }

    //////////////////////////////////////////////////////
    // Public functions
    //

    public fun sui_mainnet(): u8 { SuiMainnet }
    public fun sui_testnet(): u8 { SuiTestnet }
    public fun sui_custom(): u8 { SuiCustom }

    public fun eth_mainnet(): u8 { EthMainnet }
    public fun eth_sepolia(): u8 { EthSepolia }
    public fun eth_custom(): u8 { EthCustom }

    public fun btc_mainnet(): u8 { BtcMainnet }
    public fun btc_testnet(): u8 { BtcTestnet }

    public fun bsc_mainnet(): u8 { BscMainnet }
    public fun bsc_testnet(): u8 { BscTestnet }
    public fun bsc_custom(): u8  { BscCustom  }


    public fun base_mainnet(): u8 { BaseMainnet }
    public fun base_testnet(): u8 { BaseTestnet }
    public fun base_custom(): u8 { BaseCustom }

    public fun op_mainnet(): u8 { OPMainnet }
    public fun op_testnet(): u8 { OPTestnet }
    public fun op_custom(): u8 { OPCustom }

    public fun arb_mainnet(): u8 { ArbMainnet }
    public fun arb_testnet(): u8 { ArbTestnet }
    public fun arb_custom(): u8 { ArbCustom }

    public fun pol_mainnet(): u8 { PolMainnet }
    public fun pol_testnet(): u8 { PolTestnet }
    public fun pol_custom(): u8 { PolCustom }

    public fun avax_mainnet(): u8 { AvaxMainnet }
    public fun avax_testnet(): u8 { AvaxTestnet }
    public fun avax_custom(): u8 { AvaxCustom }

    public fun tron_mainnet(): u8 { TronMainnet }
    public fun tron_testnet(): u8 { TronTestnet }

    public fun solana_mainnet(): u8 { SolanaMainnet }
    public fun solana_testnet(): u8 { SolanaTestnet }

    public fun ltc_mainnet(): u8 { LTCMainnet }
    public fun ltc_testnet(): u8 { LTCTestnet }

    public fun doge_mainnet(): u8 { DogeMainnet }
    public fun doge_testnet(): u8 { DogeTestnet }

    public fun sui_official_mainnet(): u8 { SuiOfficialMainnet }
    public fun sui_official_testnet(): u8 { SuiOfficialTestnet }

    public fun aptos_mainnet(): u8 { AptosMainnet }
    public fun aptos_testnet(): u8 { AptosTestnet }

    public use fun route_source as BridgeRoute.source;
    public fun route_source(route: &BridgeRoute): &u8 {
        &route.source
    }

    public use fun route_destination as BridgeRoute.destination;
    public fun route_destination(route: &BridgeRoute): &u8 {
        &route.destination
    }

    public fun assert_valid_chain_id(id: u8) {
        assert!(
            id == BtcMainnet ||
            id == BtcTestnet ||
            id == SuiMainnet ||
            id == SuiTestnet ||
            id == SuiCustom ||
            id == EthMainnet ||
            id == EthSepolia ||
            id == EthCustom ||
            id == BaseMainnet ||
            id == BaseTestnet ||
            id == BaseCustom ||
            id == OPMainnet ||
            id == OPTestnet ||
            id == OPCustom ||
            id == BscMainnet ||
            id == BscTestnet ||
            id == BscCustom ||
            id == ArbMainnet ||
            id == ArbTestnet ||
            id == ArbCustom ||
            id == PolMainnet ||
            id == PolTestnet ||
            id == PolCustom ||
            id == AvaxMainnet ||
            id == AvaxTestnet ||
            id == AvaxCustom ||
            id == TronMainnet ||
            id == TronTestnet ||
            id == SolanaMainnet ||
            id == SolanaTestnet ||
            id == LTCMainnet ||
            id == LTCTestnet ||
            id == DogeMainnet ||
            id == DogeTestnet ||
            id == SuiOfficialMainnet ||
            id == SuiOfficialTestnet ||
            id == AptosMainnet ||
            id == AptosTestnet,

            EInvalidBridgeRoute
        )
    }

    public fun valid_routes(): vector<BridgeRoute> {
        vector[
            BridgeRoute { source: SuiMainnet, destination: BtcMainnet },
            BridgeRoute { source: BtcMainnet, destination: SuiMainnet },

            BridgeRoute { source: SuiTestnet, destination: BtcTestnet },
            BridgeRoute { source: BtcTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: BtcTestnet },
            BridgeRoute { source: BtcTestnet, destination: SuiCustom },

            // tron
            BridgeRoute { source: SuiMainnet, destination: TronMainnet },
            BridgeRoute { source: TronMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: TronTestnet },
            BridgeRoute { source: TronTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: TronTestnet },
            BridgeRoute { source: TronTestnet, destination: SuiCustom },

            // solana
            BridgeRoute { source: SuiMainnet, destination: SolanaMainnet },
            BridgeRoute { source: SolanaMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: SolanaTestnet },
            BridgeRoute { source: SolanaTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: SolanaTestnet },
            BridgeRoute { source: SolanaTestnet, destination: SuiCustom },

            // ltc
            BridgeRoute { source: SuiMainnet, destination: LTCMainnet },
            BridgeRoute { source: LTCMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: LTCTestnet },
            BridgeRoute { source: LTCTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: LTCTestnet },
            BridgeRoute { source: LTCTestnet, destination: SuiCustom },

            // doge
            BridgeRoute { source: SuiMainnet, destination: DogeMainnet },
            BridgeRoute { source: DogeMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: DogeTestnet },
            BridgeRoute { source: DogeTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: DogeTestnet },
            BridgeRoute { source: DogeTestnet, destination: SuiCustom },

            // sui official
            BridgeRoute { source: SuiMainnet, destination: SuiOfficialMainnet },
            BridgeRoute { source: SuiOfficialMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: SuiOfficialTestnet },
            BridgeRoute { source: SuiOfficialTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: SuiOfficialTestnet },
            BridgeRoute { source: SuiOfficialTestnet, destination: SuiCustom },

            // aptos
            BridgeRoute { source: SuiMainnet, destination: AptosMainnet },
            BridgeRoute { source: AptosMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: AptosTestnet },
            BridgeRoute { source: AptosTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: AptosTestnet },
            BridgeRoute { source: AptosTestnet, destination: SuiCustom },


            BridgeRoute { source: SuiMainnet, destination: EthMainnet },
            BridgeRoute { source: EthMainnet, destination: SuiMainnet },

            BridgeRoute { source: SuiTestnet, destination: EthSepolia },
            BridgeRoute { source: SuiTestnet, destination: EthCustom },
            BridgeRoute { source: SuiCustom, destination: EthCustom },
            BridgeRoute { source: SuiCustom, destination: EthSepolia },
            BridgeRoute { source: EthSepolia, destination: SuiTestnet },
            BridgeRoute { source: EthSepolia, destination: SuiCustom },
            BridgeRoute { source: EthCustom, destination: SuiTestnet },
            BridgeRoute { source: EthCustom, destination: SuiCustom },

            BridgeRoute { source: SuiMainnet, destination: BscMainnet },
            BridgeRoute { source: SuiMainnet, destination: OPMainnet },
            BridgeRoute { source: SuiMainnet, destination: BaseMainnet },
            BridgeRoute { source: BscMainnet, destination: SuiMainnet },
            BridgeRoute { source: OPMainnet, destination: SuiMainnet },
            BridgeRoute { source: BaseMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiMainnet, destination: ArbMainnet },
            BridgeRoute { source: SuiMainnet, destination: PolMainnet },
            BridgeRoute { source: SuiMainnet, destination: AvaxMainnet },
            BridgeRoute { source: ArbMainnet, destination: SuiMainnet },
            BridgeRoute { source: PolMainnet, destination: SuiMainnet },
            BridgeRoute { source: AvaxMainnet, destination: SuiMainnet },

            BridgeRoute { source: SuiTestnet, destination: BscTestnet },
            BridgeRoute { source: SuiTestnet, destination: BscCustom },
            BridgeRoute { source: SuiCustom, destination: BscTestnet },
            BridgeRoute { source: SuiCustom, destination: BscCustom },

            BridgeRoute { source: SuiTestnet, destination: OPTestnet },
            BridgeRoute { source: SuiTestnet, destination: OPCustom },
            BridgeRoute { source: SuiCustom, destination: OPTestnet },
            BridgeRoute { source: SuiCustom, destination: OPCustom },

            BridgeRoute { source: SuiTestnet, destination: BaseTestnet },
            BridgeRoute { source: SuiTestnet, destination: BaseCustom },
            BridgeRoute { source: SuiCustom, destination: BaseTestnet },
            BridgeRoute { source: SuiCustom, destination: BaseCustom },

            BridgeRoute { source: SuiTestnet, destination: ArbTestnet },
            BridgeRoute { source: SuiTestnet, destination: ArbCustom },
            BridgeRoute { source: SuiCustom, destination: ArbTestnet },
            BridgeRoute { source: SuiCustom, destination: ArbCustom },

            BridgeRoute { source: SuiTestnet, destination: PolTestnet },
            BridgeRoute { source: SuiTestnet, destination: PolCustom },
            BridgeRoute { source: SuiCustom, destination: PolTestnet },
            BridgeRoute { source: SuiCustom, destination: PolCustom },

            BridgeRoute { source: SuiTestnet, destination: AvaxTestnet },
            BridgeRoute { source: SuiTestnet, destination: AvaxCustom },
            BridgeRoute { source: SuiCustom, destination: AvaxTestnet },
            BridgeRoute { source: SuiCustom, destination: AvaxCustom },

            BridgeRoute { source: BscTestnet, destination: SuiTestnet },
            BridgeRoute { source: BscTestnet, destination: SuiCustom },
            BridgeRoute { source: BscCustom, destination: SuiTestnet },
            BridgeRoute { source: BscCustom, destination: SuiCustom },

            BridgeRoute { source: OPTestnet, destination: SuiTestnet },
            BridgeRoute { source: OPTestnet, destination: SuiCustom },
            BridgeRoute { source: OPCustom, destination: SuiTestnet },
            BridgeRoute { source: OPCustom, destination: SuiCustom },

            BridgeRoute { source: BaseTestnet, destination: SuiTestnet },
            BridgeRoute { source: BaseTestnet, destination: SuiCustom },
            BridgeRoute { source: BaseCustom, destination: SuiTestnet },
            BridgeRoute { source: BaseCustom, destination: SuiCustom },

            BridgeRoute { source: ArbTestnet, destination: SuiTestnet },
            BridgeRoute { source: ArbTestnet, destination: SuiCustom },
            BridgeRoute { source: ArbCustom, destination: SuiTestnet },
            BridgeRoute { source: ArbCustom, destination: SuiCustom },

            BridgeRoute { source: PolTestnet, destination: SuiTestnet },
            BridgeRoute { source: PolTestnet, destination: SuiCustom },
            BridgeRoute { source: PolCustom, destination: SuiTestnet },
            BridgeRoute { source: PolCustom, destination: SuiCustom },

            BridgeRoute { source: AvaxTestnet, destination: SuiTestnet },
            BridgeRoute { source: AvaxTestnet, destination: SuiCustom },
            BridgeRoute { source: AvaxCustom, destination: SuiTestnet },
            BridgeRoute { source: AvaxCustom, destination: SuiCustom },

        ]
    }

    public fun is_evm_l2(chain_id: u8): bool {
        // Commonly recognized EVM L2s in this file:
        // OP - Optimism
        // Arbitrum
        // Base
        // Polygon (POL)
        // bsc
        // avax
        chain_id == OPMainnet ||
        chain_id == OPTestnet ||
        chain_id == OPCustom ||
        chain_id == ArbMainnet ||
        chain_id == ArbTestnet ||
        chain_id == ArbCustom ||
        chain_id == BaseMainnet ||
        chain_id == BaseTestnet ||
        chain_id == BaseCustom ||
        chain_id == PolMainnet ||
        chain_id == PolTestnet ||
        chain_id == PolCustom ||
        chain_id == BscMainnet ||
        chain_id == BscTestnet ||
        chain_id == BscCustom ||
        chain_id == AvaxMainnet ||
        chain_id == AvaxTestnet ||
        chain_id == AvaxCustom
    }

    public fun is_eth(chain_id: u8): bool {
        chain_id == EthMainnet ||
        chain_id == EthSepolia ||
        chain_id == EthCustom
    }

    public fun is_solana(chain_id: u8): bool {
        chain_id == SolanaMainnet ||
        chain_id == SolanaTestnet
    }

    public fun is_valid_route(source: u8, destination: u8): bool {
        let route = BridgeRoute { source, destination };
        valid_routes().contains(&route)
    }

    // Checks and return BridgeRoute if the route is supported by the bridge.
    public fun get_route(source: u8, destination: u8): BridgeRoute {
        let route = BridgeRoute { source, destination };
        assert!(valid_routes().contains(&route), EInvalidBridgeRoute);
        route
    }

    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test]
    fun test_chains_ok() {
        assert_valid_chain_id(SuiMainnet);
        assert_valid_chain_id(SuiTestnet);
        assert_valid_chain_id(SuiCustom);
        assert_valid_chain_id(EthMainnet);
        assert_valid_chain_id(EthSepolia);
        assert_valid_chain_id(EthCustom);
        assert_valid_chain_id(BscMainnet);
        assert_valid_chain_id(BscTestnet);
        assert_valid_chain_id(BscCustom);
        assert_valid_chain_id(OPMainnet);
        assert_valid_chain_id(OPTestnet);
        assert_valid_chain_id(OPCustom);
        assert_valid_chain_id(BaseMainnet);
        assert_valid_chain_id(BaseTestnet);
        assert_valid_chain_id(BaseCustom);
        assert_valid_chain_id(ArbMainnet);
        assert_valid_chain_id(ArbTestnet);
        assert_valid_chain_id(ArbCustom);
        assert_valid_chain_id(PolMainnet);
        assert_valid_chain_id(PolTestnet);
        assert_valid_chain_id(PolCustom);
        assert_valid_chain_id(AvaxMainnet);
        assert_valid_chain_id(AvaxTestnet);
        assert_valid_chain_id(AvaxCustom);

        assert_valid_chain_id(TronMainnet);
        assert_valid_chain_id(TronTestnet);
        assert_valid_chain_id(SolanaMainnet);
        assert_valid_chain_id(SolanaTestnet);
        assert_valid_chain_id(LTCMainnet);
        assert_valid_chain_id(LTCTestnet);
        assert_valid_chain_id(DogeMainnet);
        assert_valid_chain_id(DogeTestnet);

        assert_valid_chain_id(SuiOfficialMainnet);
        assert_valid_chain_id(SuiOfficialTestnet);
        assert_valid_chain_id(AptosMainnet);
        assert_valid_chain_id(AptosTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_chains_error() {
        assert_valid_chain_id(100);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_sui_chains_error() {
        // this will break if we add one more sui chain id and should be corrected
        assert_valid_chain_id(4);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_eth_chains_error() {
        // this will break if we add one more eth chain id and should be corrected
        assert_valid_chain_id(13);
    }

    #[test]
    fun test_routes() {
        let valid_routes = vector[
            BridgeRoute { source: SuiMainnet, destination: EthMainnet },
            BridgeRoute { source: EthMainnet, destination: SuiMainnet },

            BridgeRoute { source: SuiTestnet, destination: EthSepolia },
            BridgeRoute { source: SuiTestnet, destination: EthCustom },
            BridgeRoute { source: SuiCustom, destination: EthCustom },
            BridgeRoute { source: SuiCustom, destination: EthSepolia },
            BridgeRoute { source: EthSepolia, destination: SuiTestnet },
            BridgeRoute { source: EthSepolia, destination: SuiCustom },
            BridgeRoute { source: EthCustom, destination: SuiTestnet },
            BridgeRoute { source: EthCustom, destination: SuiCustom },

            BridgeRoute { source: SuiMainnet, destination: BscMainnet },
            BridgeRoute { source: BscMainnet, destination: SuiMainnet },

            BridgeRoute { source: SuiTestnet, destination: BscTestnet },
            BridgeRoute { source: SuiTestnet, destination: BscCustom },
            BridgeRoute { source: SuiCustom, destination: BscTestnet },
            BridgeRoute { source: SuiCustom, destination: BscCustom },
            BridgeRoute { source: BscTestnet, destination: SuiTestnet },
            BridgeRoute { source: BscTestnet, destination: SuiCustom },
            BridgeRoute { source: BscCustom, destination: SuiTestnet },
            BridgeRoute { source: BscCustom, destination: SuiCustom },

            BridgeRoute { source: SuiMainnet, destination: OPMainnet },
            BridgeRoute { source: OPMainnet, destination: SuiMainnet },

            BridgeRoute { source: SuiTestnet, destination: OPTestnet },
            BridgeRoute { source: SuiTestnet, destination: OPCustom },
            BridgeRoute { source: SuiCustom, destination: OPTestnet },
            BridgeRoute { source: SuiCustom, destination: OPCustom },
            BridgeRoute { source: OPTestnet, destination: SuiTestnet },
            BridgeRoute { source: OPTestnet, destination: SuiCustom },
            BridgeRoute { source: OPCustom, destination: SuiTestnet },
            BridgeRoute { source: OPCustom, destination: SuiCustom },

            BridgeRoute { source: SuiMainnet, destination: BaseMainnet },
            BridgeRoute { source: BaseMainnet, destination: SuiMainnet },

            BridgeRoute { source: SuiTestnet, destination: BaseTestnet },
            BridgeRoute { source: SuiTestnet, destination: BaseCustom },
            BridgeRoute { source: SuiCustom, destination: BaseTestnet },
            BridgeRoute { source: SuiCustom, destination: BaseCustom },
            BridgeRoute { source: BaseTestnet, destination: SuiTestnet },
            BridgeRoute { source: BaseTestnet, destination: SuiCustom },
            BridgeRoute { source: BaseCustom, destination: SuiTestnet },
            BridgeRoute { source: BaseCustom, destination: SuiCustom },

            BridgeRoute { source: SuiMainnet, destination: ArbMainnet },
            BridgeRoute { source: ArbMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: ArbTestnet },
            BridgeRoute { source: SuiTestnet, destination: ArbCustom },
            BridgeRoute { source: SuiCustom, destination: ArbTestnet },
            BridgeRoute { source: SuiCustom, destination: ArbCustom },
            BridgeRoute { source: ArbTestnet, destination: SuiTestnet },
            BridgeRoute { source: ArbTestnet, destination: SuiCustom },
            BridgeRoute { source: ArbCustom, destination: SuiTestnet },

            BridgeRoute { source: PolMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiMainnet, destination: PolMainnet },
            BridgeRoute { source: PolTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiTestnet, destination: PolTestnet },
            BridgeRoute { source: PolCustom, destination: SuiTestnet },
            BridgeRoute { source: SuiTestnet, destination: PolCustom },
            BridgeRoute { source: SuiCustom, destination: PolTestnet },
            BridgeRoute { source: SuiCustom, destination: PolCustom },

            // SuiOfficial
            BridgeRoute { source: SuiMainnet, destination: SuiOfficialMainnet },
            BridgeRoute { source: SuiOfficialMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: SuiOfficialTestnet },
            BridgeRoute { source: SuiOfficialTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: SuiOfficialTestnet },
            BridgeRoute { source: SuiOfficialTestnet, destination: SuiCustom },

            // aptos
            BridgeRoute { source: SuiMainnet, destination: AptosMainnet },
            BridgeRoute { source: AptosMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: AptosTestnet },
            BridgeRoute { source: AptosTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: AptosTestnet },
            BridgeRoute { source: AptosTestnet, destination: SuiCustom },

            BridgeRoute { source: SuiMainnet, destination: TronMainnet },
            BridgeRoute { source: TronMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: TronTestnet },
            BridgeRoute { source: TronTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: TronTestnet },
            BridgeRoute { source: TronTestnet, destination: SuiCustom },

            BridgeRoute { source: SuiMainnet, destination: SolanaMainnet },
            BridgeRoute { source: SolanaMainnet, destination: SuiMainnet },
            BridgeRoute { source: SuiTestnet, destination: SolanaTestnet },
            BridgeRoute { source: SolanaTestnet, destination: SuiTestnet },
            BridgeRoute { source: SuiCustom, destination: SolanaTestnet },
            BridgeRoute { source: SolanaTestnet, destination: SuiCustom },
        ];
        let mut size = valid_routes.length();
        while (size > 0) {
            size = size - 1;
            let route = valid_routes[size];
            assert!(is_valid_route(route.source, route.destination)); // sould not assert
        }
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_1() {
        get_route(SuiMainnet, SuiMainnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_2() {
        get_route(SuiMainnet, SuiTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_3() {
        get_route(SuiMainnet, EthSepolia);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_4() {
        get_route(SuiMainnet, EthCustom);
    }


    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_5() {
        get_route(SuiMainnet, BscTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_6() {
        get_route(SuiMainnet, BscCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_7() {
        get_route(SuiMainnet, OPTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_8() {
        get_route(SuiMainnet, OPCustom);
    }

     #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_9() {
        get_route(SuiMainnet, BaseTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_10() {
        get_route(SuiMainnet, BaseCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_1() {
        get_route(EthMainnet, EthMainnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_2() {
        get_route(EthMainnet, EthCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_3() {
        get_route(EthMainnet, SuiCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_4() {
        get_route(EthMainnet, SuiTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_bsc_1() {
        get_route(BscMainnet, BscMainnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_bsc_2() {
        get_route(BscMainnet, BscCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_bsc_3() {
        get_route(BscMainnet, SuiCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_bsc_4() {
        get_route(BscMainnet, SuiTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_op_1() {
        get_route(OPMainnet, OPMainnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_op_2() {
        get_route(OPMainnet, OPCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_op_3() {
        get_route(OPMainnet, SuiCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_op_4() {
        get_route(OPMainnet, SuiTestnet);
    }


    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_base_1() {
        get_route(BaseMainnet, BaseMainnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_base_2() {
        get_route(BaseMainnet, BaseCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_base_3() {
        get_route(BaseMainnet, SuiCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_base_4() {
        get_route(BaseMainnet, SuiTestnet);
    }
}
