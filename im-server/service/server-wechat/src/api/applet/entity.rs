use lib_entity::mysql::{applet_operation, applet_operation_content, applet_user};
use sea_orm::prelude::DateTime;
use sea_orm::sqlx::types::chrono::{Local, NaiveDate, Utc};
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

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationResponse {
    pub id: String,
    pub created_time: Option<DateTime>,
    pub name: String,
    pub end_time: NaiveDate,
    pub be_open: bool,
    pub be_end: bool,
    pub contents: Vec<applet_operation_content::Model>,
}

impl OperationResponse {
    pub fn new(
        operation: applet_operation::Model,
        mut contents: Vec<applet_operation_content::Model>,
    ) -> OperationResponse {
        if !contents.is_empty() {
            contents.sort_by(|a, b| a.id.cmp(&b.id));
        }

        let today = Local::now().date_naive(); // 当前日期（无时间部分）

        Self {
            id: operation.id,
            created_time: operation.created_time,
            name: operation.name,
            end_time: operation.end_time,
            be_end: operation.end_time <= today,
            be_open: operation.be_open,
            contents,
        }
    }
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserTeamParam {
    pub operation_id: String,
}
