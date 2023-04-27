use crate::template::MiniJinjaRenderer;
use crate::utils::common::init_status_jump;
use actix_web::{get, web, HttpRequest, Responder};
use brave_config::interface::Interface;
use brave_config::utils::common::get_system_uptime;

pub fn index_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index(tmpl_env: MiniJinjaRenderer, req: HttpRequest) -> impl Responder {
    match init_status_jump() {
        Ok(http) => http,
        Err(_) => {
            let add = Interface::redirect_login_address();
            let uptime = get_system_uptime();
            tmpl_env
                .render(
                    "home_index.html",
                    minijinja::context! {
                        admin_login => add,
                        uptime
                    },
                )
                .unwrap()
                .respond_to(&req)
                .map_into_boxed_body()
        }
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
