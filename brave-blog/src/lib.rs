mod index;

use actix_cors::Cors;
use actix_web::web;
use brave_config::GLOBAL_CONFIG;

//用于blog的页面加载
pub fn blog_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(&GLOBAL_CONFIG.interface.blog_scope) //博客方面的加载
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allowed_methods(vec!["GET"]) //只允许GET
                    .allow_any_origin() //允许任何来源
                    .max_age(3600),
            )
            .service(index::index_page)
            .service(index::page_handler),
    );
}
