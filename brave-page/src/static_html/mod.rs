use crate::static_html::page::blog_static_page_config;
use actix_web::web;

mod about;
mod category;
mod contact;
mod content;
mod error_page;
mod home;
mod page;

//用于blog的页面加载
pub(crate) fn blog_static_config(cfg: &mut web::ServiceConfig) {
    cfg.service(home::home_page)
        .service(about::about_page)
        .service(error_page::error_page)
        .service(content::content_page)
        .service(category::category_page)
        .service(contact::contact_page)
        .configure(blog_static_page_config); // .service(index::page_handler),
}
