use actix_web::http::header;
use actix_web::HttpResponse;
use brave_config::init::InitStatus;
use brave_config::interface::Interface;
use brave_config::theme::{Config, PageAttr};
use brave_config::GLOBAL_CONFIG;
use std::fs::File;
use std::path::PathBuf;

//判断初始化状态
pub fn init_status_jump() -> Result<HttpResponse, ()> {
    if InitStatus::global().is_init {
        Err(())
    } else {
        let init_add = Interface::redirect_init_address();
        Ok(HttpResponse::Found()
            .append_header((header::LOCATION, init_add))
            .finish())
    }
}

//获取页面的位置
pub fn get_page_location(name: &str) -> PathBuf {
    let mut page = PathBuf::from(GLOBAL_CONFIG.get_page());
    page.push(name);
    page
}

//获取页面的属性
pub fn get_page_attr(name: &str) -> PageAttr {
    let mut config = get_page_location(name);
    config.push("conf.json");

    if config.exists() {
        match File::open(config) {
            Ok(f) => {
                let conf: Config = serde_json::from_reader(f).unwrap_or(Config {
                    name: Some("Null".to_string()),
                    mode: Some(PageAttr::Static),
                });
                conf.mode.unwrap_or(PageAttr::Static)
            }
            Err(e) => {
                log::error!("{}", e);
                PageAttr::Static
            }
        }
    } else {
        PageAttr::Static
    }
}
