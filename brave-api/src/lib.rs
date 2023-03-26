use crate::init::init_config;
use crate::login::login_config;
use crate::token::token_config;
use crate::user::user_config;
use actix_web::web;

mod init;
mod login;
mod token;
mod user;
mod utils;

pub fn api_post_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(init_config)
        .configure(token_config)
        .configure(login_config)
        .configure(user_config);
}

pub fn api_get_config(_cfg: &mut web::ServiceConfig) {}
