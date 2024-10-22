//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::PageRevisionType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "page_revision")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub revision_id: i64,
    pub revision_type: PageRevisionType,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<TimeDateTimeWithTimeZone>,
    pub revision_number: i32,
    pub page_id: i64,
    pub site_id: i64,
    pub user_id: i64,
    pub from_wikidot: bool,
    pub changes: Vec<String>,
    #[sea_orm(column_type = "VarBinary(StringLen::None)")]
    pub wikitext_hash: Vec<u8>,
    #[sea_orm(column_type = "VarBinary(StringLen::None)")]
    pub compiled_hash: Vec<u8>,
    #[serde(with = "time::serde::rfc3339")]
    pub compiled_at: TimeDateTimeWithTimeZone,
    #[sea_orm(column_type = "Text")]
    pub compiled_generator: String,
    #[sea_orm(column_type = "Text")]
    pub comments: String,
    pub hidden: Vec<String>,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub alt_title: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub slug: String,
    pub tags: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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
        belongs_to = "super::text::Entity",
        from = "Column::CompiledHash",
        to = "super::text::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Text2,
    #[sea_orm(
        belongs_to = "super::text::Entity",
        from = "Column::WikitextHash",
        to = "super::text::Column::Hash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Text1,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
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

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
