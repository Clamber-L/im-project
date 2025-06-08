use lib_entity::mysql::applet_user;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppletLoginParam {
    pub code: String,
    pub phone_code: String,
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct WechatLoginByCodeResponse {
    pub session_key: Option<String>,
    pub openid: Option<String>,
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginResponse {
    pub token: String,
    pub user_id: String,
    pub username: String,
    pub avatar: String,
    pub phone: String,
}

impl UserLoginResponse {
    pub fn new(token: String, user: applet_user::Model) -> UserLoginResponse {
        Self {
            token,
            user_id: user.id,
            username: user.username,
            avatar: user.avatar,
            phone: user.phone,
        }
    }
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPayParam {
    pub code: String,
    pub group_buy_id: String,
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCreationParam {
    pub page_num: u64,
    pub page_size: u64,
}
