use crate::admin::admin_config;
use crate::config::app::AppState;
use crate::config::env::EnvConfig;
use crate::config::init::InitStatus;
use crate::config::{GLOBAL_ENV_CONFIG, GLOBAL_YAML_CONFIG};
use crate::middleware::auth_middleware::JWTAuth;
use crate::middleware::init_middleware::InitAuth;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use brave_utils::jwt::config::JWTConfig;

#[actix_rt::main]
pub async fn web_start() -> std::io::Result<()> {
    //初始化日志
    super::log::init_log();

    //初始化配置文件
    InitStatus::new(None);

    //初始化jwt配置
    JWTConfig::new(GLOBAL_YAML_CONFIG.jwt.clone());

    //home
    println!(
        "Home service start: http://{}/",
        EnvConfig::get_api_string(),
    );

    //admin
    println!(
        "Admin service start: http://{}/{}/",
        EnvConfig::get_api_string(),
        &GLOBAL_ENV_CONFIG.admin_scope
    );

    //blog
    println!(
        "Blog service start: http://{}/{}/",
        EnvConfig::get_api_string(),
        &GLOBAL_ENV_CONFIG.blog_scope
    );
    //api
    println!(
        "API service start: http://{}/{}/",
        EnvConfig::get_api_string(),
        &GLOBAL_ENV_CONFIG.api_scope
    );

    //数据库连接的一些
    let states = AppState::new().await;

    //开启web服务
    HttpServer::new(move || {
        //api的跨域问题
        /*TODO:暂时所有源都可以通过后期更改*/
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .service(
                web::scope("/") //首页加载
                    .wrap(
                        Cors::default()
                            .allow_any_header()
                            .allowed_methods(vec!["GET"]) //只允许GET
                            .allow_any_origin() //允许任何来源
                            .max_age(3600),
                    )
                    .configure(super::admin::admin_config),
            )
            .service(
                //主要api的service
                web::scope(&*GLOBAL_ENV_CONFIG.api_scope)
                    .app_data(web::Data::new(states.clone()))
                    .wrap(JWTAuth) //身份验证
                    .wrap(InitAuth) //初始化判断
                    .wrap(cors)
                    .wrap(Logger::default()) //api的日志
                    .configure(super::handler::config_init), //服务配置
            )
            .service(
                web::scope(&GLOBAL_ENV_CONFIG.blog_scope) //博客方面的加载
                    .wrap(
                        Cors::default()
                            .allow_any_header()
                            .allowed_methods(vec!["GET"]) //只允许GET
                            .allow_any_origin() //允许任何来源
                            .max_age(3600),
                    )
                    .configure(super::blog::blog_config),
            )
            .configure(admin_config)
    })
    .bind(EnvConfig::get_api_string())?
    .run()
    .await
}
