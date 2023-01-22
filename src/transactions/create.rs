use chrono::Utc;
use super::entity;

impl entity::Transaction {
    pub fn create(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_creating_transaction() {
        let mut transaction = entity::Transaction::new();

        transaction.title.push_str("Test transaction");
        transaction.creation_date = Utc::now();
        transaction.amount = 100;
        transaction.type_operation = entity::TypeOperation::Profit;

        assert_eq!((), transaction.create().unwrap())
    }

    #[test]
    #[should_panic(expected = "Не указана сумма")]
    fn error_creating() {
        let mut transaction = entity::Transaction::new();
        assert_eq!()
    }
}
