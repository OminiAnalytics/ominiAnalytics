/*
 File: structs.rs
 Created Date: 25 Sep 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 03:41:59
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CountStruct {
    pub count: i64,
}
