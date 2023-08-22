module hello_world::voting_obc{
    use sui::tx_context::TxContext;
    use sui::tx_context;
    use sui::transfer;
    use hello_world::voting_pool;
    use sui::coin::Coin;
    use sui::obc::OBC;
    use hello_world::voting_pool::{VotingPool, VotingObc};
    use sui::coin;


    // Part 3: transfer the OBC Dao object to the sender
    entry public fun create_voting_pool(ctx: &mut TxContext ) {
        // sender address
        let sender = tx_context::sender(ctx);
        let voting_pool = voting_pool::new(ctx);
        transfer::public_transfer(voting_pool, sender);
    }

    entry public fun create_voting_obc(votingPool: &mut VotingPool,
                                       coin: Coin<OBC>,
                                       ctx: &mut TxContext ,) {
        // sender address
        let sender = tx_context::sender(ctx);
        let balance = coin::into_balance(coin);
        let voting_obc = voting_pool::request_add_voting(votingPool, balance, ctx);

        transfer::public_transfer(voting_obc, sender);
    }

    entry public fun withdraw_voting(  votingPool: &mut VotingPool,
                                       voting_obc: VotingObc,
                                       ctx: &mut TxContext ,) {
        // sender address
        let sender = tx_context::sender(ctx);
        let voting_obc = voting_pool::request_withdraw_voting(votingPool, voting_obc);
        let coin = coin::from_balance(voting_obc, ctx);
        transfer::public_transfer(coin, sender);

    }
}