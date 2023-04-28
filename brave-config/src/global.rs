use crate::authority::AuthorityConfig;
use crate::data::DataConfig;
use crate::interface::Interface;
use crate::page::PageConfig;
use crate::theme::ThemePosition;
use crate::utils::blake3::Blake3Config;
use crate::utils::jwt::JWTConfig;
use crate::utils::mail::MailConfig;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::PathBuf;

//创建yaml配置文件的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GConfig {
    pub interface: Interface,
    pub core_url: Option<String>,
    pub core_post_url: Option<String>,
    pub jwt: JWTConfig,
    pub authority: AuthorityConfig,
    blake: Option<Blake3Config>,
    pub mail: Option<MailConfig>,
    data: Option<DataConfig>,
    page: Option<PageConfig>,
    theme: Option<ThemePosition>,
    registrants: Option<i64>,
    able_register: Option<bool>,
}

impl GConfig {
    pub(crate) fn open_yaml() -> Self {
        #[cfg(debug_assertions)]
        let f_yaml = File::open("myenv.yaml").expect("Could not open file.");

        #[cfg(not(debug_assertions))]
        let f_yaml = File::open("config.yaml").expect("Could not open file.");

        serde_yaml::from_reader(f_yaml).expect("Could not read values.")
    }

    pub fn get_data(&self) -> &DataConfig {
        if let Some(data) = &self.data {
            data
        } else {
            &DataConfig {
                data_location: None,
                data: None,
            }
        }
    }

    pub fn get_blake(&self) -> Blake3Config {
        if let Some(data) = &self.blake {
            data.to_owned()
        } else {
            Blake3Config {
                salt: Some("brave".to_string()),
            }
        }
    }

    pub fn get_page(&self) -> String {
        if let Some(data) = &self.page {
            data.location.to_string()
        } else {
            "page".to_string()
        }
    }

    pub fn get_theme(&self) -> ThemePosition {
        if let Some(data) = &self.theme {
            data.to_owned()
        } else {
            ThemePosition {
                location: Some("themes".to_string()),
            }
        }
    }

    //获取可登记人数(默认走二个用户)
    pub fn get_registrants(&self) -> i64 {
        if let Some(registrants) = &self.registrants {
            registrants.to_owned()
        } else {
            2
        }
    }
    //获得是否可注册
    pub fn get_able_register(&self) -> bool {
        if let Some(able_register) = &self.able_register {
            able_register.to_owned()
        } else {
            true
        }
    }

    //获取当前目录
    pub fn get_current_path(&self) -> PathBuf {
        PathBuf::from(env::current_dir().unwrap())
    }
}
