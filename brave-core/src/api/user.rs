use crate::entity::user_entity::UserTableData;
use actix_web::error::ErrorUnauthorized;
use actix_web::{delete, post, put, web, HttpResponse, Responder};
use brave_config::app::AppState;
use brave_config::interface::Interface;
use brave_config::GLOBAL_CONFIG;
use brave_db::entity::prelude::Users;
use brave_db::entity::users;
use brave_db::enumeration::user_enum::UserStatusEnum;
use brave_utils::jwt::jwt::UserDataInfo;
use sea_orm::prelude::DateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
};
use serde::{Deserialize, Serialize};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user_info)
        .service(get_user_data_info)
        .service(delete_user)
        .service(update_user)
        .service(delete_user_soft);
}

/*获取用户的文章总信息*/
#[post("/getUserDataInfo")]
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
#[post("/getUserInfo")]
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

#[derive(FromQueryResult, Deserialize, Serialize)]
struct UsersLists {
    user_name: String,
    authority: String,
    email: String,
    create_time: DateTime,
}

/*获取用户的一些数据*/
#[post("/getUsers")]
async fn get_users(data: web::Data<AppState>, token: web::ReqData<UserDataInfo>) -> impl Responder {
    let auth = token.auth.clone();

    //只有是超级管理员才能访问
    if auth
        == GLOBAL_CONFIG
            .authority
            .get_authority_config()
            .super_admin
            .unwrap()
    {
        let db = &data.conn;
        let data = Users::find()
            .select_only()
            .columns([
                users::Column::UserName,
                users::Column::Authority,
                users::Column::Email,
                users::Column::CreateTime,
            ])
            .into_model::<UsersLists>()
            .all(db)
            .await
            .expect("Could not find Users");

        HttpResponse::Ok().json(serde_json::json!({"state": "success","data":data}))
    } else {
        ErrorUnauthorized("Lack of authority").into()
    }
}

///注销用户数据
#[delete("/user/{id}/soft")]
async fn delete_user_soft(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    query: web::Path<i32>,
) -> impl Responder {
    let auth = token.auth.clone();

    //只有是超级管理员才能访问
    if auth
        == GLOBAL_CONFIG
            .authority
            .get_authority_config()
            .super_admin
            .unwrap()
    {
        let db = &data.conn;
        let id = query.into_inner();

        let mut data: users::ActiveModel = Users::find_by_id(id)
            .one(db)
            .await
            .expect("deleteUser")
            .unwrap()
            .into();

        //判断用户是否是管理员（禁止删除管理员）
        if data.authority.clone().unwrap() == GLOBAL_CONFIG.authority.super_admin.clone().unwrap() {
            const MSG: &str = "No permission to delete";
            let json = serde_json::json!({"state": "error",  "message":MSG });
            return HttpResponse::Ok().json(json);
        }

        data.user_status = Set(UserStatusEnum::SoftDelete as i16);
        data.update(db).await.unwrap();

        HttpResponse::Ok().json(serde_json::json!({"state": "success"}))
    } else {
        ErrorUnauthorized("Lack of authority").into()
    }
}

///删除用户
#[delete("/user/{id}")]
async fn delete_user(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    query: web::Path<i32>,
) -> impl Responder {
    let auth = token.auth.clone();

    //只有是超级管理员才能访问
    if auth
        == GLOBAL_CONFIG
            .authority
            .get_authority_config()
            .super_admin
            .unwrap()
    {
        let db = &data.conn;
        let id = query.into_inner();

        let data: users::ActiveModel = Users::find_by_id(id)
            .one(db)
            .await
            .expect("deleteUser")
            .unwrap()
            .into();

        //判断用户是否是管理员（禁止删除管理员）
        if data.authority.clone().unwrap() == GLOBAL_CONFIG.authority.super_admin.clone().unwrap() {
            const MSG: &str = "The administrator cannot be deleted";
            let json = serde_json::json!({"state": "error",  "message":MSG });
            return HttpResponse::Ok().json(json);
        }

        //删除用户
        match Users::delete_by_id(id).exec(db).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({"state": "success"})),
            Err(_) => HttpResponse::Ok().json(serde_json::json!({"state": "error"})),
        }
    } else {
        ErrorUnauthorized("Lack of authority").into()
    }
}

///更新用户信息
#[put("/user/{id}")]
async fn update_user(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    query: web::Path<i32>,
    json: web::Json<UserTableData>,
) -> impl Responder {
    let auth = token.auth.clone();

    //只有是超级管理员才能访问
    if auth
        == GLOBAL_CONFIG
            .authority
            .get_authority_config()
            .super_admin
            .unwrap()
    {
        let db = &data.conn;
        let id = query.into_inner();

        //更新用户
        let mut data: users::ActiveModel = Users::find_by_id(id)
            .one(db)
            .await
            .expect("deleteUser")
            .unwrap()
            .into();

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
                return HttpResponse::Ok().json(json);
            }
        }
    } else {
        ErrorUnauthorized("Lack of authority").into()
    }
}
