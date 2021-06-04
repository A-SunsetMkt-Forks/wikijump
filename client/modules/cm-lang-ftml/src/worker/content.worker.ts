import { Prism } from "wj-prism"
import { decode, expose, ModuleProxy, transfer } from "worker-module/src/worker-lib"

// -- MODULE

const module = {
  extract(raw: ArrayBuffer) {
    return transfer(extractContent(decode(raw)))
  },

  stats(raw: ArrayBuffer) {
    const str = decode(raw)
    const content = extractContent(str)
    const words = content.trim().split(/\s+/).length
    const bytes = raw.byteLength
    return { words, bytes }
  }
}

export type ContentModuleInterface = ModuleProxy<typeof module>

expose(module)

// -- FUNCTIONS

/**
 * Extracts the actual "content" of Wikitext using the Prism grammar as a
 * parser. Replaces all other markup with spaces in order to preserve a
 * mapping between the emitted string and the original document.
 *
 * @param str - The wikitext to extract the content out of.
 */
function extractContent(str: string) {
  const tokens = Prism.tokenize(str, Prism.languages.ftml)
  let output = ""
  for (const token of tokens) {
    if (typeof token === "string") output += token
    else output += " ".repeat(token.length)
  }
  return output
}
