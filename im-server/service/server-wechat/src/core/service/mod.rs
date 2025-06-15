use rand::distr::Alphanumeric;
use rand::Rng;
use sea_orm::sqlx::types::chrono::Utc;
use std::time::SystemTime;

pub mod wechat_api;

/// 微信支付生成随机数
fn _generate_wechat_pay_nonce_str() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

fn _generate_wechat_pay_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let timestamp = since_the_epoch.as_secs().to_string();
    println!("timestamp:{}", timestamp);
    timestamp
}

pub fn _generate_short_name() -> String {
    let timestamp = Utc::now().timestamp();
    let rand_str: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    format!("{}{}", timestamp, rand_str)
}
