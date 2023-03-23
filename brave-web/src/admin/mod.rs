use crate::config::GLOBAL_ENV_CONFIG;
use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::web;

/*用于后台管理配置的*/
pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(&GLOBAL_ENV_CONFIG.admin_scope).service(
            Files::new("/", "./admin/dist")
                .index_file("index.html")
                .disable_content_disposition()
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
