/*
 File: sql.rs
 Created Date: 24 Sep 2022
 Author: realbacon
 -----
 Last Modified: 24/09/2022 01:23:44
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

pub const GET_USER: &str = "SELECT id FROM omini_managers WHERE name=$1 AND password = encode(digest($2, 'sha256'),'hex');";
