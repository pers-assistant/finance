// postgres.rs
use crate::configuration::PostgresConfig;

#[derive(Clone, Copy)]
pub struct PostgresDB {
    // pub connection: Connection,
    pub config: PostgresConfig
}


