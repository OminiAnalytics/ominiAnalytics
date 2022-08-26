/*
 * @Author: realbacon
 * @Date: 2022-08-26 18:29:24
 * @Last Modified by: realbacon
 * @Last Modified time: 2022-08-26 23:35:08
 */

// Diesel stuff
extern crate diesel;
use crate::DBPooledConnection;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};

use uuid::Uuid;

// Data structure
use crate::models::GetUser;

pub fn is_uid_valid(conn: &DBPooledConnection, uid: Uuid, device: serde_json::Value) -> bool {
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
