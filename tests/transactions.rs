mod common;

#[tokio::test]
async fn add_transactions() {
    // Arrange
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/transaction", &app.address))
        .json("{'sum': '100.0'}")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}