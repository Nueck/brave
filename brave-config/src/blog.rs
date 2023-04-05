use crate::GLOBAL_CONFIG;

//生成博客的文章的id
pub fn generate_blog_table(name: &str, table_id: &i64) -> String {
    format!(
        "http://{}:{}/{}/{}/page/{}",
        GLOBAL_CONFIG.interface.service_add,
        GLOBAL_CONFIG.interface.service_port,
        GLOBAL_CONFIG.interface.blog_scope,
        name,
        table_id,
    )
}

//获取博客的主界面
pub fn get_blog_home(name: &str) -> String {
    format!(
        "http://{}:{}/{}/{}/home",
        GLOBAL_CONFIG.interface.service_add,
        GLOBAL_CONFIG.interface.service_port,
        GLOBAL_CONFIG.interface.blog_scope,
        name,
    )
}

//获取博客的关于界面
pub fn get_blog_about(name: &str) -> String {
    format!(
        "http://{}:{}/{}/{}/about",
        GLOBAL_CONFIG.interface.service_add,
        GLOBAL_CONFIG.interface.service_port,
        GLOBAL_CONFIG.interface.blog_scope,
        name,
    )
}

//获取博客的联系
pub fn get_blog_contact(name: &str) -> String {
    format!(
        "http://{}:{}/{}/{}/contact",
        GLOBAL_CONFIG.interface.service_add,
        GLOBAL_CONFIG.interface.service_port,
        GLOBAL_CONFIG.interface.blog_scope,
        name,
    )
}

//获取博客的category
pub fn get_blog_category(name: &str) -> String {
    format!(
        "http://{}:{}/{}/{}/category",
        GLOBAL_CONFIG.interface.service_add,
        GLOBAL_CONFIG.interface.service_port,
        GLOBAL_CONFIG.interface.blog_scope,
        name,
    )
}

//获取博客的content
pub fn get_blog_content(name: &str) -> String {
    format!(
        "http://{}:{}/{}/{}/content",
        GLOBAL_CONFIG.interface.service_add,
        GLOBAL_CONFIG.interface.service_port,
        GLOBAL_CONFIG.interface.blog_scope,
        name,
    )
}

//获取博客的404
pub fn get_blog_error(name: &str) -> String {
    format!(
        "http://{}:{}/{}/{}/404",
        GLOBAL_CONFIG.interface.service_add,
        GLOBAL_CONFIG.interface.service_port,
        GLOBAL_CONFIG.interface.blog_scope,
        name,
    )
}
