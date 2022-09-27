-- Your SQL goes here
/*
 This migration creates a table with an index for each device info index
 */
CREATE TABLE IF NOT EXISTS omini_device_info_index (
    id serial PRIMARY KEY,
    uid uuid NOT NULL,
    -- user id
    OsType varchar(10) NOT NULL,
    OsVersion varchar(10) NOT NULL,
    OsName varchar(20) NOT NULL,
    OsArch varchar(10) NOT NULL,
    BrowserName varchar(40) NOT NULL,
    BrowserVersion varchar(10) NOT NULL,
    BrowserFversion varchar(15) NOT NULL,
    Country varchar(20) NOT NULL,
    CPU varchar(2) NOT NULL,
    GPU varchar(100) NOT NULL,
    Memory varchar(2) NOT NULL,
    ScreenHeight varchar(4) NOT NULL,
    ScreenWidth varchar(4) NOT NULL,
    ScreenColor varchar(10) NOT NULL,
    ColorBuffer varchar(10) NOT NULL,
    UserAgent varchar(200) NOT NULL
);

ALTER TABLE omini_device_info_index
    ADD COLUMN OS_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', OsName || ' ' || OsVersion || ' ' || OsArch || ' ' || OsType)) STORED;

ALTER TABLE omini_device_info_index
    ADD COLUMN Browser_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', BrowserName || ' ' || BrowserVersion || ' ' || BrowserFversion)) STORED;

ALTER TABLE omini_device_info_index
    ADD COLUMN Country_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', Country)) STORED;

ALTER TABLE omini_device_info_index
    ADD COLUMN CPU_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', CPU)) STORED;

ALTER TABLE omini_device_info_index
    ADD COLUMN GPU_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', GPU)) STORED;

ALTER TABLE omini_device_info_index
    ADD COLUMN Memory_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', Memory)) STORED;

ALTER TABLE omini_device_info_index
    ADD COLUMN Screen_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', ScreenHeight || ' ' || ScreenWidth || ' ' || ScreenColor || ' ' || ColorBuffer)) STORED;

ALTER TABLE omini_device_info_index
    ADD COLUMN UserAgent_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', UserAgent)) STORED;

CREATE INDEX device_idx_ua ON omini_device_info_index USING GIN (UserAgent_ts);

CREATE INDEX device_idx_os ON omini_device_info_index USING GIN (OS_ts);

CREATE INDEX device_idx_browser ON omini_device_info_index USING GIN (Browser_ts);

CREATE INDEX device_idx_country ON omini_device_info_index USING GIN (Country_ts);

CREATE INDEX device_idx_cpu ON omini_device_info_index USING GIN (CPU_ts);

CREATE INDEX device_idx_gpu ON omini_device_info_index USING GIN (GPU_ts);

CREATE INDEX device_idx_memory ON omini_device_info_index USING GIN (Memory_ts);

CREATE INDEX device_idx_screen ON omini_device_info_index USING GIN (Screen_ts);

