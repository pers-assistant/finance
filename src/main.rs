use std::net::TcpListener;
use finance::http_server::run;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let address = format!("127.0.0.1:{}", 8000);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await?;
    Ok(())
}
