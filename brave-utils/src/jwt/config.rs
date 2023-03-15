use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

//全局的变量
pub static GLOB_JWT_CONFIG: OnceCell<JWTConfig> = OnceCell::new();

//JWT的配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JWTConfig {
    pub exp_time: Option<u64>,
    pub sub: Option<String>,
    pub ref_time: Option<u64>,
    pub code_time: Option<u64>,
}

//JWTConfig配置
impl JWTConfig {
    pub fn global() -> &'static JWTConfig {
        GLOB_JWT_CONFIG
            .get()
            .expect("jwt config is not initialized")
    }

    pub fn new(jwt_config: JWTConfig) {
        GLOB_JWT_CONFIG
            .set(jwt_config)
            .expect("JWT config Initialization failure")
    }

    pub fn get_exp_time(&self) -> u64 {
        match &self.exp_time {
            None => 1000, //默认值
            Some(num) => num.to_owned(),
        }
    }

    pub fn get_ref_time(&self) -> u64 {
        match &self.ref_time {
            None => 1000, //默认值
            Some(num) => num.to_owned(),
        }
    }

    pub fn get_code_time(&self) -> u64 {
        match &self.code_time {
            None => 300, //默认值
            Some(num) => num.to_owned(),
        }
    }

    pub fn get_sub(&self) -> String {
        match &self.sub {
            None => "brave".to_string(),
            Some(s) => s.to_string(),
        }
    }
}
