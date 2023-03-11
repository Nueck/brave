mod test;

use actix_web::web;

//用于blog的页面加载
pub fn web_config_init(cfg: &mut web::ServiceConfig) {
    cfg.service(test::token_checker_handler);
}
