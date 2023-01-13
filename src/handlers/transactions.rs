use actix_web::HttpResponse;

#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(),
)]
pub async fn add_transactions() -> HttpResponse {
    HttpResponse::Ok().finish()
}