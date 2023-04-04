use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::web;
use brave_config::GLOBAL_CONFIG;

///用于后台管理配置的
pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(&GLOBAL_CONFIG.interface.admin_scope).service(
            Files::new("/", "./admin/dist")
                .index_file("index.html")
                .use_last_modified(true)
                .show_files_listing()
                .default_handler(fn_service(|req: ServiceRequest| async {
                    let (req, _) = req.into_parts();
                    let file = NamedFile::open_async("./admin/dist/index.html").await?;
                    let res = file.into_response(&req);
                    Ok(ServiceResponse::new(req, res))
                }))
                .prefer_utf8(true),
        ),
    );
}
