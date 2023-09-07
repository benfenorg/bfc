
module hello_world::voting_pool {
    use sui::balance::{Self, Balance};
    use sui::obc::OBC;
    use sui::tx_context::{Self, TxContext};
    use sui::transfer;
    use sui::object::{Self, ID, UID};
    //use sui::coin;
    //use sui::coin::Coin;
    //use hello_world::obc_dao_manager::OBCDaoManageKey;
    //use hello_world::obc_dao_manager;
    //use sui::event;
    //use hello_world::obc_dao::send_obc_dao_event;


    friend hello_world::obc_dao;
    /// votingObc objects cannot be split to below this amount.
    const MIN_STAKING_THRESHOLD: u64 = 1_000_000_000; // 1 obc

    const EInsufficientPoolTokenBalance: u64 = 0;
    const EWrongPool: u64 = 1;
    const EWithdrawAmountCannotBeZero: u64 = 2;
    const EInsufficientObcTokenBalance: u64 = 3;
    const ETokenBalancesDoNotMatchExchangeRate: u64 = 9;
    const EIncompatibleVotingObc: u64 = 12;
    const EDelegationOfZeroObc: u64 = 17;
    const EVotingObcBelowThreshold: u64 = 18;

    /// A staking pool embedded in each validator struct in the system state object.
    struct VotingPool has key, store {
        id: UID,
        /// The total number of OBC tokens in this pool,
        obc_balance: u64,
        /// Total number of pool tokens issued by the pool.
        pool_token_balance: u64,

    }

    /// Struct representing the exchange rate of the voting pool token to OBC.
    struct PoolTokenExchangeRate has store, copy, drop {
        obc_amount: u64,
        pool_token_amount: u64,
    }

    /// A self-custodial object holding the Voting Obc tokens.
    struct VotingObc has key, store {
        id: UID,
        /// ID of the staking pool we are staking with.
        pool_id: ID,
        /// The voting OBC tokens.
        principal: Balance<OBC>,
    }

    // ==== initializer ====

    /// Create a new, empty voting pool.
    public(friend) fun new(ctx: &mut TxContext) : VotingPool {
        VotingPool {
            id: object::new(ctx),
            obc_balance: 0,
            pool_token_balance: 0,
        }
    }

    // ==== voting requests ====

    /// Request to voting to a staking pool. The voting starts counting at the beginning of the next epoch,
    public(friend) fun request_add_voting(
        pool: &mut VotingPool,
        voting: Balance<OBC>,
        ctx: &mut TxContext
    ) : VotingObc {
        let obc_amount = balance::value(&voting);
        assert!(obc_amount > 0, EDelegationOfZeroObc);
        let votingobc = VotingObc {
            id: object::new(ctx),
            pool_id: object::id(pool),
            principal: voting,
        };
        votingobc
    }

    /// Request to withdraw the given voting plus rewards from a staking pool.
    /// Both the principal and corresponding rewards in OBC are withdrawn.
    /// A proportional amount of pool token withdraw is recorded and processed at epoch change time.
    public(friend) fun request_withdraw_voting(
        pool: &mut VotingPool,
        voting_obc: VotingObc,
    ) : Balance<OBC> {
        let (_, principal_withdraw) =
            withdraw_from_principal(pool, voting_obc);
        let principal_withdraw_amount = balance::value(&principal_withdraw);


        let _ = principal_withdraw_amount;

        // TODO: implement withdraw bonding period here.
        principal_withdraw
    }

    /// Withdraw the principal OBC stored in the votingdObc object, and calculate the corresponding amount of pool
    /// tokens using exchange rate at staking epoch.
    /// Returns values are amount of pool tokens withdrawn and withdrawn principal portion of OBC.
    public(friend) fun withdraw_from_principal(
        pool: &mut VotingPool,
        voting_obc: VotingObc,
    ) : (u64, Balance<OBC>) {

        // Check that the voting information matches the pool.
        assert!(voting_obc.pool_id == object::id(pool), EWrongPool);

        let exchange_rate_at_staking_epoch = pool_token_exchange_rate_at_epoch();
        let principal_withdraw = unwrap_voting_obc(voting_obc);
        let pool_token_withdraw_amount = get_token_amount(&exchange_rate_at_staking_epoch, balance::value(&principal_withdraw));

        (
            pool_token_withdraw_amount,
            principal_withdraw,
        )
    }
    public fun unwrap_voting_obc(voting_obc: VotingObc): Balance<OBC> {
        let VotingObc {
            id,
            pool_id: _,
            principal,
        } = voting_obc;
        object::delete(id);
        principal
    }

