use crate::{
    errors::{DbError, ServiceError},
    models::{AuthenticatedUser, NewUser, User},
    pagination::{Paginate, PaginatedResults},
    schema::users,
    utils::hash_password,
    PooledConn,
};

use diesel::{associations::HasTable, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum SortBy {
    #[serde(rename = "id.asc")]
    IdAsc,
    #[serde(rename = "id.desc")]
    IdDesc,
    #[serde(rename = "name.asc")]
    NameAsc,
    #[serde(rename = "name.desc")]
    NameDesc,
    #[serde(rename = "created_at.asc")]
    CreatedAtAsc,
    #[serde(rename = "created_at.desc")]
    CreatedAtDesc,
    #[serde(rename = "updated_at.asc")]
    UpdatedAtAsc,
    #[serde(rename = "updated_at.desc")]
    UpdatedAtDesc,
}
#[derive(Deserialize)]
pub struct QueryParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<SortBy>,
}

pub fn get_all_users(
    conn: &mut PooledConn,
    params: QueryParams,
) -> Result<PaginatedResults<User>, DbError> {
    use crate::schema::users::dsl::*;

    let mut query = users::table().into_boxed();

    if let Some(sort_by) = params.sort_by {
        query = match sort_by {
            SortBy::IdAsc => query.order(id.asc()),
            SortBy::IdDesc => query.order(id.desc()),
            SortBy::NameAsc => query.order(name.asc()),
            SortBy::NameDesc => query.order(name.desc()),
            SortBy::CreatedAtAsc => query.order(created_at.asc()),
            SortBy::CreatedAtDesc => query.order(created_at.desc()),
            SortBy::UpdatedAtAsc => query.order(updated_at.asc()),
            SortBy::UpdatedAtDesc => query.order(updated_at.desc()),
        }
    } else {
        query = query.order(id.asc())
    }

    let results = query
        .paginate_safe(params.page, params.per_page)
        .load_paginated(conn)?;

    Ok(results)
}

pub fn find_user_by_id(conn: &mut PooledConn, idx: i32) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.find(idx).first(conn).optional()?;

    Ok(user)
}

pub fn find_auth_user_by_id(
    conn: &mut PooledConn,
    idx: i32,
) -> Result<Option<AuthenticatedUser>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.find(idx).first(conn).optional()?;

    Ok(user)
}

pub fn find_auth_user_by_email(
    conn: &mut PooledConn,
    email_in: &str,
) -> Result<Option<AuthenticatedUser>, DbError> {
    use crate::schema::users::dsl::{email, users};

    let user = users
        .filter(email.eq(email_in))
        .first::<AuthenticatedUser>(conn)
        .optional()?;

    Ok(user)
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

pub fn update_auth_user_by_id(
    conn: &mut PooledConn,
    idx: i32,
    update: UpdateUser,
) -> Result<AuthenticatedUser, DbError> {
    use crate::schema::users::dsl::*;

    let user = diesel::update(users.find(idx))
        .set(update)
        .get_result::<AuthenticatedUser>(conn)?;

    Ok(user)
}

pub fn delete_user_by_id(conn: &mut PooledConn, idx: i32) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let deleted = diesel::delete(users.find(idx)).execute(conn)?;

    Ok(deleted)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn create_user(conn: &mut PooledConn, user: InputUser) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::users;

    let hash = &hash_password(&user.password)?;

    let new_user = NewUser {
        name: &user.name,
        email: &user.email,
        hash,
    };

    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)?;

    Ok(user)
}
