use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use crate::handlers::{health_check};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new().route("health_check", web::get().to(health_check))
    })
        .listen(listener)?
        .run();
    Ok(server)
}