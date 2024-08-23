#[allow(unused_mut_parameter)]
module bfc_system::treasury_pool {
    use sui::balance;
    use sui::balance::Balance;
    use sui::bfc::BFC;
    use sui::coin;
    use sui::coin::Coin;
    use sui::event::emit;

    use bfc_system::event;

    //friend bfc_system::bfc_system_state_inner;

    /// The `withdraw` function only called by 0x0 address.
    const ERR_NOT_ZERO_ADDRESS: u64 = 900;

    public struct TreasuryPool has key, store {
        id: UID,
        balance: Balance<BFC>,
    }

    public struct WithdrawEvent has copy, drop {
        // current balance
        balance: u64,
        // request withdraw amount
        request_amount: u64,
        // withdraw amount
        amount: u64,
    }

    public struct DepositEvent has copy, drop {
        balance: u64,
        deposit_amount: u64
    }

    public(package) fun create_treasury_pool(
        balance: Balance<BFC>,
        ctx: &mut TxContext
    ): TreasuryPool
    {
        let treasury_pool = TreasuryPool {
            id: object::new(ctx),
            balance: balance,
        };
        let treasury_pool_id = object::id(&treasury_pool);
        event::init_treasury_pool(treasury_pool_id);
        treasury_pool
    }

    public(package) fun deposit_to_treasury_pool(
        self: &mut TreasuryPool,
        bfc_coin: Coin<BFC>
    )
    {
        let origin_amount = balance::value(&self.balance);
        let deposit_amount = coin::value(&bfc_coin);
        balance::join(&mut self.balance, coin::into_balance(bfc_coin));
        emit(DepositEvent {
            balance: origin_amount,
            deposit_amount
        });
    }

    public(package) fun withdraw_to_treasury(
        self: &mut TreasuryPool,
        amount: u64,
        ctx: &mut TxContext
    ): Balance<BFC>
    {
        assert!(tx_context::sender(ctx) == @0x0, ERR_NOT_ZERO_ADDRESS);
        // Take the minimum of the amount and the remaining balance in
        // order to ensure we don't overdraft the remaining balance
        let current_balance = balance::value(&self.balance);
        let to_withdraw = std::u64::min(amount, current_balance);

        let withdraw_balance = balance::split(&mut self.balance, to_withdraw);
        emit(WithdrawEvent {
            balance: current_balance,
            request_amount: amount,
            amount: to_withdraw,
        });
        withdraw_balance
    }

    public fun get_balance(self: &TreasuryPool): u64 {
        balance::value(&self.balance)
    }
}