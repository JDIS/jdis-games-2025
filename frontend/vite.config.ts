import tailwindcss from "@tailwindcss/vite";
import { tanstackStart } from "@tanstack/react-start/plugin/vite";
import { defineConfig } from "vite";
import tsConfigPaths from "vite-tsconfig-paths";

export default defineConfig({
    server: {
        proxy: {
            "/ws": {
                target: "ws://localhost:32945",
                ws: true,
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/ws/, ""),
            },
        },
    },
    plugins: [
        tsConfigPaths({
            projects: ["./tsconfig.json"],
        }),
        tailwindcss(),
        tanstackStart({
            // https://react.dev/learn/react-compiler
            react: {
                babel: {
                    plugins: [
                        [
                            "babel-plugin-react-compiler",
                            {
                                target: "19",
                            },
                        ],
                    ],
                },
            },

            tsr: {
                quoteStyle: "double",
                semicolons: true,
                // verboseFileRoutes: false,
            },

            target: "bun",
        }),
    ],
});
