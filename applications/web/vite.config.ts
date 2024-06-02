import {defineConfig} from "vitest/config"
import {svelte} from "@sveltejs/vite-plugin-svelte"
import wasm from "vite-plugin-wasm"
import topLevelAwait from "vite-plugin-top-level-await"
import {base} from "./src/base"

export default defineConfig({
  base,
  plugins: [svelte(), wasm(), topLevelAwait()],
  build: {
    outDir: "dist",
    target: "esnext",
  },
  test: {
    include: ["src/**/*.{test,spec}.{js,ts}", "../../packages/shared/**/*.{test,spec}.{js,ts}"],
    watch: false,
  },
  server: {
    strictPort: true,
    host: "127.0.0.1",
    port: 5173,
    fs: {
      // Allow serving files from one level up to the project root
      // Alows vite dev server to access packages
      allow: ["../.."],
    },
  },
})
