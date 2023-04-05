use crate::init::InitStatus;
use crate::GLOBAL_CONFIG;
use serde::{Deserialize, Serialize};

//创建.env的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Interface {
    pub service_add: String,
    pub service_port: i16,
    pub api_scope: String,
    pub blog_scope: String,
    pub admin_scope: String,
}

impl Interface {
    pub fn get_api_string() -> String {
        format!(
            "{}:{}",
            GLOBAL_CONFIG.interface.service_add, GLOBAL_CONFIG.interface.service_port
        )
    }

    pub fn get_server_uri() -> String {
        format!(
            "http://{}:{}",
            GLOBAL_CONFIG.interface.service_add, GLOBAL_CONFIG.interface.service_port
        )
    }

    /*获取当前网站的地址*/
    pub fn get_current_address() -> String {
        format!(
            "{}:{}",
            GLOBAL_CONFIG.interface.service_add, GLOBAL_CONFIG.interface.service_port
        )
    }

    /*重新定向到主页上去*/
    pub fn redirect_home() -> String {
        format!(
            "http://{}:{}/",
            GLOBAL_CONFIG.interface.service_add, GLOBAL_CONFIG.interface.service_port,
        )
    }

    //用于获取需要重定向的位置
    /*登陆*/
    pub fn redirect_login_address() -> String {
        format!(
            "http://{}:{}/{}/login",
            GLOBAL_CONFIG.interface.service_add,
            GLOBAL_CONFIG.interface.service_port,
            GLOBAL_CONFIG.interface.admin_scope
        )
    }
    /*初始化*/
    pub fn redirect_init_address() -> String {
        format!(
            "http://{}:{}/{}/init",
            GLOBAL_CONFIG.interface.service_add,
            GLOBAL_CONFIG.interface.service_port,
            GLOBAL_CONFIG.interface.admin_scope
        )
    }

    /*用户blog*/
    pub fn redirect_admin_home() -> String {
        format!(
            "http://{}:{}/{}/{}/home",
            GLOBAL_CONFIG.interface.service_add,
            GLOBAL_CONFIG.interface.service_port,
            GLOBAL_CONFIG.interface.blog_scope,
            &InitStatus::global().username.clone().unwrap(),
        )
    }

    /*用户blog*/
    pub fn redirect_user_blog_home(name: &str) -> String {
        format!(
            "http://{}:{}/{}/{}/home",
            GLOBAL_CONFIG.interface.service_add,
            GLOBAL_CONFIG.interface.service_port,
            GLOBAL_CONFIG.interface.blog_scope,
            name,
        )
    }

    /*用户blog*/
    pub fn redirect_admin_blog() -> String {
        format!(
            "http://{}:{}/{}/{}/home",
            GLOBAL_CONFIG.interface.service_add,
            GLOBAL_CONFIG.interface.service_port,
            GLOBAL_CONFIG.interface.blog_scope,
            &InitStatus::global().username.clone().unwrap(),
        )
    }

    //重新定向到用户的category上去
    pub fn redirect_user_category(name: &str) -> String {
        format!(
            "http://{}:{}/{}/{}/category",
            GLOBAL_CONFIG.interface.service_add,
            GLOBAL_CONFIG.interface.service_port,
            GLOBAL_CONFIG.interface.blog_scope,
            name
        )
    }
}
