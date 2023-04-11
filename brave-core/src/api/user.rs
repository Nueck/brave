use crate::entity::UserTableData;
use actix_web::error::ErrorUnauthorized;
use actix_web::{delete, get, put, web, HttpResponse, Responder};
use brave_config::app::AppState;
use brave_config::interface::Interface;
use brave_config::utils::jwt::UserDataInfo;
use brave_config::GLOBAL_CONFIG;
use brave_db::entity::prelude::Users;
use brave_db::entity::users;
use brave_db::enumeration::user_enum::UserStatusEnum;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use std::path::PathBuf;
use std::{env, fs};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_info)
        .service(get_user_data_info)
        .service(update_user);
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
#[get("/user")]
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

///更新用户信息
#[put("/user")]
async fn update_user(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    query: web::Path<i32>,
    json: web::Json<UserTableData>,
) -> impl Responder {
    let auth = token.auth.clone();

    let db = &data.conn;
    let id = &token.id;

    //更新用户
    match Users::find_by_id(id).one(db).await {
        Ok(model) => {
            let mut data: users::ActiveModel = model.unwrap().into();

            data.user_name = Set(json.user_name.to_owned());
            data.user_status = Set(json.user_status);
            data.email = Set(json.email.to_owned());

            if GLOBAL_CONFIG
                .authority
                .auth
                .clone()
                .unwrap()
                .contains(&json.authority)
            {
                data.authority = Set(json.authority.to_owned());
            };

            match data.update(db).await {
                Ok(_) => HttpResponse::Ok().json(serde_json::json!({"state": "success"})),
                Err(_) => {
                    const MSG: &str = "Update failure";
                    let json = serde_json::json!({"state": "error",  "message":MSG });
                    HttpResponse::Ok().json(json)
                }
            }
        }
        Err(_) => {
            const MSG: &str = "User does not exist";
            let json = serde_json::json!({"state": "error",  "message":MSG });
            HttpResponse::Ok().json(json)
        }
    }
}
