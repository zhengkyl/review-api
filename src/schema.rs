// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        hash -> Text,
        created_at -> Timestamp,
    }
}
