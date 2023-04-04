use crate::admin::AdminState;
use crate::app::AppState;
use crate::data::DataConfig;
use crate::env::EnvConfig;
use crate::global::GConfig;
use crate::init::InitStatus;
use crate::utils::jwt::JWTConfig;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Mutex;

pub mod admin;
pub mod app;
mod authority;
pub mod blog;
pub mod data;
mod db;
pub mod env;
mod error;
pub mod global;
pub mod init;
pub mod interface;
pub mod utils;

pub static GLOB_INIT: OnceCell<Mutex<InitStatus>> = OnceCell::new();

pub static GLOBAL_CONFIG: Lazy<GConfig> = Lazy::new(|| GConfig::open_yaml());

pub static GLOBAL_ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::get_env());

pub static GLOBAL_ADMIN_STATUS: Lazy<AdminState> = Lazy::new(|| AdminState::new());

pub static GLOBAL_DATA: Lazy<DataConfig> = Lazy::new(|| GLOBAL_CONFIG.get_data());

pub async fn config_init() -> AppState {
    //初始化配置文件
    InitStatus::new(None);

    //初始化jwt配置
    JWTConfig::new(GLOBAL_CONFIG.jwt.clone());

    //数据库连接的
    AppState::new().await
}
