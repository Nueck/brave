use crate::middleware::auth_middleware::JWTAuth;
use crate::middleware::init_middleware::InitAuth;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use brave_config::app::AppState;
use brave_config::init::InitStatus;
use brave_config::interface::Interface;
use brave_config::GLOBAL_CONFIG;
use brave_utils::jwt::config::JWTConfig;

#[actix_rt::main]
pub async fn web_start() -> std::io::Result<()> {
    //初始化日志
    super::log::init_log();

    //数据库连接的
    let states = AppState::new().await;

    //初始化配置文件
    InitStatus::new(None);

    //初始化jwt配置
    JWTConfig::new(GLOBAL_CONFIG.jwt.clone());

    //home
    println!(
        "Home service start: http://{}/",
        Interface::get_api_string()
    );

    //admin
    println!(
        "Admin service start: http://{}/{}/",
        Interface::get_api_string(),
        &GLOBAL_CONFIG.interface.admin_scope
    );

    //blog
    println!(
        "Blog service start: http://{}/{}/",
        Interface::get_api_string(),
        &GLOBAL_CONFIG.interface.blog_scope
    );
    //api
    println!(
        "API service start: http://{}/{}/",
        Interface::get_api_string(),
        &GLOBAL_CONFIG.interface.api_scope
    );

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
                web::scope(&GLOBAL_CONFIG.interface.api_scope)
                    .app_data(web::Data::new(states.clone()))
                    .wrap(JWTAuth) //身份验证
                    .wrap(InitAuth) //初始化判断
                    .wrap(cors)
                    .wrap(Logger::default()) //api的日志
                    .configure(brave_api::api_config), //api的日志
            ) //api配置
            .configure(brave_admin::admin_config) //后台管理
            .configure(brave_blog::blog_config) //博客显示
            .configure(brave_home::home_config) //首页显示
    })
    .bind(Interface::get_api_string())?
    .run()
    .await
}
