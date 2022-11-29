use std::future::{self, Ready};

use crate::actions::users::find_user_by_email;

use crate::{errors::ServiceError, utils::verify_password, Pool};
use actix_identity::Identity;

use actix_web::{
    delete, get, post, web, FromRequest, HttpMessage, HttpRequest, HttpResponse, Responder,
};
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
    type Error = ServiceError;
    type Future = Ready<Result<UserId, ServiceError>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
            let user_id = identity.id().unwrap().parse::<i32>().unwrap();

            return future::ready(Ok(UserId(user_id)));
        }

        future::ready(Err(ServiceError::pls(401)))
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
) -> Result<HttpResponse, ServiceError> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        let potential = find_user_by_email(&mut conn, &auth_data.email)?;

        let Some(user) = potential else {
            return Err(ServiceError::pls(404));
        };
        let verified = verify_password(&auth_data.password, &user.hash)?;

        if !verified {
            return Err(ServiceError::pls(401));
        }

        Ok(user)
    })
    .await??;

    Identity::login(&request.extensions(), user.id.to_string()).unwrap();

    Ok(HttpResponse::Ok().finish())
}

#[get("")]
pub async fn me(user_id: UserId) -> impl Responder {
    HttpResponse::Ok().json(user_id)
}
