-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.
-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at (_tbl regclass)
    RETURNS VOID
    AS $$
BEGIN
    EXECUTE format("CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()", _tbl);
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at ()
    RETURNS TRIGGER
    AS $$
BEGIN
    IF (NEW IS DISTINCT FROM OLD AND NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at) THEN
        NEW.updated_at := CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS omini_users (
    id uuid PRIMARY KEY NOT NULL,
    created_at double precision DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL,
    updated_at double precision DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL,
    device_info jsonb NOT NULL,
    lang varchar(10) NOT NULL,
    os varchar(20) NOT NULL
);

CREATE TABLE IF NOT EXISTS omini_alive_messages (
    id uuid PRIMARY KEY NOT NULL,
    u_id uuid NOT NULL REFERENCES omini_users (id),
    created_at double precision DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL,
    mtyp varchar NOT NULL
);

