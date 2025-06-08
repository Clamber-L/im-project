use crate::api::auth::entity::{SignTokenParam, SignTokenResponse};
use crate::core::constants::ACCESS_TOKEN_URL;
use crate::core::entity::{WechatAccessTokenResponse, WechatTicketResponse};
use crate::core::AppState;
use axum::extract::State;
use lib_core::key_constants::{WECHAT_ACCESS_TOKEN, WECHAT_TICKET};
use lib_core::{ApiResult, ExtractQuery};
use lib_utils::ok_result;
use rand::distr::Alphanumeric;
use rand::Rng;
use reqwest::Client;
use sha1::{Digest, Sha1};
use std::collections::{BTreeMap, HashMap};
use std::time::SystemTime;
use tracing::info;

const TICKET_UTL: &str = "https://api.weixin.qq.com/cgi-bin/ticket/getticket";

pub async fn sign_token(
    State(state): State<AppState>,
    ExtractQuery(param): ExtractQuery<SignTokenParam>,
) -> ApiResult<SignTokenResponse> {
    info!(state.wechat_config.app_id);
    info!(state.wechat_config.secret);
    info!("{:?}", param);

    let client = Client::new();
    let redis_service = &state.redis_service;

    let access_token;
    let ticket;

    if redis_service.has_key(WECHAT_ACCESS_TOKEN).await? {
        let token = redis_service.get(WECHAT_ACCESS_TOKEN).await?.unwrap();
        access_token = token;
        println!("redis access_token:{:?}", access_token);
    } else {
        let mut token_params = HashMap::new();
        token_params.insert("appid", &state.wechat_config.app_id);
        token_params.insert("secret", &state.wechat_config.secret);

        let res: WechatAccessTokenResponse = client
            .get(ACCESS_TOKEN_URL)
            .query(&token_params)
            .send()
            .await?
            .json()
            .await?;
        access_token = res.access_token;
        redis_service
            .set_nx_ex(
                WECHAT_ACCESS_TOKEN,
                access_token.as_str(),
                res.expires_in - 100,
            )
            .await?;
        println!("not use redis:{:?}", access_token);
    }

    if redis_service.has_key(WECHAT_TICKET).await? {
        let ticket_str = redis_service.get(WECHAT_TICKET).await?.unwrap();
        ticket = ticket_str;
    } else {
        // 获取ticket
        let mut ticket_map = HashMap::new();
        ticket_map.insert("access_token", access_token.as_str());
        ticket_map.insert("type", "jsapi");

        let res: WechatTicketResponse = client
            .get(TICKET_UTL)
            .query(&ticket_map)
            .send()
            .await?
            .json()
            .await?;

        ticket = res.ticket;
        println!("ticket:{:?}", ticket);
        redis_service
            .set_nx_ex(WECHAT_TICKET, ticket.as_str(), res.expires_in - 100)
            .await?;
        println!("not use redis ticket:{:?}", ticket);
    }

    let timestamp = generate_timestamp();
    let nonce_str = generate_nonce_str();

    let sign = generate_signature(
        ticket,
        nonce_str.clone(),
        param.redirect_url.clone(),
        timestamp.clone(),
    );

    let response = SignTokenResponse::new(
        &state.wechat_config.app_id,
        nonce_str,
        sign,
        timestamp,
        param.redirect_url,
    );

    Ok(ok_result(response))
}

fn generate_signature(ticket: String, nonce_str: String, url: String, timestamp: String) -> String {
    let mut tree_map = BTreeMap::new();
    tree_map.insert("noncestr", nonce_str);
    tree_map.insert("jsapi_ticket", ticket);
    tree_map.insert("timestamp", timestamp);
    tree_map.insert("url", url);

    let sing = tree_map
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("&");

    println!("sing: {}", sing);
    let mut hasher = Sha1::new();
    hasher.update(sing.as_bytes());
    let result = hasher.finalize();

    let result = hex::encode(result);
    println!("result: {}", result);
    result
}

fn generate_nonce_str() -> String {
    let len = "Wm3WZYTPz0wzccnW".len();
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn generate_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let timestamp = since_the_epoch.as_secs().to_string();
    println!("timestamp:{}", timestamp);
    timestamp
}
