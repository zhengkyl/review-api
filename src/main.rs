#[macro_use]
extern crate diesel;

extern crate argon2;

use ::r2d2::PooledConnection;
use actix_web::middleware;
use actix_web::{cookie::Key, get, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde_json::json;
use std::time::Duration;

use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
mod actions;
mod constants;
mod errors;
mod handlers;
mod models;
mod pagination;
mod schema;
mod utils;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

use handlers::{auth, reviews, search, users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL is missing");

    openssl_probe::init_ssl_cert_env_vars();

    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build pool");

    let store = RedisSessionStore::new(redis_url)
        .await
        .expect("Failed to connect to redis");

    let session_secret = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(
                IdentityMiddleware::builder()
                    // Logout after 1 day
                    .login_deadline(Some(Duration::new(86400, 0)))
                    .build(),
            )
            .wrap(SessionMiddleware::new(
                store.clone(),
                session_secret.clone(),
            ))
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(Utc::now()))
            .service(health)
            .service(
                web::scope("/auth")
                    .service(auth::login)
                    .service(auth::logout)
                    .service(auth::me),
            )
            .service(
                web::scope("/users")
                    .service(users::get_users)
                    .service(users::get_users_id)
                    .service(users::delete_users_id)
                    .service(users::patch_users_id)
                    .service(users::post_users),
            )
            .service(
                web::scope("/search")
                    .service(search::search_movies)
                    .service(search::search_shows),
            )
            .service(
                web::scope("/reviews")
                    .service(reviews::get_reviews)
                    .service(reviews::post_reviews)
                    .service(
                        web::resource(["/{category}/{id}/{season}", "/{category}/{id}"])
                            .route(web::patch().to(reviews::patch_reviews))
                            .route(web::delete().to(reviews::delete_reviews)),
                    ),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn health(birth: web::Data<DateTime<Utc>>) -> impl Responder {
    // TODO perhaps make this useful
    HttpResponse::Ok().json(json!({
        "last_deploy": birth.into_inner().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
    }))
}
