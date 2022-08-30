/*
 File: mod.rs
 Created Date: 30 Aug 2022
 Author: realbacon
 -----
 Last Modified: 30/08/2022 09:57:5
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/


table! {
    use diesel::sql_types::*;
    omini_alive_messages (id) {
        id -> Uuid,
        u_id -> Uuid,
        s_id -> Uuid,
        created_at -> Int8,
        mtype -> Varchar,
        s_duration -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    omini_users (id) {
        id -> Uuid,
        created_at -> Int8,
        updated_at -> Int8,
        device_info -> Jsonb,
        country -> Varchar,
        ip -> Varchar,
    }
}

joinable!(omini_alive_messages -> omini_users (u_id));

allow_tables_to_appear_in_same_query!(omini_alive_messages, omini_users,);
