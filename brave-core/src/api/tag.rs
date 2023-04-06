use crate::entity::TagData;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use brave_config::app::AppState;
use brave_config::utils::jwt::UserDataInfo;
use brave_db::entity::article_tag;
use sea_orm::prelude::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, JsonValue, QueryFilter, QuerySelect};

//标签(使用数组存放)
pub(crate) fn tag_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tag).service(add_tag).service(delete_tag);
}

//添加标签
#[post("/tags")]
async fn add_tag(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    json: web::Json<TagData>,
) -> impl Responder {
    let id = &token.id;
    let db = &data.conn;
    let tag = &json.tag;
    let table = article_tag::Entity::find()
        .filter(article_tag::Column::UserId.eq(id.to_owned()))
        .one(db)
        .await
        .unwrap();

    let mut model: article_tag::ActiveModel = table.unwrap().into();
    let content = model.content.clone().unwrap();

    if content.is_null() {
        let mut tags = Vec::new();
        tags.push(JsonValue::String(tag.to_string()));
        model.content = Set(JsonValue::Array(tags));
    } else {
        if content.is_array() {
            let mut tags = content.as_array().unwrap().to_vec();
            let my_tag = JsonValue::String(tag.to_string());
            if !tags.contains(&my_tag) {
                tags.push(my_tag);
                model.content = Set(JsonValue::Array(tags));
            };
        }
    };

    match model.update(db).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"state": "success"})),
        Err(_) => HttpResponse::Ok().json(serde_json::json!({"state": "error"})),
    }
}

//获取标签
#[get("/tags")]
async fn get_tag(data: web::Data<AppState>, token: web::ReqData<UserDataInfo>) -> impl Responder {
    let id = &token.id;
    let db = &data.conn;
    let tags = article_tag::Entity::find()
        .select_only()
        .column(article_tag::Column::Content)
        .filter(article_tag::Column::UserId.eq(id.to_owned()))
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

//删除指定标签
#[delete("/tags")]
async fn delete_tag(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    json: web::Json<TagData>,
) -> impl Responder {
    let id = &token.id;
    let db = &data.conn;
    let tag = &json.tag;
    let table = article_tag::Entity::find()
        .filter(article_tag::Column::UserId.eq(id.to_owned()))
        .one(db)
        .await
        .unwrap();

    let mut model: article_tag::ActiveModel = table.unwrap().into();
    let content = model.content.clone().unwrap();

    if content.is_null() {
        return HttpResponse::Ok().json(serde_json::json!({"state": "success"}));
    } else {
        if content.is_array() {
            let mut tags = content.as_array().unwrap().to_vec();
            tags.retain(|x| x != &JsonValue::String(tag.to_string()));
            model.content = Set(JsonValue::Array(tags));
        }
    };

    match model.update(db).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"state": "success"})),
        Err(_) => HttpResponse::Ok().json(serde_json::json!({"state": "error"})),
    }
}
