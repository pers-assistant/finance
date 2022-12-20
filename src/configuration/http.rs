

#[derive(serde::Deserialize)]
pub struct HttpConfig {
    pub address: String,
    pub port: String,
}

impl HttpConfig {
}

pub fn get_configuration() -> Result<HttpConfig, config::ConfigError> {
    let config_file = "configs/http.yaml";

    let conf = config::Config::builder()
        .add_source(config::File::new(
            config_file,
            config::FileFormat::Yaml,
        ))
        .build()?;
    conf.try_deserialize::<HttpConfig>()
}