use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApplicationEntity {
    pub port: String,
    pub wechat: WechatConfig,
    pub applet: AppletConfig,
    pub redis: RedisConfig,
    pub mysql: MysqlConfig,
    pub oss: OssConfig,
    pub pay: PayConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WechatConfig {
    pub app_id: String,
    pub secret: String,
    pub redirect_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppletConfig {
    pub app_id: String,
    pub secret: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MysqlConfig {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OssConfig {
    pub key: String,
    pub secret: String,
    pub bucket: String,
    pub end_point: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayConfig {
    pub app_id: String,
    pub mch_id: String,
    pub v3_key: String,
    pub notify_url: String,
    pub key_path: String,
    pub serial_no: String,
}

impl ApplicationEntity {
    pub fn try_load_yml() -> Result<Self> {
        let file = File::open("server-wechat.yml")?;
        let config: ApplicationEntity = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}
