use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;


use crate::handlers::{health_check, add_transactions};

// struct Http<T> {
//     server: Server,
//     handlers: Vec<T>
// }

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    println!("Service started bind: {}", listener.local_addr().unwrap());
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("health_check", web::get().to(health_check))
            .route("transaction", web::post().to(add_transactions))
            .app_data(db_pool.clone())
        //todo!("Вынести добавление хэндлеров в отдельную функцию");
        //todo!("Добавить абстракцию между http сервером и типом БД");
    })
    .listen(listener)?
    .run();
    Ok(server)
}