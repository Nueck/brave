use actix_web::http::header;
use actix_web::HttpResponse;
use brave_config::init::InitStatus;
use brave_config::interface::Interface;

//判断初始化状态
pub fn init_status_jump() -> Result<HttpResponse, ()> {
    if InitStatus::global().is_init {
        Err(())
    } else {
        let init_add = Interface::redirect_init_address();
        Ok(HttpResponse::Found()
            .append_header((header::LOCATION, init_add))
            .finish())
    }
}
