use std::path::Path;

#[derive(serde::Deserialize)]
pub struct HttpConfig {
    pub address: String,
    pub port: String,
}

impl HttpConfig {
}

pub fn get_configuration(config_path: &str) -> Result<HttpConfig, config::ConfigError> {
    let config_file = Path::new(config_path).join( "http.yaml");

    let conf = config::Config::builder()
        .add_source(config::File::new(
            config_file.to_str().unwrap(),
            config::FileFormat::Yaml,
        ))
        .build()?;
    conf.try_deserialize::<HttpConfig>()
}