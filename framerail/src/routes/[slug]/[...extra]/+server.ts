import { pageDelete } from "$lib/server/page/delete"
import { pageEdit } from "$lib/server/page/edit"
import { pageMove } from "$lib/server/page/move"

/**
 * Edit or create page.
 */
export async function POST(event) {
  let data = await event.request.formData()
  let slug = event.params.slug

  let pageIdVal = data.get("page-id")?.toString()
  let pageId = pageIdVal ? parseInt(pageIdVal) : null
  let siteId = parseInt(data.get("site-id")?.toString() ?? "1")
  let comments = data.get("comments")?.toString() ?? ""
  let wikitext = data.get("wikitext")?.toString()
  let title = data.get("title")?.toString()
  let altTitle = data.get("alt-title")?.toString()
  let tagsStr = data.get("tags")?.toString().trim()
  let tags: string[] = []
  if (tagsStr?.length) tags = tagsStr.split(" ").filter((tag) => tag.length)

  let res = await pageEdit(
    siteId,
    pageId,
    slug,
    comments,
    wikitext,
    title,
    altTitle,
    tags
  )

  return new Response(JSON.stringify(res))
}

/**
 * Delete page.
 */
export async function DELETE(event) {
  let data = await event.request.formData()
  let slug = event.params.slug

  let pageIdVal = data.get("page-id")?.toString()
  let pageId = pageIdVal ? parseInt(pageIdVal) : null
  let siteId = parseInt(data.get("site-id")?.toString() ?? "1")
  let comments = data.get("comments")?.toString() ?? ""

  let res = await pageDelete(siteId, pageId, slug, comments)
  return new Response(JSON.stringify(res))
}

/**
 * Move page to new slug.
 */
export async function PUT(event) {
  let data = await event.request.formData()
  let slug = event.params.slug

  let pageIdVal = data.get("page-id")?.toString()
  let pageId = pageIdVal ? parseInt(pageIdVal) : null
  let siteId = parseInt(data.get("site-id")?.toString() ?? "1")
  let comments = data.get("comments")?.toString() ?? ""
  let newSlug = data.get("new-slug")?.toString()

  let res = await pageMove(siteId, pageId, slug, newSlug, comments)

  return new Response(JSON.stringify(res))
}
