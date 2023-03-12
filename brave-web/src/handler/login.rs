use crate::config::GLOBAL_YAML_CONFIG;
use actix_web::{get, HttpResponse, Responder};
use brave_middleware::GLOB_JOT;
use brave_utils::jwt::jwt::Claims;
use jsonwebtoken::get_current_timestamp;

#[get("/login")]
pub async fn login() -> impl Responder {
    //登陆成功后
    let claims = Claims {
        sub: "cako-blog".to_string(),
        exp: get_current_timestamp() + GLOBAL_YAML_CONFIG.jwt.exp_time.unwrap(),
    };
    let token = GLOB_JOT.generate_token(&claims);



    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": token }))
}
