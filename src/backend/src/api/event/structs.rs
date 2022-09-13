/*
 File: structs.rs
 Created Date: 13 Sep 2022
 Author: realbacon
 -----
 Last Modified: 13/09/2022 09:30:2
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ValidAliveMessage {
    pub uid: String,
}
