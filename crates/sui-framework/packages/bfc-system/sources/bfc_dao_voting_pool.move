module bfc_system::voting_pool {
    use sui::balance::{Self, Balance};
    use sui::bfc::BFC;
    use sui::clock;
    use sui::clock::Clock;
    use sui::tx_context::{Self, TxContext};
    use sui::transfer;
    use sui::object::{Self, ID, UID};
    spec module{
        pragma verify;
        //pragma aborts_if_is_strict;
    }


    friend bfc_system::bfc_dao;
    /// votingBfc objects cannot be split to below this amount.
    const MIN_STAKING_THRESHOLD: u64 = 1_000_000_000; // 1 bfc

    const EInsufficientPoolTokenBalance: u64 = 0;
    const EWrongPool: u64 = 1;
    const EWithdrawAmountCannotBeZero: u64 = 2;
    const EInsufficientBfcTokenBalance: u64 = 3;
    const ETokenBalancesDoNotMatchExchangeRate: u64 = 9;
    const EIncompatibleVotingBfc: u64 = 12;
    const EDelegationOfZeroBfc: u64 = 17;
    const EVotingBfcBelowThreshold: u64 = 18;

    const DEFAULT_VOTE_END_TIME: u64      = 1000 * 60 * 60  * 1; // 3 hours later
    const ENotEndOfStakingTime: u64 = 19;


    /// A staking pool embedded in each validator struct in the system state object.
    struct VotingPool has key, store {
        id: UID,
        /// The total number of Bfc tokens in this pool,
        bfc_balance: u64,
        /// Total number of pool tokens issued by the pool.
        pool_token_balance: u64,

    }

    /// Struct representing the exchange rate of the voting pool token to BFC.
    struct PoolTokenExchangeRate has store, copy, drop {
        bfc_amount: u64,
        pool_token_amount: u64,
    }

    /// A self-custodial object holding the Voting bfc tokens.
    struct VotingBfc has key, store {
        id: UID,
        /// ID of the staking pool we are staking with.
        pool_id: ID,
        /// The voting BFC tokens.
        principal: Balance<BFC>,
        /// when voting stake ends.
        stake_end_time: u64,

    }

    // ==== initializer ====

    /// Create a new, empty voting pool.
    public(friend) fun new(ctx: &mut TxContext) : VotingPool {
        VotingPool {
            id: object::new(ctx),
            bfc_balance: 0,
            pool_token_balance: 0,
        }
    }


    // ==== voting requests ====

    /// Request to voting to a staking pool. The voting starts counting at the beginning of the next epoch,
    public(friend) fun request_add_voting(
        pool: &VotingPool,
        voting: Balance<BFC>,
        clock: &Clock,
        ctx: &mut TxContext
    ) : VotingBfc {
        let bfc_amount = balance::value(&voting);
        assert!(bfc_amount >= MIN_STAKING_THRESHOLD, EDelegationOfZeroBfc);
        let votingbfc = VotingBfc {
            id: object::new(ctx),
            pool_id: object::id(pool),
            principal: voting,
            stake_end_time: clock::timestamp_ms(clock) + DEFAULT_VOTE_END_TIME,
        };
        votingbfc
    }

    /// Request to withdraw the given voting plus rewards from a staking pool.
    /// Both the principal and corresponding rewards in BFC are withdrawn.
    /// A proportional amount of pool token withdraw is recorded and processed at epoch change time.
    public(friend) fun request_withdraw_voting(
        pool: &VotingPool,
        voting_bfc: VotingBfc,
        clock: &Clock,
    ) : Balance<BFC> {
        let (_, principal_withdraw) =
            withdraw_from_principal(pool, voting_bfc, clock);
        principal_withdraw
    }

    /// Withdraw the principal BFC stored in the votingdBfc object, and calculate the corresponding amount of pool
    /// tokens using exchange rate at staking epoch.
    /// Returns values are amount of pool tokens withdrawn and withdrawn principal portion of BFC.
    public(friend) fun withdraw_from_principal(
        pool: &VotingPool,
        voting_bfc: VotingBfc,
        clock: &Clock,
    ) : (u64, Balance<BFC>) {

        // Check that the voting information matches the pool.
        assert!(voting_bfc.pool_id == object::id(pool), EWrongPool);
        assert!(clock::timestamp_ms(clock) > voting_bfc.stake_end_time, ENotEndOfStakingTime);


        let exchange_rate_at_staking_epoch = pool_token_exchange_rate_at_epoch();
        let principal_withdraw = unwrap_voting_bfc(voting_bfc);
        let pool_token_withdraw_amount = get_token_amount(&exchange_rate_at_staking_epoch, balance::value(&principal_withdraw));

        (
            pool_token_withdraw_amount,
            principal_withdraw,
        )
    }



    public fun unwrap_voting_bfc(voting_bfc: VotingBfc): Balance<BFC> {
        let VotingBfc {
            id,
            pool_id: _,
            principal,
            stake_end_time: _,
        } = voting_bfc;
        object::delete(id);
        principal
    }

    // ==== getters and misc utility functions ====

    public fun bfc_balance(pool: &VotingPool): u64 { pool.bfc_balance }

    public fun pool_id(voting_bfc: &VotingBfc): ID { voting_bfc.pool_id }

    public fun voting_bfc_amount(voting_bfc: &VotingBfc): u64 { balance::value(&voting_bfc.principal) }




    /// Split votingBfc `self` to two parts, one with principal `split_amount`,
    /// and the remaining principal is left in `self`.
    /// All the other parameters of the votingBfc like `voting` or `pool_id` remain the same.
    public fun split(self: &mut VotingBfc, split_amount: u64, ctx: &mut TxContext): VotingBfc {
        let original_amount = balance::value(&self.principal);
        assert!(split_amount <= original_amount, EInsufficientBfcTokenBalance);
        let remaining_amount = original_amount - split_amount;
        // Both resulting parts should have at least MIN_STAKING_THRESHOLD.
        assert!(remaining_amount >= MIN_STAKING_THRESHOLD, EVotingBfcBelowThreshold);
        assert!(split_amount >= MIN_STAKING_THRESHOLD, EVotingBfcBelowThreshold);
        VotingBfc {
            id: object::new(ctx),
            pool_id: self.pool_id,
            principal: balance::split(&mut self.principal, split_amount),
            stake_end_time: self.stake_end_time,
        }
    }




    /// Split the given votingBfc to the two parts, one with principal `split_amount`,
    /// transfer the newly split part to the sender address.
    #[test_only]
    public entry fun split_voting_bfc(votingBfc: &mut VotingBfc, split_amount: u64, ctx: &mut TxContext) {
        transfer::transfer(split(votingBfc, split_amount, ctx), tx_context::sender(ctx));
    }

    /// Consume the voting bfc `other` and add its value to `self`.
    /// Aborts if some of the staking parameters are incompatible (pool id,  activation epoch, etc.)
    #[test_only]
    public entry fun join_voting_bfc(self: &mut VotingBfc, other: VotingBfc) {
        assert!(is_equal_staking_metadata(self, &other), EIncompatibleVotingBfc);
        let VotingBfc {
            id,
            pool_id: _,
            principal,
            stake_end_time: _,
        } = other;

        object::delete(id);
        balance::join(&mut self.principal, principal);
    }

    /// Returns true if all the staking parameters of the voting bfc except the principal are identical
    public fun is_equal_staking_metadata(self: &VotingBfc, other: &VotingBfc): bool {
        (self.pool_id == other.pool_id)
    }


    public fun pool_token_exchange_rate_at_epoch(): PoolTokenExchangeRate {
        initial_exchange_rate()
    }




    public fun bfc_amount(exchange_rate: &PoolTokenExchangeRate): u64 {
        exchange_rate.bfc_amount
    }

    public fun pool_token_amount(exchange_rate: &PoolTokenExchangeRate): u64 {
        exchange_rate.pool_token_amount
    }




    fun get_token_amount(exchange_rate: &PoolTokenExchangeRate, bfc_amount: u64): u64 {
        // When either amount is 0, that means we have no voting with this pool.
        // The other amount might be non-zero when there's dust left in the pool.
        if (exchange_rate.bfc_amount == 0 || exchange_rate.pool_token_amount == 0) {
            return bfc_amount
        };
        let res = (exchange_rate.pool_token_amount as u128)
            * (bfc_amount as u128)
            / (exchange_rate.bfc_amount as u128);
        (res as u64)
    }

    fun initial_exchange_rate(): PoolTokenExchangeRate {
        PoolTokenExchangeRate { bfc_amount: 0, pool_token_amount: 0 }
    }
}