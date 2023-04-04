use crate::authority::AuthorityConfig;
use crate::data::DataConfig;
use crate::interface::Interface;
use crate::utils::blake3::Blake3Config;
use crate::utils::jwt::JWTConfig;
use crate::utils::mail::MailConfig;
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
    pub data: Option<DataConfig>,
}

impl GConfig {
    pub(crate) fn open_yaml() -> Self {
        // 读取yaml数据
        let f_yaml = File::open("myenv.yaml").expect("Could not open file.");
        // serde_yaml 解析字符串为 User 对象
        serde_yaml::from_reader(f_yaml).expect("Could not read values.")
    }

    pub fn get_data(&self) -> DataConfig {
        if let Some(data) = &self.data {
            data.to_owned()
        } else {
            DataConfig {
                data_location: None,
                data: None,
            }
        }
    }
}
