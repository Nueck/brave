use crate::utils::common::get_page_location;
use actix_files::NamedFile;
use actix_web::http::header;
use actix_web::web::Path;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use brave_config::blog::get_blog_error;
use brave_config::interface::Interface;

//用于blog的页面加载
pub(crate) fn file_load_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/{filename:.*}", web::get().to(file_load));
}

pub(crate) async fn file_load(
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let (name, filename) = path.into_inner();

    //过滤html文件
    if filename.ends_with(".html") {
        let error = get_blog_error(name.as_str());
        return Ok(HttpResponse::Found()
            .append_header((header::LOCATION, error))
            .finish());
    }

    let mut path_buf = get_page_location(name.as_str());
    path_buf.push(filename.to_string());

    match NamedFile::open_async(&path_buf).await {
        Ok(content) => Ok(content.use_last_modified(true).into_response(&req)),
        Err(_) => {
            let user_blog_home = Interface::redirect_user_blog_home(&name);
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, user_blog_home))
                .finish())
        }
    }
}

//单页面的加载
pub(crate) async fn single_index_load(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let name = path.into_inner();
    let mut page = get_page_location(name.as_str());
    page.push("index.html");

    match NamedFile::open_async(&page).await {
        Ok(content) => Ok(content.use_last_modified(true).into_response(&req)),
        Err(_) => {
            let home = Interface::redirect_user_blog(name.as_str());
            return Ok(HttpResponse::Found()
                .append_header((header::LOCATION, home))
                .finish());
        }
    }
}

#[get("/{filename:.*}")]
pub(crate) async fn single_files_load(
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let (name, filename) = path.into_inner();

    //过滤html文件
    if filename.ends_with(".html") {
        let home = Interface::redirect_user_blog(name.as_str());
        return Ok(HttpResponse::Found()
            .append_header((header::LOCATION, home))
            .finish());
    }

    let mut path_buf = get_page_location(name.as_str());
    path_buf.push(filename.to_string());

    match NamedFile::open_async(&path_buf).await {
        Ok(content) => Ok(content.use_last_modified(true).into_response(&req)),
        Err(_) => {
            let mut page = get_page_location(name.as_str());
            page.push("index.html");

            match NamedFile::open_async(&page).await {
                Ok(content) => Ok(content.use_last_modified(true).into_response(&req)),
                Err(_) => {
                    let home = Interface::redirect_user_blog(name.as_str());
                    return Ok(HttpResponse::Found()
                        .append_header((header::LOCATION, home))
                        .finish());
                }
            }
        }
    }
}
