import { defineConfig } from "vite";

export default defineConfig({
  build: {
    outDir: "public",
    emptyOutDir: false,
    manifest: true,
    rollupOptions: {
      input: {
        main: "src/resources/assets/main.ts",
        another: "src/resources/assets/another.ts",
      },
    },
  },
});
