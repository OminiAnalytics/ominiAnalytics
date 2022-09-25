-- Your SQL goes here
CREATE TABLE IF NOT EXISTS omini_managers (
    id uuid PRIMARY KEY,
    name varchar(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    status varchar(255) NOT NULL,
    profile_picture varchar(255) NOT NULL DEFAULT 'default.png',
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE EXTENSION IF NOT EXISTS pgcrypto;

