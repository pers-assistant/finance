use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::store::Store;
use crate::transactions::Transaction;

#[tracing::instrument(
    name = "Create a new transaction.",
    skip(form, pool),
    fields(
        title = %form.title,
        amount = %form.amount
    )
)]
pub async fn create_transactions(form: web::Form<Transaction>, pool: web::Data<PgPool>) -> HttpResponse {
    // let transaction = Transaction {
    //     title: form.0.title,
    //     amount: form.0.amount,
    //     type_operation: form.0.type_operation,
    // };
    HttpResponse::Ok().finish()
}

// this function could be located in a different module
pub fn transaction_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/transaction")
            .route(web::post().to(create_transactions))
    );
}