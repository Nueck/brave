use actix_files as fs;
use actix_files::{Files, NamedFile};
use actix_web::error::ErrorNotFound;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use brave_utils::common::is_html_path;
use std::path::PathBuf;

#[get("/{name}/{filename:.*}")]
pub async fn page_handler(path: web::Path<(String, String)>, req: HttpRequest) -> impl Responder {
    let (name, filename) = path.into_inner();
    let mut path_buf = PathBuf::new();
    path_buf.push("./public");
    path_buf.push(name.to_string());
    path_buf.push(filename.to_string());

    print!("{:?}", &path_buf);

    if let Some(ext) = path_buf.extension() {
        if ext == "html" {
            let content = std::fs::read_to_string(&path_buf).unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(content)
        } else {
            NamedFile::open(&path_buf).unwrap().into_response(&req)
        }
    } else {
        HttpResponse::NotFound().body("404 Not Found")
    }
}

/*用于页面的加载*/
#[get("/{name}")]
pub async fn index_page(path: web::Path<String>, req: HttpRequest) -> NamedFile {
    /*文件路径先设置在当前目录public下*/
    let mut path_buf = PathBuf::new();
    path_buf.push("./public");
    path_buf.push(path.to_string());
    path_buf.push("index.html");

    NamedFile::open(path_buf).unwrap()
}
