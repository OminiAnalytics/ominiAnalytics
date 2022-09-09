-- Your SQL goes here

/*
 This migration creates a table with an index for each device info index
 */

CREATE TABLE
    IF NOT EXISTS device_info_index (
        id INTEGER PRIMARY KEY,
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

CREATE INDEX
    OS ON device_info_index USING GIN (
        to_tsvector(
            'english',
            OsType || ' ' || OsVersion || ' ' || OsName || ' ' || OsArch
        )
    );

CREATE INDEX
    Browser ON device_info_index USING GIN (
        to_tsvector(
            'english',
            BrowserName || ' ' || BrowserVersion || ' ' || BrowserFversion
        )
    );

CREATE INDEX
    Country ON device_info_index USING GIN (
        to_tsvector('english', Country)
    );

CREATE INDEX
    CPU ON device_info_index USING GIN (to_tsvector('english', CPU));

CREATE INDEX
    GPU ON device_info_index USING GIN (to_tsvector('english', GPU));

CREATE INDEX
    Memory ON device_info_index USING GIN (
        to_tsvector('english', Memory)
    );

CREATE INDEX
    Screen ON device_info_index USING GIN (
        to_tsvector(
            'english',
            ScreenHeight || ' ' || ScreenWidth || ' ' || ScreenColor || ' ' || ColorBuffer
        )
    );

CREATE INDEX
    UserAgent ON device_info_index USING GIN (
        to_tsvector('english', UserAgent)
    );