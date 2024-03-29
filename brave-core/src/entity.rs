use sea_orm::prelude::DateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

///用户表数据
#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub(crate) struct UserTableData {
    pub(crate) user_id: i32,
    pub(crate) user_name: String,
    pub(crate) authority: String,
    pub(crate) email: String,
    pub(crate) user_status: i16,
    pub(crate) create_time: DateTime,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ChangePwdInfo {
    pub(crate) email: String,
    pub(crate) new_pwd: String,
    pub(crate) verify_code: String,
    pub(crate) code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TagData {
    pub(crate) tag: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ArchiveData {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) icon: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct ArticleData {
    pub(crate) table_id: i64,
    pub(crate) title: String,
    pub(crate) tag: Vec<String>,
    pub(crate) subtitle: String,
    pub(crate) content: String,
    pub(crate) img_url: String,
    pub(crate) html_content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct SkinPageData {
    pub(crate) title: String,
    pub(crate) img_url: String,
}
