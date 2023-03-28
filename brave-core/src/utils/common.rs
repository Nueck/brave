use brave_config::GLOBAL_CONFIG;

//判断是否是无效的用户名
pub fn is_invalid_user_name(user: &str) -> bool {
    user == GLOBAL_CONFIG.interface.api_scope
        || user == GLOBAL_CONFIG.interface.admin_scope
        || user == GLOBAL_CONFIG.interface.blog_scope
}
