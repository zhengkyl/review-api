// @generated automatically by Diesel CLI.

diesel::table! {
    film_reviews (id) {
        id -> Int4,
        tmdb_id -> Int4,
        user_id -> Int4,
        status -> Text,
        text -> Text,
        fun_before -> Bool,
        fun_during -> Bool,
        fun_after -> Bool,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    show_reviews (id) {
        id -> Int4,
        tmdb_id -> Int4,
        user_id -> Int4,
        status -> Text,
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
