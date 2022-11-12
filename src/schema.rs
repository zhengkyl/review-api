// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "media_category"))]
    pub struct MediaCategory;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "watch_status"))]
    pub struct WatchStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MediaCategory;
    use super::sql_types::WatchStatus;

    reviews (user_id, tmdb_id, category) {
        user_id -> Int4,
        tmdb_id -> Int4,
        category -> MediaCategory,
        status -> WatchStatus,
        text -> Text,
        fun_before -> Bool,
        fun_during -> Bool,
        fun_after -> Bool,
        updated_at -> Timestamp,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    reviews,
    users,
);
