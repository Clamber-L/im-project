use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "applet_pay_record")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id: String,
    pub user_id: String,
    pub trade_state: String,
    pub trade_state_desc: String,
    pub success_time: String,
    pub openid: String,
    pub amount: String,
    pub order_state: i32,
    pub operation_id: String,
    pub transaction_id: String,
    pub created_time: Option<DateTime>,
    pub updated_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
