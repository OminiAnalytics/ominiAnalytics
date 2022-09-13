-- Your SQL goes here

/*
 This migration creates a table with an index for each device info index
 */

CREATE TABLE
    IF NOT EXISTS omini_device_info_index (
        id SERIAL PRIMARY KEY,
        uid UUID NOT NULL,
        -- user id
        OsType VARCHAR(10) NOT NULL,
        OsVersion VARCHAR(10) NOT NULL,
        OsName VARCHAR(20) NOT NULL,
        OsArch VARCHAR(10) NOT NULL,
        BrowserName VARCHAR(40) NOT NULL,
        BrowserVersion VARCHAR(10) NOT NULL,
        BrowserFversion VARCHAR(15) NOT NULL,
        Country VARCHAR(20) NOT NULL,
        CPU VARCHAR(2) NOT NULL,
        GPU VARCHAR(100) NOT NULL,
        Memory VARCHAR(2) NOT NULL,
        ScreenHeight VARCHAR(4) NOT NULL,
        ScreenWidth VARCHAR(4) NOT NULL,
        ScreenColor VARCHAR(10) NOT NULL,
        ColorBuffer VARCHAR(10) NOT NULL,
        UserAgent VARCHAR(200) NOT NULL
    );

ALTER TABLE
    omini_device_info_index
ADD
    COLUMN OS_ts tsvector GENERATED ALWAYS AS (
        to_tsvector(
            'english',
            OsName || ' ' || OsVersion || ' ' || OsArch || ' ' || OsType
        )
    ) STORED;

ALTER TABLE
    omini_device_info_index
ADD
    COLUMN Browser_ts tsvector GENERATED ALWAYS AS (
        to_tsvector(
            'english',
            BrowserName || ' ' || BrowserVersion || ' ' || BrowserFversion
        )
    ) STORED;

ALTER TABLE
    omini_device_info_index
ADD
    COLUMN Country_ts tsvector GENERATED ALWAYS AS (
        to_tsvector('english', Country)
    ) STORED;

ALTER TABLE
    omini_device_info_index
ADD
    COLUMN CPU_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', CPU)) STORED;

ALTER TABLE
    omini_device_info_index
ADD
    COLUMN GPU_ts tsvector GENERATED ALWAYS AS (to_tsvector('english', GPU)) STORED;

ALTER TABLE
    omini_device_info_index
ADD
    COLUMN Memory_ts tsvector GENERATED ALWAYS AS (
        to_tsvector('english', Memory)
    ) STORED;

ALTER TABLE
    omini_device_info_index
ADD
    COLUMN Screen_ts tsvector GENERATED ALWAYS AS (
        to_tsvector(
            'english',
            ScreenHeight || ' ' || ScreenWidth || ' ' || ScreenColor || ' ' || ColorBuffer
        )
    ) STORED;

ALTER TABLE
    omini_device_info_index
ADD
    COLUMN UserAgent_ts tsvector GENERATED ALWAYS AS (
        to_tsvector('english', UserAgent)
    ) STORED;

CREATE INDEX
    device_idx_ua ON omini_device_info_index USING GIN (UserAgent_ts);

CREATE INDEX
    device_idx_os ON omini_device_info_index USING GIN (OS_ts);

CREATE INDEX
    device_idx_browser ON omini_device_info_index USING GIN (Browser_ts);

CREATE INDEX
    device_idx_country ON omini_device_info_index USING GIN (Country_ts);

CREATE INDEX
    device_idx_cpu ON omini_device_info_index USING GIN (CPU_ts);

CREATE INDEX
    device_idx_gpu ON omini_device_info_index USING GIN (GPU_ts);

CREATE INDEX
    device_idx_memory ON omini_device_info_index USING GIN (Memory_ts);

CREATE INDEX
    device_idx_screen ON omini_device_info_index USING GIN (Screen_ts);