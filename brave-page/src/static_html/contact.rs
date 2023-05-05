use crate::utils::common::get_page_location;
use actix_web::http::header;
use actix_web::web::{self, Path};
use actix_web::{get, HttpRequest, HttpResponse, Responder, Result};
use brave_config::app::AppState;
use brave_config::blog::{
    get_blog_about, get_blog_contact, get_blog_content, get_blog_error, get_blog_home,
};
use brave_config::interface::Interface;
use brave_db::entity::prelude::Users;
use brave_db::entity::users;
use minijinja::{context, Environment};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::fs;

#[get("/contact")]
pub async fn contact_page(
    data: web::Data<AppState>,
    name: Path<String>,
    _req: HttpRequest,
) -> Result<impl Responder> {
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
            let mut path_buf = get_page_location(user.user_name.as_str());
            path_buf.push("contact.html");

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
            env.add_template("contact", &string).unwrap();
            let tmpl = env.get_template("contact").unwrap();

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
