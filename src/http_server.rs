use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;


use crate::handlers::{health_check, add_transactions};

// struct Http<T> {
//     server: Server,
//     handlers: Vec<T>
// }

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    println!("Service started bind: {}", listener.local_addr().unwrap());
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("health_check", web::get().to(health_check))
            .route("transaction", web::post().to(add_transactions))
        //todo!("Вынести добавление хэндлеров в отдельную функцию");
    })
    .listen(listener)?
    .run();
    Ok(server)
}