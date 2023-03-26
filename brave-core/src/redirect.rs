use actix_web::http::header;
use actix_web::{HttpResponse, Responder, Result};
use brave_config::init::InitStatus;
use brave_config::interface::Interface;

// pub fn redirect_index() -> HttpResponse {
//     /*根据初始化状态*/
//     if InitStatus::global().is_init {
//         let home = Interface::redirect_home();
//         HttpResponse::Found()
//             .append_header((header::LOCATION, home))
//             .body("")
//     } else {
//         /*重定向到初始化*/
//         let init_add = Interface::redirect_init_address();
//         HttpResponse::Found()
//             .append_header((header::LOCATION, init_add))
//             .body("")
//     }
// }
