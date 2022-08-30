/*
 File: constants.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 30/08/2022 09:49:49
 Modified By: realbacon
 -----
 Copyright (c) 2022 Omini
 -----
*/


pub const KILL_SESSION_DELAY: u8 = 30; // 30 seconds after last alive signal -> kill session
pub const SIGNAL_TYPE_ALIVE: &str = "alive";
pub const SIGNAL_TYPE_SESSION: &str = "session";
