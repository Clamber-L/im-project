use crate::core::entity::{AppletConfig, OssConfig, PayConfig, WechatConfig};
use lib_core::RedisService;
use sea_orm::DatabaseConnection;
use std::ops::Deref;
use std::sync::Arc;

pub mod constants;
pub mod entity;
pub mod service;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub wechat_config: WechatConfig,
    pub applet_config: AppletConfig,
    pub redis_service: RedisService,
    pub request_client: reqwest::Client,
    pub mysql_client: DatabaseConnection,
    pub oss_config: OssConfig,
    pub pay_config: PayConfig,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(
        wechat_config: WechatConfig,
        applet_config: AppletConfig,
        redis_service: RedisService,
        mysql_client: DatabaseConnection,
        oss_config: OssConfig,
        pay_config: PayConfig,
    ) -> AppState {
        Self {
            inner: Arc::new(AppStateInner {
                wechat_config,
                applet_config,
                redis_service,
                request_client: reqwest::Client::new(),
                mysql_client,
                oss_config,
                pay_config,
            }),
        }
    }
}