    // ==== getters and misc utility functions ====

    public fun obc_balance(pool: &VotingPool): u64 { pool.obc_balance }

    public fun pool_id(voting_obc: &VotingObc): ID { voting_obc.pool_id }

    public fun voting_obc_amount(voting_obc: &VotingObc): u64 { balance::value(&voting_obc.principal) }




    /// Split votingObc `self` to two parts, one with principal `split_amount`,
    /// and the remaining principal is left in `self`.
    /// All the other parameters of the votingObc like `voting` or `pool_id` remain the same.
    public fun split(self: &mut VotingObc, split_amount: u64, ctx: &mut TxContext): VotingObc {
        let original_amount = balance::value(&self.principal);
        assert!(split_amount <= original_amount, EInsufficientObcTokenBalance);
        let remaining_amount = original_amount - split_amount;
        // Both resulting parts should have at least MIN_STAKING_THRESHOLD.
        assert!(remaining_amount >= MIN_STAKING_THRESHOLD, EVotingObcBelowThreshold);
        assert!(split_amount >= MIN_STAKING_THRESHOLD, EVotingObcBelowThreshold);
        VotingObc {
            id: object::new(ctx),
            pool_id: self.pool_id,
            principal: balance::split(&mut self.principal, split_amount),
        }
    }

    /// Split the given votingObc to the two parts, one with principal `split_amount`,
    /// transfer the newly split part to the sender address.
    public entry fun split_voting_obc(votingObc: &mut VotingObc, split_amount: u64, ctx: &mut TxContext) {
        transfer::transfer(split(votingObc, split_amount, ctx), tx_context::sender(ctx));
    }

    /// Consume the voting obc `other` and add its value to `self`.
    /// Aborts if some of the staking parameters are incompatible (pool id,  activation epoch, etc.)
    public entry fun join_voting_obc(self: &mut VotingObc, other: VotingObc) {
        assert!(is_equal_staking_metadata(self, &other), EIncompatibleVotingObc);
        let VotingObc {
            id,
            pool_id: _,
            principal,
        } = other;

        object::delete(id);
        balance::join(&mut self.principal, principal);
    }

    /// Returns true if all the staking parameters of the voting obc except the principal are identical
    public fun is_equal_staking_metadata(self: &VotingObc, other: &VotingObc): bool {
        (self.pool_id == other.pool_id)
    }


    public fun pool_token_exchange_rate_at_epoch(): PoolTokenExchangeRate {
        initial_exchange_rate()
    }




    public fun obc_amount(exchange_rate: &PoolTokenExchangeRate): u64 {
        exchange_rate.obc_amount
    }

    public fun pool_token_amount(exchange_rate: &PoolTokenExchangeRate): u64 {
        exchange_rate.pool_token_amount
    }



    fun get_obc_amount(exchange_rate: &PoolTokenExchangeRate, token_amount: u64): u64 {
        // When either amount is 0, that means we have no voting with this pool.
        // The other amount might be non-zero when there's dust left in the pool.
        if (exchange_rate.obc_amount == 0 || exchange_rate.pool_token_amount == 0) {
            return token_amount
        };
        let res = (exchange_rate.obc_amount as u128)
            * (token_amount as u128)
            / (exchange_rate.pool_token_amount as u128);
        (res as u64)
    }

    fun get_token_amount(exchange_rate: &PoolTokenExchangeRate, obc_amount: u64): u64 {
        // When either amount is 0, that means we have no voting with this pool.
        // The other amount might be non-zero when there's dust left in the pool.
        if (exchange_rate.obc_amount == 0 || exchange_rate.pool_token_amount == 0) {
            return obc_amount
        };
        let res = (exchange_rate.pool_token_amount as u128)
            * (obc_amount as u128)
            / (exchange_rate.obc_amount as u128);
        (res as u64)
    }

    fun initial_exchange_rate(): PoolTokenExchangeRate {
        PoolTokenExchangeRate { obc_amount: 0, pool_token_amount: 0 }
    }





}
