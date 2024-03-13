use config::{self, Config};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub user: String,
    pub password: String,
    pub db_port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_name, self.password, self.host, self.db_port, self.db_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.password, self.host, self.db_port
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    dotenv().ok();
    let mut settings = Config::default();
    for (key, value) in env::vars() {
        settings.set(
            &format!("{}.{}", env::consts::OS, key.to_ascii_uppercase()),
            value,
        )?;
    }

    settings.try_into()
}
