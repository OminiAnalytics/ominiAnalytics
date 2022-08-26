table! {
    omini_alive_messages (id) {
        id -> Uuid,
        u_id -> Uuid,
        created_at -> Float8,
        mtyp -> Varchar,
    }
}

table! {
    omini_users (id) {
        id -> Uuid,
        created_at -> Float8,
        updated_at -> Float8,
        device_info -> Jsonb,
        lang -> Varchar,
        os -> Varchar,
    }
}

joinable!(omini_alive_messages -> omini_users (u_id));

allow_tables_to_appear_in_same_query!(
    omini_alive_messages,
    omini_users,
);
