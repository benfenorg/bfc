// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::chain_ids;

// Chain IDs
const SUI_MAINNET: u8 = 0;
const SUI_TESTNET: u8 = 1;
const SUI_CUSTOM: u8 = 2;

const ETH_MAINNET: u8 = 10;
const ETH_SEPOLIA: u8 = 11;
const ETH_CUSTOM: u8 = 12;

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

public fun sui_mainnet(): u8 { SUI_MAINNET }

public fun sui_testnet(): u8 { SUI_TESTNET }

public fun sui_custom(): u8 { SUI_CUSTOM }

public fun eth_mainnet(): u8 { ETH_MAINNET }

public fun eth_sepolia(): u8 { ETH_SEPOLIA }

public fun eth_custom(): u8 { ETH_CUSTOM }

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
            id == SUI_MAINNET ||
            id == SUI_TESTNET ||
            id == SUI_CUSTOM ||
            id == ETH_MAINNET ||
            id == ETH_SEPOLIA ||
            id == ETH_CUSTOM ||
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
            BridgeRoute { source: SUI_MAINNET, destination: BtcMainnet },
            BridgeRoute { source: BtcMainnet, destination: SUI_MAINNET },

            BridgeRoute { source: SUI_TESTNET, destination: BtcTestnet },
            BridgeRoute { source: BtcTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: BtcTestnet },
            BridgeRoute { source: BtcTestnet, destination: SUI_CUSTOM },

            // tron
            BridgeRoute { source: SUI_MAINNET, destination: TronMainnet },
            BridgeRoute { source: TronMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: TronTestnet },
            BridgeRoute { source: TronTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: TronTestnet },
            BridgeRoute { source: TronTestnet, destination: SUI_CUSTOM },

            // solana
            BridgeRoute { source: SUI_MAINNET, destination: SolanaMainnet },
            BridgeRoute { source: SolanaMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: SolanaTestnet },
            BridgeRoute { source: SolanaTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: SolanaTestnet },
            BridgeRoute { source: SolanaTestnet, destination: SUI_CUSTOM },

            // ltc
            BridgeRoute { source: SUI_MAINNET, destination: LTCMainnet },
            BridgeRoute { source: LTCMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: LTCTestnet },
            BridgeRoute { source: LTCTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: LTCTestnet },
            BridgeRoute { source: LTCTestnet, destination: SUI_CUSTOM },

            // doge
            BridgeRoute { source: SUI_MAINNET, destination: DogeMainnet },
            BridgeRoute { source: DogeMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: DogeTestnet },
            BridgeRoute { source: DogeTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: DogeTestnet },
            BridgeRoute { source: DogeTestnet, destination: SUI_CUSTOM },

            // sui official
            BridgeRoute { source: SUI_MAINNET, destination: SuiOfficialMainnet },
            BridgeRoute { source: SuiOfficialMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: SuiOfficialTestnet },
            BridgeRoute { source: SuiOfficialTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: SuiOfficialTestnet },
            BridgeRoute { source: SuiOfficialTestnet, destination: SUI_CUSTOM },

            // aptos
            BridgeRoute { source: SUI_MAINNET, destination: AptosMainnet },
            BridgeRoute { source: AptosMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: AptosTestnet },
            BridgeRoute { source: AptosTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: AptosTestnet },
            BridgeRoute { source: AptosTestnet, destination: SUI_CUSTOM },


            BridgeRoute { source: SUI_MAINNET, destination: ETH_MAINNET },
            BridgeRoute { source: ETH_MAINNET, destination: SUI_MAINNET },

            BridgeRoute { source: SUI_TESTNET, destination: ETH_SEPOLIA },
            BridgeRoute { source: SUI_TESTNET, destination: ETH_CUSTOM },
            BridgeRoute { source: SUI_CUSTOM, destination: ETH_CUSTOM },
            BridgeRoute { source: SUI_CUSTOM, destination: ETH_SEPOLIA },
            BridgeRoute { source: ETH_SEPOLIA, destination: SUI_TESTNET },
            BridgeRoute { source: ETH_SEPOLIA, destination: SUI_CUSTOM },
            BridgeRoute { source: ETH_CUSTOM, destination: SUI_TESTNET },
            BridgeRoute { source: ETH_CUSTOM, destination: SUI_CUSTOM },

            BridgeRoute { source: SUI_MAINNET, destination: BscMainnet },
            BridgeRoute { source: SUI_MAINNET, destination: OPMainnet },
            BridgeRoute { source: SUI_MAINNET, destination: BaseMainnet },
            BridgeRoute { source: BscMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: OPMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: BaseMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_MAINNET, destination: ArbMainnet },
            BridgeRoute { source: SUI_MAINNET, destination: PolMainnet },
            BridgeRoute { source: SUI_MAINNET, destination: AvaxMainnet },
            BridgeRoute { source: ArbMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: PolMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: AvaxMainnet, destination: SUI_MAINNET },

            BridgeRoute { source: SUI_TESTNET, destination: BscTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: BscCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: BscTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: BscCustom },

            BridgeRoute { source: SUI_TESTNET, destination: OPTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: OPCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: OPTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: OPCustom },

            BridgeRoute { source: SUI_TESTNET, destination: BaseTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: BaseCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: BaseTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: BaseCustom },

            BridgeRoute { source: SUI_TESTNET, destination: ArbTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: ArbCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: ArbTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: ArbCustom },

            BridgeRoute { source: SUI_TESTNET, destination: PolTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: PolCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: PolTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: PolCustom },

            BridgeRoute { source: SUI_TESTNET, destination: AvaxTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: AvaxCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: AvaxTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: AvaxCustom },

            BridgeRoute { source: BscTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: BscTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: BscCustom, destination: SUI_TESTNET },
            BridgeRoute { source: BscCustom, destination: SUI_CUSTOM },

            BridgeRoute { source: OPTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: OPTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: OPCustom, destination: SUI_TESTNET },
            BridgeRoute { source: OPCustom, destination: SUI_CUSTOM },

            BridgeRoute { source: BaseTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: BaseTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: BaseCustom, destination: SUI_TESTNET },
            BridgeRoute { source: BaseCustom, destination: SUI_CUSTOM },

            BridgeRoute { source: ArbTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: ArbTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: ArbCustom, destination: SUI_TESTNET },
            BridgeRoute { source: ArbCustom, destination: SUI_CUSTOM },

            BridgeRoute { source: PolTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: PolTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: PolCustom, destination: SUI_TESTNET },
            BridgeRoute { source: PolCustom, destination: SUI_CUSTOM },

            BridgeRoute { source: AvaxTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: AvaxTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: AvaxCustom, destination: SUI_TESTNET },
            BridgeRoute { source: AvaxCustom, destination: SUI_CUSTOM },

        ]
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
        assert_valid_chain_id(SUI_MAINNET);
        assert_valid_chain_id(SUI_TESTNET);
        assert_valid_chain_id(SUI_CUSTOM);
        assert_valid_chain_id(ETH_MAINNET);
        assert_valid_chain_id(ETH_SEPOLIA);
        assert_valid_chain_id(ETH_CUSTOM);
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
fun test_chains_ok() {
    assert_valid_chain_id(SUI_MAINNET);
    assert_valid_chain_id(SUI_TESTNET);
    assert_valid_chain_id(SUI_CUSTOM);
    assert_valid_chain_id(ETH_MAINNET);
    assert_valid_chain_id(ETH_SEPOLIA);
    assert_valid_chain_id(ETH_CUSTOM);
}

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_chains_error() {
        assert_valid_chain_id(100);
    }

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_sui_chains_error() {
        // this will break if we add one more sui chain id and should be corrected
        assert_valid_chain_id(4);
    }

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_eth_chains_error() {
        // this will break if we add one more eth chain id and should be corrected
        assert_valid_chain_id(13);
    }

    #[test]
    fun test_routes() {
        let valid_routes = vector[
            BridgeRoute { source: SUI_MAINNET, destination: ETH_MAINNET },
            BridgeRoute { source: ETH_MAINNET, destination: SUI_MAINNET },

            BridgeRoute { source: SUI_TESTNET, destination: ETH_SEPOLIA },
            BridgeRoute { source: SUI_TESTNET, destination: ETH_CUSTOM },
            BridgeRoute { source: SUI_CUSTOM, destination: ETH_CUSTOM },
            BridgeRoute { source: SUI_CUSTOM, destination: ETH_SEPOLIA },
            BridgeRoute { source: ETH_SEPOLIA, destination: SUI_TESTNET },
            BridgeRoute { source: ETH_SEPOLIA, destination: SUI_CUSTOM },
            BridgeRoute { source: ETH_CUSTOM, destination: SUI_TESTNET },
            BridgeRoute { source: ETH_CUSTOM, destination: SUI_CUSTOM },

            BridgeRoute { source: SUI_MAINNET, destination: BscMainnet },
            BridgeRoute { source: BscMainnet, destination: SUI_MAINNET },

            BridgeRoute { source: SUI_TESTNET, destination: BscTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: BscCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: BscTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: BscCustom },
            BridgeRoute { source: BscTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: BscTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: BscCustom, destination: SUI_TESTNET },
            BridgeRoute { source: BscCustom, destination: SUI_CUSTOM },

            BridgeRoute { source: SUI_MAINNET, destination: OPMainnet },
            BridgeRoute { source: OPMainnet, destination: SUI_MAINNET },

            BridgeRoute { source: SUI_TESTNET, destination: OPTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: OPCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: OPTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: OPCustom },
            BridgeRoute { source: OPTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: OPTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: OPCustom, destination: SUI_TESTNET },
            BridgeRoute { source: OPCustom, destination: SUI_CUSTOM },

            BridgeRoute { source: SUI_MAINNET, destination: BaseMainnet },
            BridgeRoute { source: BaseMainnet, destination: SUI_MAINNET },

            BridgeRoute { source: SUI_TESTNET, destination: BaseTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: BaseCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: BaseTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: BaseCustom },
            BridgeRoute { source: BaseTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: BaseTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: BaseCustom, destination: SUI_TESTNET },
            BridgeRoute { source: BaseCustom, destination: SUI_CUSTOM },

            BridgeRoute { source: SUI_MAINNET, destination: ArbMainnet },
            BridgeRoute { source: ArbMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: ArbTestnet },
            BridgeRoute { source: SUI_TESTNET, destination: ArbCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: ArbTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: ArbCustom },
            BridgeRoute { source: ArbTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: ArbTestnet, destination: SUI_CUSTOM },
            BridgeRoute { source: ArbCustom, destination: SUI_TESTNET },

            BridgeRoute { source: PolMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_MAINNET, destination: PolMainnet },
            BridgeRoute { source: PolTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_TESTNET, destination: PolTestnet },
            BridgeRoute { source: PolCustom, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_TESTNET, destination: PolCustom },
            BridgeRoute { source: SUI_CUSTOM, destination: PolTestnet },
            BridgeRoute { source: SUI_CUSTOM, destination: PolCustom },

            // SuiOfficial
            BridgeRoute { source: SUI_MAINNET, destination: SuiOfficialMainnet },
            BridgeRoute { source: SuiOfficialMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: SuiOfficialTestnet },
            BridgeRoute { source: SuiOfficialTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: SuiOfficialTestnet },
            BridgeRoute { source: SuiOfficialTestnet, destination: SUI_CUSTOM },

            // aptos
            BridgeRoute { source: SUI_MAINNET, destination: AptosMainnet },
            BridgeRoute { source: AptosMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: AptosTestnet },
            BridgeRoute { source: AptosTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: AptosTestnet },
            BridgeRoute { source: AptosTestnet, destination: SUI_CUSTOM },

            BridgeRoute { source: SUI_MAINNET, destination: TronMainnet },
            BridgeRoute { source: TronMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: TronTestnet },
            BridgeRoute { source: TronTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: TronTestnet },
            BridgeRoute { source: TronTestnet, destination: SUI_CUSTOM },

            BridgeRoute { source: SUI_MAINNET, destination: SolanaMainnet },
            BridgeRoute { source: SolanaMainnet, destination: SUI_MAINNET },
            BridgeRoute { source: SUI_TESTNET, destination: SolanaTestnet },
            BridgeRoute { source: SolanaTestnet, destination: SUI_TESTNET },
            BridgeRoute { source: SUI_CUSTOM, destination: SolanaTestnet },
            BridgeRoute { source: SolanaTestnet, destination: SUI_CUSTOM },
        ];
        let mut size = valid_routes.length();
        while (size > 0) {
            size = size - 1;
            let route = valid_routes[size];
            assert!(is_valid_route(route.source, route.destination)); // sould not assert
        }
    }
