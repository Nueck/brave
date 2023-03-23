/*用于初始化时候的超级管理员的创建*/

use crate::config::app::AppState;
use crate::config::init::InitStatus;
use crate::config::GLOBAL_YAML_CONFIG;
use crate::entity::users;
use actix_web::{post, web, HttpResponse, Responder};
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde_json::json;

/*初始化的用户信息*/
#[derive(Clone, Deserialize)]
pub struct InitInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: String,
}

pub fn init_config(cfg: &mut web::ServiceConfig) {
    cfg.service(init_status).service(init);
}

/*
* 初始化超级管理员的
*/
#[post("/init")]
async fn init(data: web::Data<AppState>, info: web::Json<InitInfo>) -> HttpResponse {
    /*判断系统是否初始化*/
    if !InitStatus::global().is_init {
        let db = &data.conn;
        /*对密码加密*/
        let pwd = GLOBAL_YAML_CONFIG.blake.generate_with_salt(&info.password);
        //初始化数据
        let user = users::ActiveModel {
            user_name: Set((&info.username.as_str()).parse().unwrap()),
            authority: Set(GLOBAL_YAML_CONFIG
                .authority
                .get_authority_config()
                .super_admin
                .unwrap()),
            email: Set((&info.email.as_str()).parse().unwrap()),
            address: Set((&info.address.as_str()).parse().unwrap()),
            pwd_hash: Set(pwd),
            ..Default::default()
        };

        match users::Entity::insert(user.clone()).exec(db).await {
            Ok(_) => {
                /*设置初始化状态为true*/
                InitStatus::set(InitStatus {
                    is_init: true,
                    username: Some(user.user_name.unwrap()),
                    email: Some(user.email.unwrap()),
                    address: Some(user.address.unwrap()),
                });
                const MSG: &str = "Successful initialization";
                HttpResponse::Ok().json(serde_json::json!({"state": "success", "message": MSG }))
            }
            Err(err) => {
                log::error!("Initialization failure : {err:?}");
                const MSG: &str = "Initialization failure";
                HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
            }
        }
    } else {
        const MSG: &str = "Already initialized";
        HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
    }
}

/*判断系统是否初始化*/
#[post("/init-status")]
async fn init_status() -> impl Responder {
    /*判断系统是否初始化*/
    let bool = InitStatus::global().is_init;
    if bool {
        return HttpResponse::Ok().json(json!({ "state": "success" }));
    }
    HttpResponse::Ok().json(json!({ "state": "error" }))
}