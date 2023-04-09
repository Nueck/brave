use crate::admin::AdminState;
use crate::app::AppState;
use crate::data::DataConfig;
use crate::env::EnvConfig;
use crate::global::GConfig;
use crate::init::InitStatus;
use crate::interface::Interface;
use crate::theme::ThemeConf;
use crate::utils::jwt::JWTConfig;
use once_cell::sync::{Lazy, OnceCell};
use std::fs;
use std::path::PathBuf;
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
pub mod page;
pub mod theme;
pub mod utils;

pub(crate) static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut config_path = dirs::config_dir().expect("Failed to get config directory");
    //默认config文件夹
    config_path.push("brave");

    if !config_path.exists() {
        fs::create_dir_all(&config_path).expect("Failed to create directory")
    }
    config_path
});

pub static GLOB_INIT: OnceCell<Mutex<InitStatus>> = OnceCell::new();

pub static GLOBAL_CONFIG: Lazy<GConfig> = Lazy::new(|| GConfig::open_yaml());

pub static GLOBAL_ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::get_env());

pub static GLOBAL_ADMIN_STATUS: Lazy<AdminState> = Lazy::new(|| AdminState::new());

pub static GLOBAL_DATA: Lazy<&DataConfig> = Lazy::new(|| GLOBAL_CONFIG.get_data());

pub async fn config_init() -> AppState {
    //初始化配置文件
    InitStatus::new(None);

    //主题初始化
    ThemeConf::init();

    //初始化jwt配置
    JWTConfig::new(GLOBAL_CONFIG.jwt.clone());

    //home
    println!(
        "Home service start: http://{}/",
        Interface::get_api_string()
    );

    //admin
    println!(
        "Admin service start: http://{}/{}/",
        Interface::get_api_string(),
        &GLOBAL_CONFIG.interface.admin_scope
    );

    //blog
    println!(
        "Blog service start: http://{}/{}/",
        Interface::get_api_string(),
        &GLOBAL_CONFIG.interface.blog_scope
    );
    //api
    println!(
        "API service start: http://{}/{}/",
        Interface::get_api_string(),
        &GLOBAL_CONFIG.interface.api_scope
    );

    //是否开启自动装载
    if let Some(data) = &GLOBAL_ENV_CONFIG.template_autoload {
        if *data {
            log::info!("template auto-reloading is enabled");
        } else {
            log::warn!(
                "template auto-reloading is disabled; run with template_autoload=true to enable"
            );
        }
    };

    //数据库连接的
    AppState::new().await
}
