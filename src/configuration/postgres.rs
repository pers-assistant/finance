use std::convert::{TryFrom, TryInto};
use std::path::Path;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize)]
pub struct PostgresConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,

    pub require_ssl: bool,
}

impl PostgresConfig {
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

    pub fn with_postgres_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database("postgres");
        options.log_statements(log::LevelFilter::Trace);
        options
    }


    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.dbname)
    }


}


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