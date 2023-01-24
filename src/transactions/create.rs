use chrono::Utc;
use super::entity;


impl entity::Transaction {
    /// Create transaction in database
    pub fn create(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_creating_transaction() {

        let mut _transaction = entity::Transaction::new();

        _transaction.title.push_str("Test transaction");
        // _transaction.creation_date = Utc::now();
        _transaction.amount = 100;
        _transaction.type_operation = entity::TypeOperation::Profit;
        _transaction.create().unwrap();
        // assert_eq!((), transaction.create().unwrap())

    }

    #[test]
    #[should_panic(expected = "__")]
    fn error_creating() {
        let _transaction = entity::Transaction::new();
        panic!("__")
    }
}
