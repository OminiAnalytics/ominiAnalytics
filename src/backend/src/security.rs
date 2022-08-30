/*
 File: config.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 30/08/2022 10:09:2
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use actix_web::HttpRequest;
use actix_web::HttpRequest;
use regex::Regex;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::Sequence;
use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::{fs::File, io::Read};

pub fn valid_origin(req: &HttpRequest) -> bool {
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

// Return the ip address of the client
/*
req : HttpRequest
return : Option<IpAddr>
*/
pub fn get_ip(req: &HttpRequest) -> Option<IpAddr> {
    if env::var("ENV").map_err(|_| "DEV") == Ok(String::from("DEV")) {
        return Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
    let x_forwarded_for = req.headers().get("X-Forwarded-For");
    if x_forwarded_for.is_some() {
        let x_forwarded_for = x_forwarded_for
            .unwrap()
            .to_str()
            .unwrap()
            .split(",")
            .map(|x| x.trim())
            .filter(|&ip| ip != "127.0.0.1")
            .collect::<Vec<&str>>();
        let x_forwarded_for = x_forwarded_for.last().unwrap();
        let x_forwarded_for = x_forwarded_for.parse::<IpAddr>();
        if x_forwarded_for.is_err() {
            return None;
        } else {
            return Some(x_forwarded_for.unwrap());
        }
    } else {
        return None;
    }
}

// Check if user agent is valid using a regex
/*
ua : String
return : bool
*/
pub fn is_ua_valid(ua: &str) -> bool {
    let reg = Regex::new(r#"\((?P<i>.*?)\)(\s|$)|(?P<name>.*?)/(?P<version>.*?)(\s|$)"#).unwrap();
    if reg.is_match(ua) == true {
        return true;
    } else {
        return false;
    }
}

// Filter request using ip + useragent
/*
req : &HttpRequest
return : Option<IpAddr>
*/
pub fn request_filter(req: &HttpRequest) -> Option<IpAddr> {
    if is_ua_valid(req.headers().get("User-Agent").unwrap().to_str().unwrap()) && valid_origin(req)
    {
        return get_ip(req);
    }
    return None;
}
