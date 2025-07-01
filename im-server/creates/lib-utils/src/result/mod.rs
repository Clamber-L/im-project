pub mod request_entity;

use crate::json_response;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResult<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

pub fn ok_result<T>(data: T) -> HttpResult<T> {
    HttpResult {
        code: 200,
        message: String::from("成功"),
        data: Some(data),
    }
}

pub fn ok_result_with_none<T>() -> HttpResult<T> {
    HttpResult {
        code: 200,
        message: String::from("成功"),
        data: None,
    }
}

pub fn error_result<T>(message: &str) -> HttpResult<T> {
    HttpResult {
        code: 555,
        message: message.to_owned(),
        data: None,
    }
}

pub fn auth_error_result<T>(message: &str) -> HttpResult<T> {
    HttpResult {
        code: 403,
        message: message.to_owned(),
        data: None,
    }
}

impl<T> IntoResponse for HttpResult<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        json_response(StatusCode::OK, &self)
    }
}
