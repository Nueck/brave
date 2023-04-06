use crate::middleware::auth_middleware::JWTAuth;
#[allow(unused_imports)]
use crate::middleware::head_middleware::HeadCheck;
use crate::middleware::init_middleware::InitAuth;
use actix_cors::Cors;
#[allow(unused_imports)]
use actix_web::http;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use brave_config::interface::Interface;
use brave_config::{config_init, GLOBAL_CONFIG, GLOBAL_DATA};
use brave_data::data_config;
use brave_page::home_config;

#[actix_rt::main]
pub async fn web_start() -> std::io::Result<()> {
    //初始化日志
    super::log::init_log();
    /*配置初始化*/
    let states = config_init().await;

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
        // Api的跨域问题
        /*TODO:暂时所有源都可以通过,后期更改*/

        #[cfg(not(debug_assertions))]
        let cors = Cors::default()
            .allowed_origin(Interface::get_server_uri().as_str())
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        #[cfg(debug_assertions)]
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(states.clone())) //数据库
            .wrap(Logger::default()) //日志
            .configure(brave_page::admin::admin_config) //后台管理
            .service(
                web::scope(&GLOBAL_CONFIG.interface.api_scope)
                    .wrap(JWTAuth) //身份验证
                    .wrap(InitAuth) //初始化判断
                    .wrap(cors)
                    // .wrap(HeadCheck) //用于浏览器过滤
                    .configure(super::api::api_post_config), //api的日志
            ) //api配置
            .service(
                web::scope(&GLOBAL_CONFIG.interface.blog_scope) //博客方面的加载
                    .wrap(
                        Cors::default()
                            .allow_any_header()
                            .allowed_methods(vec!["GET"]) //只允许GET
                            .allow_any_origin() //允许任何来源
                            .max_age(3600),
                    )
                    .configure(brave_page::blog_config), //博客显示
            )
            //数据加载和上传的使用data为前缀的接口
            .service(
                web::scope(&GLOBAL_DATA.get_data_config().data.unwrap())
                    .wrap(
                        Cors::default()
                            .allow_any_header()
                            .allowed_methods(vec!["GET"]) //只允许GET
                            .allow_any_origin() //允许任何来源
                            .max_age(3600),
                    )
                    .configure(data_config),
            )
            .configure(home_config)
    })
    .bind(Interface::get_api_string())?
    .run()
    .await
}
