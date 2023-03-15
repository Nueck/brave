use actix_web::web;

mod login;
mod token;
mod user;

pub fn config_init(cfg: &mut web::ServiceConfig) {
    cfg.service(token::token_checker_handler) //用于身份验证的
        .service(login::login)
        .service(login::register)
        .service(login::sendmail)
        .service(user::get_users);
}
