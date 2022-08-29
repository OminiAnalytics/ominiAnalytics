/*
 * @Author: realbacon
 * @Date: 2022-08-27 21:07:39
 * @Last Modified by:   realbacon
 * @Last Modified time: 2022-08-27 21:07:39
 */

// Diesel stuff
extern crate diesel;
use crate::DBPooledConnection;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use uuid::Uuid;

// Data structure
use crate::models::{GetUser, NewUser};

pub fn is_uid_valid(conn: &DBPooledConnection, uid: Uuid, device: &serde_json::Value) -> bool {
    use crate::schema::omini_users::dsl::*;
    let result = omini_users
        .filter(id.eq(uid))
        .filter(device_info.eq(&device))
        .load::<GetUser>(conn)
        .expect("Error loading users");
    if result.len() == 1 {
        return true;
    } else {
        return false;
    }
}

pub fn insert_user(
    conn: &DBPooledConnection,
    dev: serde_json::Value, // device_info
    cou: &String,           // country
) -> Result<(Uuid, i64, i64, serde_json::Value, String), diesel::result::Error> {
    use crate::schema::omini_users::dsl::*;
    let uid = Uuid::new_v4();
    let new_user = NewUser {
        id: uid,
        device_info: dev,
        country: cou.to_string(),
    };
    diesel::insert_into(omini_users)
        .values(&new_user)
        .get_result(conn)
}
