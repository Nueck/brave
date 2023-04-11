use actix_web::{get, web, HttpResponse, Responder};
use brave_config::theme::ThemeConf;

pub fn skin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_skins);
}

//获取皮肤列表
#[get("/skins")]
async fn get_skins() -> impl Responder {
    if let Some(conf) = ThemeConf::read_theme_conf() {
        let data = conf.public;
        return HttpResponse::Ok().json(serde_json::json!({ "state": "success","data":data}));
    }
    HttpResponse::Ok().json(serde_json::json!({ "state": "error"}))
}
