table! {
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
    omini_users (id) {
        id -> Uuid,
        created_at -> Int8,
        updated_at -> Int8,
        device_info -> Jsonb,
    }
}

joinable!(omini_alive_messages -> omini_users (u_id));

allow_tables_to_appear_in_same_query!(
    omini_alive_messages,
    omini_users,
);
