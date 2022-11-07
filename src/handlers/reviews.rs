use actix_web::{get, post, put, web, HttpResponse};
use diesel::QueryDsl;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::{
    constants::CONNECTION_POOL_ERROR,
    diesel::{ExpressionMethods, RunQueryDsl},
    errors::ServiceError,
    handlers::auth::UserId,
    models::{FilmReview, NewFilmReview},
    Pool, PooledConn,
};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::WatchStatus"]
#[DbValueStyle = "PascalCase"]
pub enum WatchStatus {
    Completed,
    Dropped,
    Watching,
    PlanToWatch,
}

#[derive(Deserialize, Debug)]
pub struct InputReview {
    tmdb_id: i32,
    status: WatchStatus,
}

#[derive(Deserialize)]
pub struct EditReview {
    tmdb_id: i32,
    status: Option<WatchStatus>,
    text: Option<String>,
    fun_before: Option<bool>,
    fun_during: Option<bool>,
    fun_after: Option<bool>,
}

#[get("/films")]
pub async fn get_film_reviews(
    pool: web::Data<Pool>,
    user_id: UserId,
) -> Result<HttpResponse, ServiceError> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let reviews = web::block(move || get_all_film_reviews(&mut conn, user_id)).await??;

    Ok(HttpResponse::Ok().json(reviews))
}

#[post("/films")]
pub async fn review_film(
    pool: web::Data<Pool>,
    user_id: UserId,
    item: web::Json<InputReview>,
) -> Result<HttpResponse, ServiceError> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let new_film_review = NewFilmReview {
        tmdb_id: item.tmdb_id,
        status: item.status,
        user_id: i32::from(user_id),
        text: "",
        fun_before: false,
        fun_during: false,
        fun_after: false,
        updated_at: chrono::Local::now().naive_local(),
    };
    let review = web::block(move || create_film_review(&mut conn, new_film_review)).await??;
    Ok(HttpResponse::Ok().json(review))
}

fn get_all_film_reviews(
    conn: &mut PooledConn,
    req_id: UserId,
) -> Result<Vec<FilmReview>, ServiceError> {
    use crate::schema::film_reviews::dsl::*;

    film_reviews
        .filter(user_id.eq(i32::from(req_id)))
        .load::<FilmReview>(conn)
        .map_err(|_| ServiceError::InternalServerError)
        .and_then(|result| Ok(result))
}

fn create_film_review(
    conn: &mut PooledConn,
    review: NewFilmReview,
) -> Result<FilmReview, ServiceError> {
    use crate::schema::film_reviews::dsl::*;

    let res = diesel::insert_into(film_reviews)
        .values(review)
        .get_result::<FilmReview>(conn);

    res.map_err(|e| ServiceError::BadRequest(format!("{}", e)))
        .and_then(|result| Ok(result))
}

// #[put("/films")]
// pub async fn review_film(
//     user: Option<Identity>,
//     item: web::Json<EditReview>,
// ) -> Result<HttpResponse, ServiceError> {
//     Ok(HttpResponse::Ok().finish())
// }

// #[put("/shows")]
// pub async fn review_show(
//     user: Option<Identity>,
//     item: web::Json<InputReview>,
// ) -> Result<HttpResponse, ServiceError> {
//     Ok(HttpResponse::Ok().finish())
// }
