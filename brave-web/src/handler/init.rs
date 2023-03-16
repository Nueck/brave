/*用于初始化时候的超级管理员的创建*/
use crate::config::{AppState, InitStatus};
use crate::entity::prelude::Users;
use crate::entity::users;
use actix_web::error::ErrorUnauthorized;
use actix_web::{post, web, HttpResponse, Responder};
use brave_utils::common::{generation_random_number, is_valid_email};
use brave_utils::jwt::jwt::GLOB_JOT;
use brave_utils::jwt::jwt::{Claims, UserData};
use jsonwebtoken::get_current_timestamp;
use sea_orm::sea_query::impl_conditional_statement;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct InitInfo {
    pub username: String,
    pub password: String,
}

/*
* 初始化超级管理员的
*/
#[post("/init")]
pub async fn init(
    data: web::Data<AppState>,
    // info: web::Json<InitInfo>,
) -> HttpResponse {
    /*判断系统是否初始化*/
    if !InitStatus::global().is_init {
        /*设置初始化状态为true*/
        InitStatus::set(InitStatus { is_init: true });
        const MSG: &str = "Successful initialization";
        HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MSG }))
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
