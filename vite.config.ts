import path from "node:path";
import react from "@vitejs/plugin-react";
import { type CSSOptions, defineConfig, type HmrOptions } from "vite";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => {
    const hmrOptions: HmrOptions = {
        protocol: "ws",
        host: host,
        port: 1421,
    };

    const cssOptions: CSSOptions = {
        preprocessorOptions: {
            less: {
                math: "always",
                relativeUrls: true,
                javascriptEnabled: true,
            },
        },
    };

    return {
        // https://react.dev/learn/react-compiler
        plugins: [react()],
        css: cssOptions,

        resolve: {
            alias: {
                $page: path.resolve(__dirname, "./src/pages"),
                $base: path.resolve(__dirname, "./src/base"),
                $component: path.resolve(__dirname, "./src/components"),
                $bridge: path.resolve(__dirname, "./src/bridge"),
                $model: path.resolve(__dirname, "./src/models"),
                $core_css: path.resolve(__dirname, "./src/css"),
                $util: path.resolve(__dirname, "./src/utils"),
                $hook: path.resolve(__dirname, "./src/hooks"),
                $type: path.resolve(__dirname, "./src/types"),
            },
        },

        // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
        //
        // 1. prevent vite from obscuring rust errors
        clearScreen: false,
        // 2. tauri expects a fixed port, fail if that port is not available
        server: {
            port: 1420,
            strictPort: true,
            host: host || false,
            hmr: host ? hmrOptions : undefined,
            watch: {
                // 3. tell vite to ignore watching `src-tauri`
                ignored: ["**/src-tauri/**"],
            },
        },
    };
});
