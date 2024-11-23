import { client } from "$lib/server/deepwell"
import type { Optional } from "$lib/types"
import { startBlobUpload, uploadToPresignUrl } from "./file"

export async function userView(
  domain: string,
  locales: string[],
  sessionToken: Optional<string>,
  username?: string
): Promise<object> {
  return client.request("user_view", {
    domain,
    session_token: sessionToken,
    locales,
    user: username
  })
}

export async function userEdit(
  userId: number,
  params: Record<string, any>
): Promise<object> {
  let data: Record<string, any> = {}
  if (params.name !== undefined && typeof params.name === "string")
    data.name = params.name
  if (params.email !== undefined && typeof params.email === "string")
    data.email = params.email
  if (params.real_name !== undefined && typeof params.real_name === "string") {
    if (params.real_name) data.real_name = params.real_name
    else data.real_name = null
  }
  if (params.gender !== undefined && typeof params.gender === "string") {
    if (params.gender) data.gender = params.gender
    else data.gender = null
  }
  if (params.birthday !== undefined && typeof params.birthday === "string") {
    if (isNaN(Date.parse(params.birthday))) data.birthday = null
    else data.birthday = params.birthday
  }
  if (params.location !== undefined && typeof params.location === "string") {
    if (params.location) data.location = params.location
    else data.location = null
  }
  if (params.biography !== undefined && typeof params.biography === "string") {
    if (params.biography) data.biography = params.biography
    else data.biography = null
  }
  if (params.user_page !== undefined && typeof params.user_page === "string") {
    if (params.user_page) data.user_page = params.user_page
    else data.user_page = null
  }
  if (Array.isArray(params.locales) && params.locales.every((v) => typeof v === "string"))
    data.locales = params.locales
  if (params.avatar instanceof File && params.avatar.type.startsWith("image/")) {
    let presign = await startBlobUpload(userId, params.avatar.size)
    await uploadToPresignUrl(presign.presign_url, params.avatar)
    data.avatar_uploaded_blob_id = presign.pending_blob_id
  } else if (params.avatar !== undefined && params.avatar === null) data.avatar = null

  return client.request("user_edit", {
    user: userId,
    ...data
  })
}
