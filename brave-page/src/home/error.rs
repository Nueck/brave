use crate::template::MiniJinjaRenderer;
use actix_web::dev::ServiceResponse;
use actix_web::http::header::ContentType;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{web, FromRequest, HttpResponse, Responder, Result};
use brave_config::interface::Interface;
use minijinja_autoreload::AutoReloader;

pub(crate) fn not_found<B>(svc_res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let res = get_error_response(&svc_res, "Page not found");

    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        svc_res.into_parts().0,
        res.map_into_right_body(),
    )))
}

/// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let req = res.request();

    let tmpl_env = <web::Data<AutoReloader>>::extract(req)
        .into_inner()
        .unwrap();

    let tmpl = MiniJinjaRenderer { tmpl_env };

    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let ctx = minijinja::context! {
        error => error,
        status_code => res.status().as_str(),
        home =>Interface::redirect_home(),
    };

    match tmpl.render("home_error.html", ctx) {
        Ok(body) => body
            .customize()
            .with_status(res.status())
            .respond_to(req)
            .map_into_boxed_body(),

        Err(_) => fallback(error),
    }
}
