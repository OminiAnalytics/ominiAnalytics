-- Your SQL goes here

CREATE TABLE IF NOT EXISTS omini_users (
    id UUID PRIMARY KEY NOT NULL,
    created_at DOUBLE PRECISION DEFAULT EXTRACT(
        EPOCH
        FROM
            NOW()
    ) NOT NULL,
    updated_at DOUBLE PRECISION DEFAULT EXTRACT(
        EPOCH
        FROM
            NOW()
    ) NOT NULL,
    device_info JSONB NOT NULL,
    lang VARCHAR(10) NOT NULL,
    os VARCHAR(20) NOT NULL
);