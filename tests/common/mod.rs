use std::net::TcpListener;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use finance::http_server::run;
use finance::telemetry::logging::{get_subscriber, init_subscriber};
use finance::configuration::{AppConfig, PostgresConfig};

use once_cell::sync::Lazy;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/// Subscribe to logging tests
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

/// Start server for testing
pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // Init configuration
    let mut app_config = AppConfig::new("configs/").unwrap();


    // let mut configuration = get_configuration().expect("Failed to read configuration.");
    app_config.db.dbname = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&app_config.db).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &PostgresConfig) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.with_postgres_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.dbname))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_postgres_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}