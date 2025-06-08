use std::{ops::Deref, sync::Arc};

use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub struct AppStateInner {
    pub mysql_client: DatabaseConnection,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(mysql_client: DatabaseConnection) -> Self {
        Self {
            inner: Arc::new(AppStateInner { mysql_client }),
        }
    }
}
