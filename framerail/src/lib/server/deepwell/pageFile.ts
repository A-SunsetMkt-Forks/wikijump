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

