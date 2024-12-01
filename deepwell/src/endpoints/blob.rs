/*
 * endpoints/blob.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2024 Wikijump Team
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

use super::prelude::*;
use crate::hash::slice_to_blob_hash;
use crate::services::blob::{
    BlobMetadata, CancelBlobUpload, GetBlobOutput, HardDelete, HardDeleteOutput,
    StartBlobUpload, StartBlobUploadOutput,
};
use crate::services::Result;
use crate::types::Bytes;

/// Temporary endpoint to get any blob by hash.
/// Primarily for user avatars, which have no other
/// way of getting the data at the moment.
pub async fn blob_get(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<GetBlobOutput> {
    info!("Getting blob for S3 hash");
    let hash: Bytes = params.parse()?;
    let data = BlobService::get(ctx, hash.as_ref()).await?;

    let BlobMetadata {
        mime,
        size,
        created_at,
    } = BlobService::get_metadata(ctx, hash.as_ref()).await?;

    Ok(GetBlobOutput {
        data,
        mime,
        size,
        created_at,
    })
}

/// Cancel a started upload by removing the pending blob.
pub async fn blob_cancel(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<()> {
    info!("Cancelling a pending blob upload");

    let CancelBlobUpload {
        user_id,
        pending_blob_id,
    } = params.parse()?;

    BlobService::cancel_upload(ctx, user_id, &pending_blob_id).await
}

/// Starts a new upload by creating a pending blob.
pub async fn blob_upload(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<StartBlobUploadOutput> {
    info!("Creating new pending blob upload");
    let input: StartBlobUpload = params.parse()?;
    BlobService::start_upload(ctx, input).await
}

pub async fn blob_blacklist_add(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<()> {
    #[derive(Deserialize, Debug)]
    struct AddBlacklist {
        s3_hash: Bytes<'static>,
        user_id: i64,
    }

    let AddBlacklist { s3_hash, user_id } = params.parse()?;
    let s3_hash = slice_to_blob_hash(s3_hash.as_ref());

    BlobService::check_hash_not_empty(s3_hash)?;
    BlobService::check_hash_in_use(ctx, s3_hash).await?;
    BlobService::add_blacklist(ctx, s3_hash, user_id).await
}

pub async fn blob_blacklist_remove(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<()> {
    #[derive(Deserialize, Debug)]
    struct RemoveBlacklist {
        s3_hash: Bytes<'static>,
    }

    let RemoveBlacklist { s3_hash } = params.parse()?;
    let s3_hash = slice_to_blob_hash(s3_hash.as_ref());
    BlobService::remove_blacklist(ctx, s3_hash).await
}

pub async fn blob_blacklist_check(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<bool> {
    #[derive(Deserialize, Debug)]
    struct HasBlacklist {
        s3_hash: Bytes<'static>,
    }

    let HasBlacklist { s3_hash } = params.parse()?;
    let s3_hash = slice_to_blob_hash(s3_hash.as_ref());
    BlobService::on_blacklist(ctx, s3_hash).await
}

pub async fn blob_hard_delete_preview(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<HardDeleteOutput> {
    #[derive(Deserialize, Debug)]
    struct HardDeletePreview {
        s3_hash: Bytes<'static>,
    }

    let HardDeletePreview { s3_hash } = params.parse()?;
    let s3_hash = slice_to_blob_hash(s3_hash.as_ref());
    BlobService::hard_delete_preview(ctx, s3_hash).await
}

pub async fn blob_hard_delete_confirm(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<HardDeleteOutput> {
    let input: HardDelete = params.parse()?;
    BlobService::hard_delete_all(ctx, input).await
}
