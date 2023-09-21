spec obc_system::voting_pool{
    spec withdraw_from_principal {
        aborts_if false;
        aborts_if voting_obc.pool_id != object::id(pool);
    }
    spec new {
        aborts_if false;
        aborts_if ctx.ids_created + 1 > MAX_U64;
    }

    spec obc_amount {
        aborts_if false;
    }
    spec obc_balance {
        aborts_if false;

    }

    spec split {
        aborts_if false;
        let original_amount = balance::value(self.principal);
        aborts_if split_amount > original_amount;
        let remaining_amount = original_amount - split_amount;
        aborts_if remaining_amount < MIN_STAKING_THRESHOLD;
        aborts_if split_amount < MIN_STAKING_THRESHOLD;
        aborts_if ctx.ids_created + 1 > MAX_U64;

    }
    spec request_add_voting {
        aborts_if false;
        aborts_if voting.value < MIN_STAKING_THRESHOLD;
        aborts_if ctx.ids_created + 1 > MAX_U64;
    }

    spec request_withdraw_voting {
        aborts_if false;
        aborts_if voting_obc.pool_id != object::id(pool);
    }
    spec unwrap_voting_obc {
        aborts_if false;
    }
    spec pool_id {
        aborts_if false;
    }
    spec voting_obc_amount {
        aborts_if false;
    }
    spec split_voting_obc {
        aborts_if false;
        aborts_if split_amount > votingObc.principal.value;

        let remaining_amount = votingObc.principal.value - split_amount;
        aborts_if remaining_amount < MIN_STAKING_THRESHOLD;
        aborts_if split_amount < MIN_STAKING_THRESHOLD;
        aborts_if ctx.ids_created + 1 > MAX_U64;
    }
    spec join_voting_obc {
        aborts_if false;
        aborts_if other.pool_id != self.pool_id;
        aborts_if self.principal.value + other.principal.value > MAX_U64;
    }

    spec is_equal_staking_metadata {
        aborts_if false;
    }

    spec pool_token_exchange_rate_at_epoch {
        aborts_if false;
    }
}