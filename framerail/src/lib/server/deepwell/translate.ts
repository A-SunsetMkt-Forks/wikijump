import { client } from "$lib/server/deepwell/index.ts"
import type { TranslateKeys, TranslatedKeys } from "$lib/types"

export async function translate(
  locales: string[],
  keys: TranslateKeys
): Promise<TranslatedKeys> {
  return client.request("translate", {
    locales,
    messages: keys
  })
}
