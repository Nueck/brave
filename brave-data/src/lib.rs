/*用于首页显示配置的*/
use actix_web::web;

pub fn data_config(cfg: &mut web::ServiceConfig) {
    //这个可以对数据来源接口进行管理
    cfg.service(web::scope("v"));
}
