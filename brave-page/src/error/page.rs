use actix_web::http::header;
use actix_web::{HttpResponse, ResponseError};
use brave_config::interface::Interface;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageError {
    pub user_name: String,
}

impl fmt::Display for PageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.user_name)
    }
}

impl ResponseError for PageError {
    fn error_response(&self) -> HttpResponse {
        let blog = Interface::redirect_user_blog_home(&self.user_name);
        HttpResponse::BadRequest()
            .append_header((header::LOCATION, blog))
            .finish()
    }
}
