use std::net::TcpListener;
use finance::http_server::run;
use finance::telemetry::logging::{get_subscriber, init_subscriber};

use once_cell::sync::Lazy;

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
pub async fn spawn_app() -> String {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);


    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    address
}