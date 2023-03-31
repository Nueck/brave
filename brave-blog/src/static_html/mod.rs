use actix_web::web;

mod about;
mod category;
mod contact;
mod content;
mod error;
mod home;
mod page;

//用于blog的页面加载
pub fn blog_static_config(cfg: &mut web::ServiceConfig) {
    cfg.service(home::home_page)
        .service(about::about_page)
        .service(error::error_page)
        .service(content::content_page)
        .service(category::category_page)
        .service(contact::contact_page)
        .service(page::page); // .service(index::page_handler),
}
