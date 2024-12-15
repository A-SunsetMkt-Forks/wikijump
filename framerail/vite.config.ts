import { sveltekit } from "@sveltejs/kit/vite"
import { execSync } from "child_process"
import type { UserConfig } from "vite"
import pkg from "./package.json"

let pnpmVersion = null
try {
  pnpmVersion = execSync("pnpm -v").toString("utf-8").trim()
} catch (_) {}

const config: UserConfig = {
  server: {
    host: "::",
    port: 3000,
    strictPort: true
  },
  plugins: [sveltekit()],
  define: {
    // also update $lib/vite-env.d.ts if these defines are changed
    serverInfo: {
      pnpmVersion,
      frontendName: pkg.name ?? null,
      frontendVersion: pkg.version ?? null,
      frontendDescription: pkg.description ?? null,
      frontendRepository: pkg.repository ?? null,
      frontendLicense: pkg.license ?? null
    }
  }
}

export default config
