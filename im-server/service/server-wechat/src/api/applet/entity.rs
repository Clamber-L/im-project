use lib_entity::mysql::{applet_operation, applet_operation_content, applet_user};
use sea_orm::prelude::DateTime;
use sea_orm::sqlx::types::chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use wechat_pay_rust_sdk::response::SignData;

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
pub struct AppletSettingParam {
    pub setting_type: i32,
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationUserResponse {
    pub operation_name: String,
    pub commander: bool,
    pub has_operation: bool,
    pub joined: bool,
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
            be_end: operation.end_time < today,
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

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamResponse {
    pub team_id: String,
    pub has_team: bool,
    pub user_list: Vec<TeamUserResponse>,
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamUserResponse {
    pub user_id: String,
    pub username: String,
    pub avatar: String,
}

impl TeamResponse {
    pub fn new(team_id: String, user_list: Vec<TeamUserResponse>) -> TeamResponse {
        Self {
            team_id,
            has_team: true,
            user_list,
        }
    }

    pub fn new_none() -> TeamResponse {
        Self {
            team_id: "".to_string(),
            has_team: false,
            user_list: vec![],
        }
    }
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamParam {
    pub operation_id: String,
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationUserNumParam {
    pub operation_id: String,
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationUserNumResponse {
    pub today_num: usize,
    pub all_num: usize,
    pub buy_user_list: Vec<TeamUserResponse>,
}

impl OperationUserNumResponse {
    pub fn new(
        today_num: usize,
        all_num: usize,
        buy_user_list: Vec<TeamUserResponse>,
    ) -> OperationUserNumResponse {
        Self {
            today_num,
            all_num,
            buy_user_list,
        }
    }
}

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PayResponse {
    pub prepay_id: String,
    pub sign_type: String,
    pub package: String,
    pub nonce_str: String,
    pub timestamp: String,
    pub pay_sign: String,
}

impl PayResponse {
    pub(crate) fn new(prepay_id: String, sign_data: SignData) -> Self {
        Self {
            prepay_id,
            sign_type: sign_data.sign_type,
            package: sign_data.package,
            nonce_str: sign_data.nonce_str,
            timestamp: sign_data.timestamp,
            pay_sign: sign_data.pay_sign,
        }
    }
}
