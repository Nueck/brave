use crate::config::interface::Interface;
use actix_web::web::head;
use actix_web::{get, web, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

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
    //获取后端的登陆

    let add = Interface::redirect_init_add();
    let html = IndexTemplate { admin_login: &add }
        .render()
        .expect("template should be valid");
    Ok(Html(html))
}
