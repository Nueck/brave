use crate::config::{AppState, EnvConfig, GLOBAL_ENV_CONFIG, GLOBAL_YAML_CONFIG};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use cako_middleware::auth_middleware::JWTAuth;
use cako_utils::jwt::config::JWTConfig;

#[actix_rt::main]
pub async fn web_start() -> std::io::Result<()> {
    //初始化日志
    super::log::init_log();

    //初始化jwt配置
    JWTConfig::new(GLOBAL_YAML_CONFIG.jwt.clone());

    //blog
    println!(
        "Blog service start: http://{}/{}",
        EnvConfig::get_api_string(),
        GLOBAL_ENV_CONFIG.web_scope
    );
    //api
    println!(
        "API service start: http://{}/{}/",
        EnvConfig::get_api_string(),
        GLOBAL_ENV_CONFIG.api_scope
    );

    //数据库连接的一些
    let states = AppState::new().await;

    //开启web服务
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope(&GLOBAL_ENV_CONFIG.web_scope) //博客方面的加载
                    // .wrap_fn(|sreq, srv| { //用于跳转https
                    //     let host = sreq.connection_info().host().to_owned();
                    //     let uri = sreq.uri().to_owned();
                    //     let url = format!("https://{host}{uri}");
                    //
                    //     if sreq.connection_info().scheme() == "https" {
                    //         Either::Left(srv.call(sreq).map(|res| res))
                    //     } else {
                    //         println!(
                    //             "An http request has arrived here, i will redirect it to use https"
                    //         );
                    //         return Either::Right(future::ready(Ok(sreq.into_response(
                    //             HttpResponse::MovedPermanently()
                    //                 .append_header((http::header::LOCATION, url))
                    //                 .finish(),
                    //         ))));
                    //     }
                    // })
                    .wrap(
                        Cors::default()
                            .allow_any_header()
                            .allowed_methods(vec!["GET"]) //只允许GET
                            .allow_any_origin() //允许任何来源
                            .max_age(3600),
                    )
                    .configure(super::blog::web_config_init),
            )
            .service(
                //主要api的service
                web::scope(&*GLOBAL_ENV_CONFIG.api_scope.clone())
                    .app_data(web::Data::new(states.clone()))
                    .wrap(JWTAuth) //身份验证
                    .wrap(
                        //跨域支持
                        Cors::default()
                            .allowed_origin(&*EnvConfig::get_api_string())
                            .allowed_methods(vec!["GET", "POST"])
                            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                            .allowed_header(header::CONTENT_TYPE)
                            .supports_credentials()
                            .max_age(3600),
                    )
                    .wrap(Logger::default()) //api的日志
                    .configure(super::handler::config_init), //服务配置
            )
    })
    .bind(EnvConfig::get_api_string())?
    .run()
    .await
}
