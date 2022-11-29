use actix_web::{get, post, put, web, HttpResponse};

use crate::{
    actions::reviews::{
        create_review_for_user, get_all_reviews, update_review, InputReview, ReviewsQuery,
    },
    errors::ServiceError,
    handlers::auth::UserId,
    models::{EditReview, MediaCategory},
    Pool,
};

#[get("")]
pub async fn get_reviews(
    pool: web::Data<Pool>,
    user_id: UserId,
    query: web::Query<ReviewsQuery>,
) -> Result<HttpResponse, ServiceError> {
    let reviews = web::block(move || {
        let mut conn = pool.get()?;
        get_all_reviews(&mut conn, i32::from(user_id), query.into_inner())
    })
    .await??;

    Ok(HttpResponse::Ok().json(reviews))
}

#[post("")]
pub async fn post_reviews(
    pool: web::Data<Pool>,
    user_id: UserId,
    input_review: web::Json<InputReview>,
) -> Result<HttpResponse, ServiceError> {
    let review = web::block(move || {
        let mut conn = pool.get()?;
        create_review_for_user(&mut conn, i32::from(user_id), input_review.into_inner())
    })
    .await??;

    Ok(HttpResponse::Ok().json(review))
}

#[put("/{category}/{id}")]
pub async fn put_reviews_id(
    pool: web::Data<Pool>,
    user_id: UserId,
    path: web::Path<(String, i32)>,
    item: web::Json<EditReview>,
) -> Result<HttpResponse, ServiceError> {
    let (category, tmdb_id) = path.into_inner();

    let category = MediaCategory::try_from(category);

    let Ok(category) = category else {
        return Err(ServiceError::new(400, "Unrecognized media category"));
    };

    let review = web::block(move || {
        let mut conn = pool.get()?;
        update_review(
            &mut conn,
            user_id.into(),
            tmdb_id,
            category,
            item.into_inner(),
        )
    })
    .await??;

    Ok(HttpResponse::Ok().json(review))
}
