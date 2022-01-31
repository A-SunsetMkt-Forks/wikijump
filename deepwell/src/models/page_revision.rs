//! SeaORM Entity. Generated by sea-orm-codegen 0.5.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "page_revision")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub revision_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub revision_number: i32,
    pub page_id: i64,
    pub site_id: i64,
    pub user_id: i64,
    pub wikitext_hash: Vec<u8>,
    pub compiled_hash: Vec<u8>,
    pub compiled_at: DateTimeWithTimeZone,
    #[sea_orm(column_type = "Text")]
    pub compiled_generator: String,
    pub compiled_outdated: bool,
    #[sea_orm(column_type = "Text")]
    pub comments: String,
    pub hidden: Json,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub alt_title: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub slug: String,
    pub tags: Json,
    pub metadata: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::text::Entity",
        from = "Column::CompiledHash",
        to = "super::text::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Text2,
    #[sea_orm(
        belongs_to = "super::page::Entity",
        from = "Column::PageId",
        to = "super::page::Column::PageId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Page,
    #[sea_orm(
        belongs_to = "super::site::Entity",
        from = "Column::SiteId",
        to = "super::site::Column::SiteId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Site,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
    #[sea_orm(
        belongs_to = "super::text::Entity",
        from = "Column::WikitextHash",
        to = "super::text::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Text1,
}

impl Related<super::page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Page.def()
    }
}

impl Related<super::site::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Site.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
