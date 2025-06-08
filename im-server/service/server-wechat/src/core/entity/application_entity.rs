use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationEntity {
    pub port: String,
    pub wechat: WechatConfig,
    pub applet: AppletConfig,
    pub redis: RedisConfig,
    pub mysql: MysqlConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WechatConfig {
    pub app_id: String,
    pub secret: String,
    pub redirect_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppletConfig {
    pub app_id: String,
    pub secret: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MysqlConfig {
    pub url: String,
}

impl ApplicationEntity {
    pub fn try_load_yml() -> Result<Self> {
        let file = File::open("server-wechat.yml")?;
        let config: ApplicationEntity = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}
