mod page;

use crate::config::GLOBAL_ENV_CONFIG;
use actix_cors::Cors;
use actix_web::web;

//用于blog的页面加载
pub fn blog_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(&GLOBAL_ENV_CONFIG.blog_scope) //博客方面的加载
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allowed_methods(vec!["GET"]) //只允许GET
                    .allow_any_origin() //允许任何来源
                    .max_age(3600),
            )
            .service(page::index_page)
            .service(page::page_handler),
    );
}
