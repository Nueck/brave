use crate::entity::ArticlesInfo;
use crate::error::page::PageError;
use crate::files::file_load;
use actix_web::http::header;
use actix_web::web::Path;
use actix_web::{get, web, HttpResponse, Responder, Result};
use brave_config::app::AppState;
use brave_config::blog::{get_blog_about, get_blog_contact, get_blog_content, get_blog_home};
use brave_config::interface::Interface;
use brave_db::entity::prelude::{Article, Users};
use brave_db::entity::{article, users};
use minijinja::{context, Environment};
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use std::fs;
use std::path::PathBuf;

//用于blog的页面加载
pub fn blog_static_page_config(cfg: &mut web::ServiceConfig) {
    cfg.service(page)
        .service(page_error)
        .route("/{name}/page/{filename:.*}", web::get().to(file_load));
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

#[get("/{name}/page/{table_id}")]
async fn page(
    data: web::Data<AppState>,
    path: Path<(String, i64)>,
) -> Result<impl Responder, PageError> {
    /*文件路径先设置在当前目录public下*/
    let db = &data.conn;
    let (name, table_id) = path.into_inner();

    match Article::find()
        .select_only()
        .columns([article::Column::Subtitle, article::Column::Title])
        .column(article::Column::Content)
        .column(article::Column::HtmlContent)
        .column_as(article::Column::ImgUrl, "bg_img")
        .column(article::Column::Url)
        .join(JoinType::InnerJoin, article::Relation::Users.def())
        .filter(users::Column::UserName.contains(&name))
        .filter(article::Column::ArticleId.eq(table_id))
        .into_model::<ArticlesInfo>()
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
        Some(article) => {
            // Article::find().filter(article::Column::UserId.eq(user.user_id)).;
            let mut path_buf = PathBuf::new();
            path_buf.push("./page");
            path_buf.push(name.to_string());
            path_buf.push("page.html");

            //获取文章的页面数据
            let title = article.title;
            let subtitle = article.subtitle;
            let http_content = article.html_content;
            let article_content = article.content;
            let bg_img = article.bg_img;

            let string = fs::read_to_string(path_buf).unwrap();

            let mut env = Environment::new();
            env.add_template("page", &string).unwrap();
            let tmpl = env.get_template("page").unwrap();

            let personal_details = &name.to_string();
            let home = get_blog_home(&name.as_str());
            let about = get_blog_about(&name.as_str());
            let content = get_blog_content(&name.as_str());
            let contact = get_blog_contact(&name.as_str());

            //对数据处理一下
            let str = tmpl
                .render(context! {name,personal_details,home,about,content,contact,article_content,http_content,title,subtitle,bg_img})
                .unwrap();
            Ok(HttpResponse::Ok().body(str))
        }
    }
}
