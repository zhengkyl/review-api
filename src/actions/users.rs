use crate::{
    errors::DbError,
    models::{NewUser, User},
    pagination::{Paginate, PaginatedResults},
    schema::users,
    utils::hash_password,
    PooledConn,
};

use diesel::{associations::HasTable, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
}

pub fn get_all_users(
    conn: &mut PooledConn,
    params: QueryParams,
) -> Result<PaginatedResults<User>, DbError> {
    use crate::schema::users::dsl::*;

    let mut query = users::table().into_boxed();

    if let Some(sort_by) = params.sort_by {
        query = match sort_by.as_ref() {
            "id.asc" => query.order(id.asc()),
            "id.desc" => query.order(id.desc()),
            "email.asc" => query.order(email.asc()),
            "email.desc" => query.order(email.desc()),
            "created_at.asc" => query.order(created_at.asc()),
            "created_at.desc" => query.order(created_at.desc()),
            "updated_at.asc" => query.order(updated_at.asc()),
            "updated_at.desc" => query.order(updated_at.desc()),
            _ => query.order(id.asc()),
        }
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

pub fn find_user_by_email(conn: &mut PooledConn, email_in: &str) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::{email, users};

    let user = users
        .filter(email.eq(email_in))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

pub fn update_user_by_id(
    conn: &mut PooledConn,
    idx: i32,
    update: UpdateUser,
) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let user = diesel::update(users.find(idx))
        .set(update)
        .get_result::<User>(conn)?;

    Ok(user)
}

pub fn delete_user_by_id(conn: &mut PooledConn, idx: i32) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let deleted = diesel::delete(users.find(idx)).execute(conn)?;

    Ok(deleted)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub fn create_user(conn: &mut PooledConn, user: InputUser) -> Result<User, DbError> {
    use crate::schema::users::dsl::users;

    let hash = &hash_password(&user.password)?;

    let new_user = NewUser {
        first_name: &user.first_name,
        last_name: &user.last_name,
        email: &user.email,
        hash,
    };

    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)?;

    Ok(user)
}
