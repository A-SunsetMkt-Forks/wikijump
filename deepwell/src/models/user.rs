//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use super::sea_orm_active_enums::UserType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i64,
    pub user_type: UserType,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<TimeDateTimeWithTimeZone>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<TimeDateTimeWithTimeZone>,
    pub from_wikidot: bool,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub slug: String,
    pub name_changes_left: i16,
    #[serde(with = "time::serde::rfc3339")]
    pub last_name_change_added_at: TimeDateTimeWithTimeZone,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_renamed_at: Option<TimeDateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    pub email_is_alias: Option<bool>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub email_verified_at: Option<TimeDateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub multi_factor_secret: Option<String>,
    pub multi_factor_recovery_codes: Option<Vec<String>>,
    pub locales: Vec<String>,
    #[sea_orm(column_type = "VarBinary(StringLen::None)", nullable)]
    pub avatar_s3_hash: Option<Vec<u8>>,
    #[sea_orm(column_type = "Text", nullable)]
    pub real_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub gender: Option<String>,
    pub birthday: Option<TimeDate>,
    #[sea_orm(column_type = "Text", nullable)]
    pub location: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub biography: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub user_page: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::alias::Entity")]
    Alias,
    #[sea_orm(has_many = "super::blob_pending::Entity")]
    BlobPending,
    #[sea_orm(has_many = "super::file_revision::Entity")]
    FileRevision,
    #[sea_orm(has_many = "super::message::Entity")]
    Message,
    #[sea_orm(has_many = "super::message_draft::Entity")]
    MessageDraft,
    #[sea_orm(has_many = "super::message_recipient::Entity")]
    MessageRecipient,
    #[sea_orm(has_many = "super::message_record::Entity")]
    MessageRecord,
    #[sea_orm(has_many = "super::page_attribution::Entity")]
    PageAttribution,
    #[sea_orm(has_many = "super::page_lock::Entity")]
    PageLock,
    #[sea_orm(has_many = "super::page_revision::Entity")]
    PageRevision,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
}

impl Related<super::alias::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Alias.def()
    }
}

impl Related<super::blob_pending::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BlobPending.def()
    }
}

impl Related<super::file_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileRevision.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl Related<super::message_draft::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageDraft.def()
    }
}

impl Related<super::message_recipient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageRecipient.def()
    }
}

impl Related<super::message_record::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageRecord.def()
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

impl Related<super::page_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageRevision.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
