use actix_files::NamedFile;
use actix_web::{get, put, web, HttpRequest, HttpResponse, Responder, Result};
use brave_config::utils::jwt::UserDataInfo;
use brave_config::GLOBAL_CONFIG;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub(crate) fn fs_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("fs").service(get_files).service(update_files));
}

#[get("/{filename:.*}")]
pub async fn get_files(
    path: web::Path<String>,
    token: web::ReqData<UserDataInfo>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let filename = path.into_inner();
    let aud = &token.aud;

    let mut path_buf = PathBuf::new();
    path_buf.push(GLOBAL_CONFIG.get_page());
    path_buf.push(aud.to_owned());
    path_buf.push(filename.to_string());

    //判断是否存在文件
    match NamedFile::open_async(&path_buf).await {
        Ok(content) => Ok(content.into_response(&req)),
        Err(_) => {
            Ok(HttpResponse::NotFound()
                .json(json!({"state": "error", "message": "file not found"})))
        }
    }
}

#[put("/{filename:.*}")]
pub async fn update_files(
    path: web::Path<String>,
    token: web::ReqData<UserDataInfo>,
    text: String,
) -> impl Responder {
    let filename = path.into_inner();
    let aud = &token.aud;

    let mut path_buf = PathBuf::new();
    path_buf.push(GLOBAL_CONFIG.get_page());
    path_buf.push(aud.to_owned());
    path_buf.push(filename.to_string());

    let mut file = match OpenOptions::new()
        .read(false)
        .write(true)
        .append(false)
        .open(path_buf.as_path())
    {
        Ok(f) => f,
        Err(e) => {
            log::error!("{}", e);
            return HttpResponse::Ok().json(json!({"state": "error", "message": "Not found"}));
        }
    };

    match file.write_all(text.as_bytes()) {
        Ok(_) => HttpResponse::Ok().json(json!({"state": "success"})),
        Err(e) => {
            log::error!("{}", e);
            return HttpResponse::Ok()
                .json(json!({"state": "error" ,"message": "can't write file"}));
        }
    }
}
