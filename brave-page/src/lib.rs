mod error;
mod files;
mod home;
mod index;
mod static_html;

use crate::index::{index_page, main_page};
use actix_files::Files;
use actix_web::web;

///用于首页显示配置的
pub fn home_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(home::index::index_config).service(
        Files::new("/", "./templates/")
            .index_file("index.html")
            // .default_handler(fn_service(|req: ServiceRequest| async {
            //     let (req, _) = req.into_parts();
            //     Ok(ServiceResponse::new(
            //         req,
            //         HttpResponse::Found()
            //             .append_header((header::LOCATION, "/"))
            //             .finish(),
            //     ))
            // }))
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
        .service(files::file_load); // .service(index::page_handler),
}
