use crate::error::page::PageError;
use actix_web::http::header;
use actix_web::web::{Path, Query};
use actix_web::{get, web, HttpResponse, Responder, Result};
use brave_config::app::AppState;
use brave_config::blog::{get_blog_about, get_blog_contact, get_blog_content, get_blog_home};
use brave_config::interface::Interface;
use brave_db::entity::prelude::{Article, Users};
use brave_db::entity::{article, users};
use minijinja::{context, Environment};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

//用于blog的页面加载
pub fn blog_static_page_config(cfg: &mut web::ServiceConfig) {
    cfg.service(page).service(page_error); // .service(index::page_handler),
}

#[get("/{name}/page")]
async fn page_error(data: web::Data<AppState>, name: Path<String>) -> Result<impl Responder> {
    let db = &data.conn;

    match Users::find()
        .filter(users::Column::UserName.contains(&name))
        .one(db)
        .await
        .expect("Could not find Users")
    {
        None => {
            let home = Interface::redirect_home();
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, home))
                .finish())
        }
        Some(user) => {
            let blog = Interface::redirect_user_blog_home(user.user_name.as_str());
            return Ok(HttpResponse::Found()
                .append_header((header::LOCATION, blog))
                .finish());
        }
    }
}

#[derive(Deserialize)]
struct PageInfo {
    pub table_id: Option<i64>,
}

#[get("/{name}/page")]
async fn page(
    data: web::Data<AppState>,
    name: Path<String>,
    query: Query<PageInfo>,
) -> Result<impl Responder, PageError> {
    /*文件路径先设置在当前目录public下*/
    let db = &data.conn;

    match Users::find()
        .filter(users::Column::UserName.contains(&name))
        .one(db)
        .await
        .expect("Could not find Users")
    {
        None => {
            let home = Interface::redirect_home();
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, home))
                .finish())
        }
        Some(user) => {
            if query.table_id.is_none() {
                return Err(PageError {
                    user_name: user.user_name.to_string(),
                });
            }

            // Article::find().filter(article::Column::UserId.eq(user.user_id)).;
            let mut path_buf = PathBuf::new();
            path_buf.push("./page");
            path_buf.push(name.to_string());
            path_buf.push("page.html");

            let model = Article::find_by_id(query.table_id.unwrap())
                .filter(article::Column::UserId.eq(user.user_id))
                .one(db)
                .await
                .expect("Could not find Article Table ")
                .unwrap();

            //获取文章的页面数据
            let title = if let Some(data) = model.title {
                data
            } else {
                "".to_string()
            };
            let subtitle = if let Some(data) = model.subtitle {
                data
            } else {
                "".to_string()
            };

            let http_content = if let Some(data) = model.html_content {
                data
            } else {
                "".to_string()
            };

            let bg_img = if let Some(data) = model.img_url {
                data
            } else {
                "".to_string()
            };

            let string = fs::read_to_string(path_buf).unwrap();

            let mut env = Environment::new();
            env.add_template("page", &string).unwrap();
            let tmpl = env.get_template("page").unwrap();

            let name = user.user_name.to_string();
            let personal_details = user.user_name.to_string();
            let home = get_blog_home(user.user_name.as_str());
            let about = get_blog_about(user.user_name.as_str());
            let content = get_blog_content(user.user_name.as_str());
            let contact = get_blog_contact(user.user_name.as_str());

            //对数据处理一下

            let str = tmpl
                .render(context! {name,personal_details,home,about,content,contact,http_content,title,subtitle,bg_img})
                .unwrap();
            Ok(HttpResponse::Ok().body(str))
        }
    }
}
