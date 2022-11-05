use actix_identity::Identity;
use actix_web::{post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

#[derive(Serialize, Deserialize, Debug)]
pub enum WatchStatus {
    Dropped,
    Completed,
    Watching,
    PlanToWatch,
}

#[derive(Deserialize)]
pub struct InputReview {
    tmdb_id: String,
    status: WatchStatus,
    text: Option<String>,
    fun_before: Option<bool>,
    fun_during: Option<bool>,
    fun_after: Option<bool>,
}

#[put("/films")]
pub async fn review_film(
    user: Option<Identity>,
    item: web::Json<InputReview>,
) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().finish())
}

#[put("/shows")]
pub async fn review_show(
    user: Option<Identity>,
    item: web::Json<InputReview>,
) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().finish())
}
