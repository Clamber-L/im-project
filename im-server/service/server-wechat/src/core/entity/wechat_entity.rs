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
