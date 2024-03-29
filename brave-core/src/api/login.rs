use actix_web::{post, web, HttpResponse};
use brave_config::app::AppState;
use brave_config::init::InitStatus;
use brave_config::utils::common::{
    generation_random_number, is_invalid_user_name, is_valid_email, GLOBAL_CODE,
};
use brave_config::utils::fs::gen_default_skin_page;
use brave_config::utils::jwt::{Claims, UserData, GLOB_JOT};
use brave_config::GLOBAL_CONFIG;
use brave_db::entity::article_archive;
use brave_db::entity::article_category;
use brave_db::entity::article_tag;
use brave_db::entity::prelude::Users;
use brave_db::entity::users;
use brave_db::entity::users::Model;
use jsonwebtoken::get_current_timestamp;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JsonValue, PaginatorTrait, QueryFilter,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct EmailLoginInfo {
    pub email: String,
    pub verify_code: String,
    pub code: String,
}

#[derive(Deserialize)]
pub struct MailInfo {
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub verify_code: String,
    pub code: String,
}

pub fn login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(email_login)
        .service(register)
        .service(sendmail);
}

/*
* 登陆
*/
#[post("/login")]
async fn login(data: web::Data<AppState>, user_info: web::Json<UserInfo>) -> HttpResponse {
    /*
     * 登陆获取,
     * 对密码加密
     */
    let pwd = GLOBAL_CONFIG
        .get_blake()
        .generate_with_salt(&user_info.password);

    let db = &data.conn;
    /*
     *获取user数据
     */
    match Users::find()
        .filter(
            //断用户名是否是邮箱
            if is_valid_email(&user_info.username) {
                users::Column::Email.contains(&user_info.username)
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
            HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
        }
        Some(user) => {
            //判断用户的状态
            if user.user_status != 1 {
                const MSG: &str = "The user is shut down. Contact the administrator";
                return HttpResponse::Ok()
                    .json(serde_json::json!({"state": "error", "message": MSG }));
            }

            //进行密码比对
            if pwd == user.pwd_hash {
                login_success_process(user)
            } else {
                const MSG: &str = "Password error";
                HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
            }
        }
    }
}

//邮箱验证码登陆
#[post("email-login")]
async fn email_login(data: web::Data<AppState>, info: web::Json<EmailLoginInfo>) -> HttpResponse {
    if let Some(_) = GLOBAL_CODE
        .lock()
        .unwrap()
        .get(&(&info.verify_code).parse::<u32>().unwrap())
    {
        const MSG: &str = "Code is invalid";
        return HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }));
    }

    //获取user数据
    let db = &data.conn;
    match Users::find()
        .filter(users::Column::Email.contains(&info.email))
        .one(db)
        .await
        .expect("Could not find Users -- Login")
    {
        None => {
            const MSG: &str = "User does not exist";
            return HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }));
        }
        Some(user) => {
            //判断用户的状态
            if user.user_status != 1 {
                const MSG: &str = "The user is shut down. Contact the administrator";
                return HttpResponse::Ok()
                    .json(serde_json::json!({"state": "error", "message": MSG }));
            }

            //验证验证码是否正确
            match GLOB_JOT.validation_to_claim(&info.code) {
                Ok(data) => {
                    //对需要验证的code的加盐
                    let verify_code = GLOBAL_CONFIG
                        .get_blake()
                        .generate_with_salt(&info.verify_code);

                    let code = data.data.clone().unwrap().code;
                    let email = data.data.clone().unwrap().email;

                    //判断验证码是否正确
                    if verify_code == code && email == info.email.clone() {
                        let exp = data.exp;
                        GLOBAL_CODE
                            .lock()
                            .unwrap()
                            .insert((&info.verify_code).parse().unwrap(), exp);

                        return login_success_process(user);
                    }
                }
                Err(_) => {}
            }
        }
    }
    const MSG: &str = "Code is invalid";
    HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
}

