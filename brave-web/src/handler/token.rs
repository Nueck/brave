use actix_web::{post, HttpResponse, Responder};

#[post("/tokencheck")]
pub async fn token_checker_handler() -> impl Responder {
    const MESSAGE: &str = "token availability";
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
