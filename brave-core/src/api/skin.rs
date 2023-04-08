use actix_web::{get, web, HttpResponse, Responder};

pub fn skin_config(_cfg: &mut web::ServiceConfig) {}

//获取皮肤列表
#[get("/skins")]
async fn init_state() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "state": "error"}))
}
