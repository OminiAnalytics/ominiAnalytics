/*
 File: security.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 30/08/2022 09:56:38
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use crate::config::valid_origin;
use actix_web::HttpRequest;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr};

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
