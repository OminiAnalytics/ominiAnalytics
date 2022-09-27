/*
 File: structs.rs
 Created Date: 27 Sep 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 05:53:16
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Event {
    #[serde(default = "Uuid::new_v4")]
    pub creator_id: Uuid,
    pub compaign_id: Uuid,
    pub event_id: String,
    pub slug: String,
    pub descr: String,
    pub active: bool,
}

#[derive(Deserialize, Serialize)]
pub struct EventResponse {
    pub status: String,
    pub msg: String,
}
