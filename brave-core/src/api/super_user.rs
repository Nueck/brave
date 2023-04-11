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

pub fn super_user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user_info)
        .service(get_user_data_info)
        .service(delete_user_soft)
        .service(delete_user)
        .service(update_user);
}

///获取所有用户的信息
#[get("/users")]
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
                users::Column::UserId,
                users::Column::UserName,
                users::Column::Authority,
                users::Column::Email,
                users::Column::UserStatus,
                users::Column::CreateTime,
            ])
            .order_by_asc(users::Column::UserId)
            .into_model::<UserTableData>()
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

        match Users::find_by_id(id.clone()).one(db).await.expect("") {
            None => HttpResponse::Ok().json(serde_json::json!({"state": "error"})),
            Some(model) => {
                //判断用户是否是管理员（禁止删除管理员）
                if model.authority == GLOBAL_CONFIG.authority.super_admin.clone().unwrap() {
                    const MSG: &str = "The administrator cannot be deleted";
                    let json = serde_json::json!({"state": "error",  "message":MSG });
                    return HttpResponse::Ok().json(json);
                }

                //删除用户
                match Users::delete_by_id(model.user_id.clone()).exec(db).await {
                    Ok(_) => {
                        if let Some(location) = &GLOBAL_CONFIG.get_page().location {
                            let mut path_buf = PathBuf::from(env::current_dir().unwrap());
                            path_buf.push(location);
                            path_buf.push(model.user_name.to_owned());

                            if path_buf.exists() {
                                fs::remove_file(path_buf).unwrap();
                            }
                        };
                        HttpResponse::Ok().json(serde_json::json!({"state": "success"}))
                    }
                    Err(err) => {
                        log::error!("{err}");
                        HttpResponse::Ok().json(serde_json::json!({"state": "error"}))
                    }
                }
            }
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
    } else {
        ErrorUnauthorized("Lack of authority").into()
    }
}

#[test]
pub fn remove_dir_test() {
    if let Some(location) = &GLOBAL_CONFIG.get_page().location {
        let mut path_buf = PathBuf::from(env::current_dir().unwrap());
        path_buf.push(location);
        path_buf.push(model.user_name.to_owned());

        if path_buf.exists() {
            fs::remove_file(path_buf).unwrap();
        }
    };
}
