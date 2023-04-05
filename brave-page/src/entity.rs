use sea_orm::prelude::Json;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromQueryResult)]
pub(crate) struct ArticlesInfo {
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) html_content: String,
    pub(crate) bg_img_url: String,
}
