use std::{ops::Deref, sync::Arc};

use lib_rpc::pb::message::message_service_client::MessageServiceClient;
use sea_orm::DatabaseConnection;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub struct AppStateInner {
    pub mysql_client: DatabaseConnection,
    pub grpc_client: MessageServiceClient<Channel>,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(
        mysql_client: DatabaseConnection,
        grpc_client: MessageServiceClient<Channel>,
    ) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                mysql_client,
                grpc_client,
            }),
        }
    }
}
