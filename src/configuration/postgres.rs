#[derive(serde::Deserialize)]
pub struct PostgresConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

impl PostgresConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}


pub fn get_configuration() -> Result<PostgresConfig, config::ConfigError> {
    let config_file = "configs/postgres.yaml";

    let conf = config::Config::builder()
        .add_source(config::File::new(
            config_file,
            config::FileFormat::Yaml,
        ))
        .build()?;
    conf.try_deserialize::<PostgresConfig>()
}