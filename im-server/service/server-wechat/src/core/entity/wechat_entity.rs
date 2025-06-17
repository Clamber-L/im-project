use sea_orm::sqlx::types::chrono;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WechatAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WechatTicketResponse {
    #[serde(rename = "errcode")]
    pub err_code: u64,

    #[serde(rename = "errmsg")]
    pub err_msg: String,

    pub ticket: String,

    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct WechatPhoneResponse {
    pub errcode: i32,
    #[serde(rename = "phone_info")]
    pub phone_info: Option<PhoneInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PhoneInfo {
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}

#[derive(Debug, Deserialize)]
pub struct WechatPayNotifyParam {
    pub id: String,

    pub create_time: chrono::DateTime<chrono::Utc>,

    pub resource_type: String,

    pub summary: String,

    pub resource: WechatPayNotifyResource,
}

#[derive(Debug, Deserialize)]
pub struct WechatPayNotifyResource {
    pub original_type: String,

    pub algorithm: String,

    pub ciphertext: String,

    pub associated_data: String,

    pub nonce: String,
}
