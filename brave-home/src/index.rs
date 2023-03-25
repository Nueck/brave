use actix_web::http::header;
use actix_web::{get, web, HttpResponse, Responder, Result};
use askama::Template;
use brave_config::init::InitStatus;
use brave_config::interface::Interface;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    admin_login: &'a str,
}

pub fn index_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    /*根据初始化状态*/
    if InitStatus::global().is_init {
        let add = Interface::redirect_login_address();
        let html = IndexTemplate { admin_login: &add }
            .render()
            .expect("template should be valid");
        Ok(HttpResponse::Ok().body(html))
    } else {
        /*重定向到初始化*/
        let init_add = Interface::redirect_init_address();
        Ok(HttpResponse::Found()
            .append_header((header::LOCATION, init_add))
            .finish())
    }
}
