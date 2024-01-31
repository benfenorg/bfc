module poly::nb {
    use std::string;

    use sui::coin::{Self};
    use sui::transfer::transfer;

    const ENOT_ADMIN: u64 = 1;

    struct NBCoin has key {}

    struct NBCapStore has key {
        //burn_cap: BurnCapability<NBCoin>,
        //freeze_cap: FreezeCapability<NBCoin>,
        //mint_cap: MintCapability<NBCoin>,
    }

    public entry fun initialize(admin: address) {
        only_admin(admin);

        let (burn_cap, freeze_cap, mint_cap) = coin::initialize<NBCoin>(
            admin,
            string::utf8(b"NobelBoss Coin"),
            string::utf8(b"NB"),
            6, /* decimals */
            true, /* monitor_supply */
        );

        transfer(NBCapStore { }, admin);

        //coin::destroy_burn_cap(burn_cap);
        //coin::destroy_freeze_cap(freeze_cap);
        //coin::destroy_mint_cap(mint_cap);
    }

    public entry fun register(account: address) {
        //coin::register<NBCoin>(account);
    }

    public entry fun mint(
        admin: address,
        dst_addr: address,
        amount: u64,
    ) acquires NBCapStore {
        only_admin(admin);

        //let mint_cap = &borrow_global<NBCapStore>((admin)).mint_cap;
        let coins_minted = coin::mint<NBCoin>(amount);
        coin::deposit<NBCoin>(dst_addr, coins_minted);
    }

    public entry fun burn(
        admin: address,
        amount: u64,
    ) acquires NBCapStore {
        only_admin(admin);

        let admin_addr = (admin);
        //let burn_cap = &borrow_global<NBCapStore>(admin_addr).burn_cap;
        coin::burn_from<NBCoin>(admin_addr, amount, burn_cap);
        coin::burn(amount, );
    }

    public entry fun freeze_coin_store(
        admin: address,
        freeze_addr: address,
    ) acquires NBCapStore {
        only_admin(admin);

        //let freeze_cap = &borrow_global<NBCapStore>((admin)).freeze_cap;
        coin::freeze_coin_store<NBCoin>(freeze_addr, freeze_cap);
    }

    public entry fun unfreeze_coin_store(
        admin: address,
        unfreeze_addr: address,
    ) acquires NBCapStore {
        only_admin(admin);

        //let freeze_cap = &borrow_global<NBCapStore>((admin)).freeze_cap;
        coin::unfreeze_coin_store<NBCoin>(unfreeze_addr, freeze_cap);
    }

    fun only_admin(account: address) {
        assert!((account) == @poly, ENOT_ADMIN);
    }
}