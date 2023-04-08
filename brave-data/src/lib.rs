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
                .use_last_modified(true)
                .prefer_utf8(true),
        ),
    );
}
