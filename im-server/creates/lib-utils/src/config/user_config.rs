use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerUserConfig {
    pub server: ServerConfig,
    pub mysql: MysqlConfig,
    pub grpc: GrpcConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MysqlConfig {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GrpcConfig {
    pub url: String,
}

impl ServerUserConfig {
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
