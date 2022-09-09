/*
 File: config.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 8/09/2022 05:33:48
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use super::custom::constants::{BOT_UA_LIST, MAX_REQUESTS_PER_IP_V4, MAX_REQUESTS_PER_IP_V6};
use crate::db::get_connection;
use actix_web::HttpRequest;
use chrono::Utc;
use deadpool_postgres::Pool;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::Sequence;
use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
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
        let client_real_ip = x_forwarded_for.last().unwrap();
        let is_valid_ip = client_real_ip.parse::<IpAddr>();
        if is_valid_ip.is_err() {
            return None;
        } else {
            // We determine the type of ip address (v4,v6)
            // as it is important for user creation

            // ip is ipv4
            if is_valid_ip.unwrap().is_ipv4() {
                return Some(std::net::IpAddr::V4(
                    client_real_ip.parse::<Ipv4Addr>().unwrap(),
                ));
                // ip is ipv6
            } else {
                return Some(std::net::IpAddr::V6(
                    client_real_ip.parse::<Ipv6Addr>().unwrap(),
                ));
            }
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
    for keyword in BOT_UA_LIST.iter() {
        if ua.contains(keyword) {
            return false;
        }
    }
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
pub async fn request_filter(req: &HttpRequest) -> Option<IpAddr> {
    if is_ua_valid(req.headers().get("User-Agent").unwrap().to_str().unwrap()) && valid_origin(req)
    {
        return get_ip(req);
    }
    return None;
}

// Filter traffic flood by ip
const CALLS_COUNT_QUERY: &str = r#"SELECT COUNT(*)::INT2 as count FROM omini_calls WHERE ip = $1 AND $2 - created_at < 60 LIMIT(51);"#;

/*
ip : IpAddr
return : bool
*/
pub async fn prevent_flood(ip: IpAddr, pool: &Pool) -> Result<bool, String> {
    let time_now_in_sec = Utc::now().timestamp();
    let result = get_connection(pool)
        .await?
        .query(CALLS_COUNT_QUERY, &[&ip.to_string(), &time_now_in_sec])
        .await
        .unwrap()
        .into_iter()
        .next();
    let count = result.unwrap().get::<_, i16>("count");
    match ip {
        IpAddr::V4(_) => {
            if count > MAX_REQUESTS_PER_IP_V4 {
                return Ok(false);
            } else {
                return Ok(true);
            }
        }
        IpAddr::V6(_) => {
            if count > MAX_REQUESTS_PER_IP_V6 {
                return Ok(false);
            } else {
                return Ok(true);
            }
        }
    }
}
