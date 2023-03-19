use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error {}", _0)]
    InternalServerError(String),

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized, {}", _0)]
    Unauthorized(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(ref message) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": 500, "message": "fail", "error": message})),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest()
                .json(serde_json::json!({"status": 400, "message": "fail", "error": message})),
            ServiceError::Unauthorized(ref message) => HttpResponse::Unauthorized()
                .json(serde_json::json!({"status": 401, "message": "fail", "error": message})),
        }
    }
}
