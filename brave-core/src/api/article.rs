use actix_web::web::Json;
use actix_web::{post, web, HttpResponse, Responder};
use brave_config::app::AppState;
use brave_db::entity::article;
use brave_db::entity::article::Model;
use brave_db::entity::prelude::Article;
use brave_utils::jwt::jwt::UserDataInfo;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

//文章
pub fn article_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_articles_info).service(get_article_data);
}

//获取文章信息
#[post("/getArticlesInfo")]
async fn get_articles_info(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
) -> impl Responder {
    let db = &data.conn;
    let id = &token.id;

    //获取数据库中文章信息
    match Article::find_by_id(id.clone().to_owned()).all(db).await {
        Ok(table) => {
            #[derive(Clone, Deserialize, Serialize)]
            struct ArticleData {
                table_id: i32,
                title: String,
                img_url: String,
            }
            let data = table
                .into_iter()
                // .flat_map(|models: Vec<Model>| {
                //     models
                //         .into_iter()
                .map(|model: Model| ArticleData {
                    table_id: model.article_id,
                    title: model.title.unwrap(),
                    img_url: model.img_url.unwrap(),
                })
                .collect::<Vec<ArticleData>>()
                .to_vec();
            // })
            // .collect::<Vec<ArticleData>>();
            HttpResponse::Ok().json(serde_json::json!({"state": "success", "data": data }))
        }
        Err(_) => {
            const MSG: &str = "Unable to find the data";
            HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
struct EditData {
    table_id: i32,
}

//获取文章编辑信息
#[post("/getArticleEditData")]
async fn get_article_data(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
    json: Json<EditData>,
) -> impl Responder {
    let db = &data.conn;
    let id = &token.id;

    //获取数据库中文章信息
    match Article::find_by_id(json.table_id.clone().to_owned())
        .filter(article::Column::UserId.eq(id.clone().to_owned()))
        .one(db)
        .await
        .expect("Unable to find the article table")
    {
        None => {
            const MSG: &str = "Unable to find the data";
            HttpResponse::Ok().json(serde_json::json!({"state": "error", "message": MSG }))
        }
        Some(model) => {
            #[derive(Clone, Deserialize, Serialize)]
            struct ArticleEditData {
                title: String,
                subtitle: String,
                img_url: String,
                content: String,
            }

            let data = ArticleEditData {
                title: model.title.unwrap(),
                subtitle: model.subtitle.unwrap(),
                img_url: model.img_url.unwrap(),
                content: model.content.unwrap(),
            };

            HttpResponse::Ok().json(serde_json::json!({"state": "success", "data": data }))
        }
    }
}
