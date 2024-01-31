module poly::nb {
    use std::string;

    use sui::coin::{Self, Coin};
    use sui::transfer::transfer;

    const ENOT_ADMIN: u64 = 4001;

    struct NBCoin has key {}



    public entry fun initialize(admin: address) {
        only_admin(admin);

        let (burn_cap, freeze_cap, mint_cap) = coin::initialize<NBCoin>(
            admin,
            string::utf8(b"NobelBoss Coin"),
            string::utf8(b"NB"),
            6, /* decimals */
            true, /* monitor_supply */
        );



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
    )  {
        only_admin(admin);

        //let mint_cap = &borrow_global<NBCapStore>((admin)).mint_cap;
        let coins_minted = coin::mint<NBCoin>(amount);
        //coin::deposit<NBCoin>(dst_addr, coins_minted);
        transfer(coins_minted, dst_addr);
    }

    public entry fun burn(
        admin: address,
        amount: Coin<NBCoin>,
    )  {
        only_admin(admin);

        let admin_addr = (admin);
        //let burn_cap = &borrow_global<NBCapStore>(admin_addr).burn_cap;
        //coin::burn_from<NBCoin>(admin_addr, amount, burn_cap);
        coin::burn( ,amount);
    }

    public entry fun freeze_coin_store(
        admin: address,
        freeze_addr: address,
    )  {
        only_admin(admin);

        //let freeze_cap = &borrow_global<NBCapStore>((admin)).freeze_cap;
        coin::freeze_coin_store<NBCoin>(freeze_addr, freeze_cap);
    }

    public entry fun unfreeze_coin_store(
        admin: address,
        unfreeze_addr: address,
    )  {
        only_admin(admin);

        //let freeze_cap = &borrow_global<NBCapStore>((admin)).freeze_cap;
        coin::unfreeze_coin_store<NBCoin>(unfreeze_addr, freeze_cap);
    }

    fun only_admin(account: address) {
        assert!((account) == @poly, ENOT_ADMIN);
    }
}