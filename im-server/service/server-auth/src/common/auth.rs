use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use lib_core::{generate_jwt, generate_snowflake_id, ApiResult, ExtractQuery, JwtUser};
use lib_entity::mysql::app_user::{self, Entity as AppUser};
use lib_utils::{ok_result, ok_result_with_none};
use sea_orm::prelude::{DateTime, Expr};
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

pub async fn login() -> impl IntoResponse {
    let user = JwtUser {
        id: "1564864212144".to_owned(),
    };

    let token = generate_jwt(user);

    ok_result(token)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SignUpParam {
    pub username: String,
}

pub async fn signup(
    State(state): State<AppState>,
    ExtractQuery(param): ExtractQuery<SignUpParam>,
) -> ApiResult<String> {
    println!("param:{:?}", param);
    let user = AppUser::find()
        .filter(Expr::col(app_user::Column::Username).eq(&param.username))
        .one(&state.mysql_client)
        .await?;
    if user.is_none() {
        // 添加用户信息 发送邮件
        let date_time = Local::now().naive_local();

        let user = app_user::ActiveModel {
            id: Set(generate_snowflake_id()?),
            username: Set(Some(param.username.clone())),
            created_time: Set(Some(date_time)),
            signup: Set(Some(false)),
            ..Default::default()
        };
        user.insert(&state.mysql_client).await?;
    } else {
        println!("user already exists:{:?}", user.unwrap());
    }
    Ok(ok_result_with_none())
}

pub async fn insert_user(
    Extension(_user_id): Extension<String>,
    State(state): State<AppState>,
) -> ApiResult<String> {
    let last_time = DateTime::parse_from_str("2024-11-10 12:22:43", "%Y-%m-%d %H:%M:%S")?;
    let user = app_user::ActiveModel {
        id: Set("1564864212144".to_owned()),
        username: Set(Some(String::from("张三"))),
        password: Set(Some(String::from("张三"))),
        gender: Set(Some(String::from("男"))),
        birthday: Set(Some(last_time)),
        avatar: Set(Some(String::from("http://123.png"))),
        ..Default::default()
    };
    let result = app_user::Entity::insert(user)
        .exec(&state.mysql_client)
        .await?;

    Ok(ok_result(result.last_insert_id))
}
