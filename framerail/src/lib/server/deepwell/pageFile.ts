import defaults from "$lib/defaults"
import { client } from "$lib/server/deepwell"
import { startBlobUpload, uploadToPresignUrl } from "$lib/server/deepwell/file"
import type { Optional } from "$lib/types"

export async function pageFileList(
  siteId: number,
  pageId: number,
  deleted: Optional<boolean>
): Promise<object> {
  return client.request("page_get_files", {
    site_id: siteId,
    page_id: pageId,
    deleted
  })
}

export async function pageFileCreate(
  siteId: number,
  pageId: number,
  userId: number,
  name: Optional<string>,
  file: File,
  licensing: any,
  revisionComments: Optional<string>
) {
  let presign = await startBlobUpload(userId, file.size)
  await uploadToPresignUrl(presign.presign_url, file)

  return await client.request("file_create", {
    site_id: siteId,
    page_id: pageId,
    user_id: userId,
    name: name ?? file.name,
    licensing,
    uploaded_blob_id: presign.pending_blob_id,
    revision_comments: revisionComments
  })
}

export async function pageFileDelete(
  siteId: number,
  pageId: number,
  userId: number,
  fileId: string,
  lastRevisionId: number,
  revisionComments: Optional<string>
) {
  return await client.request("file_delete", {
    site_id: siteId,
    page_id: pageId,
    user_id: userId,
    file: fileId,
    last_revision_id: lastRevisionId,
    revision_comments: revisionComments
  })
}

export async function pageFileEdit(
  siteId: number,
  pageId: number,
  userId: number,
  fileId: string,
  name: string,
  file: Optional<File>,
  licensing: Optional<any>,
  lastRevisionId: number,
  revisionComments: Optional<string>
) {
  let presignId = undefined
  if (file && file instanceof File) {
    let presign = await startBlobUpload(userId, file.size)
    await uploadToPresignUrl(presign.presign_url, file)
    presignId = presign.pending_blob_id
  }

  return await client.request("file_edit", {
    site_id: siteId,
    page_id: pageId,
    user_id: userId,
    file_id: fileId,
    last_revision_id: lastRevisionId,
    name,
    licensing,
    uploaded_blob_id: presignId,
    revision_comments: revisionComments
  })
}

export async function pageFileMove(
  siteId: number,
  currentPageId: number,
  destinationPage: string | number,
  userId: number,
  fileId: string,
  lastRevisionId: number,
  name: Optional<string>,
  revisionComments: Optional<string>
) {
  return await client.request("file_move", {
    site_id: siteId,
    current_page_id: currentPageId,
    destination_page: destinationPage,
    user_id: userId,
    file_id: fileId,
    last_revision_id: lastRevisionId,
    name,
    revision_comments: revisionComments
  })
}
