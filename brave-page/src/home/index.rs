use actix_web::http::header;
use actix_web::{get, web, HttpResponse, Responder, Result};
use askama::DynTemplate;
use askama::Template;
use brave_config::init::InitStatus;
use brave_config::interface::Interface;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    admin_login: &'a str,
}

pub fn index_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    if InitStatus::global().is_init {
        let add = Interface::redirect_login_address();
        let html = IndexTemplate { admin_login: &add }.dyn_render().unwrap();
        Ok(HttpResponse::Ok().body(html))
    } else {
        let init_add = Interface::redirect_init_address();
        Ok(HttpResponse::Found()
            .append_header((header::LOCATION, init_add))
            .finish())
    }
}

// //增加地址的趣味性 (图标问题)
// #[get("/{name}")]
// pub async fn home_index(
//     path: web::Path<String>,
//     data: web::Data<AppState>,
//     req: HttpRequest,
// ) -> Result<impl Responder> {
//     if InitStatus::global().is_init {
//         let db = &data.conn;
//         let name = path.into_inner();
//
//         if name == "favicon.ico".to_owned() {
//             return Ok(NamedFile::open_async("./templates/favicon.ico")
//                 .await?
//                 .into_response(&req));
//         }
//
//         match Users::find()
//             .filter(users::Column::UserName.contains(&name))
//             .one(db)
//             .await
//             .expect("Could not find Users -- file load blog")
//         {
//             None => {
//                 log::error!("{}", &name);
//                 Ok(HttpResponse::Found()
//                     .append_header((header::LOCATION, "/"))
//                     .finish())
//             }
//             Some(_) => {
//                 let name_blog = Interface::redirect_user_blog_home(&name);
//                 Ok(HttpResponse::Found()
//                     .append_header((header::LOCATION, name_blog))
//                     .finish())
//             }
//         }
//     } else {
//         /*重定向到初始化*/
//         let init_add = Interface::redirect_init_address();
//         Ok(HttpResponse::Found()
//             .append_header((header::LOCATION, init_add))
//             .finish())
//     }
// }
