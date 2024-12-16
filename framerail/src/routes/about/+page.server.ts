import { loadInfo } from "$lib/server/load/info"

export async function load({ request, cookies }) {
  return loadInfo(request, cookies)
}
