use aliyun_oss_rust_sdk::error::OssError;
use axum::extract::multipart::MultipartError;
use axum::extract::rejection::{FormRejection, PathRejection, QueryRejection};
use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use lib_utils::{auth_error_result, error_result, json_response, HttpResult};
use std::num::ParseIntError;
use thiserror::Error;
use tracing::error;
use wechat_pay_rust_sdk::error::PayError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal server error")]
    InternalServerError,

    #[error("{0}")]
    ServiceError(&'static str),

    #[error("sea-orm sql error")]
    DbError(#[from] sea_orm::DbErr),

    // #[error("mongo error")]
    // MongoError(#[from] mongodb::error::Error),
    #[error("parse error")]
    ParseError(#[from] chrono::ParseError),

    #[error("axum error")]
    AxumError(#[from] axum::Error),

    #[error("json deserialize error")]
    JsonDeserializeError(#[from] JsonRejection),

    #[error("path deserialize error")]
    PathDeserializationError(#[from] PathRejection),

    #[error("query deserialize error")]
    QueryDeserializationError(#[from] QueryRejection),

    #[error("form deserialize error")]
    FormDeserializeError(#[from] FormRejection),

    #[error("parse int error")]
    ParseIntError(#[from] ParseIntError),

    #[error("snowflake error")]
    SnowflakeError(#[from] sonyflake::Error),

    #[error("derive_builder error")]
    JwtUserBuilderError(#[from] derive_builder::UninitializedFieldError),

    #[error("request error")]
    RequestError(#[from] reqwest::Error),

    #[error("redis error")]
    RedisError(#[from] redis::RedisError),

    #[error("serde to json error")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("wechat pay error")]
    WechatPayError(#[from] PayError),

    #[error("MultipartError")]
    MultipartError(#[from] MultipartError),

    #[error("OssError")]
    OssError(#[from] OssError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body): (StatusCode, HttpResult<()>) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, error_result("资源没有找到")),
            AppError::Unauthorized => (StatusCode::OK, auth_error_result("没有权限")),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                error_result("服务器异常，请稍后重试"),
            ),
            AppError::ServiceError(error_str) => (StatusCode::OK, error_result(error_str)),
            AppError::DbError(sql_error) => {
                error!("sql error:{:?}", sql_error);
                (StatusCode::OK, error_result("操作失败"))
            }
            // AppError::MongoError(mongo_error) => {
            //     error!("mongo error:{:?}", mongo_error);
            //     (StatusCode::OK, error_result("查询失败"))
            // }
            AppError::ParseError(parse_error) => {
                error!("时间转换失败:{:?}", parse_error);
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::AxumError(axum_error) => {
                error!("axum error:{:?}", axum_error);
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::QueryDeserializationError(query_error) => {
                error!("query param error:{:?}", query_error.body_text());
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::PathDeserializationError(path_error) => {
                error!("path error:{:?}", path_error.body_text());
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::JsonDeserializeError(json_error) => {
                error!("json error:{:?}", json_error.body_text());
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::FormDeserializeError(form_error) => {
                error!("form error:{:?}", form_error.body_text());
                (StatusCode::OK, error_result("参数错误"))
            }
            AppError::ParseIntError(parse_error) => {
                error!("parse error:{:?}", parse_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::SnowflakeError(snowflake_error) => {
                error!("snowflake_error error:{:?}", snowflake_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::JwtUserBuilderError(builder_error) => {
                error!("jwt user error:{:?}", builder_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::RequestError(request_error) => {
                error!("request error:{:?}", request_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::RedisError(redis_error) => {
                error!("redis error:{:?}", redis_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::SerdeJsonError(json_error) => {
                error!("serde error:{:?}", json_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::WechatPayError(wechat_pay_error) => {
                error!("wechat_pay error:{:?}", wechat_pay_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::MultipartError(multipart_error) => {
                error!("multipart error:{:?}", multipart_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
            AppError::OssError(oss_error) => {
                error!("oss error:{:?}", oss_error);
                (StatusCode::OK, error_result("系统内部错误"))
            }
        };
        json_response(status, &body)
    }
}
