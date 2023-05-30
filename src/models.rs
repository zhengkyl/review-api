use crate::schema::*;
use diesel::associations::Associations;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing)]
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Queryable)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(primary_key(user_id, tmdb_id, category))]
#[diesel(belongs_to(User))]
pub struct Review {
    pub user_id: i32,
    pub tmdb_id: i32,
    pub category: MediaCategory,
    pub status: WatchStatus,
    pub text: String,
    pub fun_before: bool,
    pub fun_during: bool,
    pub fun_after: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "invalid_season")]
    pub season: i32,
}

fn invalid_season(season: &i32) -> bool {
    return *season < 0;
}

#[derive(Debug, Insertable)]
#[diesel(table_name = reviews)]
pub struct NewReview<'a> {
    pub user_id: i32,
    pub tmdb_id: i32,
    pub category: MediaCategory,
    pub season: Option<i32>,
    pub status: WatchStatus,
    pub text: &'a str,
    pub fun_before: bool,
    pub fun_during: bool,
    pub fun_after: bool,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = reviews)]
pub struct EditReview {
    status: Option<WatchStatus>,
    text: Option<String>,
    fun_before: Option<bool>,
    fun_during: Option<bool>,
    fun_after: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, DbEnum, Eq, PartialEq, Hash)]
#[DieselTypePath = "crate::schema::sql_types::WatchStatus"]
#[DbValueStyle = "PascalCase"]
pub enum WatchStatus {
    Completed,
    Dropped,
    Watching,
    PlanToWatch,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, DbEnum, Eq, PartialEq, Hash)]
#[DieselTypePath = "crate::schema::sql_types::MediaCategory"]
#[DbValueStyle = "PascalCase"]
pub enum MediaCategory {
    Film,
    Show,
}

impl TryFrom<String> for MediaCategory {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Film" => Ok(MediaCategory::Film),
            "Show" => Ok(MediaCategory::Show),
            _ => Err("Unrecognized MediaCategory"),
        }
    }
}
// TODO
// pub enum ApiPermissions {}

// pub struct ApiKey {
//     pub key: String,
//     pub user_id: i32,
//     pub permissions: Vec<ApiPermissions>,
// }
