use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromQueryResult)]
pub(crate) struct ArticlesInfo {
    pub(crate) title: Option<String>,
    pub(crate) subtitle: Option<String>,
    pub(crate) content: Option<String>,
    pub(crate) html_content: Option<String>,
    pub(crate) bg_img: Option<String>,
    pub(crate) url: Option<String>,
}
