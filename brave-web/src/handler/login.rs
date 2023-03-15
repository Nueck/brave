use crate::config::{AppState, GLOBAL_YAML_CONFIG};
use crate::entity::prelude::Users;
use crate::entity::users;
use crate::entity::users::Model;
use actix_web::{post, web, HttpResponse};
use brave_utils::common::{generation_random_number, is_valid_email};
use brave_utils::jwt::jwt::Claims;
use brave_utils::jwt::jwt::GLOB_JOT;
use brave_utils::mail::MailConfig;
use jsonwebtoken::get_current_timestamp;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct MailInfo {
    pub email: String,
}

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub verify_code: String,
    pub code: String,
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
pub async fn register(data: web::Data<AppState>, info: web::Json<RegisterInfo>) -> HttpResponse {
    /*判断邮箱地址是否存在或在用户名*/
    let db = &data.conn;
    match Users::find()
        .filter(users::Column::UserEmail.contains(&info.email))
        .filter(users::Column::UserName.contains(&info.username))
        .one(db)
        .await
        .expect("Could not find Users -- Login")
    {
        None => {
            /*用户不存在的时候注册*/

            /*验证验证码是否正确*/

            const MSG: &str = "Password error";
            HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
        }
        Some(user) => {
            /*用户存在则不能注册*/
            const MSG: &str = "Password error";
            HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
        }
    }
}

// /*获取验证码*/
// #[post("/verification")]
// pub async fn verification_code(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
//     const MSG: &str = "Password error";
//     Ok(HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG })))
// }

#[post("/sendmail")]
pub async fn sendmail(data: web::Data<AppState>, mail: web::Json<MailInfo>) -> HttpResponse {
    /*将随机数发送到相应的邮箱*/
    match &GLOBAL_YAML_CONFIG.mail {
        None => {
            const MSG: &str = "The server does not support email sending";
            HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
        }
        Some(m) => {
            /*生成随机数*/
            let num = generation_random_number();
            match m.sendmail(mail.email.clone(), &num.to_string()) {
                true => {
                    /*生成加盐的数据*/
                    let claims = Claims {
                        sub: GLOBAL_YAML_CONFIG.jwt.get_sub(),
                        exp: get_current_timestamp() + GLOBAL_YAML_CONFIG.jwt.get_code_time(),
                        auth: "Have no authority".parse().unwrap(),
                    };
                    let code = GLOB_JOT.generate_token(&claims);

                    HttpResponse::Ok().json(serde_json::json!({"status": "success", "code": num }))
                }
                false => {
                    const MSG: &str = "Email sending failure";
                    HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
                }
            }
        }
    }
}
