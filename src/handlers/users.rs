use diesel::QueryDsl;
use serde::{Deserialize, Serialize};

use crate::diesel::RunQueryDsl;
use crate::handlers::auth::UserId;
use crate::PooledConn;
use crate::{constants::CONNECTION_POOL_ERROR, utils::hash_password};

use crate::errors::ServiceError;
use crate::Pool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};

use crate::schema::*;

use std::vec::Vec;

use crate::models::{NewUser, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[get("")]
pub async fn get_users(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let users = web::block(move || get_all_users(&mut conn)).await??;

    Ok(HttpResponse::Ok().json(users))
}

#[get("{id}")]
pub async fn get_users_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let users = web::block(move || get_user_by_id(&mut conn, id.into_inner())).await??;

    Ok(HttpResponse::Ok().json(users))
}

#[put("{id}")]
pub async fn put_users_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    user_id: UserId,
    update: web::Json<UpdateUser>,
) -> Result<HttpResponse, ServiceError> {
    // Temp until implement privileged users
    let permission_id = i32::from(user_id);
    let action_id = id.into_inner();
    if permission_id != action_id && permission_id != 1 {
        return Err(ServiceError::Unauthorized);
    }

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let user =
        web::block(move || update_user_by_id(&mut conn, action_id, update.into_inner())).await??;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("{id}")]
pub async fn delete_users_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    user_id: UserId,
) -> Result<HttpResponse, ServiceError> {
    // Temp until implement privileged users
    let permission_id = i32::from(user_id);
    let action_id = id.into_inner();
    if permission_id != action_id && permission_id != 1 {
        return Err(ServiceError::Unauthorized);
    }

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let _ = web::block(move || delete_user_by_id(&mut conn, action_id)).await??;

    // TODO deleting users doesn't remove session data or associated reviews
    // rn auth only protect user's routes, but needs to be addressed

    Ok(HttpResponse::NoContent()
        .content_type("application/json")
        .await
        .unwrap())
}

#[post("")]
pub async fn post_users(
    pool: web::Data<Pool>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let user = web::block(move || create_user(&mut conn, user)).await??;

    Ok(HttpResponse::Ok().json(user))
}

fn get_all_users(conn: &mut PooledConn) -> Result<Vec<User>, ServiceError> {
    use crate::schema::users::dsl::*;

    users
        .load::<User>(conn)
        .map_err(|e| ServiceError::BadRequest(format!("uh oh {}", e)))
        .and_then(|result| Ok(result))
}

fn get_user_by_id(conn: &mut PooledConn, idx: i32) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::*;
    users
        .find(idx)
        .get_result(conn)
        .map_err(|_| ServiceError::BadRequest("User not found".into()))
        .and_then(|result| Ok(result))
}

fn delete_user_by_id(conn: &mut PooledConn, idx: i32) -> Result<usize, ServiceError> {
    use crate::schema::users::dsl::*;
    diesel::delete(users.find(idx))
        .execute(conn)
        .map_err(|_| ServiceError::InternalServerError)
        .and_then(|result| Ok(result))
}

fn create_user(conn: &mut PooledConn, user: web::Json<InputUser>) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::users;

    let hash = &hash_password(&user.password)?;

    let new_user = NewUser {
        first_name: &user.first_name,
        last_name: &user.last_name,
        email: &user.email,
        hash,
        created_at: chrono::Local::now().naive_local(),
    };

    let res = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn);

    res.map_err(|_| ServiceError::InternalServerError)
        .and_then(|result| Ok(result))
}

fn update_user_by_id(
    conn: &mut PooledConn,
    idx: i32,
    update: UpdateUser,
) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::*;

    let res = diesel::update(users.find(idx))
        .set(update)
        .get_result::<User>(conn);

    res.map_err(|_| ServiceError::InternalServerError)
        .and_then(|result| Ok(result))
}
