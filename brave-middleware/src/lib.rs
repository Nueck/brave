use brave_utils::jwt::jwt::Jot;
use once_cell::sync::Lazy;

pub mod auth_middleware;

//全局的变量
pub static GLOB_JOT: Lazy<Jot> = Lazy::new(|| Jot::new());
