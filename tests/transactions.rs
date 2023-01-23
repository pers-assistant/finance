mod common;
use sqlx;
use chrono::Utc;


#[tokio::test]
async fn add_transactions() {
    // Arrange
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    let title = "Тестовая транзакция";
    let amount = 100;
    let type_operation = "Profit";

    let mut _transaction = format!("{{
    `title`: `{}`,
    `amount`: `{}`,
    `type_operation`: `{}`
    }}", title, amount, type_operation);

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/transaction", &app.address))
        .json(&_transaction)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    let saved = sqlx::query!("SELECT title, amount, type_operation FROM transaction")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.title, title);
    assert_eq!(saved.amount, amount);
}