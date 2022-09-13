/*
 File: structs.rs
 Created Date: 13 Sep 2022
 Author: realbacon
 -----
 Last Modified: 13/09/2022 08:35:18
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use postgres_types::FromSql;
use serde::{Deserialize, Serialize};

/**
 * Template :
 * {
  "Country": "France",
  "Browser": {
    "name": "Brave",
    "version": "8",
    "fversion": "8"
  },
  "Os": [
    {
      "name": "Windows",
      "version": "10"
    },
    "desktop",
    "x64"
  ],
  "Hardware": {
    "memory": 8,
    "cpu": 12,
    "gpu": "ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero) (0x0000C0DE)), SwiftShader driver)",
    "color_buffer_float": 34836,
    "screen": [
      1920,
      1080,
      24
    ]
  },
  "UserAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
}
 */

#[derive(Debug, Serialize, Deserialize, FromSql)]
pub struct Device {
    pub country: String,
    pub browser: Browser,
    pub os: Os,
    pub hardware: Hardware,
    pub useragent: String,
}

#[derive(Debug, Serialize, Deserialize, FromSql)]
pub struct Browser {
    pub name: String,
    pub version: String,
    pub fversion: String,
}

#[derive(Debug, Serialize, Deserialize, FromSql)]
pub struct Os {
    pub name: String,
    pub version: String,
    pub typ: String,
    pub arch: String,
}

#[derive(Debug, Serialize, Deserialize, FromSql)]
pub struct Hardware {
    pub memory: i8,
    pub cpu: i8,
    pub gpu: String,
    pub color_buffer_float: i32,
    pub screen: Vec<i16>,
}

/* @main */
#[derive(Serialize, Deserialize, Debug)]
pub struct NewInst {
    pub bp: BPayload,  // base64 payload and hash
    pub pag: PageCall, // page
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BPayload {
    pub dt: String,   // Base64 payload
    pub hash: String, // hash
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageCall {
    pub nam: String, // page name
    pub url: String, // url
}
