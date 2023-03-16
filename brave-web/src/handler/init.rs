/*用于初始化时候的超级管理员的创建*/
use crate::config::{AppState, InitStatus, GLOBAL_YAML_CONFIG};
use crate::entity::users;
use actix_web::{post, web, HttpResponse, Responder};
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct InitInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: String,
}

/*
* 初始化超级管理员的
*/
#[post("/init")]
pub async fn init(data: web::Data<AppState>, info: web::Json<InitInfo>) -> HttpResponse {
    /*判断系统是否初始化*/
    if !InitStatus::global().is_init {
        /*对密码加密*/
        let pwd = GLOBAL_YAML_CONFIG.blake.generate_with_salt(&info.password);
        //初始化数据
        let user = users::ActiveModel {
            user_name: Set((&info.username.as_str()).parse().unwrap()),
            user_authority: Set(GLOBAL_YAML_CONFIG
                .authority
                .get_authority_config()
                .super_admin
                .unwrap()),
            user_email: Set((&info.email.as_str()).parse().unwrap()),
            user_address: Set((&info.address.as_str()).parse().unwrap()),
            pwd_hash: Set(pwd),
            ..Default::default()
        };

        let db = &data.conn;

        match users::Entity::insert(user).exec(db).await {
            Ok(_) => {
                /*设置初始化状态为true*/
                InitStatus::set(InitStatus { is_init: true });
                const MSG: &str = "Successful initialization";
                HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MSG }))
            }
            Err(err) => {
                log::error!("Initialization failure : {err:?}");
                const MSG: &str = "Initialization failure";
                HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
            }
        }
    } else {
        const MSG: &str = "Already initialized";
        HttpResponse::Ok().json(serde_json::json!({"status": "error", "message": MSG }))
    }
}

/*判断系统是否初始化*/
#[post("/init-status")]
pub async fn init_status() -> impl Responder {
    /*判断系统是否初始化*/
    let bool = InitStatus::global().is_init;
    HttpResponse::Ok().json(json!({ "status": bool }))
}
