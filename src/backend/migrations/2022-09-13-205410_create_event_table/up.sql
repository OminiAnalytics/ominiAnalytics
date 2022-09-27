-- Your SQL goes here
CREATE TABLE IF NOT EXISTS omini_events (
    id serial PRIMARY KEY,
    /* creator id */
    creator_id uuid NOT NULL,
    /* campaign id */
    compaign_id uuid NOT NULL,
    event_id varchar(15) UNIQUE NOT NULL,
    slug varchar(30) UNIQUE NOT NULL,
    active bool NOT NULL,
    descr text NOT NULL,
    created_at bigint DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL
);

