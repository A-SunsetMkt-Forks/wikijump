/*
 * services/interaction/mod.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2023 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! A service to manage "interactions", which are relationships between entities.
//!
//! An "interaction" is a pair of two IDs and the interaction type. This is a flexible
//! system first designed by bluesoul to describe a large number of relations between objects.
//! For instance, instead of creating a separate table for user blocks or site membership,
//! we can instead define a number of relations using the interaction system.
//!
//! For example:
//! * `site` / `member` / `user` &mdash; User is a site member
//! * `user` / `block` / `user` &mdash; User has blocked another user

mod prelude {
    pub use super::super::prelude::*;
    pub use super::*;
    pub use crate::models::interaction::Model as InteractionModel;
    pub use crate::models::sea_orm_active_enums::InteractionObjectType;
    pub use paste::paste;
}

#[macro_use]
mod macros;

mod page_star;
mod page_watch;
mod site_ban;
mod site_member;
mod structs;
mod user_block;
mod user_contact;
mod user_follow;

pub use self::page_star::*;
pub use self::page_watch::*;
pub use self::site_ban::*;
pub use self::site_member::*;
pub use self::structs::*;
pub use self::user_block::*;
pub use self::user_contact::*;
pub use self::user_follow::*;

use super::prelude::*;
use crate::models::interaction::{
    self, Entity as Interaction, Model as InteractionModel,
};
use serde::Serialize;

// Base service exists here.
//
// Methods and types per-interaction are in their respective submodules,
// and are mostly generated by macros.

#[derive(Debug)]
pub struct InteractionService;

impl InteractionService {
    pub async fn create<M: Serialize>(
        ctx: &ServiceContext<'_>,
        interaction_type: InteractionType,
        dest: InteractionObject,
        from: InteractionObject,
        created_by: i64,
        metadata: &M,
    ) -> Result<InteractionModel> {
        tide::log::debug!(
            "Create interaction for {dest:?} ← {interaction_type:?} ← {from:?}",
        );

        // Get previous interaction, if present
        let txn = ctx.transaction();
        if let Some(interaction) = Self::get_optional(
            ctx,
            InteractionReference::Relationship {
                interaction_type,
                dest,
                from,
            },
        )
        .await?
        {
            tide::log::debug!("Interaction already exists, marking old item overwritten");
            let model = interaction::ActiveModel {
                interaction_id: Set(interaction.interaction_id),
                overwritten_at: Set(Some(now())),
                overwritten_by: Set(Some(created_by)),
                ..Default::default()
            };

            model.update(txn).await?;
        }

        // Insert new interaction
        let (dest_type, dest_id) = dest.into();
        let (from_type, from_id) = from.into();
        interaction_type.types().check(dest_type, from_type);

        let metadata = serde_json::to_value(metadata)?;
        let model = interaction::ActiveModel {
            interaction_type: Set(str!(interaction_type.value())),
            dest_type: Set(dest_type),
            dest_id: Set(dest_id),
            from_type: Set(from_type),
            from_id: Set(from_id),
            metadata: Set(metadata),
            created_by: Set(created_by),
            ..Default::default()
        };

        let interaction = model.insert(txn).await?;
        Ok(interaction)
    }

    pub async fn remove(
        ctx: &ServiceContext<'_>,
        reference: InteractionReference,
        deleted_by: i64,
    ) -> Result<InteractionModel> {
        tide::log::debug!("Removing interaction for {reference:?}");

        let txn = ctx.transaction();
        let interaction_id = Self::get_id(ctx, reference).await?;
        let model = interaction::ActiveModel {
            interaction_id: Set(interaction_id),
            deleted_at: Set(Some(now())),
            deleted_by: Set(Some(deleted_by)),
            ..Default::default()
        };

        let output = model.update(txn).await?;
        Ok(output)
    }

    pub async fn get_optional(
        ctx: &ServiceContext<'_>,
        reference: InteractionReference,
    ) -> Result<Option<InteractionModel>> {
        tide::log::debug!("Getting interaction for {reference:?}");

        let txn = ctx.transaction();
        let interaction = Interaction::find()
            .filter(
                Condition::all()
                    .add(reference.condition())
                    .add(interaction::Column::DeletedAt.is_null()),
            )
            .one(txn)
            .await?;

        Ok(interaction)
    }

    /// Gets the interaction ID from a reference, looking up if necessary.
    pub async fn get_id(
        ctx: &ServiceContext<'_>,
        reference: InteractionReference,
    ) -> Result<i64> {
        match reference {
            InteractionReference::Id(interaction_id) => Ok(interaction_id),
            InteractionReference::Relationship { .. } => {
                let InteractionModel { interaction_id, .. } =
                    Self::get(ctx, reference).await?;

                Ok(interaction_id)
            }
        }
    }

    pub async fn get(
        ctx: &ServiceContext<'_>,
        reference: InteractionReference,
    ) -> Result<InteractionModel> {
        find_or_error!(Self::get_optional(ctx, reference), Interaction)
    }

    pub async fn exists(
        ctx: &ServiceContext<'_>,
        reference: InteractionReference,
    ) -> Result<bool> {
        Self::get_optional(ctx, reference)
            .await
            .map(|interaction| interaction.is_some())
    }

    // TODO paginate
    pub async fn get_history(
        ctx: &ServiceContext<'_>,
        interaction_type: InteractionType,
        dest: InteractionObject,
        from: InteractionObject,
    ) -> Result<Vec<InteractionModel>> {
        tide::log::info!(
            "Getting history of interactions for {dest:?} / {interaction_type:?} / {from:?}",
        );

        let txn = ctx.transaction();
        let interactions = Interaction::find()
            .filter(interaction_condition(interaction_type, dest, from))
            .order_by_asc(interaction::Column::CreatedAt)
            .all(txn)
            .await?;

        Ok(interactions)
    }

    // TODO paginate
    pub async fn get_entries(
        ctx: &ServiceContext<'_>,
        interaction_type: InteractionType,
        object: InteractionObject,
        direction: InteractionDirection,
    ) -> Result<Vec<InteractionModel>> {
        tide::log::info!(
            "Getting {direction:?} interactions for {object:?} / {interaction_type:?}",
        );

        let (object_type, object_id) = object.into();
        let (object_type_column, object_id_column) = match direction {
            InteractionDirection::Dest => {
                (interaction::Column::DestType, interaction::Column::DestId)
            }
            InteractionDirection::From => {
                (interaction::Column::FromType, interaction::Column::FromId)
            }
        };

        let txn = ctx.transaction();
        let interactions = Interaction::find()
            .filter(
                Condition::all()
                    .add(
                        interaction::Column::InteractionType.eq(interaction_type.value()),
                    )
                    .add(object_type_column.eq(object_type))
                    .add(object_id_column.eq(object_id)),
            )
            .order_by_asc(interaction::Column::CreatedAt)
            .all(txn)
            .await?;

        Ok(interactions)
    }
}
