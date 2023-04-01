/*用于首页显示配置的*/
mod files;

use actix_files::Files;
use actix_web::web;
use brave_config::GLOBAL_DATA;

pub fn data_config(cfg: &mut web::ServiceConfig) {
    //这个可以对数据来源接口进行管理
    cfg.service(
        web::scope("v").service(
            Files::new("/", &GLOBAL_DATA.get_data_config().data_location.unwrap())
                // .default_handler(fn_service(|req: ServiceRequest| async {
                //     let (req, _) = req.into_parts();
                //     Ok(ServiceResponse::new(
                //         req,
                //         HttpResponse::Found()
                //             .append_header((header::LOCATION, "/"))
                //             .finish(),
                //     ))
                // }))
                .use_last_modified(true)
                .prefer_utf8(true),
        ),
    );
}