#[test]
    fun test_routes() {
    let valid_routes = vector[
        BridgeRoute { source: SUI_MAINNET, destination: ETH_MAINNET },
        BridgeRoute { source: ETH_MAINNET, destination: SUI_MAINNET },
        BridgeRoute { source: SUI_TESTNET, destination: ETH_SEPOLIA },
        BridgeRoute { source: SUI_TESTNET, destination: ETH_CUSTOM },
        BridgeRoute { source: SUI_CUSTOM, destination: ETH_CUSTOM },
        BridgeRoute { source: SUI_CUSTOM, destination: ETH_SEPOLIA },
        BridgeRoute { source: ETH_SEPOLIA, destination: SUI_TESTNET },
        BridgeRoute { source: ETH_SEPOLIA, destination: SUI_CUSTOM },
        BridgeRoute { source: ETH_CUSTOM, destination: SUI_TESTNET },
        BridgeRoute { source: ETH_CUSTOM, destination: SUI_CUSTOM },
    ];
    let mut size = valid_routes.length();
    while (size > 0) {
        size = size - 1;
        let route = valid_routes[size];
        assert!(is_valid_route(route.source, route.destination)); // sould not assert
    }
}

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_1() {
        get_route(SUI_MAINNET, SUI_MAINNET);
    }

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_2() {
        get_route(SUI_MAINNET, SUI_TESTNET);
    }

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_3() {
        get_route(SUI_MAINNET, ETH_SEPOLIA);
    }

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_4() {
        get_route(SUI_MAINNET, ETH_CUSTOM);
    }


    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_5() {
        get_route(SUI_MAINNET, BscTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_6() {
        get_route(SUI_MAINNET, BscCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_7() {
        get_route(SUI_MAINNET, OPTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_8() {
        get_route(SUI_MAINNET, OPCustom);
    }

     #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_9() {
        get_route(SUI_MAINNET, BaseTestnet);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_sui_10() {
        get_route(SUI_MAINNET, BaseCustom);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_1() {
        get_route(ETH_MAINNET, ETH_MAINNET);
    }
    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_1() {
        get_route(ETH_MAINNET, ETH_MAINNET);
    }

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_2() {
        get_route(ETH_MAINNET, ETH_CUSTOM);
    }

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_3() {
        get_route(ETH_MAINNET, SUI_CUSTOM);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_4() {
        get_route(ETH_MAINNET, SUI_TESTNET);
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
        get_route(BscMainnet, SUI_CUSTOM);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_bsc_4() {
        get_route(BscMainnet, SUI_TESTNET);
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
        get_route(OPMainnet, SUI_CUSTOM);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_op_4() {
        get_route(OPMainnet, SUI_TESTNET);
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
        get_route(BaseMainnet, SUI_CUSTOM);
    }

    #[test]
    #[expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_base_4() {
        get_route(BaseMainnet, SUI_TESTNET);
    }

    #[test, expected_failure(abort_code = EInvalidBridgeRoute)]
    fun test_routes_err_eth_4() {
        get_route(ETH_MAINNET, SUI_TESTNET);
    }
}