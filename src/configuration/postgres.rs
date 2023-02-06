use std::convert::{TryFrom, TryInto};
use std::path::Path;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize, Copy, Clone)]
pub struct PostgresConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
    pub require_ssl: bool,
}

impl PostgresConfig {
    /// without_db - returned database options without database name
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
    
    /// without_db - returned database options with database name postgres
    pub fn with_postgres_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database("postgres");
        options.log_statements(log::LevelFilter::Trace);
        options
    }

    /// without_db - returned database options with database name from config file
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.dbname)
    }

}

/// get_configuration parsed postgres configuration from yaml file
pub fn get_configuration(config_path: &str) -> Result<PostgresConfig, config::ConfigError> {
    let config_file = Path::new(config_path).join("postgres.yaml");

    let conf = config::Config::builder()
        .add_source(config::File::new(
            config_file.to_str().unwrap(),
            config::FileFormat::Yaml,
        ))
        .build()?;
    conf.try_deserialize::<PostgresConfig>()
}

