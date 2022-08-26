-- Your SQL goes here
CREATE TABLE IF NOT EXISTS alive_messages
(
    id         UUID PRIMARY KEY        NOT NULL,
    u_id       UUID NOT NULL,
    created_at DOUBLE PRECISION DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL,
    mtype    VARCHAR                 NOT NULL
);