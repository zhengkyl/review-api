use crate::{handlers::reviews::WatchStatus, schema::*};
use diesel::associations::Associations;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct FilmReview {
    pub id: i32,
    pub tmdb_id: i32,
    pub user_id: i32,
    pub status: WatchStatus,
    pub text: String,
    pub fun_before: bool,
    pub fun_during: bool,
    pub fun_after: bool,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = film_reviews)]
pub struct NewFilmReview<'a> {
    pub tmdb_id: i32,
    pub user_id: i32,
    pub status: WatchStatus,
    pub text: &'a str,
    pub fun_before: bool,
    pub fun_during: bool,
    pub fun_after: bool,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct ShowReview {
    pub id: i32,
    pub tmdb_id: i32,
    pub user_id: i32,
    pub status: WatchStatus,
    pub text: String,
    pub fun_before: bool,
    pub fun_during: bool,
    pub fun_after: bool,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = show_reviews)]
pub struct NewShowReview<'a> {
    pub tmdb_id: i32,
    pub user_id: i32,
    pub status: WatchStatus,
    pub text: &'a str,
    pub fun_before: bool,
    pub fun_during: bool,
    pub fun_after: bool,
    pub updated_at: chrono::NaiveDateTime,
}
