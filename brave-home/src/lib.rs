mod index;

use actix_files::Files;
use actix_web::web;

/*用于首页显示配置的*/
pub fn home_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(index::index_config).service(
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
