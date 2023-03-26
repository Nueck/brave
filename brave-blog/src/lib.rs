mod files;
mod index;

use actix_web::web;

//用于blog的页面加载
pub fn blog_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::main_page)
        .service(index::index_page)
        .service(index::home_page)
        .service(files::file_load); // .service(index::page_handler),
}
