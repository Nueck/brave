use actix_web::error::ErrorUnauthorized;
use actix_web::{post, web, HttpResponse, Responder};
use brave_config::app::AppState;
use brave_config::GLOBAL_CONFIG;
use brave_db::entity::prelude::Users;
use brave_db::entity::users;
use brave_utils::jwt::jwt::TokenData;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user_info)
        .service(get_user_article_info);
}

/*全表查询*/
#[post("/getUsers")]
async fn get_users(data: web::Data<AppState>, token: web::ReqData<TokenData>) -> impl Responder {
    let auth = token.auth.clone();

    //只有是超级管理员才能访问
    if auth
        == GLOBAL_CONFIG
            .authority
            .get_authority_config()
            .super_admin
            .unwrap()
    {
        let db = &data.conn;
        let data = Users::find()
            .into_json()
            .all(db)
            .await
            .expect("Could not find Users");
        HttpResponse::Ok().json(serde_json::json!({"state": "success","data":data}))
    } else {
        ErrorUnauthorized("Lack of authority").into()
    }
}

/*获取用户的文章总信息*/
#[post("/getUserAllArticleInfo")]
async fn get_user_article_info(
    data: web::Data<AppState>,
    token: web::ReqData<TokenData>,
) -> impl Responder {
    let db = &data.conn;
    match Users::find()
        .filter(users::Column::UserName.contains(&token.aud))
        .one(db)
        .await
        .expect("Could not find Users --- getUserAllArticleInfo")
    {
        None => {
            const MSG: &str = "User does not exist";
            let json = serde_json::json!({"state": "error",  "message":MSG });

            HttpResponse::Ok().json(json)
        }
        Some(user) => {
            let article = user.article_num;
            let album = user.album_num;
            let read = user.read_count;
            let visit = user.visit_count;

            let json = serde_json::json!({
                "state": "success",
                "data":{
                    "articleCount":article,
                    "albumNUm":album,
                    "readCount":read,
                    "visitCount":visit
                }
            });

            HttpResponse::Ok().json(json)
        }
    }
}

/*获取用户信息*/
#[post("/getUserInfo")]
async fn get_user_info(
    data: web::Data<AppState>,
    token: web::ReqData<TokenData>,
) -> impl Responder {
    let db = &data.conn;

    match Users::find()
        .filter(users::Column::UserName.contains(&token.aud))
        .one(db)
        .await
        .expect("Could not find Users --- getUserInfo")
    {
        None => {
            const MSG: &str = "User does not exist";
            let json = serde_json::json!({"state": "error",  "message":MSG });

            HttpResponse::Ok().json(json)
        }
        Some(user) => {
            let id = user.user_id;
            let username = user.user_name;
            let user_role = user.authority;

            let json = serde_json::json!({
                "state": "success",
                "data":{
                    "userId":id ,
                    "userName":username,
                    "userRole":user_role
                }
            });

            HttpResponse::Ok().json(json)
        }
    }
}
