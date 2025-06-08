// use axum::{extract::Query, response::IntoResponse, Json};
// use axum_extra::extract::WithRejection;
// use lib_core::{get_mysql_pool_or_error, AppError};
// use lib_entity::mysql::litemall_car_record::{self, Entity as LitemallCarRecord};
// use lib_utils::HttpResult;
// use sea_orm::{prelude::Expr, EntityTrait, QueryFilter, QueryOrder, QueryTrait};
// use serde::{Deserialize, Serialize};

// use super::JsonParam;

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub(crate) struct CarRecordListParam {
//     pub limit: i32,
//     pub license: Option<String>,
//     #[serde(rename = "lastId")]
//     pub last_id: Option<u64>,
// }

// pub async fn car_record_list_handler(
//     Query(param): Query<CarRecordListParam>,
// ) -> ApiResult {
//     let connection = get_mysql_pool_or_error()?;

//     let result = LitemallCarRecord::find()
//         .apply_if(param.license, |query, v| {
//             query.filter(Expr::col(litemall_car_record::Column::License).eq(v))
//         })
//         .apply_if(param.last_id, |query, v| {
//             query.filter(Expr::col(litemall_car_record::Column::Id).gt(v))
//         })
//         .order_by_desc(litemall_car_record::Column::Id)
//         .all(&connection)
//         .await?;
//     Ok(HttpResult::ok(result))
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub(crate) struct UpdateCardRecordParam {
//     pub id: i32,
//     pub start_time: Option<String>,
//     pub stopwatch_start: Option<String>,
//     pub stopwatch_end: Option<String>,
//     pub return_time: Option<String>,
// }

// pub async fn update_car_record(
//     WithRejection(Json(param), _): JsonParam<UpdateCardRecordParam>,
// ) -> ApiResult {
//     let connection = get_mysql_pool_or_error()?;

//     LitemallCarRecord::update_many()
//         .apply_if(param.start_time, |query, v| {
//             query.col_expr(litemall_car_record::Column::StartTime, Expr::value(v))
//         })
//         .apply_if(param.stopwatch_start, |query, v| {
//             query.col_expr(litemall_car_record::Column::StopwatchStart, Expr::value(v))
//         })
//         .apply_if(param.stopwatch_end, |query, v| {
//             query.col_expr(litemall_car_record::Column::StopwatchEnd, Expr::value(v))
//         })
//         .apply_if(param.return_time, |query, v| {
//             query.col_expr(litemall_car_record::Column::ReturnTime, Expr::value(v))
//         })
//         .filter(Expr::col(litemall_car_record::Column::Id).eq(param.id))
//         .exec(&connection)
//         .await?;
//     Ok(HttpResult::<String>::ok_with_message())
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub(crate) struct DeleteCarRecordParam {
//     pub id: i32,
// }

// pub async fn delete_car_record(
//     WithRejection(Json(param), _): JsonParam<DeleteCarRecordParam>,
// ) -> ApiResult {
//     let connection = get_mysql_pool_or_error()?;

//     LitemallCarRecord::update_many()
//         .col_expr(litemall_car_record::Column::Deleted, Expr::value(true))
//         .filter(Expr::col(litemall_car_record::Column::Id).eq(param.id))
//         .exec(&connection)
//         .await?;
//     Ok(HttpResult::<String>::ok_with_message())
// }
