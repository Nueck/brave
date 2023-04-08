use crate::api::article::article_config;
use crate::api::init::init_config;
use crate::api::login::login_config;
use crate::api::skin::skin_config;
use crate::api::tag::tag_config;
use crate::api::token::token_config;
use crate::api::upload::data_config;
use crate::api::user::user_config;
use actix_web::web;

mod article;
mod init;
mod login;
mod router;
mod skin;
mod tag;
mod token;
mod upload;
mod user;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(init_config)
        .configure(token_config)
        .configure(login_config)
        .configure(user_config)
        .configure(article_config)
        .configure(data_config)
        .configure(tag_config)
        .configure(skin_config);
}

// pub fn api_get_config(_cfg: &mut web::ServiceConfig) {}
