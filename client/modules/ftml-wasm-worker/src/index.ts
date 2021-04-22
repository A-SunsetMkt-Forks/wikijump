import { spawn, Thread, Worker, Transfer, ModuleThread } from "threads"
import { sleep } from "wj-util"
import type * as FTML from "ftml-wasm"
import type * as Binding from "ftml-wasm/vendor/ftml"

import workerRelativeURL from "./worker/ftml.worker.ts"
// kinda hacky, but since this gets marked as external,
// it gets to Vite with the ?url intact
import wasmRelativeURL from "ftml-wasm/vendor/ftml_bg.wasm?url"

const workerURL = new URL(workerRelativeURL, import.meta.url)
const wasmURL = new URL(wasmRelativeURL, import.meta.url).toString()

interface TypedArray extends ArrayBuffer {
  buffer: ArrayBufferLike
}
type TransferInput = string | ArrayBuffer | TypedArray

const decoder = new TextDecoder()
const encoder = new TextEncoder()

const transfer = (buffer: TransferInput) => {
  if (typeof buffer === "string") return Transfer(encoder.encode(buffer).buffer)
  if ("buffer" in buffer) return Transfer(buffer.buffer)
  if (buffer instanceof ArrayBuffer) return Transfer(buffer)
  throw new TypeError("Expected a string, ArrayBuffer, or typed array!")
}

const decode = (buffer: ArrayBuffer) => decoder.decode(buffer)

// -- WORKER MODULE

// TODO: refactor this out into its own module if it gets used enough

interface WorkerModuleOpts {
  persist?: boolean
  timeout?: number
  init?: AnyFunction
}

class WorkerModule {
  name: string
  url: URL
  worker!: ModuleThread

  private persist = false
  private timeout = 10000
  private init?: AnyFunction

  constructor(name: string, url: URL, opts?: WorkerModuleOpts) {
    this.name = name
    this.url = url
    if (opts) {
      this.persist = opts.persist ?? false
      this.timeout = opts.timeout ?? 10000
      this.init = opts.init
    }
  }

  private async _ready() {
    if (!this.worker) {
      this.worker = await spawn<ModuleThread>(
        new Worker(this.url as any, {
          name: this.name,
          credentials: "same-origin",
          type: "classic"
        })
      )
      if (this.init) await this.init()
    }
  }

  private async _terminate() {
    if (this.worker) await Thread.terminate(this.worker)
    this.worker = undefined as any
  }

  private async _restart() {
    await this._terminate()
    await this._ready()
  }

  async invoke<T>(fn: () => Promise<T>) {
    await this._ready()
    const result = this.timeout
      ? await Promise.race([fn(), sleep(this.timeout)])
      : await fn()
    if (result) {
      if (!this.persist) await this._terminate()
      return result
    } else {
      if (this.persist) await this._restart()
      else await this._terminate()
      throw new Error("Worker timed out!")
    }
  }
}

const module = new WorkerModule("ftml-wasm-worker", workerURL, {
  persist: true,
  init() {
    module.invoke(() => module.worker.init(wasmURL))
  }
})
const invoke = module.invoke.bind(module)

/** Returns FTML's (the crate) version. */
export async function version() {
  return decode(
    await invoke<ArrayBuffer>(() => module.worker.version())
  )
}

/** Preprocesses a string of wikitext.
 *  See `ftml/src/preproc/test.rs` for more information. */
export async function preprocess(str: string) {
  return decode(
    await invoke<ArrayBuffer>(() => module.worker.preprocess(transfer(str)))
  )
}

/** Tokenizes a string of wikitext. */
export async function tokenize(str: string) {
  type Return = Binding.IToken[]
  return await invoke<Return>(() => module.worker.tokenize(transfer(str)))
}

/** Parses a string of wikitext. This returns an AST and warnings list, not HTML.
 *  @see render */
export async function parse(str: string) {
  type Return = ReturnType<typeof FTML["parse"]>
  return await invoke<Return>(() => module.worker.parse(transfer(str)))
}

/** Renders a string of wikitext to HTML. */
export async function render(str: string) {
  const [htmlBuffer, styleBuffer] = await invoke<[ArrayBuffer, ArrayBuffer]>(() =>
    module.worker.render(transfer(str))
  )
  const html = decode(htmlBuffer)
  const style = decode(styleBuffer)
  return { html, style }
}

/** Renders a string of wikitext to text. */
export async function renderText(str: string) {
  return decode(
    await invoke<ArrayBuffer>(() => module.worker.renderText(transfer(str)))
  )
}

/** Renders a string of wikitext like the {@link renderHTML} function, but this
 *  function additionally returns every step in the rendering pipeline. */
export async function detailedRender(str: string) {
  type Return = ReturnType<typeof FTML["detailedRender"]>
  return await invoke<Return>(() => module.worker.detailedRender(transfer(str)))
}

/** Returns the list of warnings emitted when parsing the provided string. */
export async function warnings(str: string) {
  type Return = Binding.IParseWarning[]
  return await invoke<Return>(() => module.worker.warnings(transfer(str)))
}

/** Converts a string of wikitext into a pretty-printed list of tokens. */
export async function inspectTokens(str: string) {
  return decode(
    await invoke<ArrayBuffer>(() => module.worker.inspectTokens(transfer(str)))
  )
}
