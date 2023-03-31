use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use brave_config::GLOBAL_DATA;
use futures::TryStreamExt;
use std::io::Write;
use uuid::Uuid;

pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let filepath = format!(
            "{}/{filename}",
            &GLOBAL_DATA.get_data_config().data_location.unwrap()
        );

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}

/*用于上传图片文件然后返回信息*/
pub async fn upload_img(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

        let filepath = format!(
            "{}/{filename}",
            &GLOBAL_DATA.get_data_config().data_location.unwrap()
        );

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}
