-- Your SQL goes here
CREATE TABLE bridge_record (
       chain_id      INTEGER    NOT NULL,
       address       BYTEA      NOT NULL,
       path          INTEGER    NOT NULL,
       amount        BIGINT     NOT NULL,
       tx_hash       BYTEA      NOT NULL,
       timestamp_ms  BIGINT     NOT NULL,
       PRIMARY KEY (chain_id, address, path)
);

CREATE UNIQUE INDEX bridge_record_txn_hash_idx
    ON bridge_record (tx_hash);