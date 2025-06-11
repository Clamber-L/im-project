mod config;
mod date;
mod password;
mod result;

use axum::http::{header, HeaderValue};
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Response};
use chrono::{DateTime, NaiveDateTime, Utc};
pub use config::*;
pub use date::*;
pub use result::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize_datetime_with_option<S>(
    datetime: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match datetime {
        Some(dt) => serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_datetime<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime_result = Option::<String>::deserialize(deserializer);

    match datetime_result {
        Ok(datetime) => match datetime {
            Some(time) => {
                if !time.is_empty() {
                    let naive_datetime = NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S")
                        .map_err(serde::de::Error::custom)?;
                    Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(
                        naive_datetime,
                        Utc,
                    )))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        },
        Err(_) => Ok(None),
    }
}

pub fn json_response<T: Serialize>(status: StatusCode, body: &T) -> Response {
    match serde_json::to_string(body) {
        Ok(json) => {
            let mut response = (status, json).into_response();
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            response
        }
        Err(err) => {
            // fallback error response
            tracing::error!("JSON serialization error: {:?}", err);
            let fallback: HttpResult<()> = error_result("Internal Server Error");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::to_string(&fallback).unwrap(),
            )
                .into_response()
        }
    }
}
