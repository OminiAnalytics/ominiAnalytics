-- Your SQL goes here
-- Create a table with rules for url origin
CREATE TABLE IF NOT EXISTS omini_rules (
    id serial PRIMARY KEY,
    allowed_regex varchar(200) NOT NULL,
    created_at bigint DEFAULT EXTRACT(EPOCH FROM NOW()),
    updated_at bigint DEFAULT EXTRACT(EPOCH FROM NOW()),
    created_from_id uuid NOT NULL REFERENCES omini_managers (id),
    active boolean NOT NULL DEFAULT TRUE
);

