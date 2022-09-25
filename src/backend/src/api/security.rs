/*
 File: security.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 16/09/2022 12:52:0
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
use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// This function validates the origin based on the rules set by managers
/// or admins in the dashboard panel.
/// It returns a boolean value.
/// If the origin is valid, it returns true.
/// If the origin is invalid, it returns false.
/// # Arguments
/// * `req` - The request object.
/// * `pool` - The database connection pool.
/// # Returns
/// * `bool` - The boolean value.
pub async fn valid_origin(req: &HttpRequest, pool: &Pool) -> bool {
    let request_origin = req.headers().get("Referer");
    let request_origin = match request_origin {
        Some(origin) => Some(origin),
        None => req.headers().get("Origin"),
    };
    let allow_list = read_config(pool);
    let allow_list = match allow_list.await {
        Ok(v) => v,

        Err(_) => return false,
    };
    match request_origin {
        Some(url) => {
            let result = false;
            for reg_str in allow_list.iter() {
                let reg = Regex::new(reg_str).unwrap();
                println!("reg_str: {}", reg_str);
                if reg.is_match(url.to_str().unwrap()) == true {
                    return true;
                }
            }
            return result;
        }
        None => return false,
    }
}

/// This function reads the config from the database.
/// It returns a vector of strings with the allowed origins regex
/// # Arguments
/// * `pool` - The database connection pool.
/// # Returns
/// * `Vec<String>` - The vector of regex strings.
pub async fn read_config(pool: &Pool) -> Result<Vec<String>, tokio_postgres::Error> {
    let conn = get_connection(pool).await.unwrap();
    let rows = conn
        .query(
            "SELECT allowed_regex FROM omini_rules WHERE active = true",
            &[],
        )
        .await?;
    let mut vec_regex = Vec::new();
    for row in rows.iter() {
        let regex: String = row.get(0);
        vec_regex.push(regex);
    }
    Ok(vec_regex)
}

/// Return the ip address of the client
/// # Arguments
/// * `req` - The request object.
/// # Returns
/// * `Option<IpAddr>` - The ip address of the client.
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

/// Check if user agent is valid using a regex, return true if valid
/// else false.
/// # Arguments
/// * `ua` : String
/// # Returns
/// * `bool` - The boolean value.
pub fn is_ua_valid(ua: &str) -> bool {
    for ban_keyword in BOT_UA_LIST.iter() {
        if ua.contains(ban_keyword) {
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

/// Filter request using the ip and useragent.
/// # Arguments
/// * `req` : &HttpRequest
/// # Return
/// `Option<IpAddr>` : ip address of the client
pub async fn request_filter(req: &HttpRequest, pool: &Pool) -> Option<IpAddr> {
    if is_ua_valid(req.headers().get("User-Agent").unwrap().to_str().unwrap())
        && valid_origin(req, pool).await
    {
        return get_ip(req);
    }
    return None;
}

// Filter traffic flood by ip
const CALLS_COUNT_QUERY: &str = r#"SELECT COUNT(*)::INT2 as count FROM omini_calls WHERE ip = $1 AND $2 - created_at < 60 LIMIT(51);"#;

/// Check if the ip is flooding the server.
/// If the ip is flooding the server, it returns false.
/// If the ip is not flooding the server, it returns true.
/// # Arguments
/// `ip` : IpAddr
/// `pool` : &Pool
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
