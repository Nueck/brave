use actix_files::NamedFile;
use actix_web::http::header;
use actix_web::web::Path;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use brave_config::app::AppState;
use brave_config::interface::Interface;
use brave_db::entity::prelude::Users;
use brave_db::entity::users;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::path::PathBuf;

/*用于页面的加载*/
pub async fn main_page() -> Result<impl Responder> {
    let admin_blog_add = Interface::redirect_admin_home();
    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, admin_blog_add))
        .finish())
}

#[get("/{name}")]
pub async fn index_page(data: web::Data<AppState>, name: Path<String>) -> Result<impl Responder> {
    /*判断数据库中是否有这个用户*/
    let db = &data.conn;
    match Users::find()
        .filter(users::Column::UserName.contains(&name))
        .one(db)
        .await
        .expect("Could not find Users -- file load blog")
    {
        None => {
            let home = Interface::redirect_home();
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, home))
                .finish())
        }
        Some(_) => {
            /*将用户重定向到home下*/
            let blog = Interface::redirect_user_blog_home(&name);
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, blog))
                .finish())
        }
    }
}

/*用于页面的加载*/
#[get("/{name}/home")]
pub async fn home_page(
    data: web::Data<AppState>,
    name: Path<String>,
    req: HttpRequest,
) -> Result<impl Responder> {
    /*文件路径先设置在当前目录public下*/
    let db = &data.conn;
    match Users::find()
        .filter(users::Column::UserName.contains(&name))
        .one(db)
        .await
        .expect("Could not find Users -- file load blog")
    {
        None => {
            let home = Interface::redirect_home();
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, home))
                .finish())
        }
        Some(_) => {
            let mut path_buf = PathBuf::new();
            path_buf.push("./page");
            path_buf.push(name.to_string());
            path_buf.push("index.html");
            match NamedFile::open(path_buf) {
                Ok(content) => Ok(content.into_response(&req)),
                Err(_) => {
                    let home = Interface::redirect_home();
                    Ok(HttpResponse::Found()
                        .append_header((header::LOCATION, home))
                        .finish())
                }
            }
        }
    }
}
