use std::path::Path;

#[derive(serde::Deserialize)]
pub struct LoggerConfig {
    pub log_level: String,
}


pub fn get_configuration(config_path: &str) -> Result<LoggerConfig, config::ConfigError> {
    let config_file = Path::new(config_path).join("logger.yaml");

    let conf = config::Config::builder()
        .add_source(config::File::new(
            config_file.to_str().unwrap(),
            config::FileFormat::Yaml,
        ))
        .build()?;
    conf.try_deserialize::<LoggerConfig>()
}