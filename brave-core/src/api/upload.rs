use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpResponse};
use brave_config::GLOBAL_DATA;
use futures_util::TryStreamExt as _;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

pub fn data_config(cfg: &mut web::ServiceConfig) {
    cfg.service(upload_img).service(upload);
}

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let file_extension = Path::new(filename.as_str())
            .extension()
            .and_then(|os_str| os_str.to_str())
            .unwrap();

        let rand_filename = sanitize_filename::sanitize(format!(
            "{}.{}",
            Uuid::new_v4().to_string(),
            file_extension
        ));

        let filepath = format!(
            "{}/files/{rand_filename}",
            &GLOBAL_DATA.get_data_config().data_location.unwrap()
        );

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}

#[post("/upload/img")]
async fn upload_img(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let file_extension = Path::new(filename.as_str())
            .extension()
            .and_then(|os_str| os_str.to_str())
            .unwrap();

        let rand_filename = sanitize_filename::sanitize(format!(
            "{}.{}",
            Uuid::new_v4().to_string(),
            file_extension
        ));

        let filepath = format!(
            "{}/img/{rand_filename}",
            &GLOBAL_DATA.get_data_config().data_location.unwrap()
        );

        let uri = format!("{}/img/{rand_filename}", &GLOBAL_DATA.get_data_url());

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }

        return Ok(HttpResponse::Ok().json(serde_json::json!({ "url": uri })));
    }
    Ok(HttpResponse::Ok().into())
}
