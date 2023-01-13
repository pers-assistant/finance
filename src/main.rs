use std::net::TcpListener;
use std::process;

use finance::http_server::run;
use finance::configuration::AppConfig;
use finance::telemetry::logging::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()>{

    // Init configuration
    let app_config = AppConfig::new().unwrap_or_else(|err| {
        eprintln!("Err init configuration: {}", err);
         process::exit(1);
    });

    //init logger
    let subscriber = get_subscriber(
        "finance".into(),
        app_config.logger.log_level,
        std::io::stdout
    );
    init_subscriber(subscriber);

    let address = format!("{}:{}", app_config.http.address, app_config.http.port);
    println!("Finance service running {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await?;
    Ok(())
}
