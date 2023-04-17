// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}
