use crate::core::entity::{AppletConfig, OssConfig, WechatConfig};
use aliyun_oss_rust_sdk::oss::OSS;
use lib_core::RedisService;
use sea_orm::DatabaseConnection;
use std::ops::Deref;
use std::sync::Arc;
use wechat_pay_rust_sdk::pay::WechatPay;

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
    pub oss: OSS,
    pub wechat_pay: WechatPay,
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
        oss: OSS,
        wechat_pay: WechatPay,
    ) -> AppState {
        Self {
            inner: Arc::new(AppStateInner {
                wechat_config,
                applet_config,
                redis_service,
                request_client: reqwest::Client::new(),
                mysql_client,
                oss,
                wechat_pay,
            }),
        }
    }
}
