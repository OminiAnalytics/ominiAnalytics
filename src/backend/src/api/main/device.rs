/*
 File: device.rs
 Created Date: 07 Sep 2022
 Author: realbacon
 -----
 Last Modified: 9/09/2022 11:08:18
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use serde::{Deserialize, Serialize};
use postgres_types::FromSql;

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

#[derive(Debug, Serialize, Deserialize,FromSql,Clone)]
pub struct Device {
    pub country: String,
    pub browser: Browser,
    pub os: Os,
    pub hardware: Hardware,
    pub useragent: String,
}

#[derive(Debug, Serialize, Deserialize,FromSql,Clone)]
pub struct Browser {
    pub name: String,
    pub version: String,
    pub fversion: String,
}

#[derive(Debug, Serialize, Deserialize,FromSql,Clone)]
pub struct Os {
    pub name: String,
    pub version: String,
    pub typ: String,
    pub arch: String,
}

#[derive(Debug, Serialize, Deserialize,FromSql,Clone)]
pub struct Hardware {
    pub memory: i8,
    pub cpu: i8,
    pub gpu: String,
    pub color_buffer_float: i32,
    pub screen: Vec<i16>,
}
