-- Your SQL goes here
ALTER TABLE omini_users
    ADD created_at bigint DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL;

ALTER TABLE omini_users
    ADD updated_at bigint DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL;

ALTER TABLE omini_users
    ADD device_info jsonb NOT NULL;

ALTER TABLE omini_users
    ADD country varchar(20) NOT NULL;

ALTER TABLE omini_users
    ADD ip varchar(45) NOT NULL;

