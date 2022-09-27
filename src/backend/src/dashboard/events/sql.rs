/*
 File: sql.rs
 Created Date: 27 Sep 2022
 Author: realbacon
 -----
 Last Modified: 27/09/2022 05:58:50
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

pub const CREATE_EVENT: &str = "INSERT INTO omini_events (creator_id, compaign_id, event_id, slug, descr,active) VALUES ($1, $2, $3, $4, $5,$6) RETURNING event_id";
