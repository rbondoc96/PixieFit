import path from 'node:path';
import {undestructurePlugin} from 'babel-plugin-solid-undestructure';
import devtools from 'solid-devtools/vite';
import {defineConfig} from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
    build: {
        target: 'esnext',
    },
    css: {
        modules: {
            localsConvention: 'camelCase',
        },
    },
    optimizeDeps: {
        include: ['@kratos/common'],
    },
    plugins: [
        ...undestructurePlugin('ts'),
        devtools({
            locator: {
                targetIDE: 'vscode',
                jsxLocation: true,
            },
        }),
        solidPlugin(),
    ],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, 'src'),
        },
    },
    server: {
        port: 3000,
    },
});
