-- Your SQL goes here
CREATE TABLE IF NOT EXISTS omini_event_logs (
    id serial PRIMARY KEY,
    event_id varchar(15) NOT NULL,
    created_at bigint DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL,
    from_url varchar(255) NOT NULL)
