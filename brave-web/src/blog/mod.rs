mod page;

use actix_web::web;

//用于blog的页面加载
pub fn blog_config(cfg: &mut web::ServiceConfig) {
    cfg.service(page::index_page).service(page::page_handler);
}
