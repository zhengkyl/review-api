use std::future::{self, Ready};

use crate::constants::CONNECTION_POOL_ERROR;
use crate::diesel::ExpressionMethods;
use crate::{errors::ServiceError, models::User, utils::verify_password, Pool, PooledConn};
use actix_identity::Identity;
use actix_web::{
    delete, get, post, web, FromRequest, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub struct UserId(i32);

impl From<i32> for UserId {
    fn from(n: i32) -> Self {
        UserId(n)
    }
}
impl From<UserId> for i32 {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl FromRequest for UserId {
    type Error = actix_web::Error;
    type Future = Ready<Result<UserId, actix_web::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        // make sure to check cookie is parsed correctly
        // don't accidently add stuff after the cookie like i did
        // dbg!(req.cookie("id"));
        if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
            let user_id = identity.id().unwrap().parse::<i32>().unwrap();

            return future::ready(Ok(UserId(user_id)));
        }

        future::ready(Err(ServiceError::Unauthorized.into()))
    }
}

#[delete("")]
pub async fn logout(id: Identity) -> impl Responder {
    id.logout();
    HttpResponse::Ok()
}

#[post("")]
pub async fn login(
    request: HttpRequest,
    auth_data: web::Json<AuthData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let user = web::block(move || query_user(auth_data.into_inner(), &mut conn)).await??;

    // let user = serde_json::to_string(&user)?;

    let t = Identity::login(&request.extensions(), user.id.to_string()).unwrap();

    Ok(HttpResponse::Ok().finish())
}

#[get("")]
pub async fn me(user_id: UserId) -> impl Responder {
    HttpResponse::Ok().json(user_id)
}

fn query_user(auth_data: AuthData, conn: &mut PooledConn) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::{email, users};

    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(verified) = verify_password(&auth_data.password, &user.hash) {
            if verified {
                return Ok(user);
            }
        }
    }

    Err(ServiceError::Unauthorized)
}
