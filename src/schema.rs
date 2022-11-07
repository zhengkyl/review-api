// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "watch_status"))]
    pub struct WatchStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WatchStatus;

    film_reviews (id) {
        id -> Int4,
        tmdb_id -> Int4,
        user_id -> Int4,
        status -> WatchStatus,
        text -> Text,
        fun_before -> Bool,
        fun_during -> Bool,
        fun_after -> Bool,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WatchStatus;

    show_reviews (id) {
        id -> Int4,
        tmdb_id -> Int4,
        user_id -> Int4,
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
    film_reviews,
    show_reviews,
    users,
);
