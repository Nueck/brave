use crate::admin::AdminState;
use crate::app::AppState;
use crate::env::EnvConfig;
use crate::global::GConfig;
use crate::init::InitStatus;
use brave_utils::jwt::config::JWTConfig;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Mutex;

pub mod admin;
pub mod app;
mod authority;
pub mod data;
mod db;
pub mod env;
mod error;
pub mod global;
pub mod init;
pub mod interface;

pub static GLOB_INIT: OnceCell<Mutex<InitStatus>> = OnceCell::new();

pub static GLOBAL_CONFIG: Lazy<GConfig> = Lazy::new(|| GConfig::open_yaml());

pub static GLOBAL_ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::get_env());

pub static GLOBAL_ADMIN_STATUS: Lazy<AdminState> = Lazy::new(|| AdminState::new());

pub async fn config_init() -> AppState {
    //初始化配置文件
    InitStatus::new(None);

    //初始化jwt配置
    JWTConfig::new(GLOBAL_CONFIG.jwt.clone());

    //数据库连接的
    AppState::new().await
}
