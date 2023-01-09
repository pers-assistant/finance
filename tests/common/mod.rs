use std::net::TcpListener;
use finance::http_server::run;


pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);


    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    address
}