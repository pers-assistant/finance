use sqlx::Connection;

pub trait Store {
    fn create_transaction();
}