use crate::config::db::PGConfig;
use crate::config::GLOBAL_ENV_CONFIG;
use config::Config;
use serde::{Deserialize, Serialize};

//创建.env的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvConfig {
    pub api_add: String,
    pub api_port: i16,
    pub api_scope: String,
    pub blog_scope: String,
    pub admin_scope: String,
    pub pg: PGConfig,
}

impl EnvConfig {
    pub fn get_api_string() -> String {
        format!(
            "{}:{}",
            GLOBAL_ENV_CONFIG.api_add, GLOBAL_ENV_CONFIG.api_port
        )
    }

    pub fn get_env() -> Self {
        //初始化env数据
        Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()
            .expect("Failed to initialize env")
            .try_deserialize()
            .expect("config序列化失败")
    }
}
