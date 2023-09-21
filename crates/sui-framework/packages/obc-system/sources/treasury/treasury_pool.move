module obc_system::treasury_pool {
    use sui::balance;
    use sui::balance::Balance;
    use sui::math;
    use sui::obc::OBC;
    use sui::object;
    use sui::object::UID;
    use sui::tx_context::{TxContext, Self};

    use obc_system::event;

    friend obc_system::obc_system_state_inner;

    /// The `withdraw` function only called by 0x0 address.
    const ERR_NOT_ZERO_ADDRESS: u64 = 900;

    struct TreasuryPool has key, store {
        id: UID,
        balance: Balance<OBC>
    }

    public(friend) fun create_treasury_pool(
        balance: Balance<OBC>,
        ctx: &mut TxContext
    ): TreasuryPool
    {
        let treasury_pool = TreasuryPool {
            id: object::new(ctx),
            balance
        };
        let treasury_pool_id = object::id(&treasury_pool);
        event::init_treasury_pool(treasury_pool_id);
        treasury_pool
    }

    public(friend) fun withdraw_to_treasury(
        self: &mut TreasuryPool,
        amount: u64,
        ctx: &mut TxContext
    ): Balance<OBC>
    {
        assert!(tx_context::sender(ctx) == @0x0, ERR_NOT_ZERO_ADDRESS);
        // Take the minimum of the amount and the remaining balance in
        // order to ensure we don't overdraft the remaining balance
        let to_withdraw = math::min(amount, balance::value(&self.balance));
        let withdraw_balance = balance::split(&mut self.balance, to_withdraw);

        withdraw_balance
    }
}