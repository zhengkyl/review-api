use serde::{Deserialize, Serialize};

use crate::constants::CONNECTION_POOL_ERROR;
use crate::diesel::RunQueryDsl;
use crate::PooledConn;

use crate::errors::ServiceError;
use crate::Pool;

use actix_web::{get, web, Error, HttpResponse};

use std::vec::Vec;

use crate::models::{NewUser, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
//     Ok(web::block(|| get_all_users(db))
//         .await
//         .map(|user| HttpResponse::Ok().json(user))
//         .map_err(|_| HttpResponse::InternalServerError())?)
// }

// fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
//     let conn = pool.get().unwrap();
//     let items = users.load::<User>(&conn)?;
//     Ok(items)
// }

#[get("/")]
pub async fn get_users(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let users = web::block(move || get_all_users(&mut conn)).await??;

    Ok(HttpResponse::Ok().json(users))

    // Ok(users
    //     .map(|u| HttpResponse::Ok().json(u))
    //     .map_err(|e| HttpResponse::InsufficientStorage().json(e))?)

    // match users {
    //     Ok(user) => Ok(HttpResponse::Ok().json(user)),
    //     _ => Ok(HttpResponse::InternalServerError().json("test")),
    // }

    // Ok(web::block(move || get_all_users(pool))
    //     .await
    //     .map(|user| HttpResponse::Ok().json(user))
    //     .map_err(|_| HttpResponse::InternalServerError().json("test")))
}

fn get_all_users(conn: &mut PooledConn) -> Result<Vec<User>, ServiceError> {
    use crate::schema::users::dsl::*;

    users
        .load::<User>(conn)
        .map_err(|e| ServiceError::BadRequest(format!("uh oh {}", e)))
        .and_then(|result| Ok(result))
}
