use actix_web::http::header;
use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder, Result};
use brave_config::app::AppState;
use brave_config::blog::{
    generate_blog_table, get_blog_about, get_blog_contact, get_blog_content, get_blog_home,
};
use brave_config::interface::Interface;
use brave_db::entity::prelude::{Article, Users};
use brave_db::entity::{article, users};
use minijinja::{context, Environment};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[get("/{name}/content")]
pub async fn content_page(data: web::Data<AppState>, name: Path<String>) -> Result<impl Responder> {
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
            // Article::find().filter(article::Column::UserId.eq(user.user_id)).;
            let mut path_buf = PathBuf::new();
            path_buf.push("./page");
            path_buf.push(name.to_string());
            path_buf.push("content.html");

            let string = fs::read_to_string(path_buf).unwrap();

            #[derive(Clone, Deserialize, Serialize)]
            struct ArticleData {
                title: String,
                img_url: String,
                url: String,
            }

            //获取数据库中文章信息
            let articles = match Article::find()
                .filter(article::Column::UserId.eq(user.user_id.to_owned()))
                .order_by_desc(article::Column::ArticleId.to_owned())
                .all(db)
                .await
            {
                Ok(table) => table
                    .into_iter()
                    .map(|model: article::Model| ArticleData {
                        title: model.title.unwrap(),
                        img_url: model.img_url.unwrap(),
                        url: generate_blog_table(user.user_name.as_str(), &model.article_id),
                    })
                    .collect::<Vec<ArticleData>>()
                    .to_vec(),
                Err(_) => {
                    let home = Interface::redirect_home();
                    return Ok(HttpResponse::Found()
                        .append_header((header::LOCATION, home))
                        .finish());
                }
            };

            let mut env = Environment::new();
            env.add_template("content", &string).unwrap();
            let tmpl = env.get_template("content").unwrap();

            let name = user.user_name.to_string();
            let personal_details = user.user_name.to_string();
            let home = get_blog_home(user.user_name.as_str());
            let about = get_blog_about(user.user_name.as_str());
            let content = get_blog_content(user.user_name.as_str());
            let contact = get_blog_contact(user.user_name.as_str());

            let str = tmpl
                .render(context! {name,personal_details,home,about,content,contact,articles})
                .unwrap();
            Ok(HttpResponse::Ok().body(str))
        }
    }
}
