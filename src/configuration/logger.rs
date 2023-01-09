

#[derive(serde::Deserialize)]
pub struct LoggerConfig {
    pub log_level: String,
}


pub fn get_configuration() -> Result<LoggerConfig, config::ConfigError> {
    let config_file = "configs/logger.yaml";

    let conf = config::Config::builder()
        .add_source(config::File::new(
            config_file,
            config::FileFormat::Yaml,
        ))
        .build()?;
    conf.try_deserialize::<LoggerConfig>()
}