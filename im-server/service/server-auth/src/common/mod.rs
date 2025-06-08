pub mod auth;

use anyhow::{anyhow, Result};
use lib_entity::mysql::app_user::{Entity as AppUser, Model};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn user_by_id(id: String, conn: &DatabaseConnection) -> Result<Model> {
    let app_user = AppUser::find_by_id(id).all(conn).await?;

    if !app_user.is_empty() {
        return Ok(app_user.into_iter().next().unwrap());
    }
    Err(anyhow!("查询失败").into())
}
