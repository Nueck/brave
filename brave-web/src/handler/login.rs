use crate::config::{AppState, GLOBAL_YAML_CONFIG};
use crate::entity::prelude::Users;
use crate::entity::users;
use actix_web::{post, web, HttpResponse};
use brave_utils::common::{generation_random_number, is_valid_email};
use brave_utils::jwt::jwt::GLOB_JOT;
use brave_utils::jwt::jwt::{Claims, UserData};
use jsonwebtoken::get_current_timestamp;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
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

#[derive(Deserialize)]
pub struct ForgetInfo {
    pub email: String,
    pub new_pwd: String,
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
                    data: None,
                };
                let token = GLOB_JOT.generate_token(&claims);

                //长时间的token
                let claims = Claims {
                    sub: GLOBAL_YAML_CONFIG.jwt.get_sub(),
                    exp: get_current_timestamp() + GLOBAL_YAML_CONFIG.jwt.get_ref_time(),
                    auth: user.user_authority,
                    data: None,
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
        .filter(
            users::Column::UserEmail
                .contains(&info.email)
                .or(users::Column::UserName.contains(&info.username)),
        )
        .one(db)
        .await
        .expect("Could not find Users -- Login")
    {
        Some(_) => {
            /*用户存在则不能注册*/
            const MSG: &str = "User presence";
            HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
        }
        None => {
            /*用户不存在的时候注册*/
            /*验证验证码是否正确*/
            match GLOB_JOT.validation_to_claim(&info.code) {
                Ok(data) => {
                    //对需要验证的code的加盐
                    let verify_code = GLOBAL_YAML_CONFIG
                        .blake
                        .generate_with_salt(&info.verify_code);

                    let code = data.data.clone().unwrap().code;
                    let email = data.data.clone().unwrap().email;
                    //判断验证码是否正确
                    if verify_code == code && email == info.email.clone() {
                        /*保存数据到数据库*/
                        /*对密码加密*/
                        let pwd = GLOBAL_YAML_CONFIG.blake.generate_with_salt(&info.password);
                        //初始化数据
                        let user = users::ActiveModel {
                            user_name: Set((&info.username.as_str()).parse().unwrap()),
                            user_authority: Set("user".to_owned()),
                            user_email: Set((&info.email.as_str()).parse().unwrap()),
                            user_address: Set((&info.username.as_str()).parse().unwrap()),
                            pwd_hash: Set(pwd),
                            ..Default::default() // all other attributes are `NotSet`
                        };

                        match users::Entity::insert(user).exec(db).await {
                            Ok(_) => {
                                const MSG: &str = "Successful registration";
                                HttpResponse::Ok()
                                    .json(serde_json::json!({"status": "success", "message": MSG }))
                            }
                            Err(err) => {
                                log::error!("Registration failure : {err:?}");
                                const MSG: &str = "Registration failure";
                                HttpResponse::Ok()
                                    .json(serde_json::json!({"status": "error", "message": MSG }))
                            }
                        }
                    } else {
                        const MSG: &str = "Verification code error";
                        HttpResponse::Ok()
                            .json(serde_json::json!({"status": "code error", "message": MSG }))
                    }
                }
                Err(_) => {
                    const MSG: &str = "Error in sending data";
                    HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
                }
            }
        }
    }
}

// /*忘记密码*/
#[post("/forget")]
pub async fn forget(data: web::Data<AppState>, info: web::Json<ForgetInfo>) -> HttpResponse {
    /*判断邮箱地址是否存在或在用户名*/
    let db = &data.conn;
    match Users::find()
        .filter(users::Column::UserEmail.contains(&info.email))
        .one(db)
        .await
        .expect("Could not find Users -- Login")
    {
        None => {
            /*用户不存在*/
            const MSG: &str = "Mail does not exist";
            HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
        }
        Some(user) => {
            /*用户不存在的时候注册*/
            /*验证验证码是否正确*/
            match GLOB_JOT.validation_to_claim(&info.code) {
                Ok(data) => {
                    //对需要验证的code的加盐
                    let verify_code = GLOBAL_YAML_CONFIG
                        .blake
                        .generate_with_salt(&info.verify_code);

                    let code = data.data.clone().unwrap().code;
                    let email = data.data.clone().unwrap().email;
                    //判断验证码是否正确
                    if verify_code == code && email == info.email.clone() {
                        /*对密码加密*/
                        let pwd = GLOBAL_YAML_CONFIG.blake.generate_with_salt(&info.new_pwd);
                        /*修改数据库数据*/
                        let mut user: users::ActiveModel = user.into();
                        user.pwd_hash = Set(pwd);

                        match user.update(db).await {
                            Ok(_) => {
                                const MSG: &str = "Modified successfully";
                                HttpResponse::Ok()
                                    .json(serde_json::json!({"status": "success", "message": MSG }))
                            }
                            Err(_) => {
                                const MSG: &str = "Modification failure";
                                HttpResponse::Ok()
                                    .json(serde_json::json!({"status": "error", "message": MSG }))
                            }
                        }
                    } else {
                        const MSG: &str = "Verification code error";
                        HttpResponse::Ok()
                            .json(serde_json::json!({"status": "code error", "message": MSG }))
                    }
                }
                Err(_) => {
                    const MSG: &str = "Error in sending data";
                    HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
                }
            }
        }
    }
}

/*发送邮件*/
#[post("/sendmail")]
pub async fn sendmail(mail: web::Json<MailInfo>) -> HttpResponse {
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
                    /*生成加盐的数据 和使用token加密*/
                    let num_code = GLOBAL_YAML_CONFIG
                        .blake
                        .generate_with_salt(&num.to_string());

                    let claims = Claims {
                        sub: GLOBAL_YAML_CONFIG.jwt.get_sub(),
                        //验证码时间有效5分钟
                        exp: get_current_timestamp() + 300,
                        //由于对权限的控制，这个生成的token是无法用在登陆
                        auth: "Have no authority".to_string(),
                        data: Some(UserData {
                            code: num_code,
                            email: mail.email.clone().to_string(),
                        }),
                    };
                    let code = GLOB_JOT.generate_token(&claims);

                    HttpResponse::Ok().json(serde_json::json!({"status": "success", "code": code }))
                }
                false => {
                    const MSG: &str = "Email sending failure";
                    HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
                }
            }
        }
    }
}
