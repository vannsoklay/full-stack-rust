use actix_web::{error::ResponseError, HttpResponse, Responder};
use derive_more::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Internal Server Error {}", _0)]
    InternalServerError(String),

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized, {}", _0)]
    Unauthorized(String),

    #[display(fmt = "NotFound, {}", _0)]
    NotFound(String),
}

pub trait ResponseSuccess {
    fn message(message: String) -> HttpResponse;
    fn get(object: Value) -> HttpResponse;
    fn get_all(array_object: Vec<Value>) -> HttpResponse;
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Success {}

impl ResponseSuccess for Success {
    fn message(message: String) -> HttpResponse {
        return HttpResponse::Ok()
            .json(serde_json::json!({"status": 200,"message": "success", "ok": message }));
    }
    fn get(object: Value) -> HttpResponse {
        return HttpResponse::Ok()
            .json(serde_json::json!({"status": 200, "message": "success", "data": object}));
    }
    fn get_all(array_object: Vec<Value>) -> HttpResponse {
        return HttpResponse::Ok()
            .json(serde_json::json!({"status": 200, "message": "success", "data": array_object}));
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::NotFound(ref message) => HttpResponse::NotFound()
                .json(serde_json::json!({"status": 404, "message": "fail", "error": message})),
            Error::InternalServerError(ref message) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": 500, "message": "fail", "error": message})),
            Error::BadRequest(ref message) => HttpResponse::BadRequest()
                .json(serde_json::json!({"status": 400, "message": "fail", "error": message})),
            Error::Unauthorized(ref message) => HttpResponse::Unauthorized()
                .json(serde_json::json!({"status": 401, "message": "fail", "error": message})),
        }
    }
}
