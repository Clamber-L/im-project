use anyhow::Result;
use mongodb::{Client, Database as MongoDB};
use redis::aio::MultiplexedConnection;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn mongo_client(url: &str, db_name: &str) -> MongoDB {
    let client = Client::with_uri_str(url).await.unwrap();
    let database = client.database(db_name);
    database
}

pub async fn mysql_client(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}

pub async fn redis_client(redis_url: &str) -> Result<MultiplexedConnection> {
    let client = redis::Client::open(redis_url)?;
    let connection = client.get_multiplexed_async_connection().await?;
    Ok(connection)
}
