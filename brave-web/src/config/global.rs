use crate::config::authority::AuthorityConfig;
use crate::config::interface::Interface;
use brave_utils::blake3::Blake3Config;
use brave_utils::jwt::config::JWTConfig;
use brave_utils::mail::MailConfig;
use serde::{Deserialize, Serialize};
use std::fs::File;

//创建yaml配置文件的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GConfig {
    pub interface: Interface,
    pub core_url: Option<String>,
    pub core_post_url: Option<String>,
    pub jwt: JWTConfig,
    pub authority: AuthorityConfig,
    pub blake: Blake3Config,
    pub mail: Option<MailConfig>,
}

impl GConfig {
    pub(crate) fn open_yaml() -> Self {
        // 读取yaml数据
        let f_yaml = File::open("myenv.yaml").expect("Could not open file.");
        // serde_yaml 解析字符串为 User 对象
        serde_yaml::from_reader(f_yaml).expect("Could not read values.")
    }
}
