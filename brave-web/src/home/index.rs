use actix_web::{get, web, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub fn index_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let html = IndexTemplate.render().expect("template should be valid");
    Ok(Html(html))
}
