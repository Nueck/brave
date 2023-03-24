use crate::config::admin::AdminState;
use crate::config::env::EnvConfig;
use crate::config::global::GConfig;
use crate::config::init::InitStatus;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Mutex;

pub mod admin;
pub mod app;
mod authority;
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
