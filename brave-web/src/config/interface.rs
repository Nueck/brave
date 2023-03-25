use crate::config::GLOBAL_CONFIG;
use serde::{Deserialize, Serialize};

//创建.env的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Interface {
    pub api_add: String,
    pub api_port: i16,
    pub api_scope: String,
    pub blog_scope: String,
    pub admin_scope: String,
}

impl Interface {
    pub fn get_api_string() -> String {
        format!(
            "{}:{}",
            GLOBAL_CONFIG.interface.api_add, GLOBAL_CONFIG.interface.api_port
        )
    }

    //用于获取需要重定向的位置
    pub fn redirect_init_add() -> String {
        format!(
            "http://{}:{}/{}/login",
            GLOBAL_CONFIG.interface.api_add,
            GLOBAL_CONFIG.interface.api_port,
            GLOBAL_CONFIG.interface.admin_scope
        )
    }
}
