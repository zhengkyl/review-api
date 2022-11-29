use std::fmt::Display;

use actix_web::{error::BlockingError, http::StatusCode, HttpResponse, ResponseError};
use awc::error::{JsonPayloadError, SendRequestError};
use serde_json::json;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct ServiceError {
    pub status: u16,
    pub message: String,
}

impl ServiceError {
    pub fn new<T: Into<String>>(status: u16, message: T) -> Self {
        ServiceError {
            status,
            message: message.into(),
        }
    }
    pub fn pls(status: u16) -> Self {
        ServiceError {
            status,
            message: "L + Ratio".into(),
        }
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            400 => write!(f, "i'm so sorry that you're stupid: {}", self.message),
            401 => write!(f, "sir, ✋👮 i need to see your id: {}", self.message),
            404 => write!(f, "out of stock. try again later: {}", self.message),
            _ => write!(f, "internal server did an oopsie 👉👈: {}", self.message),
        }
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match StatusCode::from_u16(self.status) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({"message": self.message}))
    }
}

impl From<BlockingError> for ServiceError {
    fn from(e: BlockingError) -> Self {
        ServiceError::new(500, e.to_string())
    }
}

impl From<actix_web::Error> for ServiceError {
    fn from(e: actix_web::Error) -> Self {
        ServiceError::new(500, e.to_string())
    }
}

impl From<r2d2::Error> for ServiceError {
    fn from(e: r2d2::Error) -> Self {
        ServiceError::new(500, e.to_string())
    }
}

impl From<argon2::Error> for ServiceError {
    fn from(e: argon2::Error) -> Self {
        ServiceError::new(500, e.to_string())
    }
}

impl From<SendRequestError> for ServiceError {
    fn from(e: SendRequestError) -> Self {
        ServiceError::new(500, e.to_string())
    }
}
impl From<JsonPayloadError> for ServiceError {
    fn from(e: JsonPayloadError) -> Self {
        ServiceError::new(500, e.to_string())
    }
}

impl From<DbError> for ServiceError {
    fn from(e: DbError) -> Self {
        ServiceError::new(500, e.to_string())
    }
}

impl From<diesel::result::Error> for ServiceError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => ServiceError::pls(404),
            err => ServiceError::new(500, format!("diesel says {}", err)),
        }
    }
}
