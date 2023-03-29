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
    cfg.service(get_articles_info);
}

//获取文章信息
#[post("/getArticlesInfo")]
async fn get_articles_info(
    data: web::Data<AppState>,
    token: web::ReqData<UserDataInfo>,
) -> impl Responder {
    let db = &data.conn;
    let id = &token.id;

    #[derive(Clone, Deserialize, Serialize)]
    struct ArticleData {
        title: String,
        img_url: String,
    }

    //获取数据库中文章信息
    let data = Article::find()
        .filter(article::Column::UserId.eq(id.clone().to_owned()))
        .all(db)
        .await
        .into_iter()
        .flat_map(|models: Vec<Model>| {
            models
                .into_iter()
                .map(|model: Model| ArticleData {
                    title: model.title.unwrap(),
                    img_url: model.img_url.unwrap(),
                })
                .collect::<Vec<ArticleData>>()
                .to_vec()
        })
        .collect::<Vec<ArticleData>>();

    HttpResponse::Ok().json(serde_json::json!({"state": "success", "data": data }))
}
