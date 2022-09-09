/*
 File: db.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 9/09/2022 11:50:6
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/


use std::net::IpAddr;

use crate::db::get_connection;
use deadpool_postgres::Pool;
use postgres_types::Json;
use actix_web::web::Data;
use uuid::Uuid;

use super::device::Device;


async fn user_exists_ipv6(pool: &Pool, ip: IpAddr) -> Result<Option<Uuid>, String> {
    // IpV6 represents only one device
    // so we don't have to search the user based on his device
    let conn = get_connection(pool).await?;
    let result = conn
                .query(FETCH_USER_QUERY_FOR_V6, &[&ip.to_string()])
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
    dev: super::device::Device,
    _ip: String, 
) -> Result<Uuid, tokio_postgres::Error> {
    // Get a connection from the pool
    let conn = get_connection(&pool).await.unwrap();
    // Insert the user into the database
    let new_id = Uuid::new_v4();
    let country = &dev.country;
    let user_id = conn
        .query(
            INSERT_USER_QUERY,
            &[&new_id,&_ip,&Json::<Device>(dev.clone()),&country],
        ).await;

    Ok(new_id)
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
pub async fn user_exists(device: &super::device::Device, ip: IpAddr, pool: &Pool) -> Option<Uuid> {
    // Based on each field of the device
    // and their weight, we calculate the most likely user
    // and return it
    match ip {
        IpAddr::V6(_) => {
            let user = user_exists_ipv6(pool, ip).await;
            return user.unwrap();
        }
        IpAddr::V4(_) => {
        }
    };
    None
    //Some(Uuid::new_v4())
}

const INSERT_USER_QUERY: &str = "INSERT INTO omini_users (id, ip, device_info, country) VALUES ($1, $2, $3, $4)";

const FETCH_USER_QUERY_FOR_V6: &str = "SELECT
      CASE WHEN EXISTS 
      (
           SELECT id FROM omini_users WHERE ip = $1  GROUP BY id
      )
      THEN CAST(id as VARCHAR)
      ELSE 'false'
   END as exists
   FROM omini_users;";

   /*OsType VARCHAR(10) NOT NULL,
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
        UserAgent VARCHAR(200) NOT NULL */
const CREATE_DEVICE_INDEX : &str = "INSERT INTO device_info_index (os_type, os_version, os_name, os_arch,\
   browser_name, browser_version, browser_fversion, country, cpu, gpu, memory, screen_height, screen_width,\
   screen_color, color_buffer, user_agent) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)";
