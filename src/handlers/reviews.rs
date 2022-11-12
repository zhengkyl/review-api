use actix_web::{get, post, put, web, HttpResponse};
use diesel::QueryDsl;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::{
    constants::CONNECTION_POOL_ERROR,
    diesel::{ExpressionMethods, RunQueryDsl},
    errors::ServiceError,
    handlers::auth::UserId,
    models::{NewReview, Review},
    schema::reviews,
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::MediaCategory"]
#[DbValueStyle = "PascalCase"]
pub enum MediaCategory {
    Film,
    Show,
}

#[derive(Deserialize, Debug)]
pub struct InputReview {
    tmdb_id: i32,
    category: MediaCategory,
    status: WatchStatus,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = reviews)]
pub struct EditReview {
    status: Option<WatchStatus>,
    text: Option<String>,
    fun_before: Option<bool>,
    fun_during: Option<bool>,
    fun_after: Option<bool>,
}

#[get("")]
pub async fn get_reviews(
    pool: web::Data<Pool>,
    user_id: UserId,
) -> Result<HttpResponse, ServiceError> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let reviews = web::block(move || get_all_reviews(&mut conn, user_id)).await??;

    Ok(HttpResponse::Ok().json(reviews))
}

#[post("")]
pub async fn post_reviews(
    pool: web::Data<Pool>,
    user_id: UserId,
    item: web::Json<InputReview>,
) -> Result<HttpResponse, ServiceError> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let new_film_review = NewReview {
        category: item.category,
        tmdb_id: item.tmdb_id,
        status: item.status,
        user_id: i32::from(user_id),
        text: "",
        fun_before: false,
        fun_during: false,
        fun_after: false,
        updated_at: chrono::Local::now().naive_local(),
    };
    let review = web::block(move || create_review(&mut conn, new_film_review)).await??;
    Ok(HttpResponse::Ok().json(review))
}

#[put("/{id}")]
pub async fn put_reviews_id(
    pool: web::Data<Pool>,
    user_id: UserId,
    id: web::Path<i32>,
    item: web::Json<EditReview>,
) -> Result<HttpResponse, ServiceError> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let review =
        web::block(move || edit_review(&mut conn, user_id, id.into_inner(), item.into_inner()))
            .await??;
    Ok(HttpResponse::Ok().json(review))
}

fn get_all_reviews(conn: &mut PooledConn, req_id: UserId) -> Result<Vec<Review>, ServiceError> {
    use crate::schema::reviews::dsl::*;

    reviews
        .filter(user_id.eq(i32::from(req_id)))
        .load::<Review>(conn)
        .map_err(|_| ServiceError::InternalServerError)
        .and_then(|result| Ok(result))
}

fn create_review(conn: &mut PooledConn, review: NewReview) -> Result<Review, ServiceError> {
    use crate::schema::reviews::dsl::*;

    let res = diesel::insert_into(reviews)
        .values(review)
        .get_result::<Review>(conn);

    res.map_err(|e| ServiceError::BadRequest(format!("{}", e)))
        .and_then(|result| Ok(result))
}

fn edit_review(
    conn: &mut PooledConn,
    req_id: UserId,
    idx: i32,
    edits: EditReview,
) -> Result<Review, ServiceError> {
    use crate::schema::reviews::dsl::*;
    let res = diesel::update(
        reviews
            .filter(id.eq(idx))
            .filter(user_id.eq(i32::from(req_id))),
    )
    .set(edits)
    .get_result::<Review>(conn);

    res.map_err(|_| ServiceError::InternalServerError)
        .and_then(|result| Ok(result))
}
