use mongodb::Database;
use sea_orm::DatabaseConnection;
use std::ops::Deref;
use std::sync::Arc;

use crate::ConnectionMap;

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub struct AppStateInner {
    pub mongo_database: Database,
    pub mysql_client: DatabaseConnection,
    pub connection_map: ConnectionMap,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(
        mongo_database: Database,
        mysql_client: DatabaseConnection,
        connection_map: ConnectionMap,
    ) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                mongo_database,
                mysql_client,
                connection_map,
            }),
        }
    }
}
