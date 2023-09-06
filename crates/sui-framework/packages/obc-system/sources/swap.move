module obc_system::swap {
    use sui::balance::{Self, Balance};
    use sui::coin::{Self, Coin};
    use sui::obc::OBC;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    use obc_system::vault;
    use obc_system::treasury::{Self, Treasury};
    use obc_system::utils;

    /// Errors
    const ERR_ZERO_AMOUNT: u64 = 0;

    /// Constants
    const TickSpacing: u32 = 60;

    /// Mint swap obc to stablecoin
    public entry fun mint<StableCoinType>(
        treasury: &mut Treasury,
        coin_obc: Coin<OBC>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        assert!(coin::value<OBC>(&coin_obc) > 0, ERR_ZERO_AMOUNT);
        if (utils::cmp<OBC, StableCoinType>() < 1) {
            swap_internal<OBC, StableCoinType>(
                treasury,
                true,
                coin_obc,
                coin::zero<StableCoinType>(ctx),
                amount,
                ctx,
            );
        } else {
            swap_internal<StableCoinType, OBC>(
                treasury,
                false,
                coin::zero<StableCoinType>(ctx),
                coin_obc,
                amount,
                ctx,
            );
        };
    }

    /// Burn swap stablecoin to obc
    public entry fun burn<StableCoinType>(
        treasury: &mut Treasury,
        coin_sc: Coin<StableCoinType>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        assert!(coin::value<StableCoinType>(&coin_sc) > 0, ERR_ZERO_AMOUNT);
        if (utils::cmp<OBC, StableCoinType>() < 1) {
            swap_internal<OBC, StableCoinType>(
                treasury,
                false,
                coin::zero<OBC>(ctx),
                coin_sc,
                amount,
                ctx,
            );
        } else {
            swap_internal<StableCoinType, OBC>(
                treasury,
                true,
                coin_sc,
                coin::zero<OBC>(ctx),
                amount,
                ctx,
            );
        };
    }

    fun transfer_or_delete<CoinType>(
        balance: Balance<CoinType>,
        ctx: &mut TxContext
    ) {
        if (balance::value(&balance) > 0) {
            transfer::public_transfer(coin::from_balance(balance, ctx), tx_context::sender(ctx));
        } else {
            balance::destroy_zero(balance);
        }
    }

    /// Internal swap
    fun swap_internal<CoinTypeA, CoinTypeB>(
        treasury: &mut Treasury,
        a2b: bool, // true a->b , false b->a
        coin_a: Coin<CoinTypeA>,
        coin_b: Coin<CoinTypeB>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        let vault_key = treasury::generate_vault_key<CoinTypeA, CoinTypeB>(TickSpacing);
        let mut_vault = treasury::borrow_mut_vault<CoinTypeA, CoinTypeB>(treasury, vault_key);
        let current_sqrt_price = vault::vault_current_sqrt_price(mut_vault);
        let (balance_a, balance_b) = vault::swap<CoinTypeA, CoinTypeB>(
            mut_vault,
            coin_a,
            coin_b,
            a2b,
            true,
            amount,
            0, // ? unuse
            current_sqrt_price,
            ctx
        );
        transfer_or_delete(balance_a, ctx);
        transfer_or_delete(balance_b, ctx);
    }
}
