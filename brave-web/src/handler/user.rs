
use actix_web::{ HttpResponse, Responder, web, Error, post};
use serde::Serialize;
use crate::config::AppState;
use crate::entity::users::{Entity as Users, Model};
use crate::entity::users;
use sea_orm::{Database, DatabaseConnection, EntityTrait};


#[derive(Serialize)]
struct UserObj{
    user_name:String
}


#[post("/users")]
pub async fn get_users(data:web::Data<AppState>)->Result<impl Responder,Error>{
    let db= &data.conn;
    let data :Vec<users::Model> =Users::find().all(db)
        .await
        .expect("could not find Users");


    Ok(HttpResponse::Ok().json(serde_json::json!({"status": "success"})))
}
