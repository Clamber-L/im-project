use super::entity::{
    OperationUserResponse, PayResponse, TeamResponse, TeamUserResponse, UserPayedParam,
};
use crate::api::applet::entity::{
    AppletLoginParam, AppletSettingParam, CreateTeamParam, OperationResponse,
    OperationUserNumParam, OperationUserNumResponse, UserCreationParam, UserLoginResponse,
    UserPayParam, UserTeamParam,
};
use crate::core::entity::WechatPayNotifyParam;
use crate::core::service::wechat_api::{
    access_token, get_user_phone, user_by_code, user_wechat_pay,
};
use crate::core::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use lib_core::{
    generate_jwt, generate_snowflake_id, ApiResult, AppError, ExtractJson, ExtractQuery, JwtUser,
};
use lib_entity::mysql::applet_settings::Model;
use lib_entity::mysql::prelude::{
    AppletOperation, AppletOperationContent, AppletOperationTeam, AppletOperationTeamUser,
    AppletPayCentreRecord, AppletPayRecord, AppletSettings, AppletUser, AppletUserCreation,
};
use lib_entity::mysql::{
    applet_operation, applet_operation_content, applet_operation_team, applet_operation_team_user,
    applet_pay_centre_record, applet_pay_record, applet_settings, applet_user,
    applet_user_creation,
};
use lib_utils::request_entity::PageResult;
use lib_utils::{error_result, ok_result, ok_result_with_none, today_date};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::LockType;
use sea_orm::sqlx::types::chrono::{Local, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    TransactionTrait,
};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};
use wechat_pay_rust_sdk::pay::PayNotifyTrait;

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

