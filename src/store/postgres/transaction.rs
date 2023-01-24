use sqlx::Connection;
use crate::store::Store;
use super::postgresdb::PostgresDB;

impl Store for PostgresDB {
    fn create_transaction() {
        todo!()
    }
}