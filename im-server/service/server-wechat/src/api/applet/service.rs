use crate::api::applet::entity::{
    AppletLoginParam, CreateTeamParam, OperationResponse, UserCreationParam, UserLoginResponse,
    UserPayParam, UserTeamParam,
};
use crate::core::service::wechat_api::{access_token, get_user_phone, user_by_code};
use crate::core::AppState;
use axum::extract::State;
use axum::Json;
use lib_core::{
    generate_jwt, generate_snowflake_id, ApiResult, ExtractJson, ExtractQuery, JwtUser,
};
use lib_entity::mysql::prelude::{
    AppletOperation, AppletOperationContent, AppletOperationTeamUser, AppletUser,
    AppletUserCreation,
};
use lib_entity::mysql::{
    applet_operation, applet_operation_content, applet_operation_team, applet_operation_team_user,
    applet_pay_centre_record, applet_user, applet_user_creation,
};
use lib_utils::request_entity::PageResult;
use lib_utils::{error_result, ok_result, ok_result_with_none};
use sea_orm::prelude::Expr;
use sea_orm::sqlx::types::chrono::{Local, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use std::collections::HashMap;
use tracing::error;

use super::entity::{TeamResponse, TeamUserResponse};

/// 登录
pub async fn login(
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<AppletLoginParam>,
) -> ApiResult<UserLoginResponse> {
    println!("param:{:?}", param);
    let redis_service = &state.redis_service;

    // 获取openid
    let res = user_by_code(
        &state.request_client,
        param.code.clone(),
        state.applet_config.app_id.clone(),
        state.applet_config.secret.clone(),
    )
    .await?;
    if res.openid.is_none() {
        return Ok(error_result("登录失败，请稍后再试"));
    }

    println!("res:{:?}", res);
    // 根据openid判断是否为新用户
    let applet_user_option = AppletUser::find()
        .filter(Expr::col(applet_user::Column::OpenId).eq(res.openid.clone().unwrap()))
        .one(&state.mysql_client)
        .await?;

    if let None = applet_user_option {
        // 新用户
        // 1.获取access_token
        let access_token = access_token(
            &state.request_client,
            redis_service,
            state.applet_config.app_id.clone(),
            state.applet_config.secret.clone(),
        )
        .await?;

        // 2.获取手机号码
        let user_phone_result =
            get_user_phone(&state.request_client, param.phone_code, access_token).await?;
        if user_phone_result.is_some() {
            let user_phone = user_phone_result.unwrap();
            let id = generate_snowflake_id()?;
            let user = applet_user::ActiveModel {
                id: Set(id.clone()),
                username: Set(format!("微信用户{}", user_phone)),
                open_id: Set(res.openid.unwrap()),
                phone: Set(user_phone),
                avatar: Set(
                    "https://jmzbase.oss-cn-beijing.aliyuncs.com/vs4f2vx8rumjkww9wjrc.jpg"
                        .to_string(),
                ),
                created_time: Set(Some(Utc::now().naive_utc())),
                updated_time: Set(Some(Utc::now().naive_utc())),
            }
            .insert(&state.mysql_client)
            .await?;
            let token = generate_jwt(JwtUser { id });
            Ok(ok_result(UserLoginResponse::new(token, user)))
        } else {
            println!("获取用户信息失败");
            println!("user_phone_result:{:?}", user_phone_result.unwrap());
            Ok(error_result("获取用户信息失败，请稍后再试"))
        }
    } else {
        let model = applet_user_option.unwrap();
        let token = generate_jwt(JwtUser {
            id: model.clone().id,
        });
        Ok(ok_result(UserLoginResponse::new(token, model)))
    }
}

/// 修改用户信息
pub async fn update_user(
    State(state): State<AppState>,
    user: JwtUser,
    ExtractJson(param): ExtractJson<UserLoginResponse>,
) -> ApiResult<UserLoginResponse> {
    println!("user:{:?}", user);
    println!("params:{:?}", param);
    let user_option = AppletUser::find_by_id(param.user_id)
        .one(&state.mysql_client)
        .await?;
    if let Some(user) = user_option {
        let mut active_user: applet_user::ActiveModel = user.into();
        active_user.avatar = Set(param.avatar);
        active_user.username = Set(param.username);
        active_user.updated_time = Set(Some(Utc::now().naive_utc()));
        let applet_user = active_user.update(&state.mysql_client).await?;
        return Ok(ok_result(UserLoginResponse::new(param.token, applet_user)));
    };
    Ok(error_result("操作失败，请稍后再试"))
}

/// 首页作品列表
pub async fn creation_list(
    State(state): State<AppState>,
    ExtractQuery(param): ExtractQuery<UserCreationParam>,
) -> ApiResult<PageResult<applet_user_creation::Model>> {
    println!("param:{:?}", param);
    let paginate = AppletUserCreation::find()
        .order_by_asc(applet_user_creation::Column::Id)
        .paginate(&state.mysql_client, param.page_size);
    let items_and_pages_number = paginate.num_items_and_pages().await?;

    let items = paginate.fetch_page(param.page_num - 1).await?;
    let page_result = PageResult::new(
        param.page_num,
        param.page_size,
        items_and_pages_number,
        items,
    );
    Ok(ok_result(page_result))
}

/// 活动详情
pub async fn operation(
    State(state): State<AppState>,
    user: JwtUser,
) -> ApiResult<OperationResponse> {
    let operation_option = AppletOperation::find()
        .filter(Expr::col(applet_operation::Column::BeOpen).eq(true))
        .one(&state.mysql_client)
        .await?;
    if let Some(operation) = operation_option {
        // 获取活动详情
        let contents = AppletOperationContent::find()
            .filter(
                Expr::col(applet_operation_content::Column::OperationId).eq(operation.id.clone()),
            )
            .all(&state.mysql_client)
            .await?;
        return Ok(ok_result(OperationResponse::new(operation, contents)));
    }
    Ok(ok_result_with_none())
}

/// 检查是否加入团购 团购的身份
pub async fn user_team(
    State(state): State<AppState>,
    user: JwtUser,
    ExtractQuery(param): ExtractQuery<UserTeamParam>,
) -> ApiResult<TeamResponse> {
    let user_id = user.id;

    println!("param:{:?}", param);
    // 检查本次团购是否到期
    let operation_option = AppletOperation::find_by_id(param.operation_id)
        .one(&state.mysql_client)
        .await?;
    println!("operation_option:{:?}", operation_option);
    if let Some(operation) = operation_option {
        if Local::now().date_naive() > operation.end_time {
            return Ok(error_result("本次活动已到期"));
        }
        let team_user_list = AppletOperationTeamUser::find()
            .filter(
                Expr::col(applet_operation_team_user::Column::OperationId).eq(operation.id.clone()),
            )
            .all(&state.mysql_client)
            .await?;
        println!("team_user_list:{:?}", team_user_list);

        if !team_user_list.is_empty() {
            let join_user_option = team_user_list
                .iter()
                .find(|team_user| team_user.user_id == user_id);
            println!("join_user_option:{:?}", join_user_option);
            if join_user_option.is_some() {
                // 参加了团购 返回队伍列表
                let join_user = join_user_option.unwrap();
                // 获取用户所在team的全部人员
                let team_user_list = AppletOperationTeamUser::find()
                    .filter(
                        Expr::col(applet_operation_team_user::Column::OperationId)
                            .eq(operation.id)
                            .and(
                                Expr::col(applet_operation_team_user::Column::TeamId)
                                    .eq(join_user.team_id.clone()),
                            ),
                    )
                    .all(&state.mysql_client)
                    .await?;
                println!("team_user_list:{:?}", team_user_list);
                let users = AppletUser::find().all(&state.mysql_client).await?;
                let user_map: HashMap<String, applet_user::Model> = users
                    .into_iter()
                    .map(|user| (user.id.clone(), user.clone()))
                    .collect();
                // 组装数据
                let user_response: Vec<TeamUserResponse> = team_user_list
                    .into_iter()
                    .map(|team_user| TeamUserResponse {
                        user_id: team_user.user_id.clone(),
                        username: user_map.get(&team_user.user_id).unwrap().username.clone(),
                        avatar: user_map.get(&team_user.user_id).unwrap().avatar.clone(),
                    })
                    .collect();
                return Ok(ok_result(TeamResponse::new(
                    join_user.team_id.clone(),
                    user_response,
                )));
            }
        }
    }
    Ok(ok_result(TeamResponse::new_none()))
}

/// 创建团队
pub async fn create_team(
    State(state): State<AppState>,
    user: JwtUser,
    ExtractJson(param): ExtractJson<CreateTeamParam>,
) -> ApiResult<String> {
    println!("param:{:?}", param);
    let team = applet_operation_team::ActiveModel {
        id: Set(generate_snowflake_id()?),
        team_user_id: Set(user.id.clone()),
        operation_id: Set(param.operation_id.clone()),
        created_time: Set(Some(Utc::now().naive_local())),
        updated_time: Set(Some(Utc::now().naive_local())),
    }
    .insert(&state.mysql_client)
    .await?;
    // 保存团队用户表
    applet_operation_team_user::ActiveModel {
        id: Set(generate_snowflake_id()?),
        operation_id: Set(param.operation_id),
        team_id: Set(team.id),
        user_id: Set(user.id),
        created_time: Set(Some(Utc::now().naive_local())),
        updated_time: Set(Some(Utc::now().naive_local())),
    }
    .insert(&state.mysql_client)
    .await?;
    Ok(ok_result_with_none())
}

/// 支付
pub async fn pay(
    State(state): State<AppState>,
    user: JwtUser,
    Json(param): Json<UserPayParam>,
) -> ApiResult<String> {
    // 获取用户openid
    let res = user_by_code(
        &state.request_client,
        param.code.clone(),
        state.applet_config.app_id.clone(),
        state.applet_config.secret.clone(),
    )
    .await?;
    if res.openid.is_none() {
        return Ok(error_result("支付发生错误，请稍后再试!"));
    }
    let openid = res.openid.unwrap();

    // 获取用户信息
    let applet_user = AppletUser::find()
        .filter(Expr::col(applet_user::Column::OpenId).is(&openid))
        .one(&state.mysql_client)
        .await?;
    if applet_user.is_none() {
        error!("支付时遇到位置的用户信息:{:?}", openid);
        return Ok(error_result("支付发生错误，请稍后再试!"));
    }

    let applet_user = applet_user.unwrap();

    // 生成支付中间表
    let record = applet_pay_centre_record::ActiveModel {
        id: Set(generate_snowflake_id()?),
        user_id: Set(user.id),
        group_buy_id: Set(param.group_buy_id),
        created_time: Set(Some(Utc::now().naive_local())),
        updated_time: Set(Some(Utc::now().naive_local())),
    };
    record.insert(&state.mysql_client).await?;
    Ok(ok_result_with_none())
}
