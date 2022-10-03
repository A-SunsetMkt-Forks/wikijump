//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use super::sea_orm_active_enums::UserType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    #[sea_orm(column_type = "Text", unique)]
    pub slug: String,
    pub name_changes_left: i16,
    pub last_renamed_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", unique)]
    pub email: String,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
    pub user_type: UserType,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub multi_factor_secret: Option<String>,
    pub multi_factor_recovery_codes: Option<Json>,
    #[sea_orm(column_type = "Text")]
    pub locale: String,
    pub avatar_s3_hash: Option<Vec<u8>>,
    #[sea_orm(column_type = "Text", nullable)]
    pub display_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub real_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub gender: Option<String>,
    pub birthday: Option<Date>,
    #[sea_orm(column_type = "Text", nullable)]
    pub location: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub biography: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub user_page: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::page_revision::Entity")]
    PageRevision,
    #[sea_orm(has_many = "super::page_attribution::Entity")]
    PageAttribution,
    #[sea_orm(has_many = "super::page_lock::Entity")]
    PageLock,
    #[sea_orm(has_many = "super::file_revision::Entity")]
    FileRevision,
}

impl Related<super::page_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageRevision.def()
    }
}

impl Related<super::page_attribution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageAttribution.def()
    }
}

impl Related<super::page_lock::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageLock.def()
    }
}

impl Related<super::file_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileRevision.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