//注册
#[post("/register")]
async fn register(data: web::Data<AppState>, info: web::Json<RegisterInfo>) -> HttpResponse {
    //判断邮箱地址是否存在或在用户名
    let db = &data.conn;

    //判断用户是否存在于接口名上
    if is_invalid_user_name(&info.username) {
        const MSG: &str = "The user name cannot be created";
        return HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }));
    }

    if !InitStatus::global().able_register {
        return HttpResponse::Ok()
            .json(serde_json::json!({"state": "error", "message": "Register is closed" }));
    }

    if let Some(_) = GLOBAL_CODE
        .lock()
        .unwrap()
        .get(&(&info.verify_code).parse::<u32>().unwrap())
    {
        const MSG: &str = "Code is invalid";
        return HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }));
    }

    if judge_registers_number_effective(db).await {
        match Users::find()
            .filter(
                users::Column::Email
                    .contains(&info.email)
                    .or(users::Column::UserName.contains(&info.username)),
            )
            .one(db)
            .await
            .expect("Could not find Users -- Login")
        {
            Some(_) => {
                //用户存在则不能注册
                const MSG: &str = "User or Email already exists";
                HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
            }
            None => {
                //验证验证码是否正确
                match GLOB_JOT.validation_to_claim(&info.code) {
                    Ok(data) => {
                        //对需要验证的code的加盐
                        let verify_code = GLOBAL_CONFIG
                            .get_blake()
                            .generate_with_salt(&info.verify_code);

                        let code = data.data.clone().unwrap().code;
                        let email = data.data.clone().unwrap().email;
                        //获取失效时间
                        let exp = data.exp;
                        //判断验证码是否正确
                        if verify_code == code && email == info.email.clone() {
                            GLOBAL_CODE
                                .lock()
                                .unwrap()
                                .insert((&info.verify_code).parse().unwrap(), exp);

                            //对密码加密
                            let pwd = GLOBAL_CONFIG.get_blake().generate_with_salt(&info.password);
                            //初始化数据
                            let user = users::ActiveModel {
                                user_name: Set((&info.username.as_str()).parse().unwrap()),
                                authority: Set("user".to_owned()),
                                email: Set((&info.email.as_str()).parse().unwrap()),
                                address: Set((&info.username.as_str()).parse().unwrap()),
                                pwd_hash: Set(pwd),
                                ..Default::default() // all other attributes are `NotSet`
                            };

                            let insert_status = match users::Entity::insert(user).exec(db).await {
                                Ok(table) => {
                                    gen_default_skin_page(&info.username);

                                    let tags = article_tag::ActiveModel {
                                        user_id: Set(table.last_insert_id),
                                        content: Set(JsonValue::Array(Vec::new())),
                                        ..Default::default()
                                    };

                                    let archive = article_archive::ActiveModel {
                                        user_id: Set(table.last_insert_id),
                                        content: Set(JsonValue::Array(Vec::new())),
                                        ..Default::default()
                                    };
                                    let category = article_category::ActiveModel {
                                        user_id: Set(table.last_insert_id),
                                        content: Set(JsonValue::Array(Vec::new())),
                                        ..Default::default()
                                    };

                                    //添加分类和归档和tag
                                    article_tag::Entity::insert(tags).exec(db).await.unwrap();
                                    article_archive::Entity::insert(archive)
                                        .exec(db)
                                        .await
                                        .unwrap();
                                    article_category::Entity::insert(category)
                                        .exec(db)
                                        .await
                                        .unwrap();

                                    true
                                }

                                Err(err) => {
                                    log::error!("Registration failure : {err:?}"); //打印错误日志
                                    false
                                }
                            };

                            if insert_status {
                                const MSG: &str = "Successful registration";
                                HttpResponse::Ok()
                                    .json(serde_json::json!({"state": "success", "message": MSG }))
                            } else {
                                const MSG: &str = "Registration failure";
                                HttpResponse::Ok()
                                    .json(serde_json::json!({"state": "error", "message": MSG }))
                            }
                        } else {
                            const MSG: &str = "Verification code error";
                            HttpResponse::Ok()
                                .json(serde_json::json!({"state": "code error", "message": MSG }))
                        }
                    }
                    Err(_) => {
                        const MSG: &str = "Error in sending data";
                        HttpResponse::Ok()
                            .json(serde_json::json!({"state": "error", "message": MSG }))
                    }
                }
            }
        }
    } else {
        const MSG: &str = "The number of registrations has been capped";
        HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
    }
}

/*发送邮件*/
#[post("/sendmail")]
async fn sendmail(mail: web::Json<MailInfo>) -> HttpResponse {
    //将随机数发送到相应的邮箱
    match &GLOBAL_CONFIG.mail {
        None => {
            const MSG: &str = "The server does not support email sending";
            HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
        }
        Some(m) => {
            match &mail.email {
                None => {
                    const MSG: &str = "Mailbox is nonexistence";
                    HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
                }
                Some(email) => {
                    //生成随机数
                    if email.is_empty() {
                        const MSG: &str = "Mailbox is empty";
                        return HttpResponse::Ok()
                            .json(serde_json::json!({"state": "error", "message": MSG }));
                    }

                    let num = generation_random_number();
                    match m.sendmail(email.to_string(), &num.to_string()).await {
                        true => {
                            //生成加盐的数据 和使用token加密
                            let num_code = GLOBAL_CONFIG
                                .get_blake()
                                .generate_with_salt(&num.to_string());

                            let claims = Claims {
                                id: 0,
                                aud: "email".to_string(),
                                sub: GLOBAL_CONFIG.jwt.get_sub(),
                                //验证码时间有效5分钟
                                exp: get_current_timestamp() + 300,
                                //由于对权限的控制，这个生成的token是无法用在登陆
                                auth: "Have no authority".to_string(),
                                data: Some(UserData {
                                    code: num_code,
                                    email: email.to_string(),
                                }),
                                refresh: false,
                            };
                            let code = GLOB_JOT.generate_token(&claims);

                            HttpResponse::Ok().json(
                                serde_json::json!({"state": "success", "data":{"code": code}  }),
                            )
                        }
                        false => {
                            const MSG: &str = "Email sending failure";
                            HttpResponse::Ok()
                                .json(serde_json::json!({"state": "error", "message": MSG }))
                        }
                    }
                }
            }
        }
    }
}

async fn judge_registers_number_effective(db: &DatabaseConnection) -> bool {
    match Users::find().count(db).await {
        Ok(count) => count < InitStatus::global().registrants as u64,
        Err(_) => false,
    }
}

fn login_success_process(user: Model) -> HttpResponse {
    //短时间的token
    let claims = Claims {
        id: user.user_id.clone(),
        aud: user.user_name.clone(),
        sub: GLOBAL_CONFIG.jwt.get_sub(),
        exp: get_current_timestamp() + GLOBAL_CONFIG.jwt.get_exp_time(),
        auth: user.authority.clone(),
        data: None,
        refresh: false,
    };
    let token = GLOB_JOT.generate_token(&claims);

    //长时间的token
    let claims = Claims {
        id: user.user_id.clone(),
        aud: user.user_name.clone(),
        sub: GLOBAL_CONFIG.jwt.get_sub(),
        exp: get_current_timestamp() + GLOBAL_CONFIG.jwt.get_ref_time(),
        auth: user.authority.clone(),
        data: None,
        refresh: true,
    };
    let ref_token = GLOB_JOT.generate_token(&claims);

    let json = serde_json::json!({"state": "success",  "data":{"token": token ,"refreshToken": ref_token} });
    HttpResponse::Ok().json(json)
}
