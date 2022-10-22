#[macro_use]
extern crate diesel;

extern crate argon2;

use ::r2d2::PooledConnection;
use actix_web::{cookie::Key, get, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};

mod constants;
mod errors;
mod handlers;
mod models;
mod schema;
mod utils;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

use handlers::users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");

    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build pool");

    let session_secret = Key::generate();
    // let session_secret =
    //     std::env::var("SESSION_SECRET_KEY").expect("SESSION_SECRET_KEY is missing");

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                session_secret.clone(),
            ))
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(
                web::scope("/users")
                    .service(users::get_users)
                    .service(users::get_users_id)
                    .service(users::delete_users_id)
                    .service(users::put_users_id)
                    .service(users::post_users),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
