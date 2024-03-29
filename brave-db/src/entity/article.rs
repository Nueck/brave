//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "article")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub article_id: i64,
    pub user_id: i32,
    pub title: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    pub publish_time: DateTime,
    pub view_count: i64,
    pub messages_count: i64,
    pub messages_content: Option<Json>,
    pub tag: Vec<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub img_url: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub html_content: Option<String>,
    pub subtitle: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::UserId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
