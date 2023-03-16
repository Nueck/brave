use crate::db::connect_db;
use brave_utils::blake3::Blake3Config;
use brave_utils::jwt::config::JWTConfig;
use brave_utils::mail::MailConfig;
use config::Config;
use once_cell::sync::{Lazy, OnceCell};
use sea_orm::{ConnectOptions, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::{env, fmt, fs};

//设置全局变量
pub static GLOBAL_ENV_CONFIG: Lazy<EnvConfig> = Lazy::new(|| EnvConfig::get_env());
pub static GLOBAL_YAML_CONFIG: Lazy<GConfig> = Lazy::new(|| GConfig::open_yaml());
//全局的一个只能呢个变化一次的
pub static GLOB_INIT: OnceCell<InitStatus> = OnceCell::new();

/*初始化的状态 */
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InitStatus {
    pub is_init: bool,
}

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

/*权限信息*/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorityConfig {
    pub auth: Option<Vec<String>>,
    pub super_admin: Option<String>,
    pub admin: Option<String>,
}

//创建yaml配置文件的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GConfig {
    pub core_url: Option<String>,
    pub core_post_url: Option<String>,
    pub jwt: JWTConfig,
    pub authority: AuthorityConfig,
    pub blake: Blake3Config,
    pub mail: Option<MailConfig>,
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

impl InitStatus {
    pub fn global() -> &'static InitStatus {
        GLOB_INIT
            .get()
            .expect("InitStatus config is not initialized")
    }

    pub fn new(init: InitStatus) {
        /*首先先读取配置文件中的配置*/
        // 获取 home 目录
        let mut config_path = dirs::config_dir().expect("Failed to get config directory");

        // 添加myapp文件夹到路径
        config_path.push("brave");
        // 创建brave文件夹（如果不存在）
        if !config_path.exists() {
            fs::create_dir_all(&config_path).expect("Failed to create directory")
        }
        config_path.push("config");
        /*判断文件是否存在 .config/brave/config*/
        if config_path.exists() {
            let f_config = std::fs::File::open(config_path).expect("Could not open conf file");
            let config: InitStatus =
                serde_json::from_reader(f_config).expect("Could not read conf file");

            GLOB_INIT
                .set(config)
                .expect("InitStatus Initialization failure")
        } else {
            /*需要创建文件并且将数据存在配置文件*/
            let json = serde_json::to_string_pretty(&init).expect("InitStatus to Json failure");
            /*将文件保存在配置文件中*/
            let mut file = File::create(config_path.as_path()).expect("Could not open conf file");
            file.write_all(json.as_bytes())
                .expect("Description Failed to write the configuration file");

            GLOB_INIT
                .set(init)
                .expect("InitStatus config Initialization failure")
        }
    }
}

/*对数据库的配置*/
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
            //设置默认密码
            url.push_str(":123456");
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

        let mut opt = ConnectOptions::new(url);

        //基础设置(这里先这样写死后期可能开放出来配置)
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

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

impl AuthorityConfig {
    #[allow(unused)]
    fn new() -> AuthorityConfig {
        AuthorityConfig {
            /*默认值*/
            auth: Some(vec![
                "super".to_string(),
                "admin".to_string(),
                "user".to_string(),
            ]),
            super_admin: Some("super".to_string()),
            admin: Some("admin".to_string()),
        }
    }

    /*获取默认的*/
    pub fn get_authority_config(&self) -> Self {
        let mut auth_config = AuthorityConfig::new();

        if let Some(auth) = &self.auth {
            auth_config.auth = Some(auth.clone());
        }

        if let Some(admin) = &self.admin {
            auth_config.admin = Some(admin.clone());
        }

        if let Some(super_admin) = &self.super_admin {
            auth_config.super_admin = Some(super_admin.clone());
        }

        auth_config
    }
}

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
