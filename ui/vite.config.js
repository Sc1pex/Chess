import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { defineConfig } from "vite";
import { comlink } from "vite-plugin-comlink";

export default defineConfig({
  plugins: [comlink(), wasm(), topLevelAwait()],
  worker: {
    plugins: () => [comlink(), wasm(), topLevelAwait()],
  },
});
