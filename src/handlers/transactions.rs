use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use crate::app::App;

use crate::store::{PostgresDB, Store};
use crate::transactions::Transaction;

#[tracing::instrument(
    name = "Create a new transaction.",
    skip(form, app),
    fields(
        title = %form.title,
        amount = %form.amount
    )
)]
pub async fn create_transactions<DB: Store>(form: web::Form<Transaction>, app: web::Data<App<DB>>) -> HttpResponse {
    app.store.;
    HttpResponse::Ok().finish()
}

// transaction_config configure routes for http server
pub fn transaction_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/transaction")
            .route(web::post().to(create_transactions))
    );
}