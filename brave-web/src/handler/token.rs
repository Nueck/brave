use crate::config::GLOBAL_YAML_CONFIG;
use actix_web::{post, web, HttpResponse, Responder};
use brave_utils::jwt::jwt::{Claims, TokenData, GLOB_JOT};
use jsonwebtoken::get_current_timestamp;

#[post("/tokencheck")]
pub async fn token_checker_handler() -> impl Responder {
    const MESSAGE: &str = "token availability";
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[post("/updateToken")]
pub async fn update_token_handler(token: web::ReqData<TokenData>) -> impl Responder {
    let refresh = &token.refresh;
    let auth = &token.auth;

    if *refresh {
        //短时间的token
        let claims = Claims {
            sub: GLOBAL_YAML_CONFIG.jwt.get_sub(),
            exp: get_current_timestamp() + GLOBAL_YAML_CONFIG.jwt.get_exp_time(),
            auth: auth.to_string(),
            data: None,
            refresh: false,
        };
        let token = GLOB_JOT.generate_token(&claims);

        //长时间的token
        let claims = Claims {
            sub: GLOBAL_YAML_CONFIG.jwt.get_sub(),
            exp: get_current_timestamp() + GLOBAL_YAML_CONFIG.jwt.get_ref_time(),
            auth: auth.to_string(),
            data: None,
            refresh: true,
        };
        let ref_token = GLOB_JOT.generate_token(&claims);
        let json = serde_json::json!({"status": "success",  "data":{"token": token ,"refreshToken": ref_token} });

        HttpResponse::Ok().json(json)
    } else {
        const MSG: &str = "Not refresh token";
        let json = serde_json::json!({"status": "error",  "messages":MSG });

        HttpResponse::Ok().json(json)
    }
}
