use crate::db::connect_db;
use cako_utils::jwt::config::JWTConfig;
use config::Config;
use once_cell::sync::Lazy;
use sea_orm::{ConnectOptions, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::{env, fmt};

//设置全局变量
pub static GLOBAL_ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::get_env());
pub static GLOBAL_YAML_CONFIG: Lazy<GConfig> = Lazy::new(|| GConfig::open_yaml());

//创建.env的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvConfig {
    pub api_add: String,
    pub api_port: i16,
    pub api_scope: String,
    pub web_scope: String,
    pub pg: PGConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PGConfig {
    pub user: Option<String>,
    pub password: Option<String>,
    pub dbname: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

//创建yaml配置文件的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GConfig {
    pub core_url: Option<String>,
    pub core_post_url: Option<String>,
    pub jwt: JWTConfig,
}

//数据库连接问题
#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

//获取AppState
impl AppState {
    pub async fn new() -> Self {
        //连接数据库
        let conn = connect_db().await;
        AppState { conn }
    }
}

//修改deadpool中代码
impl PGConfig {
    pub fn get_pb_connect_opt(&self) -> Result<ConnectOptions, ConfigError> {
        let mut url = String::new();

        //初始化pgsql
        url.push_str("postgres://");
        if let Some(user) = &self.user {
            url.push_str(user.as_str());
        } else if let Ok(user) = env::var("USER") {
            url.push_str(user.as_str());
        } else {
            return Err(ConfigError::UserNameMissing);
        }

        if let Some(password) = &self.password {
            //添加用户名和密码间距
            url.push_str(":");
            url.push_str(password);
        } else {
            url.push_str(":");
            //设置默认密码
            url.push_str("123456");
        }

        if let Some(host) = &self.host {
            url.push_str("@");
            url.push_str(host.as_str());
        }
        if let Some(port) = self.port {
            url.push_str(":");
            let url_port = port.to_string();
            url.push_str(&*url_port);
        }

        match &self.dbname {
            Some(dbname) => match dbname.as_str() {
                "" => return Err(ConfigError::DbnameMissing),
                dbname => {
                    url.push_str("/");
                    url.push_str(dbname);
                }
            },
            None => return Err(ConfigError::DbnameEmpty),
        };

        let opt = ConnectOptions::new(url);

        Ok(opt)
    }
}

impl GConfig {
    fn open_yaml() -> Self {
        // 读取yaml数据
        let f_yaml = std::fs::File::open("config.yaml").expect("Could not open file.");
        // serde_yaml 解析字符串为 User 对象
        serde_yaml::from_reader(f_yaml).expect("Could not read values.")
    }
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

//这里引用deadpool中的代码
#[derive(Copy, Clone, Debug)]
pub enum ConfigError {
    DbnameMissing,
    DbnameEmpty,
    UserNameMissing,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbnameMissing => write!(f, "configuration property \"dbname\" not found"),
            Self::DbnameEmpty => write!(
                f,
                "configuration property \"dbname\" contains an empty string",
            ),
            Self::UserNameMissing => write!(f, "configuration property \"user\" not found",),
        }
    }
}
