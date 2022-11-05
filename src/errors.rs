use std::fmt::Display;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use awc::error::{JsonPayloadError, SendRequestError};

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

impl From<r2d2::Error> for ServiceError {
    fn from(_: r2d2::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<SendRequestError> for ServiceError {
    fn from(_: SendRequestError) -> Self {
        ServiceError::InternalServerError
    }
}
impl From<JsonPayloadError> for ServiceError {
    fn from(_: JsonPayloadError) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<diesel::result::Error> for ServiceError {
    fn from(_: diesel::result::Error) -> Self {
        ServiceError::InternalServerError
    }
}
