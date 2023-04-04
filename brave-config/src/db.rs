use crate::error::ConfigError;
use sea_orm::ConnectOptions;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PGConfig {
    pub user: Option<String>,
    pub password: Option<String>,
    pub dbname: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
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
