use crate::entity::ArchiveData;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use brave_config::app::AppState;
use brave_config::utils::jwt::UserDataInfo;
use brave_db::entity::article_archive;
use sea_orm::prelude::Json;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait, JsonValue, QuerySelect, Set};
use serde_json::{json, Value};

pub fn archive_config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_archive)
        .service(delete_archive)
        .service(update_archive)
        .service(get_archives);
}

//添加归档
#[post("/archive")]
async fn add_archive(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    json: web::Json<ArchiveData>,
) -> impl Responder {
    let id = &token.id;
    let db = &data.conn;
    let name = &json.name;
    let icon = &json.icon;

    let table = article_archive::Entity::find()
        .filter(article_archive::Column::UserId.eq(id.to_owned()))
        .one(db)
        .await
        .unwrap();

    let mut model: article_archive::ActiveModel = table.unwrap().into();
    let content = model.content.clone().unwrap();

    if content.is_null() {
        //添加空的
        let archives: Vec<JsonValue> = Vec::new();
        model.content = Set(JsonValue::Array(archives));
    } else {
        if content.is_array() {
            let mut archives: Vec<JsonValue> = content.as_array().unwrap().to_vec();
            let archives_id = archives.len();

            let data = ArchiveData {
                id: archives_id,
                name: name.to_string(),
                icon: icon.to_string(),
            };

            let v = json!(data);
            if !archives.contains(&v) {
                archives.push(v);
                model.content = Set(JsonValue::Array(archives));
            };
        }
    };

    match model.update(db).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"state": "success"})),
        Err(_) => HttpResponse::Ok().json(serde_json::json!({"state": "error"})),
    }
}

//获取归档
#[get("/archives")]
async fn get_archives(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
) -> impl Responder {
    let id = &token.id;
    let db = &data.conn;
    let tags = article_archive::Entity::find()
        .select_only()
        .column(article_archive::Column::Content)
        .filter(article_archive::Column::UserId.eq(id.to_owned()))
        .into_tuple::<Json>()
        .one(db)
        .await
        .unwrap();

    if tags.is_none() {
        HttpResponse::Ok().json(serde_json::json!({"state": "success", "data": [] }))
    } else {
        HttpResponse::Ok().json(serde_json::json!({"state": "success", "data": tags }))
    }
}

//更新指定归档
#[put("/archive")]
async fn update_archive(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    json: web::Json<ArchiveData>,
) -> impl Responder {
    let id = &token.id;
    let db = &data.conn;

    let table = article_archive::Entity::find()
        .filter(article_archive::Column::UserId.eq(id.to_owned()))
        .one(db)
        .await
        .unwrap();

    let mut model: article_archive::ActiveModel = table.unwrap().into();
    let content = model.content.clone().unwrap();

    let mut state = false;

    if content.is_array() {
        let mut archives: Vec<Value> = content.as_array().unwrap().to_vec();

        let archives_id = json.clone().id;
        let name = json.clone().name;
        let icon = json.clone().icon;

        for archive in &mut archives {
            if archive.is_object() {
                if archive["id"] == archives_id {
                    *archive.get_mut("name").unwrap() = json!(name);
                    *archive.get_mut("icon").unwrap() = json!(icon);
                    state = true;
                }
            }
        }
        model.content = Set(JsonValue::Array(archives));
    };

    if state {
        match model.update(db).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({"state": "success"})),
            Err(_) => HttpResponse::Ok().json(serde_json::json!({"state": "error"})),
        }
    } else {
        HttpResponse::Ok().json(serde_json::json!({"state": "error"}))
    }
}

//删除指定归档
#[delete("/archive/{id}")]
async fn delete_archive(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = &token.id;
    let db = &data.conn;
    let archives_id = path.into_inner();
    let table = article_archive::Entity::find()
        .filter(article_archive::Column::UserId.eq(id.to_owned()))
        .one(db)
        .await
        .unwrap();

    let mut model: article_archive::ActiveModel = table.unwrap().into();
    let content = model.content.clone().unwrap();

    let mut state = false;

    if content.is_array() {
        let mut archives: Vec<JsonValue> = content.as_array().unwrap().to_vec();
        archives.remove(archives_id as usize);
        model.content = Set(JsonValue::Array(archives));
        state = true;
    };
    if state {
        match model.update(db).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({"state": "success"})),
            Err(_) => HttpResponse::Ok().json(serde_json::json!({"state": "error"})),
        }
    } else {
        HttpResponse::Ok().json(serde_json::json!({"state": "error"}))
    }
}
