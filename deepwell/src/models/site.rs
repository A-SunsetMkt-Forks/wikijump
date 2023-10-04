//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "site")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub site_id: i64,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: Option<TimeDateTimeWithTimeZone>,
    pub deleted_at: Option<TimeDateTimeWithTimeZone>,
    pub from_wikidot: bool,
    #[sea_orm(column_type = "Text")]
    pub slug: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub tagline: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub locale: String,
    #[sea_orm(column_type = "Text")]
    pub default_page: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub custom_domain: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::filter::Entity")]
    Filter,
    #[sea_orm(has_many = "super::page::Entity")]
    Page,
    #[sea_orm(has_many = "super::page_category::Entity")]
    PageCategory,
    #[sea_orm(has_many = "super::page_revision::Entity")]
    PageRevision,
    #[sea_orm(
        belongs_to = "super::site_domain::Entity",
        from = "Column::CustomDomain",
        to = "super::site_domain::Column::Domain",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SiteDomain,
}

impl Related<super::filter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Filter.def()
    }
}

impl Related<super::page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Page.def()
    }
}

impl Related<super::page_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageCategory.def()
    }
}

impl Related<super::page_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageRevision.def()
    }
}

impl Related<super::site_domain::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SiteDomain.def()
    }
}

impl Related<super::site_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SiteMember.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
