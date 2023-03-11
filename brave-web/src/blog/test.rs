use actix_web::{get, HttpResponse, Responder};

#[get("/{name}")]
pub async fn token_checker_handler() -> impl Responder {
    const MESSAGE: &str = "BLOG page loaded";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
