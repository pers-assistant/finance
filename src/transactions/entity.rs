use chrono::{DateTime, Utc};

pub enum TypeOperation {
    Profit,
    Expense,
    Transfer,
}

pub struct Transaction {
    pub(crate) title: String,
    pub(crate) creation_date: DateTime<Utc>,
    pub(crate) amount: u32,
    pub(crate) type_operation: TypeOperation
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction{
            title: String::new(),
            creation_date: Default::default(),
            amount: 0,
            type_operation: TypeOperation::Profit,
        }
    }
}