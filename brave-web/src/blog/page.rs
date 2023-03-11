use actix_web::{get,HttpResponse, Responder};

/*用于页面的加载*/
#[get("/{name}")]
pub async fn page_handler() -> impl Responder {
    const MESSAGE: &str = "BLOG page loaded";
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
