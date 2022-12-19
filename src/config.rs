pub struct Config {
    pub http: HttpConfig,
    pub db: DatabaseConfig,
}

pub struct HttpConfig {
    pub ipaddress: String,
    pub port: String,
}

pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}