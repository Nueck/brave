use crate::db::PGConfig;
use config::Config;
use serde::{Deserialize, Serialize};

//创建.env的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvConfig {
    pub pg: PGConfig,
    pub template_autoload: Option<bool>,
    pub enable_ssl: Option<bool>, //TODO:之后写SSL
}

impl EnvConfig {
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
