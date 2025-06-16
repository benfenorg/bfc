// @generated automatically by Diesel CLI.

diesel::table! {
    bridge_record (chain_id, address, path) {
        chain_id -> Int4,
        address -> Bytea,
        path -> Int4,
        amount -> Int8,
        tx_hash -> Bytea,
        timestamp_ms -> Int8,
    }
}

diesel::table! {
    limit_config (chain_id, path) {
        chain_id -> Int4,
        path -> Int4,
        limits -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bridge_record,
    limit_config,
);
