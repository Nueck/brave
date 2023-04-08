use serde::{Deserialize, Serialize};

///用户表数据
#[derive(Deserialize, Serialize)]
pub(crate) struct UserTableData {
    pub(crate) user_id: i32,
    pub(crate) user_name: String,
    pub(crate) authority: String,
    pub(crate) email: String,
    pub(crate) user_status: i16,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct TagData {
    pub(crate) tag: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct ArticleData {
    pub(crate) table_id: i64,
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) content: String,
    pub(crate) img_url: String,
    pub(crate) html_content: String,
}
