use crate::actions::users::{
    create_user, delete_user_by_id, find_user_by_id, get_all_users, update_auth_user_by_id,
    InputUser, QueryParams, UpdateUser,
};
use crate::handlers::auth::UserId;

use crate::errors::ServiceError;
use crate::Pool;

use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde_json::json;

#[get("")]
pub async fn get_users(
    pool: web::Data<Pool>,
    query: web::Query<QueryParams>,
) -> Result<HttpResponse, ServiceError> {
    let users = web::block(move || {
        let mut conn = pool.get()?;
        get_all_users(&mut conn, query.into_inner())
    })
    .await??;

    Ok(HttpResponse::Ok().json(users))
}

#[get("{id}")]
pub async fn get_users_id(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        find_user_by_id(&mut conn, id.into_inner())
    })
    .await??;

    let Some(user) = user else {
        return Err(ServiceError::pls(404));
    };

    Ok(HttpResponse::Ok().json(user))
}

#[patch("{id}")]
pub async fn patch_users_id(
    pool: web::Data<Pool>,
    path_id: web::Path<i32>,
    user_id: UserId,
    update: web::Json<UpdateUser>,
) -> Result<HttpResponse, ServiceError> {
    // Temp until implement privileged users
    let user_id = i32::from(user_id);
    let path_id = path_id.into_inner();

    if user_id != path_id && user_id != 1 {
        return Err(ServiceError::pls(401));
    }

    let user = web::block(move || {
        let mut conn = pool.get()?;
        update_auth_user_by_id(&mut conn, path_id, update.into_inner())
    })
    .await??;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("{id}")]
pub async fn delete_users_id(
    pool: web::Data<Pool>,
    path_id: web::Path<i32>,
    user_id: UserId,
) -> Result<HttpResponse, ServiceError> {
    // Temp until implement privileged users
    let user_id = i32::from(user_id);
    let path_id = path_id.into_inner();

    if user_id != path_id && user_id != 1 {
        return Err(ServiceError::pls(401));
    }

    let deleted = web::block(move || {
        let mut conn = pool.get()?;
        delete_user_by_id(&mut conn, path_id)
    })
    .await??;

    // TODO deleting users doesn't remove session data or associated reviews
    // rn auth only protect user's routes, but needs to be addressed

    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted })))
}

#[post("")]
pub async fn post_users(
    pool: web::Data<Pool>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, ServiceError> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        create_user(&mut conn, user.into_inner())
    })
    .await??;

    Ok(HttpResponse::Ok().json(user))
}
