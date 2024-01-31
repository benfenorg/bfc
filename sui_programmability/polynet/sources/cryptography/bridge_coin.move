module poly_bridge::bridge_coin {
    use std::string::{String};

    use sui::coin;

    use poly_bridge::lock_proxy;

    const ENOT_BRIDGE_ADMIN: u64 = 4001;

    const HUGE_U64: u64 = 10000000000000000000;

    public entry fun initialize<BridgeCoinType>(
        admin: address,
        name: String,
        symbol: String,
        decimals: u8,
    ) {
        only_admin(admin);


        //create_currency
        let (burn_cap, freeze_cap, mint_cap) = coin::initialize<BridgeCoinType>(
            admin,
            name,
            symbol,
            decimals,
            true, /* monitor_supply */
        );

        let initial_lock = coin::mint<BridgeCoinType>(HUGE_U64, &mint_cap);
        lock_proxy::initTreasury<BridgeCoinType>(admin);
        lock_proxy::deposit<BridgeCoinType>(initial_lock);

        //coin::destroy_burn_cap(burn_cap);
        //coin::destroy_freeze_cap(freeze_cap);
        //coin::destroy_mint_cap(mint_cap);
    }

    fun only_admin(account: address) {
        assert!(lock_proxy::is_admin(account), ENOT_BRIDGE_ADMIN);
    }
}