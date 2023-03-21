use crate::handler::init::init_config;
use crate::handler::login::login_config;
use crate::handler::token::token_config;
use crate::handler::user::user_config;
use actix_web::web;

mod init;
mod login;
mod token;
mod user;

pub fn config_init(cfg: &mut web::ServiceConfig) {
    cfg.configure(init_config)
        .configure(token_config)
        .configure(login_config)
        .configure(user_config);
}
