use crate::api::applet::entity::WechatLoginByCodeResponse;
use crate::core::constants::{ACCESS_TOKEN_URL, LOGIN_URL, LOGIN_USER_PHONE_URL};
use crate::core::entity::{PayConfig, WechatAccessTokenResponse, WechatPhoneResponse};
use anyhow::Result;
use lib_core::key_constants::WECHAT_ACCESS_TOKEN;
use lib_core::{AppError, RedisService};
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use std::collections::HashMap;
use std::fs::read_to_string;
use wechat_pay_rust_sdk::error::PayError;
use wechat_pay_rust_sdk::model::{AmountInfo, MicroParams, PayerInfo};
use wechat_pay_rust_sdk::pay::WechatPay;
use wechat_pay_rust_sdk::response::MicroResponse;

/// 获取access_token
pub async fn access_token(
    client: &Client,
    redis_service: &RedisService,
    app_id: String,
    secret: String,
) -> Result<String, AppError> {
    let access_token;

    if redis_service.has_key(WECHAT_ACCESS_TOKEN).await? {
        let token = redis_service.get(WECHAT_ACCESS_TOKEN).await?.unwrap();
        access_token = token;
        println!("redis access_token:{:?}", access_token);
    } else {
        let mut token_params = HashMap::new();
        token_params.insert("appid", app_id);
        token_params.insert("secret", secret);

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
    Ok(access_token)
}

/// 根据code获取用户手机号码
pub async fn get_user_phone(
    client: &Client,
    code: String,
    access_token: String,
) -> Result<Option<String>, AppError> {
    // 请求体 JSON
    let mut body = HashMap::new();
    body.insert("code", code);

    // 构建 URL + query
    let url_parse =
        reqwest::Url::parse_with_params(LOGIN_USER_PHONE_URL, &[("access_token", access_token)]);
    if url_parse.is_err() {
        return Ok(None);
    }

    // 发送请求
    let resp = client
        .post(url_parse.unwrap())
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&body)?)
        .send()
        .await?;

    println!("use phone response:{:?}", resp);
    // 处理响应
    if resp.status().is_success() {
        let result: WechatPhoneResponse = resp.json().await?;
        println!("phone result:{:?}", result);
        if result.errcode == 0 {
            if let Some(info) = result.phone_info {
                return Ok(Some(info.phone_number));
            }
        }
    }

    Ok(None)
}

/// 根据code获取用户信息 openid
pub async fn user_by_code(
    client: &Client,
    code: String,
    app_id: String,
    secret: String,
) -> Result<WechatLoginByCodeResponse, AppError> {
    let mut code_param = HashMap::new();
    code_param.insert("appid", app_id);
    code_param.insert("secret", secret);
    code_param.insert("js_code", code);
    code_param.insert("grant_type", "authorization_code".to_string());

    let res: WechatLoginByCodeResponse = client
        .get(LOGIN_URL)
        .query(&code_param)
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

/// 微信支付
pub async fn user_wechat_pay(
    pay_config: &PayConfig,
    description: String,
    order_id: String,
    total_amount: i32,
    open_id: String,
) -> Result<MicroResponse, AppError> {
    let pay_config = pay_config.clone();

    let path = &pay_config.key_path;

    // 读取文件内容为字符串
    let contents = read_to_string(path);
    if contents.is_err() {
        return Err(AppError::WechatPayError(PayError::WeixinNotFound));
    }

    let contents = contents.unwrap();
    let wechat_pay = WechatPay::new(
        &pay_config.app_id,
        &pay_config.mch_id,
        &contents,
        &pay_config.serial_no,
        &pay_config.v3_key,
        &pay_config.notify_url,
    );
    let response = wechat_pay
        .micro_pay(MicroParams {
            description,
            out_trade_no: order_id,
            amount: AmountInfo {
                total: total_amount,
            },
            payer: PayerInfo { openid: open_id },
            attach: None,
            detail: None,
            time_expire: None,
            scene_info: None,
        })
        .await?;
    Ok(response)
}

// fn _test_wechat_pay() {
//     // 常规配置参数
//     let mut params: HashMap<String, Value> = HashMap::new();
//
//     // 1. 基本字段
//     params.insert("appid".into(), json!(app_id));
//     params.insert("mchid".into(), json!(mch_id));
//     params.insert("description".into(), json!(description));
//     params.insert("out_trade_no".into(), json!(order_id));
//     params.insert("notify_url".into(), json!(notify_url));
//
//     // 2. 金额参数
//     let amount = json!({
//         "total": total_amount,
//         "currency": "CNY"
//     });
//     params.insert("amount".into(), amount);
//
//     // 3. 用户在商户appid下的唯一标识。
//     let payer = json!({
//         "openid": open_id,
//     });
//     params.insert("payer".into(), payer);
//
//     // 4. 转为 JSON 字符串
//     let params_str = serde_json::to_string(&params)?;
//     println!("请求参数 ===> {}", params_str);
//
//     // 5. 微信预支付
//     let response_body = wechat_http_order_post(client, WECHAT_PAY_API, &params_str).await?;
//     if let Some(Value::String(prepay_id)) = response_body.get("prepay_id") {
//         info!("prepay_id: {}", prepay_id);
//
//         // 6. 获取签名
//         let nonce_str = generate_wechat_pay_nonce_str();
//
//         let timestamp = generate_wechat_pay_timestamp();
//
//         let sign_content = format!(
//             "{}\n{}\n{}\nprepay_id={}\n",
//             app_id, timestamp, nonce_str, prepay_id
//         );
//
//         // 4. 加载私钥
//         let private_key = std::fs::read_to_string("apiclient_key.pem").unwrap();
//         let sign = crate::core::service::wechat_api::sha256_sign(private_key, sign_content);
//         info!("签名串: {:?}", sign);
//     }
// }
