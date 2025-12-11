import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import path from "path";

export default defineConfig({
  root: path.resolve(__dirname, "web"),
  base: "/",
  plugins: [wasm(), topLevelAwait()],
  build: {
    outDir: "../dist",
    target: "es2022",
    emptyOutDir: true,
  },
  server: {
    fs: {
      allow: [path.resolve(__dirname)],
    },
  },
  resolve: {
    alias: {
      "@pkg": path.resolve(__dirname, "pkg"),
    },
  },
});
