spec obc_system::voting_pool{
    spec withdraw_from_principal{
        aborts_if false;
        aborts_if voting_obc.pool_id != object::id(pool);
    }
    spec new{
        aborts_if false;
        aborts_if ctx.ids_created + 1 > MAX_U64;
    }
}