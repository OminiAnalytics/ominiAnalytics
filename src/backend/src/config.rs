/*
 File: config.rs
 Created Date: 28 Aug 2022
 Author: realbacon
 -----
 Last Modified: 29/08/2022 02:57:29
 Modified By: realbacon
 -----
 Copyright (c) 2022 Omini
 -----
*/

use actix_web::HttpRequest;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::Sequence;
use std::{fs::File, io::Read};

pub fn valid_origin(req: HttpRequest) -> bool {
    let request_origin = req.headers().get("Referer");
    let cfg = read_config();
    let cfg = match cfg {
        Ok(cfg) => cfg,

        Err(_) => return false,
    };
    match request_origin {
        Some(url) => {
            let result = false;
            for reg_str in cfg.autorized_urls.iter() {
                let reg = Regex::new(reg_str).unwrap();
                if reg.is_match(url.to_str().unwrap()) == true {
                    return true;
                }
            }
            return result;
        }
        None => return false,
    }
}

// read the file "config.yaml" and parse it
pub fn read_config() -> Result<Config, std::io::Error> {
    let mut file = File::open("./src/config.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let cfg: Config = serde_yaml::from_str(&contents[..]).unwrap();
    Ok(cfg)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: String,
    autorized_urls: Vec<String>,
    dashboard_creds: Sequence,
}
