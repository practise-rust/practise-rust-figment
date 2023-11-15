use anyhow::Result;
use figment::{
    providers::{Env, Format, Toml, Yaml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct IpAddr {
    pub ips: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

pub fn load_config() -> Result<Config> {
    let config = Figment::new()
        .merge(Toml::file("config/default.toml"))
        .merge(Toml::file("config/local.toml"))
        .merge(Yaml::file("config/prod.yaml"))
        .merge(Env::prefixed("APP_").split("_"))
        .extract();

    config.map_err(|e| e.into())
}
