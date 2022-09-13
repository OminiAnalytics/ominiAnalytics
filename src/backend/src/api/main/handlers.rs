/*
 File: db.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 13/09/2022 08:34:6
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use super::sql::*;
use crate::db::get_connection;
use actix_web::web::Data;
use deadpool_postgres::Pool;
use postgres_types::Json;
use std::net::IpAddr;
use uuid::Uuid;

use super::structs::Device;

/// Handler for ipv6 devices
async fn user_exists_ipv6(pool: &Pool, ip: IpAddr) -> Result<Option<Uuid>, tokio_postgres::Error> {
    // IpV6 represents only one device
    // so we don't have to search the user based on his device
    let conn = get_connection(pool)
        .await
        .expect("Error getting connection");
    let result = conn
        .query_one(FETCH_USER_QUERY_FOR_V6, &[&ip.to_string()])
        .await?;
    let exists: String = result.get("exists");
    if exists != "false" {
        return Ok(Some(Uuid::parse_str(&exists[..]).unwrap()));
    } else {
        return Ok(None);
    }
}

/// Insert a new user into the database
///
/// Parse the device keys and insert them into device_info_index
/// # Arguments
/// * `pool` - The database pool
/// * `ip` - The ip of the user
/// * `device` - The device of the user
/// * `country` - The country of the user
pub async fn insert_user(
    pool: Data<Pool>,
    device: Device,
    _ip: String,
) -> Result<Option<Uuid>, tokio_postgres::Error> {
    // Get a connection from the pool
    let conn = get_connection(&pool).await.unwrap();
    // Insert the user into the database
    let new_id = Uuid::new_v4();
    let _insert_query = conn
        .query_opt(
            INSERT_USER_QUERY,
            &[&new_id, &_ip, &Json(&device), &device.country],
        )
        .await?;
    // Insert the device keys into the device_info_index
    // Check first that we have 3 ints in the device hardware screen Vector
    if device.hardware.screen.len() != 3 {
        return Ok(None);
    };

    let _device_info_index_insert_query = conn
        .query_opt(
            INSERT_DEVICE_INDEX_QUERY,
            &[
                &device.os.typ,
                &device.os.version,
                &device.os.name,
                &device.os.arch,
                &device.browser.name,
                &device.browser.version,
                &device.browser.fversion,
                &device.country,
                &device.hardware.cpu.to_string(),
                &device.hardware.gpu,
                &device.hardware.memory.to_string(),
                &device.hardware.screen[0].to_string(),
                &device.hardware.screen[1].to_string(),
                &device.hardware.screen[2].to_string(),
                &device.hardware.color_buffer_float.to_string(),
                &device.useragent,
                &new_id,
            ],
        )
        .await?;
    Ok(Some(new_id))
}

/// Handler for ipv4 devices
pub async fn user_exists_ipv4(
    pool: &Pool,
    ip: IpAddr,
    device: &Device,
) -> Result<Option<Uuid>, String> {
    // Get a connection from the pool
    let conn = get_connection(pool).await?;
    let country = &device.country[..];
    // Get the user id from the database
    let result = conn
        .query(
            FETCH_USER_QUERY_FOR_V4,
            &[&ip.to_string(), &Json(device), &country.to_string()],
        )
        .await
        .unwrap()
        .into_iter()
        .next()
        .unwrap(); // Get first row
    let exists: String = result.get("exists");
    if exists != "false" {
        return Ok(Some(Uuid::parse_str(&exists[..]).unwrap()));
    } else {
        return Ok(None);
    }
}
/// Algorithm that select most likely user based on device info and ip address.
///
/// If the most likely user is under treshold, it returns None.
///
/// If the most likely user is over treshold, it returns the most likely user.
/// # Arguments
/// * `pool` - A deadpool postgres pool
/// * `device_info` - A Device struct containing device info
/// * `ip` - An IpAddr containing the ip address
/// # Returns
/// * `Option<Uuid>` - An Option containing the user id or None if no user was found
/// # Example
/// ```
/// let user_that_exists = user_exists_algo(&pool, &device_info, &ip).await;
/// Some("311cf909-fd46-472b-84a3-d395e738ba25")
/// let  user_that_doesnt_exist = user_exists_algo(&pool, &device_info, &ip).await;
/// None
/// ```
pub async fn user_exists(device: &Device, ip: IpAddr, pool: &Pool) -> Option<Uuid> {
    // Based on each field of the device
    // and their weight, we calculate the most likely user
    // and return it
    match ip {
        IpAddr::V6(_) => {
            let user = user_exists_ipv6(pool, ip).await;
            return user.unwrap();
        }
        IpAddr::V4(_) => {
            let user = user_exists_ipv4(pool, ip, &device).await;
            return user.unwrap();
        }
    };
}
