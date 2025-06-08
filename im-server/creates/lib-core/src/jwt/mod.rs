use crate::AppError;
use anyhow::Result;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use chrono::{Duration, Utc};
use derive_builder::Builder;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;

const JWT_SECRET: &str = "jwt_secret";
const HEADER_TOKEN_KEY: &str = "accessToken";

#[derive(Debug, Serialize, Deserialize, Builder, Default, Clone)]
#[builder(setter(into))]
pub struct JwtUser {
    pub id: String,
}

impl<S> FromRequestParts<S> for JwtUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(token) = parts.headers.get(HEADER_TOKEN_KEY) {
            println!("token:{:?}", token);
            if let Ok(user) = verification_jwt(token.to_str().unwrap()) {
                return Ok(user);
            }
        };
        Err(AppError::Unauthorized)
    }
}

pub fn generate_jwt(user: JwtUser) -> String {
    let time = Utc::now() + Duration::days(7);

    let mut claims = BTreeMap::new();
    claims.insert("id", user.id);
    claims.insert("expire", time.timestamp().to_string());

    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET.as_bytes()).unwrap();

    claims.sign_with_key(&key).unwrap()
}

pub fn verification_jwt(token: &str) -> Result<JwtUser> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET.as_bytes())?;
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;

    let id = claims.get("id");
    let expire = claims.get("expire");

    if expire.is_none()
        || expire.unwrap() == ""
        || expire.unwrap().parse::<i64>()? <= Utc::now().timestamp()
    {
        return Err(AppError::InternalServerError.into());
    }

    if let Some(id) = id {
        Ok(JwtUserBuilder::default().id(id).build()?)
    } else {
        Err(AppError::InternalServerError.into())
    }
}
