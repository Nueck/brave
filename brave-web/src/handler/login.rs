use crate::config::{AppState, GLOBAL_YAML_CONFIG};
use crate::entity::prelude::Users;
use crate::entity::users;
use actix_web::{post, web, HttpResponse};
use brave_utils::common::is_valid_email;
use brave_utils::jwt::jwt::Claims;
use brave_utils::jwt::jwt::GLOB_JOT;
use jsonwebtoken::get_current_timestamp;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

/*
* 登陆
*/
#[post("/login")]
pub async fn login(data: web::Data<AppState>, user_info: web::Json<UserInfo>) -> HttpResponse {
    /*
     * 登陆获取,
     * 对密码加密
     */
    let pwd = GLOBAL_YAML_CONFIG
        .blake
        .generate_with_salt(&user_info.password);

    let db = &data.conn;
    /*
     *获取user数据
     */
    match Users::find()
        .filter(
            //断用户名是否是邮箱
            if is_valid_email(&user_info.username) {
                users::Column::UserEmail.contains(&user_info.username)
            } else {
                users::Column::UserName.contains(&user_info.username)
            },
        )
        .one(db)
        .await
        .expect("Could not find Users -- Login")
    {
        None => {
            const MSG: &str = "User does not exist";
            HttpResponse::Ok().json(serde_json::json!({"status": "nonexistence", "message": MSG }))
        }
        Some(user) => {
            /*进行密码比对*/
            if pwd == user.pwd_hash {
                //短时间的token
                let claims = Claims {
                    sub: GLOBAL_YAML_CONFIG.jwt.get_sub(),
                    exp: get_current_timestamp() + GLOBAL_YAML_CONFIG.jwt.get_exp_time(),
                    auth: user.user_authority.clone(),
                };
                let token = GLOB_JOT.generate_token(&claims);

                //长时间的token
                let claims = Claims {
                    sub: GLOBAL_YAML_CONFIG.jwt.get_sub(),
                    exp: get_current_timestamp() + GLOBAL_YAML_CONFIG.jwt.get_ref_time(),
                    auth: user.user_authority,
                };
                let ref_token = GLOB_JOT.generate_token(&claims);

                HttpResponse::Ok().json(serde_json::json!({"status": "success",  "data":{"token": token ,"refreshToken": ref_token} ,}))
            } else {
                const MSG: &str = "Password error";
                HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
            }
        }
    }
}

/*注册*/
#[post("/register")]
pub async fn register(data: web::Data<AppState>) -> HttpResponse {
    /*判断邮箱地址*/
    const MSG: &str = "Password error";
    HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
}

// /*获取验证码*/
// #[post("/verification")]
// pub async fn verification_code(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
//     const MSG: &str = "Password error";
//     Ok(HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG })))
// }

// #[post("/sendmail")]
// pub async fn sendmail(data: web::Data<AppState>) -> Result<HttpResponse> {
//     const MSG: &str = "Password error";
//     Ok(HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG })))
// }
