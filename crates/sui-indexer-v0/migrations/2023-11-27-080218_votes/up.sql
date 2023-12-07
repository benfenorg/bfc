-- Your SQL goes here

CREATE TABLE dao_votes (
    id                          BIGSERIAL     PRIMARY KEY,
    transaction_digest          base58digest  NOT NULL,
    sender                      VARCHAR(255)  NOT NULL,
    pid                         BIGINT        NOT NULL,
    agree                       BOOLEAN       NOT NULL,
    vote                        BIGINT        NOT NULL,
    voter                       address       NOT NULL
);

CREATE INDEX dao_votes_voter ON dao_votes (voter);
