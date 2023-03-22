use crate::config::env::EnvConfig;
use crate::config::global::GConfig;
use crate::config::init::InitStatus;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Mutex;

pub mod app;
mod authority;
mod db;
pub mod env;
mod error;
pub mod global;
pub mod init;

//设置全局变量
pub static GLOBAL_ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::get_env());
pub static GLOBAL_YAML_CONFIG: Lazy<GConfig> = Lazy::new(|| GConfig::open_yaml());
//全局的服务器信息
pub static GLOB_INIT: OnceCell<Mutex<InitStatus>> = OnceCell::new();
