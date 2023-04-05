pub mod admin;
mod entity;
mod error;
mod files;
mod functionally;
mod home;
mod index;
mod macros;
mod single_page;
mod static_html;
mod utils;

use crate::files::file_load_config;
use crate::index::{index_page, main_page};
use actix_files::Files;
use actix_web::web;

///用于首页显示配置的
pub fn home_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(home::index::index_config).service(
        Files::new("/", "./templates/")
            .redirect_to_slash_directory()
            .use_last_modified(true)
            .prefer_utf8(true),
    );
}

//用于blog的页面加载
pub fn blog_config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(main_page))
        .route("/", web::get().to(main_page))
        .route("/{name}", web::get().to(index_page))
        .route("/{name}/", web::get().to(index_page))
        .configure(static_html::blog_static_config)
        .configure(file_load_config); // .service(index::page_handler),
}
