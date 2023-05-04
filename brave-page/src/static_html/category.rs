use crate::entity::ArticlesInfo;
use crate::files::file_load;
use crate::utils::common::get_page_location;
use actix_web::http::header;
use actix_web::web::{self, Path};
use actix_web::{get, HttpResponse, Responder, Result};
use brave_config::app::AppState;
use brave_config::blog::{
    get_blog_about, get_blog_contact, get_blog_content, get_blog_error, get_blog_home,
};
use brave_config::interface::Interface;
use brave_db::entity::article;
use brave_db::entity::prelude::Users;
use brave_db::entity::users;
use minijinja::{context, Environment};
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use std::fs;
use std::path::PathBuf;

pub(crate) fn blog_static_category_config(cfg: &mut web::ServiceConfig) {
    cfg.service(category_page)
        .service(category_info_page)
        .route("/{name}/category/{filename:.*}", web::get().to(file_load));
}

//用于分类和内容显示
#[get("/{name}/category")]
pub async fn category_page(
    data: web::Data<AppState>,
    name: Path<String>,
) -> Result<impl Responder> {
    //文件路径先设置在当前目录public下
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
            let mut path_buf = get_page_location(user.user_name.as_str());
            path_buf.push("category.html");

            let string = match fs::read_to_string(path_buf) {
                Ok(t) => t,
                Err(_) => {
                    let error = get_blog_error(user.user_name.as_str());
                    return Ok(HttpResponse::Found()
                        .append_header((header::LOCATION, error))
                        .finish());
                }
            };

            let mut env = Environment::new();
            env.add_template("category", &string).unwrap();
            let tmpl = env.get_template("category").unwrap();

            //用于渲染的数据
            let name = user.user_name.to_string();
            let personal_details = user.user_name.to_string();
            let home = get_blog_home(user.user_name.as_str());
            let about = get_blog_about(user.user_name.as_str());
            let content = get_blog_content(user.user_name.as_str());
            let contact = get_blog_contact(user.user_name.as_str());

            let str = tmpl
                .render(context! {name,personal_details,home,about,content,contact})
                .unwrap();
            Ok(HttpResponse::Ok().body(str))
        }
    }
}

//用于分类和内容显示
#[get("/{name}/category/{category}")]
pub async fn category_info_page(
    data: web::Data<AppState>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    /*文件路径先设置在当前目录public下*/
    let db = &data.conn;

    let (name, category) = path.into_inner();

    let articles = article::Entity::find()
        .select_only()
        .columns([article::Column::Subtitle, article::Column::Title])
        .column(article::Column::HtmlContent)
        .column_as(article::Column::ImgUrl, "bg_img")
        .column(article::Column::Url)
        .join(JoinType::InnerJoin, article::Relation::Users.def())
        .filter(users::Column::UserName.contains(&name))
        .filter(article::Column::Tag.contains(&category))
        .into_model::<ArticlesInfo>()
        .all(db)
        .await
        .unwrap();

    if articles.is_empty() {
        let category = Interface::redirect_user_category(&name);
        Ok(HttpResponse::Found()
            .append_header((header::LOCATION, category))
            .finish())
    } else {
        //作为渲染的地方
        let mut path_buf = PathBuf::new();
        path_buf.push("./page");
        path_buf.push(&name.to_string());
        path_buf.push("content.html");

        let string = match fs::read_to_string(path_buf) {
            Ok(t) => t,
            Err(_) => {
                let error = get_blog_error(&name);
                return Ok(HttpResponse::Found()
                    .append_header((header::LOCATION, error))
                    .finish());
            }
        };

        let mut env = Environment::new();
        env.add_template("category/tag", &string).unwrap();
        let tmpl = env.get_template("category/tag").unwrap();

        let name = &name.to_string();
        let personal_details = &name.to_string();
        let home = get_blog_home(&name);
        let about = get_blog_about(&name);
        let content = get_blog_content(&name);
        let contact = get_blog_contact(&name);

        let str = tmpl
            .render(context! {name,personal_details,home,about,content,contact,articles})
            .unwrap();
        Ok(HttpResponse::Ok().body(str))
    }
}
