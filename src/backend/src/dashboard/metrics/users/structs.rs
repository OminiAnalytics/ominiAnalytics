/*
 File: structs.rs
 Created Date: 25 Sep 2022
 Author: realbacon
 -----
 Last Modified: 25/09/2022 02:22:4
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConnectedUsers {
    pub count: i64,
}
