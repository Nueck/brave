use actix_web::{get, post, web, HttpResponse, Responder};
use brave_config::utils::jwt::{Claims, UserDataInfo, GLOB_JOT};
use brave_config::GLOBAL_CONFIG;
use jsonwebtoken::get_current_timestamp;

pub fn token_config(cfg: &mut web::ServiceConfig) {
    cfg.service(token_checker_handler)
        .service(update_token_handler);
}

#[get("/tokencheck")]
async fn token_checker_handler() -> impl Responder {
    const MESSAGE: &str = "token availability";
    HttpResponse::Ok().json(serde_json::json!({"state": "success", "message": MESSAGE}))
}

#[post("/updateToken")]
async fn update_token_handler(token: web::ReqData<UserDataInfo>) -> impl Responder {
    let refresh = &token.refresh;
    let auth = &token.auth;
    let aud = &token.auth;
    let id = &token.id;

    if refresh.to_owned() {
        //短时间的token
        let claims = Claims {
            id: id.to_owned(),
            aud: aud.to_owned(),
            sub: GLOBAL_CONFIG.jwt.get_sub(),
            exp: get_current_timestamp() + GLOBAL_CONFIG.jwt.get_exp_time(),
            auth: auth.to_string(),
            data: None,
            refresh: false,
        };
        let token = GLOB_JOT.generate_token(&claims);

        //长时间的token
        let claims = Claims {
            id: id.to_owned(),
            aud: aud.to_owned(),
            sub: GLOBAL_CONFIG.jwt.get_sub(),
            exp: get_current_timestamp() + GLOBAL_CONFIG.jwt.get_ref_time(),
            auth: auth.to_string(),
            data: None,
            refresh: true,
        };
        let ref_token = GLOB_JOT.generate_token(&claims);
        let json = serde_json::json!({"state": "success",  "data":{"token": token ,"refreshToken": ref_token} });

        HttpResponse::Ok().json(json)
    } else {
        const MSG: &str = "Not refresh token";
        let json = serde_json::json!({"state": "error",  "message":MSG });

        HttpResponse::Ok().json(json)
    }
}
