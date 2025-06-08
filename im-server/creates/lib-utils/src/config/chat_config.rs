use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerChatConfig {
    pub server: ServerConfig,
    pub mongo: MongoConfig,
    pub mysql: MysqlConfig,
    pub nacos: NacosConfig,
    pub grpc: GrpcConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MongoConfig {
    pub url: String,
    pub db_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MysqlConfig {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NacosConfig {
    pub addr: String,
    pub namespace: String,
    pub app_name: String,
    pub group: String,
    pub service_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GrpcConfig {
    pub port: u16,
}

impl ServerChatConfig {
    pub fn try_load() -> Result<Self> {
        let result = match (
            File::open("app.yml"),
            File::open("/etc/config/app.yml"),
            env::var("SERVER_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config Not Found"),
        };
        Ok(result?)
    }
}
