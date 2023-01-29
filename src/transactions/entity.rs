use chrono::{DateTime, Utc};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum TypeOperation {
    Profit,
    Expense,
    Transfer,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Transaction {
    pub title: String,
    // pub creation_date: DateTime<Utc>,
    // pub date_operation: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub amount: u32,
    pub type_operation: TypeOperation
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction{
            title: String::new(),
            // creation_date: Utc::now(),
            // date_operation: Utc::now(),
            amount: 0,
            type_operation: TypeOperation::Profit,
        }
    }
}