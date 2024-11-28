/*
 * services/blob/service.rs
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
use crate::constants::SYSTEM_USER_ID;
use crate::hash::slice_to_blob_hash;
use crate::models::blob_blacklist::{
    self, Entity as BlobBlacklist, Model as BlobBlacklistModel,
};
use crate::models::blob_pending::{
    self, Entity as BlobPending, Model as BlobPendingModel,
};
use crate::models::file::{self, Entity as File, Model as FileModel};
use crate::models::file_revision::{
    self, Entity as FileRevision, Model as FileRevisionModel,
};
use crate::models::page::{self, Entity as Page, Model as PageModel};
use crate::models::site::{self, Entity as Site, Model as SiteModel};
use crate::models::user::{self, Entity as User, Model as UserModel};
use crate::services::file::{DeleteFile, FileService};
use crate::utils::assert_is_csprng;
use bytes::Bytes;
use cuid2::cuid;
use futures::TryStreamExt;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use s3::request::request_trait::ResponseData;
use s3::serde_types::HeadObjectResult;
use sea_orm::{
    prelude::*, DatabaseBackend, FromQueryResult, Statement, StreamTrait,
    TransactionTrait, UpdateResult,
};
use sea_query::value::ArrayType;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str;
use std::sync::Arc;
use time::format_description::well_known::Rfc2822;
use time::{Duration, OffsetDateTime};

/// How many samples to provide when providing hard deletion stats.
const SAMPLE_COUNT: u16 = 10;

/// Hash for empty blobs.
///
/// Even though it is not the SHA-512 hash, for simplicity we treat the hash
/// value with all zeroes to be the blob address for the empty blob.
/// This empty blob is not actually stored in S3 but instead is a "virtual blob",
/// considered to have always been present in `BlobService`.
pub const EMPTY_BLOB_HASH: BlobHash = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

/// MIME type for empty blobs.
pub const EMPTY_BLOB_MIME: &str = "inode/x-empty; charset=binary";

/// Created UNIX timestamp for empty blobs.
///
/// Timestamp is 2019/01/18 at midnight, the date of the first Wikijump commit.
pub const EMPTY_BLOB_TIMESTAMP: i64 = 1547769600;

/// The subdirectory in the S3 bucket where all pending uploads are kept.
pub const PRESIGN_DIRECTORY: &str = "uploads";

#[derive(Debug)]
pub struct BlobService;

impl BlobService {
    // File-related operations

    /// Creates an S3 presign URL to allow an end user to upload a blob.
    /// This is the start to the upload process for any kind of file.
    ///
    /// # Returns
    /// The generated presign URL, which can be uploaded to.
    pub async fn start_upload(
        ctx: &ServiceContext<'_>,
        StartBlobUpload { user_id, blob_size }: StartBlobUpload,
    ) -> Result<StartBlobUploadOutput> {
        info!("Creating upload by {user_id} with promised length {blob_size}");
        let config = ctx.config();
        let txn = ctx.transaction();

        // Convert expected length integer type, then check it
        let blob_size = i64::try_from(blob_size).map_err(|_| Error::BlobTooBig)?;
        if blob_size > config.maximum_blob_size {
            error!(
                "Blob proposed to upload is too big ({} > {})",
                blob_size, config.maximum_blob_size,
            );

            return Err(Error::BlobTooBig);
        }

        // Generate primary key and random S3 path
        let pending_blob_id = cuid();
        let s3_path = {
            let mut path = format!("{PRESIGN_DIRECTORY}/");

            {
                let mut rng = thread_rng();
                assert_is_csprng(&rng);
                Alphanumeric.append_string(
                    &mut rng,
                    &mut path,
                    config.presigned_path_length,
                );
            }

            path
        };

        info!("Creating presign upload URL for blob at path {s3_path} with primary key {pending_blob_id}");

        // Create presign URL
        let bucket = ctx.s3_bucket();
        let presign_url = bucket
            .presign_put(&s3_path, config.presigned_expiry_secs, None, None)
            .await?;

        // Get timestamps
        let created_at = now();
        let expires_at = created_at
            .checked_add(Duration::seconds(i64::from(config.presigned_expiry_secs)))
            .expect("getting expiration timestamp overflowed");

        // Add pending blob entry
        let model = blob_pending::ActiveModel {
            external_id: Set(pending_blob_id),
            expected_length: Set(blob_size),
            s3_path: Set(s3_path),
            presign_url: Set(presign_url),
            created_by: Set(user_id),
            created_at: Set(created_at),
            expires_at: Set(expires_at),
            ..Default::default()
        };

        let BlobPendingModel {
            external_id: pending_blob_id,
            presign_url,
            ..
        } = model.insert(txn).await?;

        debug!("New presign upload URL will last until {expires_at}");

        Ok(StartBlobUploadOutput {
            pending_blob_id,
            presign_url,
            expires_at,
        })
    }

    async fn get_pending_blob_path(
        ctx: &ServiceContext<'_>,
        user_id: i64,
        pending_blob_id: &str,
    ) -> Result<PendingBlob> {
        let txn = ctx.transaction();
        let row = BlobPending::find_by_id(pending_blob_id).one(txn).await?;
        let BlobPendingModel {
            s3_path,
            s3_hash,
            created_by,
            expected_length,
            ..
        } = match row {
            Some(pending) => pending,
            None => return Err(Error::BlobNotFound),
        };

        if user_id != created_by {
            error!("User mismatch, user ID {user_id} is attempting to use blob uploaded by {created_by}");
            return Err(Error::BlobWrongUser);
        }

        Ok(PendingBlob {
            s3_path,
            expected_length,
            moved_hash: s3_hash,
        })
    }

    pub async fn cancel_upload(
        ctx: &ServiceContext<'_>,
        user_id: i64,
        pending_blob_id: &str,
    ) -> Result<()> {
        info!("Cancelling upload for blob for pending ID {pending_blob_id}");
        let txn = ctx.transaction();
        let PendingBlob { s3_path, .. } =
            Self::get_pending_blob_path(ctx, user_id, pending_blob_id).await?;

        BlobPending::delete_by_id(pending_blob_id).exec(txn).await?;

        if Self::head(ctx, &s3_path).await?.is_some() {
            let bucket = ctx.s3_bucket();
            bucket.delete_object(&s3_path).await?;
        }

        Ok(())
    }

    /// Helper function to do the actual "move" step of blob finalization.
    /// This is where, after uploading to the presign URL, the S3 object is
    /// then moved to its permanent location with a hashed name.
    ///
    /// NOTE: Because S3 changes cannot be rolled back on error, we are
    ///       creating a separate transaction here so that `blob_pending`
    ///       changes are persistent even if the outer request fails.
    async fn move_uploaded(
        ctx: &ServiceContext<'_>,
        pending_blob_id: &str,
        s3_path: &str,
        expected_length: usize,
    ) -> Result<FinalizeBlobUploadOutput> {
        let state = ctx.state();
        let db_state = Arc::clone(&state);

        // Produce temporary context in a new transaction
        let txn = db_state.database.begin().await?;
        let inner_ctx = ServiceContext::new(&state, &txn);
        let result = Self::move_uploaded_inner(
            &inner_ctx,
            pending_blob_id,
            s3_path,
            expected_length,
        )
        .await;

        // Commit separate transaction, recording a move (if it occurred)
        txn.commit().await?;
        result
    }

    async fn move_uploaded_inner(
        ctx: &ServiceContext<'_>,
        pending_blob_id: &str,
        s3_path: &str,
        expected_length: usize,
    ) -> Result<FinalizeBlobUploadOutput> {
        let bucket = ctx.s3_bucket();
        let txn = ctx.transaction();

        debug!("Download uploaded blob from S3 uploads to get metadata");
        let response = bucket.get_object(s3_path).await?;
        let data: Vec<u8> = match response.status_code() {
            200 => response.into(),
            404 => {
                error!("No blob uploaded at presign path {s3_path}");
                return Err(Error::BlobNotUploaded);
            }
            _ => {
                error!("Unable to retrieve uploaded blob at {s3_path} from S3");
                let error = s3_error(&response, "finalizing uploaded blob")?;
                return Err(error);
            }
        };

        if expected_length != data.len() {
            error!(
                "Expected blob length of {} bytes, instead found {} uploaded. Deleting pending.",
                expected_length,
                data.len(),
            );
            bucket.delete_object(&s3_path).await?;
            return Err(Error::BlobSizeMismatch);
        }

        // Special handling for empty blobs
        if data.is_empty() {
            debug!("File being created is empty, special case");
            return Ok(FinalizeBlobUploadOutput {
                s3_hash: EMPTY_BLOB_HASH,
                mime: str!(EMPTY_BLOB_MIME),
                size: 0,
                created: false,
            });
        }

        debug!("Updating blob metadata in database and S3");

        // If the blob exists, then just delete the uploaded one.
        //
        // If it doesn't, then we need to move it. However, within S3
        // we cannot "move" objects, we have to upload and delete the original.
        //
        // In either case, we delete the blob at the temporary upload location.

        let result = Self::direct_upload(ctx, data).await?;
        bucket.delete_object(&s3_path).await?;

        // Update pending blob with hash
        let model = blob_pending::ActiveModel {
            external_id: Set(str!(pending_blob_id)),
            s3_hash: Set(Some(result.s3_hash.to_vec())),
            ..Default::default()
        };
        model.update(txn).await?;

        // Return
        Ok(result)
    }

    /// Takes a blob and uploads it to its final destination in S3.
    ///
    /// This is used in the above `move_uploaded_inner()` method to
    /// "move" the S3 blob. This is done by uploading to the final
    /// destination, then afterwards, deleting the blob at the temporary
    /// upload location.
    pub(crate) async fn direct_upload(
        ctx: &ServiceContext<'_>,
        data: Vec<u8>,
    ) -> Result<FinalizeBlobUploadOutput> {
        let bucket = ctx.s3_bucket();

        // Get hash for blob
        let s3_hash = sha512_hash(&data);
        let hex_hash = blob_hash_to_hex(&s3_hash);

        // Convert size to correct integer type
        let size: i64 = data.len().try_into().expect("Buffer size exceeds i64");

        match Self::head(ctx, &hex_hash).await? {
            Some(result) => {
                debug!("Blob with hash {hex_hash} already exists");

                // TODO: Should we ever update the mime type?
                //       In case of changing file formats, etc.

                // Content-Type header should be returned
                let mime = result.content_type.ok_or(Error::S3Response)?;

                Ok(FinalizeBlobUploadOutput {
                    s3_hash,
                    mime,
                    size,
                    created: false,
                })
            }
            None => {
                debug!("Blob with hash {hex_hash} to be created");

                // Determine MIME type for the new blob
                let mime = ctx.mime().get_mime_type(data.clone()).await?;

                // Upload S3 object
                let response = bucket
                    .put_object_with_content_type(&hex_hash, &data, &mime)
                    .await?;

                // We assume all unexpected statuses are errors, even if 1XX or 2XX
                match response.status_code() {
                    200 => Ok(FinalizeBlobUploadOutput {
                        s3_hash,
                        mime,
                        size,
                        created: true,
                    }),
                    _ => s3_error(&response, "creating final S3 blob")?,
                }
            }
        }
    }

    pub async fn finish_upload(
        ctx: &ServiceContext<'_>,
        user_id: i64,
        pending_blob_id: &str,
    ) -> Result<FinalizeBlobUploadOutput> {
        info!("Finishing upload for blob for pending ID {pending_blob_id}");

        let PendingBlob {
            s3_path,
            expected_length,
            moved_hash,
        } = Self::get_pending_blob_path(ctx, user_id, pending_blob_id).await?;

        let output = match moved_hash {
            // Need to move from pending to main hash area
            None => {
                let expected_length = expected_length
                    .try_into()
                    .map_err(|_| Error::BlobSizeMismatch)?;

                Self::move_uploaded(ctx, pending_blob_id, &s3_path, expected_length)
                    .await?
            }

            // Already moved
            Some(hash_vec) => {
                let BlobMetadata { mime, size, .. } =
                    Self::get_metadata(ctx, &hash_vec).await?;

                debug_assert_eq!(expected_length, size);

                FinalizeBlobUploadOutput {
                    s3_hash: slice_to_blob_hash(&hash_vec),
                    mime,
                    size,
                    created: false,
                }
            }
        };

        // Return result based on blob status
        Ok(output)
    }

    // Hard-deletion operations

    /// Lists information about a blob which is being considered for hard deletion.
    /// This method does not mutate any data.
    pub async fn hard_delete_list(
        ctx: &ServiceContext<'_>,
        s3_hash: BlobHash,
    ) -> Result<HardDeleteOutput> {
        Self::hard_delete_inner(ctx, HardDeleteInner::DryRun { s3_hash }).await
    }

    /// Hard deletes the specified blob and all duplicates.
    ///
    /// This is a very powerful method and needs to be used carefully.
    /// It should only be accessible to platform staff.
    ///
    /// As opposed to normal soft deletions, this method will completely
    /// remove a file from Wikijump. The data will be entirely purged
    /// and the data will be replaced with the blank file.
    ///
    /// This method should only be used very rarely to clear content such
    /// as severe copyright violations, abuse content, or comply with court orders.
    pub async fn hard_delete_all(
        ctx: &ServiceContext<'_>,
        HardDelete { s3_hash, user_id }: HardDelete,
    ) -> Result<HardDeleteOutput> {
        let s3_hash = slice_to_blob_hash(s3_hash.as_ref());
        Self::hard_delete_inner(ctx, HardDeleteInner::Commit { s3_hash, user_id }).await
    }

    /// Inner implementation, which runs the hard deletion procedure but may not actually delete.
    ///
    /// By running the actual deletion system, we can verify that the predicted set to delete is in
    /// fact what will be deleted in a run.
    async fn hard_delete_inner(
        ctx: &ServiceContext<'_>,
        input: HardDeleteInner,
    ) -> Result<HardDeleteOutput> {
        let txn = ctx.transaction();
        let (s3_hash, deleter_user_id) = match input {
            HardDeleteInner::Commit { s3_hash, user_id } => (s3_hash, Some(user_id)),
            HardDeleteInner::DryRun { s3_hash } => (s3_hash, None),
        };
        // NOTE: Instead of an explicit "dry_run" variable, the value of "is real run"
        //       or "is dry run" is derived from whether "deleter_user_id" is None or not.
        //       If there's no user ID to record as responsible, it must necessarily
        //       be a dry run.

        if s3_hash == EMPTY_BLOB_HASH {
            error!("Cannot hard delete the empty blob");
            return Err(Error::BadRequest);
        }

        let mut revisions = SamplerCounter::new();
        let mut files = SamplerCounter::new();
        let mut pages = SamplerCounter::new();
        let mut sites = SamplerCounter::new();
        let mut total_files_deleted = 0;

        match deleter_user_id {
            None => info!(
                "Checking result of hard deletion of all blobs matching hash {}",
                blob_hash_to_hex(&s3_hash),
            ),
            Some(user_id) => {
                info!(
                    "Hard deleting all blobs matching hash {} (done by user ID {})",
                    blob_hash_to_hex(&s3_hash),
                    user_id,
                );
                // TODO add to audit log
            }
        }

        // Get all latest file revisions with this hash, which we then delete
        // so it's no longer the most recent revision (which we can't hide).

        let query = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            str!("
                SELECT
                    f.site_id AS site_id,
                    f.page_id AS page_id,
                    f.file_id AS file_id,
                    r1.revision_id AS revision_id
                FROM file AS f
                JOIN file_revision AS r1
                    ON f.file_id = r1.file_id
                LEFT OUTER JOIN file_revision AS r2
                    ON (f.file_id = r2.file_id AND r1.revision_number < r2.revision_number)
                WHERE r2.revision_id IS NULL
                AND r1.s3_hash = $1
            "),
            [Value::from(s3_hash.to_vec())],
        );

        let mut stream = txn.stream(query).await?;
        while let Some(row) = stream.try_next().await? {
            let site_id: i64 = row.try_get_by_index(0)?;
            let page_id: i64 = row.try_get_by_index(1)?;
            let file_id: i64 = row.try_get_by_index(2)?;
            let revision_id: i64 = row.try_get_by_index(3)?;

            total_files_deleted += 1;

            if deleter_user_id.is_some() {
                // Only do deletions when running for real
                FileService::delete_with_erased_s3_hash(
                    ctx,
                    DeleteFile {
                        site_id,
                        page_id,
                        file: file_id.into(),
                        last_revision_id: revision_id,
                        revision_comments: format!(
                            "Hard delete {}",
                            blob_hash_to_hex(&s3_hash)
                        ),
                        user_id: SYSTEM_USER_ID,
                    },
                )
                .await?;
            }
        }

        // Go through all the revisions with the matching S3 hash and delete / hide it

        let mut stream = FileRevision::find()
            .filter(file_revision::Column::S3Hash.eq(s3_hash.as_slice()))
            .stream(txn)
            .await?;

        while let Some(rev) = stream.try_next().await? {
            revisions.add(rev.revision_id);
            files.add(rev.file_id);
            pages.add(rev.page_id);
            sites.add(rev.site_id);

            if deleter_user_id.is_some() {
                // Amend 'hidden' to add 's3_hash'
                let hidden = {
                    let column = str!("s3_hash"); // avoid double-allocating String
                    let mut hidden = rev.hidden;
                    if !hidden.contains(&column) {
                        hidden.push(column);
                        hidden.sort();
                    }
                    hidden
                };

                // Run UPDATE
                let model = file_revision::ActiveModel {
                    revision_id: Set(rev.revision_id),
                    s3_hash: Set(EMPTY_BLOB_HASH.to_vec()),
                    hidden: Set(hidden),
                    ..Default::default()
                };

                model.update(txn).await?;
            }
        }

        // Update all users using this blob to remove this as a profile picture
        // (But first getting a set of sample records)

        let sample_user_ids: Vec<i64> = User::find()
            .select_only()
            .column(user::Column::UserId)
            .filter(user::Column::AvatarS3Hash.eq(s3_hash.as_slice()))
            .limit(u64::from(SAMPLE_COUNT))
            .into_tuple()
            .all(txn)
            .await?;

        let total_users = match deleter_user_id {
            Some(_) => {
                // Mutate, update all users and get count
                let model = user::ActiveModel {
                    avatar_s3_hash: Set(None),
                    ..Default::default()
                };

                User::update_many()
                    .set(model)
                    .filter(user::Column::AvatarS3Hash.eq(s3_hash.as_slice()))
                    .exec(txn)
                    .await?
                    .rows_affected
            }
            None => {
                // Read-only, just get the count
                User::find()
                    .select_only()
                    .column_as(user::Column::UserId.count(), "count")
                    .filter(user::Column::AvatarS3Hash.eq(s3_hash.as_slice()))
                    .into_tuple()
                    .one(txn)
                    .await?
                    .expect("No results from COUNT aggregate query")
            }
        };

        if let Some(user_id) = deleter_user_id {
            // Delete and blacklist the hash, nobody should be uploading new versions
            // Only do so if we are actually mutating.
            try_join!(
                BlobService::add_blacklist(ctx, s3_hash, user_id),
                BlobService::hard_delete(ctx, &s3_hash),
            )?;
        }

        // Finish counting and sampling
        let (total_revisions, sample_revision_ids) = revisions.finish();
        let (total_files, sample_file_ids) = files.finish();
        let (total_pages, sample_page_ids) = pages.finish();
        let (total_sites, sample_site_ids) = sites.finish();

        Ok(HardDeleteOutput {
            total_revisions,
            total_files,
            total_files_deleted,
            total_pages,
            total_sites,
            total_users,
            sample_revision_ids,
            sample_file_ids,
            sample_page_ids,
            sample_site_ids,
            sample_user_ids,
        })
    }

    // Blacklist operations

    pub async fn add_blacklist(
        ctx: &ServiceContext<'_>,
        hash: BlobHash,
        created_by: i64,
    ) -> Result<()> {
        info!("Adding hash {} to blacklist", blob_hash_to_hex(&hash));

        if Self::on_blacklist(ctx, hash).await? {
            debug!("Already blacklisted, skipping");
            return Ok(());
        }

        let txn = ctx.transaction();
        let model = blob_blacklist::ActiveModel {
            s3_hash: Set(hash.to_vec()),
            created_by: Set(created_by),
            ..Default::default()
        };
        model.insert(txn).await?;
        Ok(())
    }

    pub async fn remove_blacklist(
        ctx: &ServiceContext<'_>,
        hash: BlobHash,
    ) -> Result<()> {
        info!("Removing hash {} to blacklist", blob_hash_to_hex(&hash));
        let txn = ctx.transaction();
        BlobBlacklist::delete_by_id(hash.to_vec()).exec(txn).await?;
        Ok(())
    }

    pub async fn on_blacklist(ctx: &ServiceContext<'_>, hash: BlobHash) -> Result<bool> {
        info!(
            "Checking if hash {} is on blacklist",
            blob_hash_to_hex(&hash),
        );

        let txn = ctx.transaction();
        let exists = BlobBlacklist::find()
            .filter(blob_blacklist::Column::S3Hash.eq(hash.as_slice()))
            .one(txn)
            .await?
            .is_some();

        Ok(exists)
    }

    // Getters

    pub async fn get_optional(
        ctx: &ServiceContext<'_>,
        hash: &[u8],
    ) -> Result<Option<Vec<u8>>> {
        // Special handling for empty blobs
        if hash == EMPTY_BLOB_HASH {
            debug!("Returning the empty blob");
            return Ok(Some(Vec::new()));
        }

        // Retrieve blob from S3
        let bucket = ctx.s3_bucket();
        let hex_hash = blob_hash_to_hex(hash);
        let response = bucket.get_object(&hex_hash).await?;
        match response.status_code() {
            200 => Ok(Some(response.into())),
            404 => Ok(None),
            _ => s3_error(&response, "fetching S3 blob"),
        }
    }

    #[inline]
    pub async fn get(ctx: &ServiceContext<'_>, hash: &[u8]) -> Result<Vec<u8>> {
        find_or_error!(Self::get_optional(ctx, hash), Blob)
    }

    pub async fn get_metadata_optional(
        ctx: &ServiceContext<'_>,
        hash: &[u8],
    ) -> Result<Option<BlobMetadata>> {
        // Special handling for empty blobs
        if hash == EMPTY_BLOB_HASH {
            return Ok(Some(BlobMetadata {
                mime: str!(EMPTY_BLOB_MIME),
                size: 0,
                created_at: OffsetDateTime::from_unix_timestamp(EMPTY_BLOB_TIMESTAMP)
                    .unwrap(),
            }));
        }

        // Retrieve metadata from S3
        let hex_hash = blob_hash_to_hex(hash);
        match Self::head(ctx, &hex_hash).await? {
            None => Ok(None),
            Some(result) => {
                // Headers should be passed in
                let size = result.content_length.ok_or(Error::S3Response)?;
                let mime = result.content_type.ok_or(Error::S3Response)?;
                let created_at = {
                    let timestamp = result.last_modified.ok_or(Error::S3Response)?;

                    OffsetDateTime::parse(&timestamp, &Rfc2822)
                        .map_err(|_| Error::S3Response)?
                };

                Ok(Some(BlobMetadata {
                    mime,
                    size,
                    created_at,
                }))
            }
        }
    }

    #[inline]
    pub async fn get_metadata(
        ctx: &ServiceContext<'_>,
        hash: &[u8],
    ) -> Result<BlobMetadata> {
        find_or_error!(Self::get_metadata_optional(ctx, hash), Blob)
    }

    #[allow(dead_code)] // TEMP
    pub async fn exists(ctx: &ServiceContext<'_>, hash: &[u8]) -> Result<bool> {
        // Special handling for the empty blob
        if hash == EMPTY_BLOB_HASH {
            debug!("Checking existence of the empty blob");
            return Ok(true);
        }

        // Fetch existence from S3
        let hex_hash = blob_hash_to_hex(hash);
        let result = Self::head(ctx, &hex_hash).await?;
        Ok(result.is_some())
    }

    /// Possibly retrieve blob contents, if a flag is set.
    ///
    /// This utility conditionally retrieves the
    /// text given by the specified hash only
    /// if the flag `should_fetch` is true.
    /// Otherwise, it does no action, returning `None`.
    pub async fn get_maybe(
        ctx: &ServiceContext<'_>,
        should_fetch: bool,
        hash: &[u8],
    ) -> Result<Option<Vec<u8>>> {
        if should_fetch {
            let data = Self::get(ctx, hash).await?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    async fn head(
        ctx: &ServiceContext<'_>,
        path: &str,
    ) -> Result<Option<HeadObjectResult>> {
        let bucket = ctx.s3_bucket();
        let (result, status) = bucket.head_object(path).await?;

        match status {
            200 | 204 => Ok(Some(result)),
            404 => Ok(None),
            _ => {
                let response = ResponseData::new(Bytes::new(), status, HashMap::new());
                s3_error(&response, "heading S3 blob")
            }
        }
    }

    pub async fn hard_delete(ctx: &ServiceContext<'_>, hash: &[u8]) -> Result<()> {
        // Special handling for empty blobs
        //
        // Being virtual, having always existed, they cannot be deleted.
        // So this is a no-op.
        if hash == EMPTY_BLOB_HASH {
            debug!("Ignoring attempt to hard delete the empty blob");
            return Ok(());
        }

        // Delete from S3
        let bucket = ctx.s3_bucket();
        let hex_hash = blob_hash_to_hex(hash);

        let response = bucket.delete_object(&hex_hash).await?;
        match response.status_code() {
            204 => Ok(()),
            _ => s3_error(&response, "hard-deleting S3 blob"),
        }
    }
}

/// Helper method to parse out an S3 error response and print the message (if any).
fn s3_error<T>(response: &ResponseData, action: &str) -> Result<T> {
    let error_message = match str::from_utf8(response.bytes()) {
        Ok("") => "(no content)",
        Ok(m) => m,
        Err(_) => "(invalid UTF-8)",
    };

    error!(
        "Error while {} (HTTP {}): {}",
        action,
        response.status_code(),
        error_message,
    );

    // TODO replace with S3 backend-specific error
    Err(Error::S3Response)
}

#[derive(Debug)]
enum HardDeleteInner {
    Commit { s3_hash: BlobHash, user_id: i64 },
    DryRun { s3_hash: BlobHash },
}

#[derive(Debug)]
struct PendingBlob {
    s3_path: String,
    expected_length: i64,
    moved_hash: Option<Vec<u8>>,
}

/// Helper struct to produce a count of items and a sample list.
#[derive(Debug)]
struct SamplerCounter<T: Hash + Ord + Eq> {
    items: HashSet<T>,
}

impl<T: Hash + Ord + Eq> SamplerCounter<T> {
    #[inline]
    fn new() -> Self {
        SamplerCounter {
            items: HashSet::new(),
        }
    }

    fn add(&mut self, item: T) {
        self.items.insert(item);
    }

    fn finish(self) -> (usize, Vec<T>) {
        let count = self.items.len();
        let samples = {
            let mut samples = self
                .items
                .into_iter()
                .take(usize::from(SAMPLE_COUNT))
                .collect::<Vec<_>>();
            samples.sort();
            samples
        };

        (count, samples)
    }
}
