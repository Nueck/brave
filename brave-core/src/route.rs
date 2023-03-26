use brave_config::GLOBAL_CONFIG;

// 判断是否是有效的路由
pub fn is_valid_route(path: &str) -> bool {
    path == format!("/")
        || path == format!("/{}/", &GLOBAL_CONFIG.interface.blog_scope)
        || path == format!("/{}/", &GLOBAL_CONFIG.interface.admin_scope)
        || path == format!("/{}/", "css")
        || path == format!("/{}/", "js")
        || path == format!("/{}/", "img")
}
