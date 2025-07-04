-- Your SQL goes here
CREATE TABLE limit_config (
      chain_id  INTEGER    NOT NULL,
      path      INTEGER    NOT NULL,
      limits    BIGINT     NOT NULL,
      PRIMARY KEY (chain_id, path)
);