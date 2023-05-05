pub mod admin;
mod entity;
mod error;
mod files;
mod functionally;
mod home;
mod index;
mod macros;
mod static_html;
mod template;
mod utils;

use crate::files::{file_load_config, single_files_load, single_index_load};
use crate::home::error::not_found;
use crate::index::{index_page, main_page};
use crate::template::template_init;
use crate::utils::{SingleGuard, StaticGuard};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use actix_web::web;
use brave_config::GLOBAL_CONFIG;

//用于页面路由加载
pub fn page_config(cfg: &mut web::ServiceConfig) {
    let tmpl_reload = web::Data::new(template_init());

    cfg.service(
        web::scope("")
            .app_data(tmpl_reload.clone())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allowed_methods(vec!["GET"]) //只允许GET
                    .allow_any_origin() //允许任何来源
                    .max_age(3600),
            )
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found))
            .service(
                web::scope(&GLOBAL_CONFIG.interface.blog_scope)
                    .route("", web::get().to(main_page))
                    .route("/", web::get().to(main_page))
                    .service(
                        web::scope("/{name}")
                            .guard(StaticGuard)
                            .configure(blog_config),
                    )
                    .service(
                        web::scope("/{name}")
                            .guard(SingleGuard)
                            .route("", web::get().to(single_index_load))
                            .route("/", web::get().to(single_index_load))
                            .service(single_files_load),
                    ),
            )
            .configure(home_config),
    );
}

///用于首页显示配置的
fn home_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(home::index::index_config).service(
        Files::new("/", "./templates/")
            .redirect_to_slash_directory()
            .use_last_modified(true)
            .prefer_utf8(true),
    );
}

//用于blog的页面加载
fn blog_config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(index_page))
        .route("/", web::get().to(index_page))
        .configure(static_html::blog_static_config)
        .configure(file_load_config);
}
