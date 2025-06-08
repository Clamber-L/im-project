use crate::state::AppState;
use app_user_friend::Entity as AppUserFriend;
use axum::{extract::State, Extension};
use lib_core::{generate_snowflake_id, ApiResult, ExtractJson};
use lib_entity::mysql::app_chat::Model;
use lib_entity::mysql::{app_chat, app_chat_user, app_user_friend};
use lib_utils::{error_result, ok_result, ok_result_with_none};
use sea_orm::{prelude::Expr, sqlx::types::chrono::Utc, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use sonyflake::Sonyflake;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FriendAddParam {
    pub to_user_id: String,
}

/// json param must be last in the function
pub async fn add_friend(
    Extension(user_id): Extension<String>,
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<FriendAddParam>,
) -> ApiResult<String> {
    // 添加好友 检查是否已经是好友
    let app_friend = AppUserFriend::find()
        .filter(Expr::col(app_user_friend::Column::UserId).eq(user_id.clone()))
        .filter(Expr::col(app_user_friend::Column::FriendUserId).eq(param.to_user_id.clone()))
        .one(&state.mysql_client)
        .await?;
    if let Some(_) = app_friend {
        return Ok(error_result("你们已经是好友了"));
    }

    let self_app_user_friend = app_user_friend::ActiveModel {
        id: Set(generate_snowflake_id()?),
        user_id: Set(Some(user_id.clone())),
        friend_user_id: Set(Some(param.to_user_id.clone())),
        follow_time: Set(Some(Utc::now().naive_local())),
        ..Default::default()
    };
    AppUserFriend::insert(self_app_user_friend)
        .exec(&state.mysql_client)
        .await?;

    let follow_app_user_friend = app_user_friend::ActiveModel {
        id: Set(generate_snowflake_id()?),
        user_id: Set(Some(param.to_user_id.clone())),
        friend_user_id: Set(Some(user_id.clone())),
        follow_time: Set(Some(Utc::now().naive_local())),
        ..Default::default()
    };
    AppUserFriend::insert(follow_app_user_friend)
        .exec(&state.mysql_client)
        .await?;

    // 添加好友，默认开启一个chat
    let chat = app_chat::ActiveModel {
        id: Set(Sonyflake::new().and_then(|x| x.next_id())?.to_string()),
        created_time: Set(Some(Utc::now().naive_local())),
        chat_type: Set(Some(1)),
        chat_state: Set(Some(0)),
        ..Default::default()
    };
    let last_result_id = app_chat::Entity::insert(chat)
        .exec(&state.mysql_client)
        .await?
        .last_insert_id;

    // chat_user 添加两人
    let self_chat_user = app_chat_user::ActiveModel {
        id: Set(Sonyflake::new().and_then(|x| x.next_id())?.to_string()),
        created_time: Set(Some(Utc::now().naive_local())),
        user_id: Set(Some(user_id)),
        chat_id: Set(Some(last_result_id.clone())),
        ..Default::default()
    };
    app_chat_user::Entity::insert(self_chat_user)
        .exec(&state.mysql_client)
        .await?;

    let friend_chat_user = app_chat_user::ActiveModel {
        id: Set(Sonyflake::new().and_then(|x| x.next_id())?.to_string()),
        created_time: Set(Some(Utc::now().naive_local())),
        user_id: Set(Some(param.to_user_id.clone())),
        chat_id: Set(Some(last_result_id)),
        ..Default::default()
    };
    app_chat_user::Entity::insert(friend_chat_user)
        .exec(&state.mysql_client)
        .await?;

    Ok(ok_result_with_none())
}

pub async fn chat_list(
    Extension(user_id): Extension<String>,
    State(state): State<AppState>,
) -> ApiResult<Vec<Model>> {
    // 获取聊天列表
    let chat_list = app_chat_user::Entity::find()
        .filter(Expr::col(app_chat_user::Column::UserId).eq(user_id))
        .all(&state.mysql_client)
        .await?;

    if chat_list.is_empty() {
        Ok(ok_result(vec![]))
    } else {
        let chat_ids = chat_list
            .into_iter()
            .map(|x| x.chat_id.unwrap())
            .collect::<Vec<String>>();
        let chat_list = app_chat::Entity::find()
            .filter(Expr::col(app_chat::Column::Id).is_in(chat_ids))
            .all(&state.mysql_client)
            .await?;
        Ok(ok_result(chat_list))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageParam {
    pub to_user_id: String,
    pub content: String,
    pub chat_id: String,
}

pub async fn send_message(
    Extension(_user_id): Extension<String>,
    State(_state): State<AppState>,
) -> ApiResult<String> {
    // 数据入库
    // 调用grpc发送消息
    Ok(ok_result_with_none())
}
