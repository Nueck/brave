use actix_files::NamedFile;
use actix_web::web::Path;
use actix_web::{get, HttpRequest, HttpResponse, Responder, Result};
use brave_config::GLOBAL_DATA;
use std::path::PathBuf;

#[get("/{filename:.*}")]
pub async fn files(filename: Path<String>, req: HttpRequest) -> Result<impl Responder> {
    //获取存储数据的位置
    let data_location = &GLOBAL_DATA.get_data_config().data_location.unwrap();

    let mut path_buf = PathBuf::new();
    path_buf.push(data_location);
    path_buf.push(filename.to_string());

    /*判断是否存在文件*/
    match NamedFile::open_async(&path_buf).await {
        Ok(content) => Ok(content.into_response(&req)),
        Err(_) => Ok(HttpResponse::NotFound().body("Resource does not exist")),
    }
}
