/*
 File: sql.rs
 Created Date: 12 Sep 2022
 Author: realbacon
 -----
 Last Modified: 13/09/2022 12:10:54
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

pub const INSERT_USER_QUERY: &str =
    "INSERT INTO omini_users (id, ip, device_info, country) VALUES ($1, $2, $3, $4)";

pub const FETCH_USER_QUERY_FOR_V6: &str = "SELECT
      CASE WHEN EXISTS 
      (
           SELECT id FROM omini_users WHERE ip = $1  GROUP BY id
      )
      THEN CAST(id as VARCHAR)
      ELSE 'false'
   END as exists
   FROM omini_users;";
pub const FETCH_USER_QUERY_FOR_V4: &str = "SELECT * FROM USER_EXISTS($1,$2,$3) AS exists;";

/*OsType VARCHAR(10) NOT NULL,
OsVersion VARCHAR(10) NOT NULL,
OsName VARCHAR(20) NOT NULL,
OsArch VARCHAR(10) NOT NULL,
OsType VARCHAR(10) NOT NULL,
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
UserAgent VARCHAR(200) NOT NULL */
pub const INSERT_DEVICE_INDEX_QUERY : &str = "INSERT INTO omini_device_info_index (OsType, OsVersion, OsName, OsArch,\
   BrowserName, BrowserVersion, BrowserFversion, Country, CPU, GPU, Memory, ScreenHeight, ScreenWidth,\
   ScreenColor, ColorBuffer, UserAgent, uid) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)";
