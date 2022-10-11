use std::fmt::Display;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    Unauthorized,
}
impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://rust-lang.github.io/rfcs/2005-match-ergonomics.html
        match self {
            ServiceError::InternalServerError => write!(f, "Service Error"),
            ServiceError::BadRequest(reason) => write!(f, "L + Ratio: {}", reason),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
