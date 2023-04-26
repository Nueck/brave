use crate::entity::ChangePwdInfo;
use actix_web::{get, put, web, HttpResponse, Responder};
use brave_config::app::AppState;
use brave_config::interface::Interface;
use brave_config::utils::common::GLOBAL_CODE;
use brave_config::utils::jwt::{UserDataInfo, GLOB_JOT};
use brave_config::GLOBAL_CONFIG;
use brave_db::entity::prelude::Users;
use brave_db::entity::users;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_info)
        .service(get_user_data_info)
        .service(chang_pwd);
}

///获取用户的文章总信息
#[get("/user/articles/info")]
async fn get_user_data_info(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
) -> impl Responder {
    let db = &data.conn;
    match Users::find()
        .filter(users::Column::UserName.contains(&token.aud))
        .one(db)
        .await
        .expect("Could not find Users --- getUserAllArticleInfo")
    {
        None => {
            const MSG: &str = "User does not exist";
            let json = serde_json::json!({"state": "error",  "message":MSG });

            HttpResponse::Ok().json(json)
        }
        Some(user) => {
            let article = user.article_num;
            let messages = user.messages_count;
            let read = user.read_count;
            let visit = user.visit_count;

            let json = serde_json::json!({
                "state": "success",
                "data":{
                    "articleNum":article,
                    "readCount":read,
                    "visitCount":visit,
                    "messagesCount":messages
                }
            });

            HttpResponse::Ok().json(json)
        }
    }
}

///获取用户信息
#[get("/user/info")]
async fn get_user_info(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
) -> impl Responder {
    let db = &data.conn;

    match Users::find()
        .filter(users::Column::UserName.contains(&token.aud))
        .one(db)
        .await
        .expect("Could not find Users --- getUserInfo")
    {
        None => {
            const MSG: &str = "User does not exist";
            let json = serde_json::json!({"state": "error",  "message":MSG });

            HttpResponse::Ok().json(json)
        }
        Some(user) => {
            let id = user.user_id;
            let username = user.user_name;
            let user_role = user.authority;
            let url = Interface::redirect_user_blog_home(&username);

            let json = serde_json::json!({
                "state": "success",
                "data":{
                    "userId":id ,
                    "userName":username,
                    "userRole":user_role,
                    "userHomeUrl":url
                }
            });

            HttpResponse::Ok().json(json)
        }
    }
}

#[put("/user/password")]
pub(crate) async fn chang_pwd(
    data: web::Data<AppState>,
    info: web::Json<ChangePwdInfo>,
) -> HttpResponse {
    if let Some(_) = GLOBAL_CODE
        .lock()
        .unwrap()
        .get(&(&info.verify_code).parse::<u32>().unwrap())
    {
        const MSG: &str = "Code is invalid";
        return HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }));
    }

    //判断邮箱地址是否存在或在用户名
    let db = &data.conn;
    match Users::find()
        .filter(users::Column::Email.contains(&info.email))
        .one(db)
        .await
        .expect("Could not find Users -- Login")
    {
        None => {
            //用户不存在
            const MSG: &str = "Mail does not exist";
            HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
        }
        Some(user) => {
            //用户不存在的时候注册
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
                        //对密码加密
                        let pwd = GLOBAL_CONFIG.get_blake().generate_with_salt(&info.new_pwd);
                        //修改数据库数据
                        let mut user: users::ActiveModel = user.into();
                        user.pwd_hash = Set(pwd);

                        match user.update(db).await {
                            Ok(_) => {
                                const MSG: &str = "Modified successfully";
                                HttpResponse::Ok()
                                    .json(serde_json::json!({"state": "success", "message": MSG }))
                            }
                            Err(_) => {
                                const MSG: &str = "Modification failure";
                                HttpResponse::Ok()
                                    .json(serde_json::json!({"state": "error", "message": MSG }))
                            }
                        }
                    } else {
                        const MSG: &str = "Verification code error";
                        HttpResponse::Ok()
                            .json(serde_json::json!({"state": "error", "message": MSG }))
                    }
                }
                Err(_) => {
                    const MSG: &str = "Error in sending data";
                    HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
                }
            }
        }
    }
}
