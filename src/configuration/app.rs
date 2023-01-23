use crate::configuration::{http, postgres, logger};

pub struct AppConfig {
    pub http: http::HttpConfig,
    pub db: postgres::PostgresConfig,
    pub logger: logger::LoggerConfig
}

impl AppConfig {
    pub fn new(config_path: &str) -> Result<AppConfig, &'static str> {
        let http_config = http::get_configuration(config_path).unwrap();
        let postgres_config = postgres::get_configuration(config_path).unwrap();
        let logger_config = logger::get_configuration(config_path).unwrap();

        let app_config = AppConfig{
            http: http_config,
            db: postgres_config,
            logger: logger_config,
        };

        Ok(app_config)
    }
}
