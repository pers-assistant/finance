use std::net::TcpListener;
use std::process;

use finance::http_server::run;
use finance::configuration::AppConfig;

#[tokio::main]
async fn main() -> std::io::Result<()>{

    // Init configuration
    let app_config = AppConfig::new().unwrap_or_else(|err| {
        eprintln!("Err init configuartion: {}", err);
         process::exit(1);
    });

    let address = format!("{}:{}", app_config.http.address, app_config.http.port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await?;
    Ok(())
}