pub async fn settings(
    State(state): State<AppState>,
    ExtractQuery(param): ExtractQuery<AppletSettingParam>,
) -> ApiResult<Model> {
    let setting_option = AppletSettings::find()
        .filter(Expr::col(applet_settings::Column::SettingType).eq(param.setting_type))
        .one(&state.mysql_client)
        .await?;
    if let Some(setting) = setting_option {
        Ok(ok_result(setting))
    } else {
        Ok(ok_result_with_none())
    }
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
pub async fn operation(State(state): State<AppState>) -> ApiResult<OperationResponse> {
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
    Ok(error_result("活动不存在或已经结束！"))
}

/// 用户团购身份
pub async fn operation_user(
    State(state): State<AppState>,
    user: JwtUser,
) -> ApiResult<OperationUserResponse> {
    // 获取当前开启的团购
    let operation = match AppletOperation::find()
        .filter(Expr::col(applet_operation::Column::BeOpen).eq(true))
        .one(&state.mysql_client)
        .await?
    {
        Some(op) => op,
        None => {
            return Ok(ok_result(OperationUserResponse {
                has_operation: false,
                operation_name: String::new(),
                commander: false,
                joined: false,
            }));
        }
    };

    let operation_name = operation.name.clone();

    // 查找当前用户是否是团长
    if AppletOperationTeam::find()
        .filter(
            Expr::col(applet_operation_team::Column::OperationId)
                .eq(operation.id.clone())
                .and(Expr::col(applet_operation_team::Column::TeamUserId).eq(user.id.clone())),
        )
        .one(&state.mysql_client)
        .await?
        .is_some()
    {
        Ok(ok_result(OperationUserResponse {
            operation_name,
            has_operation: true,
            commander: true,
            joined: true,
        }))
    } else {
        // 查找是否加入团队
        let joined = AppletOperationTeamUser::find()
            .filter(
                Expr::col(applet_operation_team_user::Column::OperationId)
                    .eq(operation.id)
                    .and(Expr::col(applet_operation_team_user::Column::UserId).eq(user.id)),
            )
            .one(&state.mysql_client)
            .await?
            .is_some();

        Ok(ok_result(OperationUserResponse {
            operation_name,
            has_operation: true,
            commander: false,
            joined,
        }))
    }
}

/// 检查是否加入团购 团购的身份
pub async fn user_team(
    State(state): State<AppState>,
    user: JwtUser,
    ExtractQuery(param): ExtractQuery<UserTeamParam>,
) -> ApiResult<TeamResponse> {
    let user_id = user.id;

    // 检查本次团购是否到期
    let operation_option = AppletOperation::find_by_id(param.operation_id)
        .one(&state.mysql_client)
        .await?;
    if let Some(operation) = operation_option {
        if Local::now().date_naive() > operation.end_time {
            return Ok(error_result("本次活动已结束"));
        }

        let operation_id = operation.id.clone();

        let team_user_list = AppletOperationTeamUser::find()
            .filter(
                Expr::col(applet_operation_team_user::Column::OperationId).eq(operation_id.clone()),
            )
            .all(&state.mysql_client)
            .await?;

        if !team_user_list.is_empty() {
            let join_user_option = AppletOperationTeamUser::find()
                .filter(
                    Expr::col(applet_operation_team_user::Column::OperationId)
                        .eq(operation_id.clone())
                        .and(
                            Expr::col(applet_operation_team_user::Column::UserId)
                                .eq(user_id.clone()),
                        ),
                )
                .one(&state.mysql_client)
                .await?;
            if join_user_option.is_some() {
                // 参加了团购 返回队伍列表
                let join_user = join_user_option.unwrap();
                // 获取用户所在team的全部人员
                let team_user_list = AppletOperationTeamUser::find()
                    .filter(
                        Expr::col(applet_operation_team_user::Column::OperationId)
                            .eq(operation_id.clone())
                            .and(
                                Expr::col(applet_operation_team_user::Column::TeamId)
                                    .eq(join_user.team_id.clone()),
                            ),
                    )
                    .all(&state.mysql_client)
                    .await?;
                let user_ids: Vec<String> =
                    team_user_list.iter().map(|u| u.user_id.clone()).collect();
                let users = AppletUser::find()
                    .filter(Expr::col(applet_user::Column::Id).is_in(user_ids.clone()))
                    .all(&state.mysql_client)
                    .await?;
                let user_map: HashMap<String, applet_user::Model> = users
                    .into_iter()
                    .map(|user| {
                        let id = user.id.clone();
                        (id, user)
                    })
                    .collect();
                // 组装数据
                let user_response: Vec<TeamUserResponse> = team_user_list
                    .into_iter()
                    .filter_map(|team_user| {
                        user_map
                            .get(&team_user.user_id)
                            .map(|user| TeamUserResponse {
                                user_id: team_user.user_id,
                                username: user.username.clone(),
                                avatar: user.avatar.clone(),
                            })
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
    // 判断活动是否到期
    let operation_option = AppletOperation::find_by_id(param.operation_id.clone())
        .one(&state.mysql_client)
        .await?;
    if let Some(operation) = operation_option {
        if Local::now().date_naive() > operation.end_time {
            return Ok(error_result("当前活动已到期，无法参加"));
        }
        // 判断是否已经加入团队
        let team_user_option = AppletOperationTeamUser::find()
            .filter(Expr::col(applet_operation_team_user::Column::UserId).eq(user.id.clone()))
            .one(&state.mysql_client)
            .await?;
        if team_user_option.is_some() {
            return Ok(error_result("你已经参与过本次活动"));
        }
    }

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

/// 团购人数
pub async fn operation_user_num(
    State(state): State<AppState>,
    ExtractQuery(param): ExtractQuery<OperationUserNumParam>,
) -> ApiResult<OperationUserNumResponse> {
    println!("operation_user_num param:{:?}", param);
    let operation_users = AppletOperationTeamUser::find()
        .filter(Expr::col(applet_operation_team_user::Column::OperationId).eq(param.operation_id))
        .all(&state.mysql_client)
        .await?;
    println!("operation users len:{:?}", operation_users.len());

    if operation_users.is_empty() {
        Ok(ok_result(OperationUserNumResponse::new(0, 0, vec![])))
    } else {
        // 判断今日人数
        let (start, end) = today_date();
        let today_num = operation_users
            .iter()
            .filter_map(|user| user.created_time)
            .filter(|&dt| dt > start.naive_local() && dt < end.naive_local())
            .count();
        // 获取用户列表信息
        let users = AppletUser::find().all(&state.mysql_client).await?;
        let user_map: HashMap<String, applet_user::Model> = users
            .into_iter()
            .map(|user| {
                let id = user.id.clone();
                (id, user)
            })
            .collect();
        let user_response: Vec<TeamUserResponse> = operation_users
            .iter()
            .filter_map(|team_user| {
                user_map
                    .get(&team_user.user_id)
                    .map(|user| TeamUserResponse {
                        user_id: team_user.user_id.clone(),
                        username: user.username.clone(),
                        avatar: user.avatar.clone(),
                    })
            })
            .collect();

        Ok(ok_result(OperationUserNumResponse::new(
            today_num,
            operation_users.len(),
            user_response,
        )))
    }
}

/// 支付
pub async fn create_team_pay(
    State(state): State<AppState>,
    user: JwtUser,
    ExtractJson(param): ExtractJson<UserPayParam>,
) -> ApiResult<PayResponse> {
    // 获取用户信息
    let applet_user = AppletUser::find_by_id(user.id.clone())
        .one(&state.mysql_client)
        .await?;
    if applet_user.is_none() {
        error!("支付时遇到位置的用户信息:{:?}", user);
        return Ok(error_result("支付发生错误，请稍后再试!"));
    }

    let applet_user = applet_user.unwrap();

    let operation_option = AppletOperation::find_by_id(param.operation_id.clone())
        .one(&state.mysql_client)
        .await?;

    let operation = match operation_option {
        None => return Ok(error_result("活动已结束!")),
        Some(operation) if operation.end_time < Utc::now().date_naive() => {
            return Ok(error_result("活动已结束!"));
        }
        Some(operation) => operation,
    };
    // 判断用户是否在本次活动中支付完成
    let pay_record_option = AppletPayRecord::find()
        .filter(
            Expr::col(applet_pay_record::Column::OperationId)
                .eq(param.operation_id)
                .and(Expr::col(applet_pay_record::Column::UserId).eq(user.id.clone())),
        )
        .one(&state.mysql_client)
        .await?;
    if pay_record_option.is_some() {
        return Ok(error_result("不允许重复支付同一次活动!"));
    }

    // 创建支付中间表
    let centre_record = applet_pay_centre_record::ActiveModel {
        id: Set(generate_snowflake_id()?),
        user_id: Set(user.id.clone()),
        amount: Set(operation.amount.clone()),
        operation_id: Set(operation.id),
        create_team: Set(param.create_team),
        payed: Set(false),
        join_team_id: Set(param.join_team_id),
        created_time: Set(Some(Utc::now().naive_local())),
        updated_time: Default::default(),
    }
    .insert(&state.mysql_client)
    .await?;

    // 调用微信支付
    let pay_res = user_wechat_pay(
        &state.wechat_pay,
        operation.name.clone(),
        centre_record.id,
        operation.amount.parse().unwrap(),
        applet_user.clone().open_id,
    )
    .await?;
    println!("pay res:{:?}", pay_res);
    if pay_res.prepay_id.is_none() || pay_res.sign_data.is_none() {
        return Ok(error_result("支付失败"));
    };

    Ok(ok_result(PayResponse::new(
        pay_res.prepay_id.unwrap(),
        pay_res.sign_data.unwrap(),
    )))
}

pub async fn pay_callback(
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<WechatPayNotifyParam>,
) -> Result<impl IntoResponse, AppError> {
    info!("接收到微信支付回调");
    println!("param:{:?}", param);
    let data = &state.wechat_pay.decrypt_paydata(
        param.resource.ciphertext,
        param.resource.nonce,
        param.resource.associated_data,
    )?;
    info!("data:{:?}", data);
    println!("out_trade_no:{:?}", &data.out_trade_no);
    // 获取中间支付表信息
    sleep(Duration::from_secs(2)).await;
    let centre_record_option = AppletPayCentreRecord::find_by_id(&data.out_trade_no)
        .lock(LockType::Update)
        .one(&state.mysql_client)
        .await?;
    println!("centre record:{:?}", centre_record_option);
    if centre_record_option.is_some() {
        let centre_record = centre_record_option.unwrap();
        // 开启事务
        let txn = state.mysql_client.begin().await?;
        let mut active_centre_record: applet_pay_centre_record::ActiveModel =
            centre_record.clone().into();
        active_centre_record.payed = Set(true);
        active_centre_record.update(&state.mysql_client).await?;

        // 生成订单表
        applet_pay_record::ActiveModel {
            id: Set(generate_snowflake_id()?),
            user_id: Set(centre_record.user_id.clone()),
            trade_state: Set(data.trade_state.clone()),
            trade_state_desc: Set(data.trade_state_desc.clone()),
            success_time: Set(data.success_time.clone()),
            openid: Set(data.payer.openid.clone()),
            amount: Set(centre_record.amount),
            created_time: Set(Some(Utc::now().naive_local())),
            updated_time: Set(Some(Utc::now().naive_local())),
            transaction_id: Set(data.transaction_id.clone()),
            order_state: Set(1),
            operation_id: Set(centre_record.operation_id.clone()),
        }
        .insert(&state.mysql_client)
        .await?;

        // 判断是否创建团购
        if centre_record.create_team {
            // 创建团队
            let team = applet_operation_team::ActiveModel {
                id: Set(generate_snowflake_id()?),
                team_user_id: Set(centre_record.user_id.clone()),
                operation_id: Set(centre_record.operation_id.clone()),
                created_time: Set(Some(Utc::now().naive_local())),
                updated_time: Set(Some(Utc::now().naive_local())),
            }
            .insert(&state.mysql_client)
            .await?;
            // 加入团队
            applet_operation_team_user::ActiveModel {
                id: Set(generate_snowflake_id()?),
                operation_id: Set(centre_record.operation_id),
                team_id: Set(team.id),
                user_id: Set(centre_record.user_id),
                created_time: Set(Some(Utc::now().naive_local())),
                updated_time: Set(Some(Utc::now().naive_local())),
            }
            .insert(&state.mysql_client)
            .await?;
        } else {
            // 加入团队
            let join_team_id = centre_record.join_team_id;
            applet_operation_team_user::ActiveModel {
                id: Set(generate_snowflake_id()?),
                operation_id: Set(centre_record.operation_id),
                team_id: Set(join_team_id),
                user_id: Set(centre_record.user_id),
                created_time: Set(Some(Utc::now().naive_local())),
                updated_time: Set(Some(Utc::now().naive_local())),
            }
            .insert(&state.mysql_client)
            .await?;
        }

        txn.commit().await?;
    }

    let mut res_map = HashMap::new();
    res_map.insert("code", "SUCCESS");
    res_map.insert("message", "成功");
    Ok((StatusCode::OK, Json(res_map)))
}

pub async fn user_pay(
    State(state): State<AppState>,
    _user: JwtUser,
    ExtractJson(param): ExtractJson<UserPayedParam>,
) -> ApiResult<()> {
    let pay_record_option = AppletPayRecord::find()
        .filter(Expr::col(applet_pay_record::Column::UserId).eq(param.payed_user_id))
        .one(&state.mysql_client)
        .await?;
    if pay_record_option.is_none() {
        return Ok(error_result("邀请者未支付，你不能加入他的团队"));
    }
    Ok(ok_result_with_none())
}
