//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use super::sea_orm_active_enums::FileRevisionType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "file_revision")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub revision_id: i64,
    pub revision_type: FileRevisionType,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: TimeDateTimeWithTimeZone,
    pub revision_number: i32,
    pub file_id: i64,
    pub page_id: i64,
    pub site_id: i64,
    pub user_id: i64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "VarBinary(StringLen::None)")]
    pub s3_hash: Vec<u8>,
    #[sea_orm(column_type = "Text")]
    pub mime_hint: String,
    pub size_hint: i64,
    pub licensing: Json,
    pub changes: Vec<String>,
    #[sea_orm(column_type = "Text")]
    pub comments: String,
    pub hidden: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file::Entity",
        from = "Column::FileId",
        to = "super::file::Column::FileId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    File,
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
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
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
