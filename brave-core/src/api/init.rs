/*用于初始化时候的超级管理员的创建*/

use actix_web::{get, post, web, HttpResponse, Responder};
use brave_config::app::AppState;
use brave_config::init::InitStatus;
use brave_config::GLOBAL_CONFIG;
use brave_db::entity::users;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use serde::Deserialize;

/*初始化的用户信息*/
#[derive(Clone, Deserialize)]
pub struct InitInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: String,
}

pub fn init_config(cfg: &mut web::ServiceConfig) {
    cfg.service(init_state).service(init);
}

/*
* 初始化超级管理员的
*/
#[post("/init")]
async fn init(data: web::Data<AppState>, info: web::Json<InitInfo>) -> HttpResponse {
    /*判断系统是否初始化*/
    if !InitStatus::global().is_init {
        let db = &data.conn;
        //对密码加密
        let pwd = GLOBAL_CONFIG.get_blake().generate_with_salt(&info.password);

        //初始化数据
        let user = users::ActiveModel {
            user_name: Set((&info.username.as_str()).parse().unwrap()),
            authority: Set(GLOBAL_CONFIG
                .authority
                .get_authority_config()
                .super_admin
                .unwrap()),
            email: Set((&info.email.as_str()).parse().unwrap()),
            address: Set((&info.username.as_str()).parse().unwrap()),
            pwd_hash: Set(pwd),
            ..Default::default()
        };

        match users::Entity::insert(user.clone()).exec(db).await {
            Ok(_) => {
                /*设置初始化状态为true*/
                InitStatus::set(InitStatus {
                    is_init: true,
                    username: Some(user.user_name.clone().unwrap()),
                    email: Some(user.email.clone().unwrap()),
                    address: Some(user.address.clone().unwrap()),
                    registrants: GLOBAL_CONFIG.get_registrants(),
                    able_register: GLOBAL_CONFIG.get_able_register(),
                });

                const MSG: &str = "Successful initialization";
                let json =
                    serde_json::json!({"state": "success", "data":{"isInit":true} ,"message": MSG});

                HttpResponse::Ok().json(json)
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

//判断系统是否初始化
#[get("/init/state")]
async fn init_state() -> impl Responder {
    /*判断系统是否初始化*/
    if InitStatus::global().is_init {
        const MSG: &str = "Already initialized";
        return HttpResponse::Ok().json(serde_json::json!({ "state": "success","message": MSG }));
    }
    HttpResponse::Ok().json(serde_json::json!({ "state": "error"}))
}

//判断系统是否可注册
#[get("/init/able_register")]
async fn init_able_register() -> impl Responder {
    if InitStatus::global().able_register {
        const MSG: &str = "Can be registered";
        return HttpResponse::Ok().json(serde_json::json!({ "state": "success","message": MSG }));
    }
    HttpResponse::Ok().json(serde_json::json!({ "state": "error"}))
}
